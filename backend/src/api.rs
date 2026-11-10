use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::domain::{compile_criteria, AcceptanceGraph, Evidence, MilestoneState};
use crate::executor;
use crate::integrations::github;
use crate::store;
use crate::AppState;

/// The receipt and milestone pages report the rail that actually handled the
/// payment, not the current global setting. A historical Kite receipt must not
/// look like a ledger payment after the demo is switched back to ledger mode.
fn engine_report_for(state: &AppState, payment: Option<&store::PaymentRow>) -> Value {
    let mut engines = state.cfg.engine_report();
    if let (Some(payment), Some(obj)) = (payment, engines.as_object_mut()) {
        obj.insert("payment_rail".into(), json!(payment.rail));
    }
    engines
}

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(json!({"error": self.message}))).into_response()
    }
}

type ApiResult<T> = Result<Json<T>, ApiError>;

fn internal(error: impl std::fmt::Display) -> ApiError {
    tracing::error!(%error, "api error");
    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
}

pub async fn health(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "ok": true,
        "service": "tranche",
        "version": env!("CARGO_PKG_VERSION"),
        "time": chrono::Utc::now().to_rfc3339(),
        "engines": state.cfg.engine_report(),
    }))
}

pub async fn programs(State(state): State<AppState>) -> ApiResult<Value> {
    let programs = store::list_programs(&state.pool).await.map_err(internal)?;
    Ok(Json(json!({"programs": programs})))
}

pub async fn milestones(State(state): State<AppState>) -> ApiResult<Value> {
    let milestones = store::list_milestones(&state.pool).await.map_err(internal)?;
    Ok(Json(json!({
        "milestones": milestones,
        "engines": state.cfg.engine_report(),
    })))
}

pub async fn milestone_detail(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Value> {
    let milestone = store::get_milestone(&state.pool, &id)
        .await
        .map_err(|_| ApiError::new(StatusCode::NOT_FOUND, "Milestone not found"))?;
    build_detail(&state, milestone).await.map(Json)
}

async fn build_detail(
    state: &AppState,
    milestone: store::MilestoneRow,
) -> Result<Value, ApiError> {
    let graph: AcceptanceGraph = serde_json::from_str(&milestone.graph_json)
        .unwrap_or_else(|_| compile_criteria(&milestone.criteria));
    let sample: Value = serde_json::from_str(&milestone.sample_json).unwrap_or(Value::Null);
    let submission = store::latest_submission(&state.pool, &milestone.id)
        .await
        .map_err(internal)?;

    let (submission_json, node_runs, verdict, payment, receipt_slug) = match &submission {
        Some(sub) => {
            let nodes = store::node_runs(&state.pool, &sub.id).await.map_err(internal)?;
            let verdict_row = store::verdict_for(&state.pool, &sub.id).await.map_err(internal)?;
            let payment_row = store::payment_for(&state.pool, &sub.id).await.map_err(internal)?;
            let receipt = store::receipt_for(&state.pool, &sub.id).await.map_err(internal)?;
            let evidence: Value =
                serde_json::from_str(&sub.evidence_json).unwrap_or(Value::Null);
            let verdict_json = verdict_row.map(|v| {
                let reasons: Value =
                    serde_json::from_str(&v.serv_reasons_json).unwrap_or(Value::Array(vec![]));
                json!({
                    "id": v.id,
                    "serv": {
                        "decision": v.serv_decision,
                        "score": v.serv_score,
                        "summary": v.serv_summary,
                        "reasons": reasons,
                        "engine": v.serv_engine,
                    },
                    "reppo": {
                        "eval_id": v.reppo_eval_id,
                        "status": v.reppo_status,
                        "score": v.reppo_score,
                        "decision": v.reppo_decision,
                        "confidence": v.reppo_confidence,
                        "critique": v.reppo_critique,
                        "mode": v.reppo_mode,
                    },
                    "final_decision": v.final_decision,
                    "assurance": v.assurance,
                    "created_at": v.created_at,
                })
            });
            (
                Some(json!({
                    "id": sub.id,
                    "builder": sub.builder,
                    "evidence": evidence,
                    "evidence_hash": sub.evidence_hash,
                    "created_at": sub.created_at,
                })),
                nodes,
                verdict_json,
                payment_row,
                receipt.map(|r| r.slug),
            )
        }
        None => (None, vec![], None, None, None),
    };

    let events = store::events_for(&state.pool, &milestone.id)
        .await
        .map_err(internal)?;
    let graph_summary = graph.summary();

    let all_submissions = store::submissions_for(&state.pool, &milestone.id)
        .await
        .map_err(internal)?;
    let mut history = Vec::with_capacity(all_submissions.len());
    for item in &all_submissions {
        if let Some(v) = store::verdict_for(&state.pool, &item.id).await.map_err(internal)? {
            history.push(json!({
                "submission_id": item.id,
                "created_at": item.created_at,
                "serv_decision": v.serv_decision,
                "serv_score": v.serv_score,
                "final_decision": v.final_decision,
            }));
        }
    }

    Ok(json!({
        "milestone": {
            "id": milestone.id,
            "program_id": milestone.program_id,
            "program_name": milestone.program_name,
            "title": milestone.title,
            "builder": milestone.builder,
            "criteria": milestone.criteria,
            "amount_cents": milestone.amount_cents,
            "ceiling_cents": milestone.ceiling_cents,
            "currency": milestone.currency,
            "state": milestone.state,
            "created_at": milestone.created_at,
            "updated_at": milestone.updated_at,
        },
        "graph": graph,
        "graph_summary": graph_summary,
        "sample": sample,
        "submission": submission_json,
        "history": history,
        "node_runs": node_runs,
        "verdict": verdict,
        "payment": payment,
        "receipt_slug": receipt_slug,
        "events": events,
        "engines": engine_report_for(&state, payment.as_ref()),
        "public_base_url": state.cfg.public_base_url,
        "policy": {
            "auto_release_ceiling_cents": milestone.ceiling_cents,
            "tranche_cents": milestone.amount_cents,
            "currency": milestone.currency,
        }
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateMilestone {
    pub title: String,
    pub builder: String,
    pub criteria: String,
    pub amount_cents: i64,
    pub ceiling_cents: i64,
    #[serde(default)]
    pub program_id: Option<String>,
}

pub async fn create_milestone(
    State(state): State<AppState>,
    Json(body): Json<CreateMilestone>,
) -> ApiResult<Value> {
    let title = body.title.trim();
    let criteria = body.criteria.trim();
    if title.len() < 3 || title.len() > 140 {        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Title must be 3–140 characters.",
        ));
    }
    if criteria.len() < 20 || criteria.len() > 4000 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Criteria must be 20–4000 characters.",
        ));
    }
    if body.amount_cents <= 0 || body.ceiling_cents <= 0 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Amount and ceiling must be positive.",
        ));
    }
    if body.amount_cents > 10_000_000 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "The demo accepts tranches up to 100,000 USDC.",
        ));
    }
    if body.ceiling_cents > body.amount_cents {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "The auto-release ceiling cannot exceed the tranche amount.",
        ));
    }

    let program_id = body
        .program_id
        .unwrap_or_else(|| "prog_serv02".to_string());
    if store::list_programs(&state.pool)
        .await
        .map_err(internal)?
        .iter()
        .all(|p| p.id != program_id)
    {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Unknown program.",
        ));
    }

    let graph = compile_criteria(criteria);
    let graph_json = serde_json::to_string(&graph).map_err(internal)?;
    let id = format!("ms_{}", uuid::Uuid::new_v4().simple());
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO milestones (id, program_id, title, builder, criteria, graph_json, amount_cents, ceiling_cents, currency, state, sample_json, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'USDC', 'open', '{}', ?, ?)",
    )
    .bind(&id)
    .bind(&program_id)
    .bind(title)
    .bind(body.builder.trim())
    .bind(criteria)
    .bind(graph_json)
    .bind(body.amount_cents)
    .bind(body.ceiling_cents)
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(internal)?;

    store::add_event(
        &state.pool,
        &id,
        "milestone_created",
        "Acceptance graph compiled and published before work was submitted.",
    )
    .await
    .map_err(internal)?;

    Ok(Json(json!({"id": id, "graph": graph})))
}

#[derive(Debug, Deserialize)]
pub struct PreviewGraph {
    pub criteria: String,
}

/// Compiles criteria into an acceptance graph without publishing it.
/// The operator reviews this graph before any work is submitted.
pub async fn preview_graph(Json(body): Json<PreviewGraph>) -> ApiResult<Value> {
    let criteria = body.criteria.trim();
    if criteria.len() < 20 || criteria.len() > 4000 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Criteria must be 20–4000 characters.",
        ));
    }
    let graph = compile_criteria(criteria);
    Ok(Json(json!({
        "graph": graph,
        "graph_summary": graph.summary(),
    })))
}

#[derive(Debug, Deserialize)]
pub struct SubmitEvidence {
    #[serde(default)]
    pub repo_url: String,
    #[serde(default)]
    pub demo_url: String,
    #[serde(default)]
    pub readme_url: String,
    #[serde(default)]
    pub claim: String,
}

pub async fn submit_evidence(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<SubmitEvidence>,
) -> ApiResult<Value> {
    let milestone = store::get_milestone(&state.pool, &id)
        .await
        .map_err(|_| ApiError::new(StatusCode::NOT_FOUND, "Milestone not found"))?;
    let state_value = MilestoneState::from_str(&milestone.state);
    if !state_value.accepts_submission() {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "This milestone is not accepting submissions right now.",
        ));
    }

    let evidence = Evidence {
        repo_url: body.repo_url.trim().to_string(),
        demo_url: body.demo_url.trim().to_string(),
        readme_url: body.readme_url.trim().to_string(),
        claim: body.claim.trim().to_string(),
    };
    ensure_submission_capacity(&state, &milestone.id).await?;
    if !evidence.is_complete() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "A repository URL, a demo URL, and a claim of at least 20 characters are required.",
        ));
    }
    if evidence.claim.len() > 4000 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "The claim is limited to 4,000 characters.",
        ));
    }
    if evidence.repo_url.len() > 2048 || evidence.demo_url.len() > 2048 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "URLs are limited to 2,048 characters.",
        ));
    }
    if github::parse_repo_url(&evidence.repo_url).is_none() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "The repository must be a public GitHub URL.",
        ));
    }
    crate::net::validate_url_shape(&evidence.demo_url)
        .map_err(|message| ApiError::new(StatusCode::BAD_REQUEST, message))?;
    if !evidence.readme_url.is_empty() {
        crate::net::validate_url_shape(&evidence.readme_url)
            .map_err(|message| ApiError::new(StatusCode::BAD_REQUEST, message))?;
    }

    let submission_id = insert_submission(&state, &milestone, &evidence)
        .await
        .map_err(internal)?;
    store::add_event(
        &state.pool,
        &milestone.id,
        "evidence_submitted",
        "Builder submitted evidence for verification.",
    )
    .await
    .map_err(internal)?;

    Ok(Json(json!({"submission_id": submission_id})))
}

pub async fn submit_sample(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Value> {
    let milestone = store::get_milestone(&state.pool, &id)
        .await
        .map_err(|_| ApiError::new(StatusCode::NOT_FOUND, "Milestone not found"))?;
    let state_value = MilestoneState::from_str(&milestone.state);
    if !state_value.accepts_submission() {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "This milestone is not accepting submissions right now.",
        ));
    }
    let evidence: Evidence = serde_json::from_str(&milestone.sample_json)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "No sample evidence is configured."))?;
    ensure_submission_capacity(&state, &milestone.id).await?;
    if !evidence.is_complete() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "The sample evidence is incomplete.",
        ));
    }
    let submission_id = insert_submission(&state, &milestone, &evidence)
        .await
        .map_err(internal)?;
    store::add_event(
        &state.pool,
        &milestone.id,
        "sample_submitted",
        "Seeded sample evidence submitted.",
    )
    .await
    .map_err(internal)?;
    start_verification(state, submission_id.clone(), milestone.id.clone()).await?;
    Ok(Json(json!({"submission_id": submission_id})))
}

async fn ensure_submission_capacity(state: &AppState, milestone_id: &str) -> Result<(), ApiError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM submissions WHERE milestone_id = ?")
        .bind(milestone_id)
        .fetch_one(&state.pool)
        .await
        .map_err(internal)?;
    if count >= 20 {
        return Err(ApiError::new(
            StatusCode::TOO_MANY_REQUESTS,
            "This demo milestone has reached its submission limit. Reset the demo database to run it again.",
        ));
    }
    Ok(())
}

async fn insert_submission(
    state: &AppState,
    milestone: &store::MilestoneRow,
    evidence: &Evidence,
) -> anyhow::Result<String> {    let submission_id = format!("sub_{}", uuid::Uuid::new_v4().simple());
    let hash = executor::evidence_hash(evidence);
    sqlx::query(
        "INSERT INTO submissions (id, milestone_id, builder, evidence_json, evidence_hash, created_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&submission_id)
    .bind(&milestone.id)
    .bind(&milestone.builder)
    .bind(evidence.canonical_json())
    .bind(&hash)
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&state.pool)
    .await?;
    store::set_state(&state.pool, &milestone.id, "submitted").await?;
    Ok(submission_id)
}

pub async fn verify_submission(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Value> {
    let submission = store::get_submission(&state.pool, &id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Submission not found"))?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM node_runs WHERE submission_id = ?")
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .map_err(internal)?;
    if count > 0 {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "This submission has already been verified. Submit a revision instead.",
        ));
    }
    start_verification(state, submission.id, submission.milestone_id)
        .await?;
    Ok(Json(json!({"status": "verifying"})))
}

async fn start_verification(
    state: AppState,
    submission_id: String,
    milestone_id: String,
) -> Result<(), ApiError> {
    let updated = sqlx::query(
        "UPDATE milestones SET state = 'verifying', updated_at = ? WHERE id = ? AND state IN ('draft','open','submitted','bounced','needs_review')",
    )
    .bind(chrono::Utc::now().to_rfc3339())
    .bind(&milestone_id)
    .execute(&state.pool)
    .await
    .map_err(internal)?;
    if updated.rows_affected() == 0 {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "This milestone is already being verified or has been released.",
        ));
    }
    let task_state = state.clone();
    let task_submission = submission_id.clone();
    tokio::spawn(async move {
        if let Err(error) = executor::run(task_state.clone(), task_submission.clone()).await {
            tracing::error!(%error, submission = %task_submission, "verification failed");
            let _ = store::set_state(&task_state.pool, &milestone_id, "needs_review").await;
            let _ = store::add_event(
                &task_state.pool,
                &milestone_id,
                "verification_error",
                &format!("Verification stopped unexpectedly: {error}"),
            )
            .await;
        }
    });
    Ok(())
}

pub async fn approve_payment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Value> {
    if let Some(expected) = state.cfg.operator_token.as_deref() {
        let provided = headers
            .get("x-operator-token")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        if provided != expected {
            return Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "A valid X-Operator-Token header is required to approve a settlement.",
            ));
        }
    }
    let payment = store::get_payment(&state.pool, &id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Payment not found"))?;
    if payment.status == "settled" {
        return Err(ApiError::new(StatusCode::CONFLICT, "Payment already settled."));
    }
    if payment.status == "blocked" {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "A blocked payment cannot be approved. The submission must be revised and reverified.",
        ));
    }
    let verdict = store::verdict_for(&state.pool, &payment.submission_id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::CONFLICT, "No verdict exists for this payment."))?;
    if verdict.final_decision == "bounce" {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "The verdict blocked this payment.",
        ));
    }
    if verdict.serv_decision != "accept" {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "The reasoning verdict did not accept the submission.",
        ));
    }

    let submission = store::get_submission(&state.pool, &payment.submission_id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Submission not found"))?;
    let milestone = store::get_milestone(&state.pool, &submission.milestone_id)
        .await
        .map_err(internal)?;

    let updated = sqlx::query("UPDATE payments SET status = 'pending', note = 'Operator approved the release; settlement in progress.' WHERE id = ? AND status IN ('held','failed')")
        .bind(&id)
        .execute(&state.pool)
        .await
        .map_err(internal)?;    if updated.rows_affected() == 0 {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "This payment is already being settled or is no longer approvable.",
        ));
    }

    let settlement = crate::payments::settle(
        &state.cfg.kite,
        &id,
        payment.amount_cents,
        &payment.currency,
        &submission.builder,
        &milestone.title,
    )
    .await;

    sqlx::query(
        "UPDATE payments SET status = ?, tx_ref = ?, explorer_url = ?, session_ref = ?, approval_url = ?, note = ? WHERE id = ?",
    )
    .bind(&settlement.status)
    .bind(settlement.tx_ref.as_deref())
    .bind(settlement.explorer_url.as_deref())
    .bind(settlement.session_ref.as_deref())
    .bind(settlement.approval_url.as_deref())
    .bind(&settlement.note)
    .bind(&id)
    .execute(&state.pool)
    .await
    .map_err(internal)?;

    if settlement.status == "settled" {
        store::set_state(&state.pool, &milestone.id, "released")
            .await
            .map_err(internal)?;
        executor::ensure_receipt(&state, &payment.submission_id, &milestone.id)
            .await
            .map_err(internal)?;
        store::add_event(
            &state.pool,
            &milestone.id,
            "payment_released",
            "Operator approved the held release; funds settled.",
        )
        .await
        .map_err(internal)?;
    } else {
        store::set_state(&state.pool, &milestone.id, "needs_review")
            .await
            .map_err(internal)?;
        store::add_event(
            &state.pool,
            &milestone.id,
            "payment_failed",
            &format!("Approved settlement failed: {}", settlement.note),
        )
        .await
        .map_err(internal)?;
    }

    Ok(Json(json!({
        "payment_id": id,
        "status": settlement.status,
        "tx_ref": settlement.tx_ref,
        "note": settlement.note,
    })))
}

pub async fn refresh_reppo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Value> {
    let milestone = store::get_milestone(&state.pool, &id)
        .await
        .map_err(|_| ApiError::new(StatusCode::NOT_FOUND, "Milestone not found"))?;
    let submission = store::latest_submission(&state.pool, &milestone.id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "No submission for this milestone"))?;
    let verdict = store::verdict_for(&state.pool, &submission.id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "No verdict yet"))?;
    if verdict.reppo_mode != "live_lite" {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Reppo is not in live mode for this verdict.",
        ));
    }
    let Some(eval_id) = verdict.reppo_eval_id.as_deref() else {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "No Reppo eval id is stored."));
    };
    let Some(reppo) = crate::integrations::reppo::poll_once(&state.cfg.reppo, &state.http, eval_id).await
    else {
        return Ok(Json(json!({"status": "pending"})));
    };

    let assurance = if verdict.serv_engine == "serv_live" && reppo.status == "settled" {
        "high"
    } else if verdict.serv_engine == "serv_live" {
        "medium"
    } else {
        "low"
    };
    let final_decision = if reppo.rejected() { "bounce" } else { &verdict.final_decision };

    sqlx::query(
        "UPDATE verdicts SET reppo_status = ?, reppo_score = ?, reppo_decision = ?, reppo_confidence = ?, reppo_critique = ?, assurance = ?, final_decision = ? WHERE id = ?",
    )
    .bind(&reppo.status)
    .bind(reppo.score)
    .bind(reppo.decision.as_deref())
    .bind(reppo.confidence.as_deref())
    .bind(reppo.critique.as_deref())
    .bind(assurance)
    .bind(final_decision)
    .bind(&verdict.id)
    .execute(&state.pool)
    .await
    .map_err(internal)?;

    if reppo.rejected() {
        sqlx::query("UPDATE payments SET status = 'blocked', note = 'No funds moved. The independent Reppo verdict rejected the submission.' WHERE submission_id = ? AND status = 'held'")
            .bind(&submission.id)
            .execute(&state.pool)
            .await
            .map_err(internal)?;
        store::set_state(&state.pool, &milestone.id, "bounced")
            .await
            .map_err(internal)?;
        executor::ensure_receipt(&state, &submission.id, &milestone.id)
            .await
            .map_err(internal)?;
    } else {
        store::add_event(
            &state.pool,
            &milestone.id,
            "reppo_settled",
            "The independent Reppo verdict arrived. The payment remains held for operator approval.",
        )
        .await
        .map_err(internal)?;
    }

    Ok(Json(json!({
        "status": reppo.status,
        "decision": reppo.decision,
        "score": reppo.score,
        "confidence": reppo.confidence,
        "critique": reppo.critique,
        "assurance": assurance,
    })))
}

pub async fn kite_retry(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Value> {
    let payment = store::get_payment(&state.pool, &id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Payment not found"))?;
    if payment.rail != "kite_passport" {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "This payment is not on the Kite Passport rail.",
        ));
    }
    if payment.status == "settled" {
        return Err(ApiError::new(StatusCode::CONFLICT, "Payment already settled."));
    }
    let submission = store::get_submission(&state.pool, &payment.submission_id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Submission not found"))?;
    let milestone = store::get_milestone(&state.pool, &submission.milestone_id)
        .await
        .map_err(internal)?;

    let settlement = crate::payments::settle(
        &state.cfg.kite,
        &id,
        payment.amount_cents,
        &payment.currency,
        &submission.builder,
        &milestone.title,
    )
    .await;

    sqlx::query(
        "UPDATE payments SET status = ?, tx_ref = ?, explorer_url = ?, session_ref = ?, approval_url = ?, note = ? WHERE id = ?",
    )
    .bind(&settlement.status)
    .bind(settlement.tx_ref.as_deref())
    .bind(settlement.explorer_url.as_deref())
    .bind(settlement.session_ref.as_deref())
    .bind(settlement.approval_url.as_deref())
    .bind(&settlement.note)
    .bind(&id)
    .execute(&state.pool)
    .await
    .map_err(internal)?;

    if settlement.status == "settled" {
        store::set_state(&state.pool, &milestone.id, "released")
            .await
            .map_err(internal)?;
        executor::ensure_receipt(&state, &payment.submission_id, &milestone.id)
            .await
            .map_err(internal)?;
    }

    Ok(Json(json!({
        "status": settlement.status,
        "approval_url": settlement.approval_url,
        "request_id": settlement.session_ref,
        "note": settlement.note,
    })))
}

pub async fn kite_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Value> {
    let payment = store::get_payment(&state.pool, &id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Payment not found"))?;
    if payment.rail != "kite_passport" {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "This payment is not on the Kite Passport rail.",
        ));
    }
    let Some(request_id) = payment.session_ref.as_deref() else {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "No Kite approval request id is stored for this payment.",
        ));
    };
    let previous_status = payment.status.clone();
    let settlement = crate::payments::poll_status(&state.cfg.kite, request_id).await;
    sqlx::query(
        "UPDATE payments SET status = ?, tx_ref = ?, explorer_url = ?, session_ref = ?, approval_url = ?, note = ? WHERE id = ?",
    )
    .bind(&settlement.status)
    .bind(settlement.tx_ref.as_deref())
    .bind(settlement.explorer_url.as_deref())
    .bind(settlement.session_ref.as_deref())
    .bind(settlement.approval_url.as_deref())
    .bind(&settlement.note)
    .bind(&id)
    .execute(&state.pool)
    .await
    .map_err(internal)?;

    let submission = store::get_submission(&state.pool, &payment.submission_id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Submission not found"))?;
    let milestone = store::get_milestone(&state.pool, &submission.milestone_id)
        .await
        .map_err(internal)?;

    if settlement.status == "settled" {
        store::set_state(&state.pool, &milestone.id, "released")
            .await
            .map_err(internal)?;
        executor::ensure_receipt(&state, &payment.submission_id, &milestone.id)
            .await
            .map_err(internal)?;
        if previous_status != "settled" {
            store::add_event(
                &state.pool,
                &milestone.id,
                "payment_released",
                "Kite Passport approval completed; funds settled.",
            )
            .await
            .map_err(internal)?;
        }
    } else if settlement.status == "failed" && previous_status != "failed" {
        store::set_state(&state.pool, &milestone.id, "needs_review")
            .await
            .map_err(internal)?;
        store::add_event(
            &state.pool,
            &milestone.id,
            "payment_failed",
            &format!("Kite approval check failed: {}", settlement.note),
        )
        .await
        .map_err(internal)?;
    }

    Ok(Json(json!({
        "status": settlement.status,
        "tx_ref": settlement.tx_ref,
        "approval_url": settlement.approval_url,
        "note": settlement.note,
    })))
}

pub async fn receipt_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<Value> {
    let receipt = store::receipt_by_slug(&state.pool, &slug)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Receipt not found"))?;
    let submission = store::get_submission(&state.pool, &receipt.submission_id)
        .await
        .map_err(internal)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Submission not found"))?;
    let milestone = store::get_milestone(&state.pool, &submission.milestone_id)
        .await
        .map_err(internal)?;
    let nodes = store::node_runs(&state.pool, &submission.id)
        .await
        .map_err(internal)?;
    let verdict = store::verdict_for(&state.pool, &submission.id)
        .await
        .map_err(internal)?;
    let payment = store::payment_for(&state.pool, &submission.id)
        .await
        .map_err(internal)?;
    let graph: AcceptanceGraph = serde_json::from_str(&milestone.graph_json)
        .unwrap_or_else(|_| compile_criteria(&milestone.criteria));
    let evidence: Value = serde_json::from_str(&submission.evidence_json).unwrap_or(Value::Null);
    let graph_hash = {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(milestone.graph_json.as_bytes());
        hex::encode(hasher.finalize())
    };

    Ok(Json(json!({
        "receipt": receipt,
        "graph_hash": graph_hash,
        "milestone": {
            "id": milestone.id,
            "title": milestone.title,
            "builder": milestone.builder,
            "criteria": milestone.criteria,
            "amount_cents": milestone.amount_cents,
            "currency": milestone.currency,
            "program_name": milestone.program_name,
        },
        "submission": {
            "id": submission.id,
            "evidence": evidence,
            "evidence_hash": submission.evidence_hash,
            "created_at": submission.created_at,
        },
        "graph": graph,
        "node_runs": nodes,
        "verdict": verdict.map(|v| json!({
            "serv": {
                "decision": v.serv_decision,
                "score": v.serv_score,
                "summary": v.serv_summary,
                "reasons": serde_json::from_str::<Value>(&v.serv_reasons_json).unwrap_or(Value::Array(vec![])),
                "engine": v.serv_engine,
            },
            "reppo": {
                "status": v.reppo_status,
                "score": v.reppo_score,
                "decision": v.reppo_decision,
                "confidence": v.reppo_confidence,
                "critique": v.reppo_critique,
                "mode": v.reppo_mode,
            },
            "final_decision": v.final_decision,
            "assurance": v.assurance,
            "created_at": v.created_at,
        })),
        "payment": payment,
        "engines": engine_report_for(&state, payment.as_ref()),
        "public_base_url": state.cfg.public_base_url,
    })))
}

use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ProgramRow {
    pub id: String,
    pub name: String,
    pub operator: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct MilestoneRow {
    pub id: String,
    pub program_id: String,
    pub title: String,
    pub builder: String,
    pub criteria: String,
    pub graph_json: String,
    pub amount_cents: i64,
    pub ceiling_cents: i64,
    pub currency: String,
    pub state: String,
    pub sample_json: String,
    pub created_at: String,
    pub updated_at: String,
    pub program_name: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct MilestoneListRow {
    pub id: String,
    pub title: String,
    pub builder: String,
    pub amount_cents: i64,
    pub ceiling_cents: i64,
    pub currency: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub program_name: String,
    pub submission_count: i64,
    pub last_decision: Option<String>,
    pub last_payment_status: Option<String>,
    pub last_receipt_slug: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct SubmissionRow {
    pub id: String,
    pub milestone_id: String,
    pub builder: String,
    pub evidence_json: String,
    pub evidence_hash: String,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct NodeRunRow {
    pub id: String,
    pub submission_id: String,
    pub node_id: String,
    pub seq: i64,
    pub kind: String,
    pub label: String,
    pub status: String,
    pub detail: String,
    pub evidence_url: Option<String>,
    pub cost_usd: f64,
    pub latency_ms: i64,
    pub output_json: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct VerdictRow {
    pub id: String,
    pub submission_id: String,
    pub serv_decision: String,
    pub serv_score: i64,
    pub serv_summary: String,
    pub serv_reasons_json: String,
    pub serv_engine: String,
    pub reppo_eval_id: Option<String>,
    pub reppo_status: String,
    pub reppo_score: Option<i64>,
    pub reppo_decision: Option<String>,
    pub reppo_confidence: Option<String>,
    pub reppo_critique: Option<String>,
    pub reppo_mode: String,
    pub final_decision: String,
    pub assurance: String,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PaymentRow {
    pub id: String,
    pub submission_id: String,
    pub amount_cents: i64,
    pub currency: String,
    pub rail: String,
    pub status: String,
    pub tx_ref: Option<String>,
    pub explorer_url: Option<String>,
    pub session_ref: Option<String>,
    pub note: String,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ReceiptRow {
    pub id: String,
    pub submission_id: String,
    pub slug: String,
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct EventRow {
    pub id: String,
    pub milestone_id: String,
    pub kind: String,
    pub message: String,
    pub created_at: String,
}

pub async fn list_programs(pool: &SqlitePool) -> Result<Vec<ProgramRow>, sqlx::Error> {
    sqlx::query_as::<_, ProgramRow>("SELECT * FROM programs ORDER BY created_at ASC")
        .fetch_all(pool)
        .await
}

pub async fn list_milestones(pool: &SqlitePool) -> Result<Vec<MilestoneListRow>, sqlx::Error> {
    sqlx::query_as::<_, MilestoneListRow>(
        r#"SELECT
            m.id, m.title, m.builder, m.amount_cents, m.ceiling_cents, m.currency,
            m.state, m.created_at, m.updated_at, p.name AS program_name,
            (SELECT COUNT(*) FROM submissions s WHERE s.milestone_id = m.id) AS submission_count,
            (SELECT v.final_decision FROM verdicts v
                JOIN submissions s ON s.id = v.submission_id
                WHERE s.milestone_id = m.id
                ORDER BY v.created_at DESC LIMIT 1) AS last_decision,
            (SELECT pa.status FROM payments pa
                JOIN submissions s ON s.id = pa.submission_id
                WHERE s.milestone_id = m.id
                ORDER BY pa.created_at DESC LIMIT 1) AS last_payment_status,
            (SELECT r.slug FROM receipts r
                JOIN submissions s ON s.id = r.submission_id
                WHERE s.milestone_id = m.id
                ORDER BY r.created_at DESC LIMIT 1) AS last_receipt_slug
        FROM milestones m
        JOIN programs p ON p.id = m.program_id
        ORDER BY
            CASE m.state
                WHEN 'needs_review' THEN 0
                WHEN 'verifying' THEN 1
                WHEN 'submitted' THEN 2
                WHEN 'open' THEN 3
                WHEN 'bounced' THEN 4
                WHEN 'released' THEN 5
                ELSE 6
            END,
            m.created_at DESC"#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_milestone(pool: &SqlitePool, id: &str) -> Result<MilestoneRow, sqlx::Error> {
    sqlx::query_as::<_, MilestoneRow>(
        r#"SELECT m.*, p.name AS program_name
           FROM milestones m JOIN programs p ON p.id = m.program_id
           WHERE m.id = ?"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn latest_submission(
    pool: &SqlitePool,
    milestone_id: &str,
) -> Result<Option<SubmissionRow>, sqlx::Error> {
    sqlx::query_as::<_, SubmissionRow>(
        "SELECT * FROM submissions WHERE milestone_id = ? ORDER BY created_at DESC, rowid DESC LIMIT 1",
    )
    .bind(milestone_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_submission(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<SubmissionRow>, sqlx::Error> {
    sqlx::query_as::<_, SubmissionRow>("SELECT * FROM submissions WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn node_runs(
    pool: &SqlitePool,
    submission_id: &str,
) -> Result<Vec<NodeRunRow>, sqlx::Error> {
    sqlx::query_as::<_, NodeRunRow>(
        "SELECT * FROM node_runs WHERE submission_id = ? ORDER BY seq ASC",
    )
    .bind(submission_id)
    .fetch_all(pool)
    .await
}

pub async fn verdict_for(
    pool: &SqlitePool,
    submission_id: &str,
) -> Result<Option<VerdictRow>, sqlx::Error> {
    sqlx::query_as::<_, VerdictRow>(
        "SELECT * FROM verdicts WHERE submission_id = ? ORDER BY created_at DESC LIMIT 1",
    )
    .bind(submission_id)
    .fetch_optional(pool)
    .await
}

pub async fn payment_for(
    pool: &SqlitePool,
    submission_id: &str,
) -> Result<Option<PaymentRow>, sqlx::Error> {
    sqlx::query_as::<_, PaymentRow>(
        "SELECT * FROM payments WHERE submission_id = ? ORDER BY created_at DESC LIMIT 1",
    )
    .bind(submission_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_payment(pool: &SqlitePool, id: &str) -> Result<Option<PaymentRow>, sqlx::Error> {
    sqlx::query_as::<_, PaymentRow>("SELECT * FROM payments WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn receipt_for(
    pool: &SqlitePool,
    submission_id: &str,
) -> Result<Option<ReceiptRow>, sqlx::Error> {
    sqlx::query_as::<_, ReceiptRow>("SELECT * FROM receipts WHERE submission_id = ?")
        .bind(submission_id)
        .fetch_optional(pool)
        .await
}

pub async fn receipt_by_slug(
    pool: &SqlitePool,
    slug: &str,
) -> Result<Option<ReceiptRow>, sqlx::Error> {
    sqlx::query_as::<_, ReceiptRow>("SELECT * FROM receipts WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await
}

pub async fn events_for(
    pool: &SqlitePool,
    milestone_id: &str,
) -> Result<Vec<EventRow>, sqlx::Error> {
    sqlx::query_as::<_, EventRow>(
        "SELECT * FROM events WHERE milestone_id = ? ORDER BY created_at ASC, rowid ASC LIMIT 50",
    )
    .bind(milestone_id)
    .fetch_all(pool)
    .await
}

pub async fn set_state(
    pool: &SqlitePool,
    milestone_id: &str,
    state: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE milestones SET state = ?, updated_at = ? WHERE id = ?")
        .bind(state)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(milestone_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_event(
    pool: &SqlitePool,
    milestone_id: &str,
    kind: &str,
    message: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO events (id, milestone_id, kind, message, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(format!("evt_{}", uuid::Uuid::new_v4().simple()))
    .bind(milestone_id)
    .bind(kind)
    .bind(message)
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}

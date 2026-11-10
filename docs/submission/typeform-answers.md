# Typeform answers — SERV Hackathon 02

Form: <https://openserv.typeform.com/submissions02?typeform-source=www.openserv.ai>

The form is 21 fields, 11 required. These are copy-paste answers for the Kite
track. Replace anything in brackets before submitting.

## Required fields

**1. What's your full name?**
Phi Bao

**2. What's your email address?**
kiter0211@gmail.com

**3. Are you new to building with OpenServ?**
No

**4. Which track are you submitting to?**
Kite

**5. Is this a new project, or an existing project with new work for this hackathon?**
A new project

**6. What is your project called?**
Tranche

**7. Tell us about your project. What does it do, who is it for, what problem does it solve, and how does it work?**

Tranche is a proof-gated payment layer for agent work. A program operator writes
a milestone in plain English and sets a tranche amount. Tranche compiles that
rubric into an acceptance graph: deterministic checks for the repository, README,
live demo, numeric claims, and reviewer-injection safety; a SERV Reasoning node
for judgment; and an independent Reppo evaluation. The Rust policy gate releases
the tranche only when the evidence passes and assurance is high enough. Anything
else is blocked or held for a human.

The product is for grant programs, bounty boards, hackathons, and agent
marketplaces that pay strangers for work they cannot fully verify. Today a
reviewer reads a claim, clicks a link, and hopes. Tranche makes that decision
executable and leaves a public receipt with the graph hash, evidence hash, both
verdicts, the policy decision, and the settlement reference.

The live demo includes a real 0.10 USDC payout settled through Kite Passport on
Base, plus blocked receipts where SERV and Reppo both rejected padded and
injected claims and zero funds moved. The self-serve demo uses a labeled ledger
because every Kite payout needs the operator passkey.

Two-minute walkthrough:
https://github.com/PhiBao/tranche/raw/main/docs/submission/tranche-demo.mp4

**8. Share the URL for your project, such as a demo, website, or GitHub repository.**
https://tranche-serv.fly.dev

**9. Share the URL of your X post about your project.**
[Paste the X post URL after posting. The post text is in
`docs/submission/x-post.md`.]

**10. Is data collection enabled for your project?**
Yes

**11. Which best describes your team?**
I'm participating solo

## Optional fields

**12. Share one additional project URL, if you have one.**
https://github.com/PhiBao/tranche

**13. Upload your project logo or icon.**
`docs/submission/tranche-logo.png`

**14. How much experience do you have building AI agents?**
Extensive experience

**15. What country are you based in?**
[Your country]

**16. What company or school are you affiliated with?**
[Optional]

**17. What is your current role?**
Independent builder

**18. Share the URL of your LinkedIn or GitHub profile, or your personal website.**
https://github.com/PhiBao

**19. What has your experience with SERV been like?**

SERV Reasoning is the part of the stack that made the release decision feel
auditable. The structured verdict, the PromptGuard check, and the shadow-agent
reasoning gave us a decision we could store and show next to the evidence, rather
than a model response we had to interpret. The main tradeoff is latency: a live
review adds several seconds to the release path, so the UI shows node-level
progress and the payment is held until the verdict lands. We also added a
deterministic safety node before SERV so a reviewer-directed instruction cannot
reach the model at all.

**20. May we share your feedback as a quote on our website or social media?**
Yes, OpenServ may share my feedback as a quote.

**21. Would you like to receive OpenServ updates and event announcements?**
Yes, I agree to receive updates.

## Before submitting

- [ ] Post on X and paste the post URL in field 9. The form will not submit
      without it.
- [ ] Enable data collection in the OpenServ organization settings and answer
      **Yes** in field 10.
- [ ] Upload `docs/submission/tranche-logo.png` if the form accepts images.
- [ ] Confirm the name, country, company/school, and role.
- [ ] Submit before **2026-11-15 23:59 UTC**.

## X post text

> Most agent demos show money moving.
>
> The useful part is the refusal.
>
> I built **Tranche** for the @openservai SERV Hackathon 02, Kite track.
>
> A milestone rubric becomes an executable acceptance graph: code checks → SERV
> Reasoning → an independent @reppo verdict → a Rust policy gate → @GoKiteAI
> Passport.
>
> A real **0.10 USDC payout settled on Base**, approved with a passkey. Receipt:
> https://tranche-serv.fly.dev/receipt/?slug=c6ac72kq2rc4
>
> SERV and Reppo both rejected the padded claim and the injection attempt. Both
> got a receipt: **0.00 USDC moved.**
>
> Live: https://tranche-serv.fly.dev
> Code: https://github.com/PhiBao/tranche
>
> #SERVHackathon #BuildonSERV

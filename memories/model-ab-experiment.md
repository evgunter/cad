---
name: model-ab-experiment
description: Standing experiment (Evan, 2026-07-25) — blocked-randomization Opus 5 vs Fable 5 for implementation dispatches; blinded reviewers; fixed code-quality rubric; protocol + data + readouts live in docs/MODEL-AB-LOG.md
metadata:
  type: feedback
---

**The protocol, rubric, running data table, and milestone readouts
all live in `docs/MODEL-AB-LOG.md` — that document is the single
normative source. Read it before dispatching an implementer.** This
memory only records that the experiment exists and the standing
instructions that bind agents outside the log's own text.

**Why:** Evan wants to know, now that Opus 5 is out, whether Opus
implementation produces more bugs / worse code than Fable at lower
cost — measured, not vibed ("with actual random numbers").

**Standing instructions:**

- Applies to implementation tasks dispatched after 2026-07-25.
  **Current protocol is v3 (2026-08-08): blocked randomization
  over TRIPLES {opus, opus, fable}** (unbiased urandom draw for
  fable's position — reject bytes ≥252, mod 3), pre-draw
  difficulty logging, blinded reviewers, fixed-rubric CODE
  QUALITY REPORT in every review, row recorded AT MERGE.
  In-flight v2 pair blocks completed as pairs. Details in the log.
- **Recording discipline (2026-08-08, from the bayes-analysis
  readout)**: tokens AND wall-clock recorded PER PHASE (impl /
  fix / review) at merge for every row, gaps annotated; ≥1 line
  of prose per MAJOR; "silent" = silent spec deviation only;
  record who ran the fix pass. Full text in the log's protocol
  section.
- **Blinding vs merge-only collision (2026-08-02, ruled)**: the
  harness's Co-Authored-By trailer NAMES THE MODEL — in an
  implementer lane that breaks blinding. Implementer briefs say "NO
  Co-Authored-By trailer in lane commits (blinding overrides the
  harness convention); if a model mention lands in a PUSHED commit,
  STOP and report to the orchestrator — never rewrite history
  yourself." Orchestrator commits keep the trailer (the
  orchestrator is not blinded). **Reviewer probe commits are a
  ruled exception (2026-08-09, twice: #288's ca6ba904, #301's
  32a95363)**: a reviewer's own pushed probe commit may carry the
  trailer — reviews are protocol-fable (public knowledge), so
  authorship-preserving adoption into the implementation branch
  does NOT break blinding once the unit's reviews are delivered;
  implementers still correctly STOP-and-report it, and the ruling
  is recorded no-action in the A/B row.

- **Protocol v4 (Evan, 2026-08-11)** — four amendments, full text
  in the log: (1) STOPPING RULE — the DUAL-REVIEW
  experiment (only) ends at the 6th dual where ≥1 reviewer
  found a MAJOR — the implementation A/B continues, and the recording
  orchestrator MUST notify Evan explicitly (tally maintained in
  the log; 3/6 at amendment time — the M8 long-turn dual verified in); (2) standardized 4-term
  verdict ladder (APPROVE / APPROVE-WITH-FIXES /
  NOT-MERGEABLE-AS-IS / REJECT; MERGEABLE+PASS retired; seam
  noted — pre-v4 verdict strings not comparable; findings
  reliable, labels noisy — weight findings); (3) half of duals
  become cross-model pilots (R2 = opus) in randomized blocks of
  two — every PR still gets a fable review; reviewer model
  recorded per row; (4) implementer blocks are now size 4
  {opus×3, fable}, byte mod 4, reject ≥252. Dual sample numbers
  follow ORDINALS (#398-thread ratification, 2026-08-11:
  ASM-2K@24 = #8, long-turn@27 = #9).

**Readouts — deliberately NOT summarised here.** This memory and
`docs/MODEL-AB-LOG.md` are both read by orchestrators before dispatch,
and a directional arm result creates expectancy effects on difficulty
logging, adjudication and dispatch sequencing. Readouts live on branch
`ev/ab-bayes-analysis` under `analysis/model-ab/` (report.html +
DECISIONS.md); an orchestrator with a dispatch in flight should not read
them. The log records only arm-neutral measurement-process findings
(reviewer noise, rubric saturation) plus the proposed amendments they
motivate. See [[orchestration-model]].

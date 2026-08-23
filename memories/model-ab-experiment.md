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
- **Blinding vs merge-only collision**: the harness's
  Co-Authored-By trailer NAMES THE MODEL, which in an implementer
  lane breaks blinding. Implementer briefs say "NO Co-Authored-By
  trailer in lane commits (blinding overrides the harness
  convention); if one lands in a PUSHED commit, note it in the PR
  body and CARRY ON — never rewrite history, and never stop the
  unit over it." Orchestrator commits keep the trailer (the
  orchestrator is not blinded), and so may a reviewer's own probe
  commits: reviews are protocol-fable, so authorship-preserving
  adoption into the implementation branch does not break blinding.

- **Protocol v4 (Evan, 2026-08-11)** — four amendments, full text
  in the log: (1) STOPPING RULE — the DUAL-REVIEW
  experiment (only) ends at the 6th dual where ≥1 reviewer
  found a MAJOR — the implementation A/B continues, and the recording
  orchestrator MUST notify Evan explicitly (the tally lives in the
  log, never here); (2) standardized 4-term
  verdict ladder (APPROVE / APPROVE-WITH-FIXES /
  NOT-MERGEABLE-AS-IS / REJECT; MERGEABLE+PASS retired; seam
  noted — pre-v4 verdict strings not comparable; findings
  reliable, labels noisy — weight findings); (3) half of duals
  become cross-model pilots (R2 = opus) in randomized blocks of
  two — every PR still gets a fable review; reviewer model
  recorded per row; (4) implementer blocks are size 4
  {opus×3, fable}, byte mod 4. Dual sample numbers
  follow ORDINALS (#398-thread ratification).

- **Protocol v5 (Evan, 2026-08-18)** — full text in the log:
  reviews gain a STYLE LANE alongside the claims to falsify
  (brief text: `docs/prompts/reviewer-style-lane.md`, which dispatches point at
  by path rather than pasting), and a disclosed deviation that is not an improvement
  owes a concretely scheduled followup before merge. **Seam: review
  figures are not comparable across 2026-08-18** — the instrument
  changed, so expect findings counts to rise without any change in
  implementation quality; a readout spanning the boundary reports the
  two eras separately.

**Every live number of this experiment — block, slot, ordinal,
dual tally — is claimed from `docs/MODEL-AB-LOG.md` ON MAIN at
dispatch.** No copy lives in this memory, in a plan, or in an
M-log; a second copy is stale the moment it is written.

**Readouts — deliberately NOT summarised here, and not in the log
either** (standing rule, Evan 2026-08-11). Both files are read by
orchestrators before dispatch, and a directional arm result creates
expectancy effects on difficulty logging, adjudication and dispatch
sequencing. Readouts live on branch `ev/ab-bayes-analysis` under
`analysis/model-ab/` (report.html, DECISIONS.md, readouts-archive.md);
**an orchestrator with a dispatch in flight should not read them.**
Analysis methodology is likewise not orchestrator protocol. See
[[orchestration-model]].

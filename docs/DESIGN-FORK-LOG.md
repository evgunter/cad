# Design-fork review — recommendation log

**This is process data (an experiment log), not a design
reference** — nothing here binds kernel design; it moves out of
`docs/` when the experiment concludes.

Standing experiment (Ev, in-chat, 2026-09-24). **The protocol in
force lives in `docs/DESIGN-FORK-PROTOCOL.md`.** This log holds the
rows; each names the protocol it was recorded under by commit hash,
so `git show <hash>:docs/DESIGN-FORK-PROTOCOL.md` reads that version.

## Rows

Columns: **protocol** (commit hash of the protocol in force when the
row opened); **fork** (the `work/` item and the `[ev]` PR); **problem**
(one line, as the orchestrator stated it); **A** and **B** (first
recommendation, one line of argument, confidence, framing rejected
y/n, ratified text challenged y/n); **agree** (on the first reports);
**reconciliation** (each round: who, shown what, what moved — or
"none"); **to Ev** (the recommendation or stated split the PR
carried); **Ev** (the decision, and date); **match** (which of the
first recommendations, if any, Ev's decision took); **A/B** (the
urandom byte and which model was A — filled in only after Ev decides).

| # | date | protocol | fork | problem | A | B | agree | reconciliation | to Ev | Ev | match | A/B |
|---|------|----------|------|---------|---|---|-------|----------------|-------|----|-------|-----|
| 1 | 2026-09-25 | bb10a4cdd | `work/carve/full-revolve-emits-split-planar-walls.md`, PR 3258 | A full revolve emits each planar wall as two same-key halves, so F7 refuses it as a boolean operand above the kernel: what should the revolve produce, and where is maximality owed? | The revolve runs the structural merge as its own final stage; widen F7's sweeps clause by one clause; plain `Band(s)`; π revolve separate (lean: recognise a half turn, or park). Likely. Framing n; ratified y | The same final stage; rewrite F7 as the general rule; `Band(s)` (unsure); π now (decide θ vs π, mint the end cap Shared). Likely. Framing n; ratified y | yes on the seat and naming; split on F7 wording and π timing | none | Merge stage, F7 wording carrying both, `Band(s)`, π parked (the orchestrator's lean) | Construct each planar wall as one face in the first place, F7 unchanged; `Band(s)` yes; π parked at P2 (2026-09-25) | neither on the seat; both on naming; A on π timing | byte 216, A = Opus, B = Fable |
| 2 | 2026-09-28 | bb10a4cdd | `work/dup/the-chart-partition-has-a-topo-src-home-and-a-test-home.md`, PR 3310 | The chart partition (a body's faces grouped by surface key) is spelled three times because the public offset doors take caller-built `ChartMove` groups; Ev asked whether that is a symptom of the API's shape and whether internals like it can be abstracted away. | (b3) the doors take the solids plus a rule over a door-built chart view, one distance per chart by construction; `ChartMove`, its four policing refusals and the split-chart gap go; an 'inward by t' verb is sugar on top. Sure it is a symptom, likely on the shape. Framing rejected: n. Ratified text challenged: n. | (c) keep `ChartMove` as the primitive (the per-chart scalar is the compare-free shape) and add `offset_inward(body, solid, t)`, which O4 names; do not publish the partition. Likely. Framing rejected: partly (the leak is the per-chart scalar, not the partition). Ratified text challenged: n. | n | Round 1: each shown the other's report and asked five targeted questions. A conceded that a structural split-chart check (two moves naming one key) makes `ChartMove` sound, so the case for (b3) is API cleanliness rather than correctness; A held (b3) at about 60/40, with B's verb as sugar. B accepted that (b3) meets the compare-free bar, withdrew its D8 objection after quoting D8 (it binds recipes, not kernel doors), judged A's lift form better, and moved to (b3) with (c) as data-shaped sugar. | Converged: (b3), with a data-shaped 'inward by t' door over it; the grouping private to topo. | Took the recommendation: "the recommendation sounds good!" (PR 3310, 2026-09-28). | A (first reports); both after reconciliation | no byte drawn; A=opus, B=fable (labels by delivery order; blinding broken, see note) |
| 3 | 2026-09-28 | bb10a4cdd | `work/contact/area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step.md`, PR 3350 | A declared cap contact whose faces overlap in area is admitted, and the operand after it carries two coplanar adjacent faces that the next boolean's F7 gate refuses as an undeclared contact of a pair the user declared: what should the kernel do with such a contact, what should the operand be, and where does the fix belong? | Fix at the merge stage: delete a seam edge left dangling inside the merged face, with its free end, at any angle (repeated pruning, a topological test replacing the collinearity test); separately, lean yes that a boolean never ships a planar declared group it could not glue (refuse the step). Likely. Framing rejected: y (the rest door is not on the path; the skip is the merge door's `GroupNotClosed{ScaffoldingEmptyLoop}`). Ratified text challenged: y (DESIGN.md 'never elides vertices'). | Fix the merge door: generalise the `kev` repair to a junction all of whose edges are killed shared edges, chains included; and a boolean output stage may not ship a planar declared group it could not glue (refuse the step). Likely. Framing rejected: y (same correction). Ratified text challenged: y (same clause, reword proposed). | yes, on both decisions and on the clause | none | Both decisions, with the clause reworded (the edit in this PR) | Took both decisions and the clause as proposed: "sounds good!" (PR 3350, 2026-09-28) | both (A and B agreed) | byte 58, A = Opus, B = Fable |

## Notes

**Row 1.** Ev's first comment on the PR was a question, not an answer: why
not have the revolve "stop doing that and just emit the right thing to
begin with"? Both designers had SEEN that option and dismissed it.
- A: "Building the wire case 'directly maximal' has no different final
  state … one design, not two."
- B: "building the disc as one face inside the wire sweep (same final
  body — an implementation choice under A, not a design)".

Both judged the final state by the BODY returned, not by the construction
that returns it. A build-split-then-merge is a worse final state of the
code even when its output is identical. That is `docs/prompts/designer.md`
§3's "trace it to where it starts and prefer the answer that acts there",
and here the defect starts one layer DOWN from where both designers put the
fix. Ev asked whether the prompt should say "one level up or down", or
whether the designers just did not use the instruction. On this row they
reached the option and then ranked it away, citing §4's "weigh only the
final state".

**Row 2.** Two deviations from the protocol, recorded before Ev answers:
- **Labels.** No `/dev/urandom` byte was drawn at dispatch; A and B were assigned by delivery order (A = first report delivered). The mapping is on `analysis/design-fork/chart-partition-offset-api`.
- **Blinding.** Broken before the PR opened: the orchestrator's chat updates to Ev named which model had written each first report. This row still says A and B only, but Ev's decision on this fork was not made blind.


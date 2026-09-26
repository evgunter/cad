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
| 2 | 2026-09-26 | bb10a4cdd | `work/decide/the-apothems-sign-is-a-value-read.md`, PR 3283 | Six decisions on a parameter bulge stand on the apothem's sign, which no value-free rule reaches and nothing upstream supplies: what should the kernel do about them, and at which layer? | Not a tier question: the profile's pair pass recomputes an adjacent pair's shared vertex as a root, so `pair_contacts` should take the shared vertices and spell the other root sqrt-free; ship nothing in the tier; keep the read's dial off as the Phase 1 record. Likely. Framing y; ratified n | The same fix in the profile (adjacency-aware pair contacts); don't ship the read for the six; lean against landing the dial even off; the bracket's 28 owe the same look. Likely. Framing y; ratified n | yes on the layer and the fix; split on landing the dial off | none | The profile fix (a PATHS row); the read not shipped, the dial not landed (the orchestrator's lean), the bracket's 28 filed for a structural look | | | |

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


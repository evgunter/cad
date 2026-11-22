---
id: the-pads-and-the-brackets-stored-ceilings-are-stale
kind: issue
title: m10_9's stored ceilings for the pad and the filleted bracket are stale: the drive's whole-box ceilings are lower, and no gating row reads them
status: open
opened: 2026-11-22
---


Found by BOXFINISH's `boxfinish/signed-radius`, re-measuring
`crates/editor-core/tests/m10_9_pins_interval.rs`'s `measured_studies`
with the ceiling instrument
(`m10_9_evidence_interval::m10_9_ceilings_with_and_without_the_door`,
ε = 1e-9), on its head and on its base (main `ae076e05` with
`boxfinish/radial-readers`), which read alike.

| study | stored `certifies_at` / `refuses_at` | the drive's ceiling, door on | door off | over band at the ceiling |
|---|---|---|---|---|
| `r2_rounded_pad` | 3.151e3 / 3.152e3 | 1.1137e3 .. 1.1140e3 | the same | `arc_span`, 2 of 20, `[-1.0001e-9, 1.0001e-9]` |
| `r2_filleted_bracket` | 3.870e2 / 7.624e2 | 6.023e2 .. 6.025e2 | 5.598e2 .. 5.599e2 | `line_span`, 1 of 13 (door on); `pcurve_envelope` (door off) |

The over-band enclosures are identical on base and head at every scale
probed (the pad's `arc_span` reads ±1.0055e-9 at 1.12e3·ε and
±2.8288e-9 at 3.151e3·ε on both), so the move predates that branch;
which change made it is not bisected.

**Why no row caught it.** `certifies_at` is documented as a scale that
"certified whole", by the drive (`m10_8_harness::certifies_whole`: one
leaf, `certified == 1`). The gating rows that read it
(`m10_9_no_registrant_lies_on_any_measured_document`,
`m10_9_what_the_registry_buys`, `m10_9_the_pad_at_both_rule_f_dials`)
ask a REPLAY for its refusal, and a replay does not refuse where a
profile predicate is over the band and settled by its recourse
(`arc_span`'s `end_in_stretch`): the pad replays whole at 3.151e3·ε and
at 3.3e3·ε. So the stored scale is one the drive no longer certifies,
and the rows stay green on it.

The instrument's other answers are endpoint probes, not ceilings: it
reports "no refusal" for a document that certifies at its top scale
without bisecting below it.

**Repair shape.** Re-measure the two brackets and store them; make one
gating row ask the drive, not a replay, at each `certifies_at`.

**The test that says it is closed.** A gating row drives each study at
its `certifies_at` in one leaf and certifies it, and at its
`refuses_at` and does not.

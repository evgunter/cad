---
id: a-short-run-outs-stored-chord-reads-a-declared-fillet-joint-transversal
kind: issue
title: A short run out's stored chord reads a declared fillet joint transversal, and the refusal blames scene resolution
status: open
opened: 2026-09-29
priority: P2
cost: M
design: true
---

**Found by the second review of PR 3266, and re-measured there.** Take
`crates/profile/tests/arc_fillet.rs`'s
`a_short_closing_run_out_on_the_arrival_circle_is_named_the_run_out`
construction (`line_arc_internal` at r = 0.5, entry moved `turn` rad past
T2 on the R = 2 arrival circle) at ε = 1e-12 and sweep the turn in units
of ε:

| turn | outcome |
| --- | --- |
| 1ε, 2ε | `Escalated` on `path_junction_turn` |
| 5ε, 10ε, 15ε, 20ε | `FilletCarrierBelowSceneResolution { predicate: "carrier_line_circle", scale: 1.886, resolution: 4.19e-16, margin: 1.66e-10 … 1.01e-11 }` |
| 30ε, 40ε, 50ε | `Escalated` on `carrier_line_circle` (margins 4.0e-12, 2.7e-12, 1.5e-12) |
| 100ε | builds |

**What reads it transversal.** `fillets_carry_their_tangency`
(`crates/profile/src/path.rs`, the `JointClass::Transversal if
stores_an_arc` arm, ~3254) re-reads the fillet arc's declared outgoing
joint on the stored form. The run out is a few ε of arc, so its sagitta is
far below the band and `build_seg` stores it as a line. The joint is then
classified by `seg::carrier_line_circle_margin`
(`crates/profile/src/seg.rs`, ~587) against that chord's line, whose
direction rounds at about ε_mach·R/chord. The margin falls as the chord
grows (1.66e-10 at 5ε to 1.5e-12 at 50ε), which points at the chord's
conditioning and away from the scene's scale.

**Why the refusal is misattributed.** `FilletCarrierBelowSceneResolution`'s
doc (`crates/profile/src/path.rs`, ~1105–1114) says the variant means the
clearance's resolution floor, `scale·2^-52`, is coarser than ε. Here the
error's own `resolution` field reads 4.2e-16, four decades below ε = 1e-12.
The floor is not the cause, and an author following the variant's levers
(a smaller scene, a coarser ε) is sent the wrong way.

**Remedy (to decide).** Either the stored-form re-read should not classify a
declared joint against a chord it knows is below the band (the run out is
on the carrier by construction, and `PendingRunOut::rides` has just said so),
or the refusal should be a variant naming the short chord, with a lever
that works (a longer run, or no run at all). In both cases,
`FilletCarrierBelowSceneResolution` should only be raised when its
`resolution` is at least ε.

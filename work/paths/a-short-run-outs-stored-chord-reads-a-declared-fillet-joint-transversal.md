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

## Wider since the joint reads its turn (2026-10-19)

`paths/joint-reads-its-turn` (Ev's ruling on #5354) decides a joint by
its turn levered by the longer leg's arm, so the stored chord now reads
as what it is, a corner of half its sweep at the fillet's arm, rather
than through the chord line's rounding. The predicate the refusal names
is `path_junction_turn`, and the class reaches the default ε and wide
margins:

- the construction above, at `CAD_TOLERANCE_EPS=1e-9`: turns 1e-7 to
  3e-5 rad refuse `FilletCarrierBelowSceneResolution` (margin 2.5e-8 at
  1e-7, `resolution` 4.2e-16); at 1e-12 turns 1e-11 to 1e-6; at 1e-6,
  1e-4 to 1e-3. The run out builds once its sagitta, about `turn²`
  m, clears the band, so `arc_fillet.rs`'s row now reads turns of a few
  `√(Kε)`.
- grid A's recourse census (`review_fillet_overrun_nearest_fit_r1/r2`):
  following the reported deficit leaves an arc leg's remnant past the
  fillet short enough to flatten, and the door refuses it the same way
  at 4 entries at 1e-9 and 30 at 1e-6 (`margin` 2.8e-6 to 5e-4, every
  `resolution` near 1e-17). The census counts them past the corner
  (`common::past_the_corner`) with a per-row pin.

The refusal is right in kind: the stored chord turns definitely from
the fillet, and the body extruded from it would build that corner sharp
under a constructed tangency. What this row asks is unchanged and more
pressing: the variant blames the scene's resolution, whose own field
reads four to thirteen decades below ε.

---
id: main-red-the-teapots-spout-join-refuses-in-the-join-since-6862
kind: issue
title: main red: the teapot's spout union refuses at the join's attachment gate since PR 6862, where the tour and the north-star rows pin the germ frame refusal
status: closed
opened: 2026-11-12
priority: P1
cost: E
closed: 2026-11-12
pr: 6879
branch: sectarms/spline-curved-at-rest
---



Found by SEAL's lane on PR 6860 (`seal/cut-tool-material`): CI's python suite on the PR's merge with main. Main's push CI does not run the python suite, so the red shows on the next PR that runs it. PR `placing/named-source` hit it too.

## What

The teapot's spout union (the belly's sphere against the spout's lofted NURBS walls) used to stop at the join, which found no frame for that germ pair (`BooleanError::GermFrameUnsupported { a_kind: Sphere, b_kind: Nurbs }`). Since the merge of PR 6862 (`sectarms/join-reads-spline-implicit`, `7d83cdff9e`), the spline-implicit lane traces that section. The union now refuses one step later, in the join's attachment gate: `BooleanError::Join`, "the operands' sections could not be joined: an Euler operation refused: geometry attachment gate: … a spline beside a curved analytic surface has no certificate".

Measured on wheels built from `7d83cdff9e^1` (`c8c2b4f70b`: `inner_kind` `germ_frame_unsupported`) and from `7d83cdff9e` (`inner_kind` `join`), the debug cdylib, running the teapot's own recipe.

- **The north-star row.** `crates/pncad-py/tests/test_north_star.py`, `TestTeapot.test_the_handle_join_builds_and_the_spout_join_refuses`, pinned the old arm. PR 6860 re-pins it to the join's arm and the gate's sentence, and asserts the germ frame text is gone.
- **The tour scene.** `demos/tour/src/teapot.rs` `wall3_pinned` still matches `GermFrameUnsupported { Sphere, Nurbs }`, so the tour's teapot wall 3 is expected red at the next nightly. Not run here: the lane did not build the demos.

## Done when

Tour wall 3 pins what the kernel now does: the join's refusal at the attachment gate, with its payload and the ε at which it holds. The module docs' narration of the spout's refusal reads the same. Or the attachment gate certifies a spline section edge beside a sphere, and both rows pin the built union.

## The stop moved again: containment (2026-11-12, `sectarms/spline-curved-at-rest`)

With the at-rest arm for a sphere, cylinder or torus beside a spline wall
(`edge_nurbs::curved_nurbs_limbs`), the spout union's section edges
certify at the attachment gate and the join completes. Measured on a
wheel built from that branch, running the teapot's recipe at the default
ε: `inner_kind` `containment`, "the Boolean cannot tell what is inside
the solid: one of its faces is a spline (NURBS) surface, which the
inside/outside test has no way yet to cross"
(`work/inside/point-containments-ray-test-has-no-spline-arm.md`).

- **The north-star row** is re-pinned there in that PR
  (`TestTeapot.test_the_handle_join_builds_and_the_spout_join_refuses`:
  the arm, the test's sentence, and neither the join's sentence nor the
  certificate's).
- **The tour scene** is not: `wall3_pinned` still matches the germ
  frame, and what it has to pin is now
  `BooleanError::Containment(PointInSolidError::KindUnsupported { kind: Nurbs })`
  at the default ε. The tour pins "at every ε", and that is no longer
  one stop: at ε = 1e-12 a ball's chord on a loft wall refuses at the
  gate on the lane's own image
  (`a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung`),
  so wall 3's rows at 1e-6 and 1e-12 want measuring before it is pinned.
  Not run here: the lane did not build the demos.

## Closed: the tour's wall 3 pins what the kernel does (2026-11-12, PR 6879's fix pass)

`demos/tour/src/teapot.rs` `wall3_pinned` is re-pinned, measured on the
tour's own test (`teapot::wall_probes_run_here::every_teapot_wall_is_attempted_by_the_suite`,
the `ci-tour` profile) at each ε row:

| ε | the spout union's refusal |
|---|---|
| 1e-9 (default) | `Containment(KindUnsupported { face, kind: Nurbs })`, `face` the spout's outer wall |
| 1e-6 | the same |
| 1e-12 | `Join(… Euler(Certification { error: Escalated { check: PlaneNurbsHullValue, … } }))`, the hull value 1.01e-12 m |

- The face is resolved in the spout's body and matched by its corners
  (the root at `SPOUT_R0` and `SPOUT_OUTER_TIP`), as the germ frame's B
  face was. The belly is no longer named: the containment refusal
  carries one face.
- The 1e-6 row no longer floors in the sweep
  (`SplineCrossingUnsettled { why: Floor }`, which the module's
  narration recorded): it reaches containment as the default row does.
- The predicate takes the run's tolerance and panics at an ε it was not
  measured at.
- The module's narration, the wall's comment and the note's caption say
  the same.

The north-star row was re-pinned in the same PR. Both rows move again
when `work/inside/point-containments-ray-test-has-no-spline-arm.md`
lands.

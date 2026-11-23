---
id: under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop
kind: issue
title: Under the guided lift, the ray walk's schedule axes meet axis-aligned geometry exactly at the nominal (plate_param's point_in_loop_arm gate, boss_union's point_in_arc_loop_side graze), so the revalue apart move flips them and the builds part
status: open
opened: 2026-10-17
priority: P3
cost: M
---


Found by the shared replay session unit (`inlane/sym-replay-session`). The revalue instrument now reads the door's session with the guided lift (`editor_core::coincide::with_replay_session`).

- **Move:** Apart (`crates/editor-core/tests/revalue_corpus.rs`, `Move::Apart`). Every one of plate_param's 41 continuous variables moves by its own multiple of 1e-7; every authored length is a free variable, not only `hole_r`.
- **Decision:** #6035, `point_in_loop_arm`. The margin is 0.0 exactly at p0 and 1.9852281229370488e-7 at p1, so the sign flips. After it the two builds decide different predicates: they agree through #6035 and first differ at #6036, of 15,539. The apart move compares only those 6,035 (no laundering in them).
- **Repro:** `REVALUE_APART=1 REVALUE_DETAIL=1 REVALUE_DOC=plate_param cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`

Under the pinned lift the profile did not move with its variables, so this decision never moved either.

**What the margin is.** Read from the code, and confirmed by PR 5257's review; the fix is not yet run. `point_in_loop_arm` is no reading of the point against the loop. It is the ray-schedule gate in `topo::splitting::containment::walk_schedule` (`crates/topo/src/splitting/containment.rs:645`), reached from `polygon_walk`. Its margin is the in-plane displacement that one schedule direction commands at the loop's extent, `|r − n̂(n̂·r)| / |r| · extent`. That is exactly 0 when the face's normal is exactly that schedule member, as an axis-aligned plate face's is. The apart move tilts the plane by about 1e-7, so the same member then casts a ray along a direction normalised from a ~1e-7 in-plane vector, and the walk after it is another walk. The point is not on the loop; the gate is choosing a probe direction from a margin whose zero is the document's axis alignment. So the fix is the gate's, not the classification's: a gate that is not decided at an exact zero for an axis-aligned plane, for example a threshold away from zero on the sine. This is a different mechanism from boss_union's `curve_pair_span_branch` flip, which was a span placement read at an undefined foot (`geom_brep::curve_pair::Span::read`, fixed by reading the carrier first).

**A second instance, boss_union (`point_in_arc_loop_side`).** With the narrow phase's axis merge removed (the kernel fix tried and rejected on `bool-pair-gap`'s item, branch `inlane/bool-pair-gap-order`), boss_union's apart move compares 5,804 decisions and parts at #5804, `point_in_arc_loop_side` (`topo::ray_walk::ray_crossings`). That margin, a vertex's offset from the ray's line, is exactly 0 at the nominal and 7.000005503301634e-8 at p1: an axis-aligned schedule ray passes exactly through an axis-aligned vertex, so p0's walk grazes and retries where p1's does not. Same root: the schedule's axis members meet authored axis-aligned geometry exactly. One fix serves both, for example leading oblique schedule directions, which no authored axis-aligned face normal or vertex offset meets exactly. Once the instrument pairs decisions by what they decide (that item), the apart move reaches it on main too.
- **Repro:** on `inlane/bool-pair-gap-order`, `REVALUE_APART=1 REVALUE_DETAIL=1 REVALUE_DOC=boss_union cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`

The gating row `revalue_plate_param` is now pinned under the in-step move (15,539 compared, 7,877 moved, nothing laundered). The apart move's power over this document is lost until this is resolved.

declared_tangency's exact-zero flips, once described here as a second instance, are another mechanism and have their own item, `under-the-guided-lift-declared-tangencys-pcurve-chart-azimuth-frame-flips-from-an-exact-zero`.

**Pinned (2026-10-19, PATHS #5068's fix pass).** `revalue_corpus::revalue_plate_param_apart_to_its_parting` holds plate_param under the apart move over the 5,939 decisions before the `point_in_loop_arm` flip (5,079 moved, nothing laundered), and pins the parting there. A fix here moves that pin, and returns `revalue_plate_param` to the apart move.

---
id: every-pair-question-has-one-total-table
kind: unit
title: Audit every geometric pair question (point/curve/surface against each other) for one total kind-pair table with one method per pair and no runtime fallback, and propose the general principle for DESIGN.md
status: open
opened: 2026-10-14
priority: P2
cost: M
design: true
needs_ev: true
refs: [two-curve-curve-meeting-finders, 5027]
---


## Ev's question

On PR 5027 (2026-10-14), after accepting the C5 curve-pair paragraph, Ev asked whether the principle should generalize further, and whether anything else is out of compliance with it. "That could be a future unit."

The principle in question: every geometric pair question has exactly one total kind-pair table, one method per pair, and no runtime fallback. The pairs are point, curve and surface against each other.

## Known non-compliance (orchestrator's first list, 2026-10-14)

- **Curve × surface has no table.**
  - `boolean::carrier_touch` is a bisection with conic-only speed bounds; it refuses spiric and spline carriers.
  - `circle_roots` and `conic_quadric` each have their own door.
  - Conic × plane is solved twice: `carrier_cross::circle_meets_plane`/`ellipse_meets_plane` and `splitting::conic_plane_candidates`.
- **The census edge-face cut** reads only chords and conics; anything else is `Uncrossable` (restread row).
- **Line × line is spelled three times.** `the-curve-pair-reader` absorbs this.
- **Point against curve has four readings.** `the-curve-pair-reader` absorbs this.
- **Point against face has two routines.** They now share one core, and each keeps its own boundary decision.
- **geom-brep SSI and props** run six subdivision walks of their own (`issues/geom-brep-keeps-six-subdivision-walks-of-its-own`).

## What it owes

1. **Sweep every pair question** in the kernel. For each, record:
   - the table it should have;
   - its arms today;
   - which pairs are off-table, duplicated, or reached through a runtime fallback.
2. **File each gap** on its owner's slate.
3. **Propose the general principle as a `docs/DESIGN.md` revision** in its own `[ev]` PR, weighed by a designer pair if the shape of the principle is itself a fork.

## The audit (2026-10-14, main at the branch point)

The sweep's patterns:
- `match` on two kinds;
- `_ =>` arms and their values: a default value, `None`/`false`, a typed refusal, or delegation to another method;
- `.or_else` and "else" ladders;
- the refusal vocabulary (`Unread`, `Uncrossable`, `*Unsupported`, `Intractable`, `NoArm`, `Unsettled`, `Uncertain`, `OutsideInventory`, `KindsDiffer`, …);
- formula text for the common residuals.

**Blind spot.** The sweep reads shapes, not reachability. Several things can be under-counted:
- dispatch written as sequential `if let`/`matches!` chains, Option gates or trait methods;
- a wildcard that a caller makes unreachable;
- a `None` that a caller later types;
- a question spelled in a different algebraic form.

Tests, `*_rows.rs` and `*_probes.rs` were excluded. These were skimmed only: `sweep/blend/{reach,surgery,open}`, `mesh`, `step-export`, `editor-core/mate`, `coincide::prove`, and `topo::{shell,offset_together,offset_general}`. Line numbers are in the filed rows.

| Question | The table it should have | Arms today | Off-table, duplicated, or fallback | Filed on |
|---|---|---|---|---|
| Section of two surfaces | `intersect::route` (C5) | `route`/`route_pose` exhaustive, no wildcard; only `replace_face` consults it | Five tables beside it: `section_cert::classify` (`_ => Intractable`), `join::kind_arm`/`pair_section_frame_at` (`_ =>` → runtime edge-lane fallback), `offset_derive::section_closed`, `chord_join::section_case` (declared narrowing), `offset_axial` profile solve. Parallel unequal cylinders are refused by `route_pose` and served three other ways | `hone/surface-pair-sections-are-classified-by-tables-beside-route` |
| Which pairs a moved face can section | `route_pose` | `route_pose` says S×S, Cy×S served without a pose check | `section_closed` then refuses both: two sources of truth | `shell/replace-face-trusts-route-pose-for-pairs-section-closed-refuses` |
| Tangency point / tangent line of two surfaces | an outcome of the section table | `TangentPoint`/`TangentLine` in the C5 arms | `census::point::face_face` (private enum), `locus::closed_form_locus`, `section_cert` `Tangent`: three methods each | `tang/a-carrier-pair-tangency-is-read-by-three-methods` |
| Census face-pair confirm | dispatch to the pair's one reader | four readers | trial ladder: planes, then carrier ladder, then point touch, then tangent locus | `restread/census-face-pair-confirm-is-a-trial-ladder` |
| Second-order tangency (wedge) | one jet reading | `dihedral::tangent_second_order` | `contact_verify`'s own meter | already `pred/lever-arm-fold-six-hand-rolled-siblings` (issue 1439) |
| Same carrier (±) | one coincidence table | `carrier_eq` ladder, mixed kinds `Distinct`, cone/Nurbs/Approx `OutsideInventory` | seven readings; `vtxfac::one_carrier` datum fallback; `merge_faces` unread → silent `false`; `adopt` raw eps | `hone/carrier-coincidence-is-decided-by-seven-readings` |
| Coaxiality of revolution carriers | one coaxiality reading | none shared | five tests; `glue::coaxial_rows` `_ => continue` and a dropped `Err` | `hone/coaxiality-of-two-revolution-carriers-is-tested-five-ways` |
| Point to surface: residual, on-carrier | one residual per kind | `implicit_residual` exhaustive (Nurbs/Approx poison) | seven spellings over different kind subsets; on-carrier decided exact in `contain`, linearized in `solid_contain` | `hone/point-to-carrier-distance-is-spelled-seven-times`; `shell/offset-axial-reads-an-unlisted-kind-as-a-default-value` (latent `_ => 0`) |
| Point to surface: chart coordinates | one inversion per kind | `topo::chart::Chart` | three copies; `stable_azimuth` twice | `chart/chart-inversion-is-spelled-three-times` |
| Point in face / in solid | one point-in-face per kind | `contain::face_placement` | `_ =>` delegation, `Trim(None)` gaps, `solid_contain::face_geo` catch-all and fallback chains | `inside/point-in-face-and-point-in-solid-read-a-curved-trim-by-different-chains` |
| Curve crossing surface | one `(CurveKind, SurfaceKind)` table | none: per-pair root doors | `wall_crossing`/`line_wall_roots_of`/`conic_quadric_roots` wildcards; enclosure-then-roots-then-`carrier_touch` and harmonic-else-sampled ladders; circle×plane in about six places; line×cone twice; census confirm two arms | `roots/curve-surface-crossing-has-no-kind-pair-table` |
| Curve meeting curve; point on span; `carrier_cross`; census edge-face cut | `(CurveKind, CurveKind)` table, one point-on-span reading (C5) | being built | three line×line spellings, four point-on-curve readings, `carrier_cross` `_ => Ok(None)` → `Unread`, `Uncrossable` | `fuse/the-curve-pair-reader` (another lane, not refiled) |
| Tangent-contact certificate lane | product of unary predicates | `tangent::tangent_certificate_lane` | complies: `_ => false` becomes a typed `NotCertifiable` | none |
| Blend support pair → arm | the blend's own table | `battery::classify_arm` | a declared lane with a typed refusal; its coaxiality test is one of the five above | under coaxiality |
| SSI and props subdivision | one certified walk | six walks | duplicated method | already `issues/geom-brep-keeps-six-subdivision-walks-of-its-own` |
| Face separation / clearance | kind-agnostic (boxes, interval subdivision) | no pair table | not a kind-pair question; `editor-core` clearance refuses Nurbs/Approx typed | none |

Single-kind wildcards that return a value rather than refuse (for example `pcurve_cache` `chart_image_harmonic` `_ => 0`, `param_rate_gate` `_ => 1`, `mesh` `circle_radius` NaN, `sectors`/`merge_faces` `_ => Curved`) are not pair tables. They fall under the same rule's spirit: an exhaustive match whose unlisted kind refuses typed. Only the `offset_axial` pair, which feeds a decision, is filed here. The rest are left for the principle's ratification to decide whether they are in scope.

Part (b) of 4: the nine items, at head 3494faec95.

1. **A body that meshes on main does not refuse.** The route reads stored image facts before it asks the doors: `crates/mesh/src/tessellate.rs:274` (`route_of`).
   - Order: rings → trimmed; an outer image of a non-iso form (`Pcurve::is_non_iso_form`, crates/geom-brep/src/pcurve_cache.rs:1161) → trimmed with no door asked; otherwise the doors.
   - Your eight tilted-ball cells: crates/mesh/tests/route_rows.rs:200. They mesh to main's bytes.
   - An iso-imaged face whose door escalates cannot be built: the mint escalates at the same band (`pcurve_cylinder_chart_incident` and `pcurve_map_residual`). The pinned row is an image-less wall put 3 zero-bands off its surface: route_rows.rs:329. It refuses `Escalated` with the door's cause at ε 1e-9, 1e-6 and 1e-12 (the offset is band-relative).

2. **`NotIsoRectangle` is split.**
   - Incidence misses now answer `OffSurface` on the refusing path, in crates/geom-brep/src/props/curved.rs:
     - cylinder "not a rim" when the rim fit misses;
     - sphere `props_meridian_great`;
     - torus `props_rim_fit` for a coaxial circle;
     - torus `props_meridian_fit` and `props_meridian_plane`.
   - Kernel states now answer the new `Defect` (crates/geom-brep/src/props/mod.rs:411), and spans the new `SpanUncertified` (mod.rs:392).
   - `NotIsoRectangle` is only the decided verdict now, standing row 2.
   - Rows: crates/geom-brep/tests/incidence_at_the_shape_door.rs (three OffSurface rows) and torus_meridian_radial.rs (Defect). Every SpanUncertified pin was re-pinned.

3. **Only what the chart lane can read is routed there.** A door-decided face goes to the trimmed lane only where `face_lift` reads it (tessellate.rs:274). A `Gap` cause gives `DomainUnread`; a structural miss gives `UnsupportedCurvedShape` with the door's verdict.
   - D9 re-pinned on witnesses with their images minted (`witness_bodies::minted`):
     - keyway `0xa110_eb9f_658b_9246`: the face meshes, and its complement refuses `TrimLoopOrientation` because of the fixture's winding;
     - lens `0x1d3b_f531_e34e_7286`: OuterOpen, main's answer;
     - half cap and bow tie: back on main's constants.
   - The same re-pin applies to mesh11_arc_branch, mesh7r1/r2 probes and the mesh `iso_rectangle_door`.

4. **K stream.** `FaceRoute` (tessellate.rs:375) keeps `asked` (the doors) apart from `read` (the domain read). The fold at tessellate.rs:750 splices `asked` only when the lane runs (not on a memo hit) and splices `read` always. The contract is stated in `tessellate_with`'s doc. Row: route_rows.rs:237 — nine bodies record main's K stream, on a miss and on a memo hit, at all three ε.

5. **The rectangle lane needs the doors' answer.** A `Rectangle` token (crates/mesh/src/curved.rs:516) is minted only by `iso_rectangle_shape` (curved.rs:555), which matches every `PropsError` variant exhaustively, and `tessellate_curved` requires it (curved.rs:193). `Routes::lane` returns `Routed::To` or `Routed::Refused` (tessellate.rs:366, 386). With the branch door skipped, 6 rows go red.

6. **Rows added.**
   - Red row for the kind pre-pass: crates/geom-brep/tests/iso_rectangle_door.rs:554; it is red without the pre-pass at props/curved.rs:377. The one-home table is `IsoChart` at props/curved.rs:448.
   - Escalated and OffSurface through `tessellate`: route_rows.rs:329 and :366.
   - Kind before band: route_rows.rs:430.

7. **Smaller items:**
   - `chords`, `face_target` and `arm_steps` take the route value.
   - `has_trim_carrier` and `has_tilted_circle` are gone.
   - `Routes::witness` is hoisted out of the per-face loops.
   - Docs updated: types.rs `UnsupportedCurvedShape`, sizing, walk, PERF-1-SPEC, pncad-py.
   - The notched rows are in the nextest slow set.
   - cover-kind-match is back at 10 (one lazy `.cover()` site in the torus split).

8. **Issues filed:**
   - work/conearm/the-branch-door-reads-an-escalated-apex-or-pole-decision-as-one-branch.md
   - work/tesslane/a-seam-cornered-cylinder-notchs-rectangle-neighbour-exceeds-its-certificate.md
   - work/ciw/a-chart-branch-guard-row-is-red-locally-and-green-on-the-hosted-kernel-job.md (notes that PR 7051 fixed the count)
   - work/tesslane/a-rectangle-the-walk-refuses-has-no-witness-the-chart-lane-can-read.md
   - plus a note on work/tesswalk/apex-crossing-branch-premise-has-no-body-level-witness.md

9. **Design questions:** answered in the PR body (part c), under "The route" and "No face that meshes on main changes lane".

**step-import `split_iso_side`.**
- What moved it: on 3a2d0aa0b5 the route sent any harmonic image with nonzero off-iso coefficients to the trimmed lane. Under the oblique placement the rims carry sine terms of 1e-17, so rectangles were routed as moving: [16, 16].
- What moved it back: from 2bf2bfe084 only stored non-iso forms route (Projected, FocalSection, Spiric, Fitted, General), and a harmonic image always goes to the doors. Back to [28, 64], matching main. The armed mesh run independently panicked on the same thing on a posed tube (`ringed_curved_faces_that_still_refuse_answer_by_cause`).

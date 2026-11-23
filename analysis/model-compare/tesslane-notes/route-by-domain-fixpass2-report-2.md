Report part 2 of 3.

**D. Docs:**
- `tessellate_with`, `Routes` and `FaceRoute` now say what the fold does: the doors and the split-side fact are spliced where the lane runs; the domain read is spliced on every run, including a memo hit.
- `SizingTols::band`: three consumers (the wrap strip, the trimmed lane's singular vertices, and a trimmed face's domain read at its turn via `Route::ready`), plus the route, which reads the band before the value exists.
- validate.rs: the `Defect` arm has its own comment; the `SpanUncertified` comment is back on its own arm.
- `NotIsoRectangle`'s plane arm: the doc was right and the code was wrong. All four curved doors asked of a plane (`require_iso_rectangle`, `require_one_chart_branch`, `boundary_material_sign`, `curved_face`) now answer `Defect`, so the routing would never rank one row 2 and send it to a lane. Two geom-brep rows were re-pinned.
- `torus_classify`: one comment.

**E. D9.** The corpus is restored to the four unminted witnesses, with their constants moved. The minted forms are kept as four new D9 bodies (`*_minted`); I chose D9 bodies over route_rows rows.

| witness | unminted, head | minted, head |
|---|---|---|
| keyway | `0xe457_7191_dbec_3540`, `DomainUnread` `Gap` | `0xa110_eb9f…`, `TrimLoopOrientation` on the complement face (the keyway face meshes) |
| lens | `0x72fb_aee1_b444_dff6`, `DomainUnread` `Gap` | `0x1d3b_f531…`, `DomainUnread` `OuterOpen` (main's minted answer) |
| half cap | `0x72fb…`, `DomainUnread` `Gap` | `0xdd6f_a9f2…` = main's constant, `NotOneChartBranch` |
| bow tie | `0xbd72_11b8_010f_3772`, `DomainUnread` `Gap` | `0x9cdd_b7a3…` = main's constant, `NotIsoRectangle { props_rim_side }` |

Main's unminted constants were 0x9d8e_44fd…, 0xfdb2_0be5…, 0xdd6f…, 0x9cdd…. The header names each move and why, as the file's rule asks.

**F. Memo-hit cost.** Measured on this head after the integration, with a temporary row (not committed): 60 warm hits per body at δ 0.05, the mesh test binary, three alternating runs, medians.

| body | main 184e484ed2 (µs) | head (µs) |
|---|---|---|
| washer | 151–161 | 230–267 |
| rounded prism | 186–194 | 282–376 |
| cone | 89–92 | 123–152 |
| donut | 317–533 | 419–510 |
| ball | 147–185 | 184–243 |

Filed P3, with both cures: `work/tesslane/a-memo-hit-pays-for-the-route-the-lane-used-to-skip.md`.

**G. Rows.**
1. route_rows' ellipse row now goes by `an_ellipse_wall_in_band_with_no_image_refuses_as_the_lane_does`. Its doc says what it pins and names geom-brep `iso_rectangle_door::a_carrier_of_no_iso_kind_is_decided_before_an_edge_in_band` as the pre-pass's pin: in every mesh body the ellipse comes first in its loop.
2. `route_rows::an_obliquely_placed_rectangle_meshes_on_the_rectangle_lane_to_mains_bytes`: the washer posed obliquely, minted, digest `0xac0a_86a0_1d7d_e5b0`. I read that digest on main 184e484ed2 at 1e-9, 1e-6 and 1e-12, and it equals this head's. Under the harmonic-widened mutant (non-exact-zero harmonic channels read as moving) the digest becomes `0xe5bb_b25e…`, so the row goes red.
3. `far_walls_of_many_arcs`' `early` takes (class, name) pairs: tilt → `OffSurface`/`Escalated` `props_cylinder_circle_tilt` or `NotIsoRectangle` `props_rim_axis_parallel` (which one depends on ε, measured at all three); whole turn → `SpanUncertified`; the two open loops → `ChartLoopOpen`. Green at all three ε.

**H. Structure.**
- `Route::Trimmed(Trim)`, where `Trim` is either `Structural` (rings, a non-iso image, fact 4, a spline chart) or `Read(FaceLift)` (the doors decided no rectangle and the route read the domain).
- One function reads a domain, `read_domain(body, fk, band, verdict)`. It maps a non-`Gap` miss to the doors' verdict where there is one.
- `Route::ready(self, ..) -> Ready { Planar, Curved(Rectangle), Trimmed(FaceLift) }`. The worker runs on `Ready`: no `Option` and no `unreachable!`.
- I designed it together with fact 4, which is `Trim::Structural` with the token unspent.
- I first tried reading every domain at the face's turn. That moved three unminted D9 witnesses from `DomainUnread` to a chord-pass `UnsupportedCurve` (the chord pass runs before the turn and needs an image to bound), so for faces the doors decide no rectangle the read stays in the route.

**I.** Filed P3: `work/tesslane/three-readings-of-a-non-iso-boundary-disagree-on-a-line-on-a-sphere.md`, citing topo props.rs:4984/4998, props/curved.rs:524 and pcurve_cache.rs:1161.

**J.** Retitled: "A rectangle the curved walk refuses cannot be handed to the chart lane by any route, so whether that lane meshes it is unmeasured".

**CI follow-ups:**
- from-f64: the samples now step in `T`.
- clippy: `#![allow(clippy::expect_used)]` on the incidence test file, as the crate's other carrier-building rows have it. My local clippy had lacked `--all-features`; it now runs CI's exact invocation.
- Slow set: the notched split-ruling row is added beside its file's other slow row. Not trimmed: its invariant is that every split cell answers as the unsplit wall does, and that wall meshes here, so each cell must mesh.

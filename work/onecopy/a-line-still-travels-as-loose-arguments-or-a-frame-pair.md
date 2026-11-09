---
id: a-line-still-travels-as-loose-arguments-or-a-frame-pair
kind: issue
title: A line still travels as loose (point, dir) arguments, or as a conic's (center, axis) frame pair
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-09
pr: 6665
---


**Open question first: is a ray a `geom::Line` read forward (advance `s > 0`), or a half-line type of its own?** The ray doors below cannot be typed until that is decided. A half-line type would carry the forward reading (`advance`, ahead or behind) that every ray door now decides by hand. A `geom::Line` would leave each door to decide `s > 0` itself.

Found by the sweep of `onecopy/line-tuples` (PR 6601) and corrected by its review. That unit ported the line half of `plane-and-line-tuples-restate-the-placement`: every `(Point3<T>, Vec3<T>)` tuple that stood for a line now carries a `geom::Line`. The line holders below are left. Each is a loose two-argument pair, which the tuple sweep cannot see, a door on the open ray question, or a family too large to port beside the others.

**The ray doors** (on the question above):
- `crates/topo/src/boolean/solid_contain.rs`: `ray_crossings`, `carrier_roots`, `edge_meets`, `ray_passes_beside`, `line_torus_roots` and about five more take the ray as loose `q: Point3<T>, d: Vec3<T>`. `reach_margin` and `reaches_root_in_band` take it as a `(q, d)` tuple. `line_torus_roots` also takes the torus axis loose (`center, axis`).
- `crates/topo/src/boolean/solid_contain/curved_edges.rs` has two doors: `read` takes the 3-tuple `(q, d, reach)` (~`:91`), and `narrow` takes `(q, d)` (~`:238`).
- `crates/topo/src/boolean/trace.rs` `ray_crossings` (~`:194`) takes `origin, dir` loose, with `t > 0` read by its own doc.
- `side_reader.rs` `ray_counts` (test support) and `r1_probes.rs` `centre_and_axis` feed the same doors.
- `crates/topo/src/ray_walk.rs` `ray_crossings` (~`:700`) is generic over `RaySpace` (`q: P, d: P::Disp`, plane or space). It is in scope only if the answer gives the half-line a planar sibling beside `Line2`; otherwise it stays generic over its space.

The family is left whole because porting half of it would leave two spellings at one boundary.

**`topo` `boolean/join.rs` the conic loop's frame.**
- `turn_of`, `rotational_sense`, `Turn::facing`, `Turn::frame`, `SectionLoop::turn`, `SectionLoops::conic`, `conic_section_frame_at`, `pk_germ_frame` and `PkSection::Loop` carry the conic's axis line as `(center, axis)`, read as the moment `axis·((p − c)×v)`. That is about 30 source sites and 22 test rows.
- The same structs carry the germs `(site, dir)`, which are tangent samples, not lines. So a port has to separate the two first.
- In the same family: `crates/topo/src/boolean/section_cert.rs` `torus_plane_loops` (~`:1054`) takes the torus as the 4-tuple `(c, a, R, r)`. Its axis line is `(c, a)`; its plane, `(p0, n)`, is the plane row's.

**`MappedSource::RevolvedPoint { axis_origin, axis_dir }`** (`geom-brep` `mapped.rs`). A revolve axis held as two fields and read across five crates:
- `geom-brep` `certify.rs`;
- `sweep` `revolve/full.rs`, `revolve/surfaces.rs` and `revolve/turn.rs`;
- `step-import` `adopt.rs`;
- `topo` `transform.rs`, `offset_axial.rs`, `replace_face.rs` and `splitting/containment.rs`.

Fix: settle the ray question, then a `geom::Line<T>` (or the half-line type) at each door, as the line-tuples unit did.

The sweep cannot see a line passed as two loose arguments under other names, or one stored as two struct fields. The shapes it did run:
- `q: Point3<T>,` followed by `d: Vec3<T>` on the next line;
- `(Point3<T>, Vec3<T>)` in type position.

## Closed (2026-11-09, PR 6665)

- **The ray question.** It is ruled as `geom::Ray { line }`, with one plain reading `advance(p)` and one decided reading `ahead(at, site, band)` under the reader's own row.
  - The solid_contain ray doors, `curved_edges`, `trace::ray_crossings` and test-support `ray_counts` take `&Ray`.
  - Every hand-spelled forward test reads `Ray::ahead` under its existing row, so the K stream is unchanged.
  - The line × surface root doors take `geom::Line`s.
  - `ray_walk` stays generic over its space: there is no `Ray2`.
- **`join.rs`'s conic loop frame**, `section_cert`'s torus axis and `MappedSource::RevolvedPoint` carry a `geom::Line`. A stored `Frame` is read through `Frame::axis_line()`, and a stored surface through `Surface::axis_line()`.
- **The swept axis pairs in topo, geom and geom-brep** take or hold a `Line` (the PR lists them).
- **What remains is filed:**
  - `props/curved.rs`'s family is `work/conearm/props-curved-carries-a-revolution-axis-as-loose-origin-and-axis.md`;
  - the holders that carry a full frame are on `analytic-surfaces-spell-their-frame-field-by-field`.

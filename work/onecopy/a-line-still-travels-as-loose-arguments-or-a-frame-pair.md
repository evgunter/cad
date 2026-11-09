---
id: a-line-still-travels-as-loose-arguments-or-a-frame-pair
kind: issue
title: A line still travels as loose (point, dir) arguments, or as a conic's (center, axis) frame pair
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by the sweep of `onecopy/line-tuples`. That unit ported the line half of `plane-and-line-tuples-restate-the-placement`: every `(Point3<T>, Vec3<T>)` tuple that stood for a line now carries a `geom::Line`. These line holders are left. Each is either a loose two-argument pair, which the tuple sweep cannot see, or a family too large to port beside the others:

- **The ray-cast family**, `crates/topo/src/boolean/solid_contain.rs`. `ray_crossings`, `carrier_roots`, `edge_meets`, `ray_passes_beside`, `line_torus_roots` and about five more take the ray as loose `q: Point3<T>, d: Vec3<T>`. `reach_margin` and `reaches_root_in_band` take it as a `(q, d)` tuple, and `curved_edges::read` as `(q, d, reach)` (`crates/topo/src/boolean/curved_edges.rs` `narrow`). `side_reader.rs` `ray_counts` (test support) and `r1_probes.rs` `centre_and_axis` feed the same doors. `line_torus_roots` also takes the torus axis loose (`center, axis`). The family was left whole because porting half of it would leave two spellings at one boundary. Also open: whether a ray (advance `s > 0`) is a `geom::Line` read forward, or a half-line type of its own.
- **`topo` `boolean/join.rs` the conic loop's frame.** `turn_of`, `rotational_sense`, `Turn::facing`, `Turn::frame`, `SectionLoop::turn`, `SectionLoops::conic`, `conic_section_frame_at`, `pk_germ_frame` and `PkSection::Loop` carry the conic's axis line as `(center, axis)`, read as the moment `axis·((p − c)×v)`. That is about 30 source sites and 22 test rows. The same structs carry the germs `(site, dir)`, which are tangent samples, not lines. So a port has to separate the two first.
- **`geom-brep` `mapped.rs` `MappedSource::RevolvedPoint { axis_origin, axis_dir }`.** A revolve axis held as two fields and read across `geom-brep` `certify.rs`, `topo` `transform.rs` and `offset_axial.rs`.

Fix: a `geom::Line<T>` at each door, as the line-tuples unit did. For the rays, decide the half-line question first.

The sweep cannot see a line passed as two loose arguments under other names, or one stored as two struct fields. The shapes it did run are `q: Point3<T>,` followed by `d: Vec3<T>` on the next line, and `(Point3<T>, Vec3<T>)` in type position.

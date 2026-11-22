---
id: unclaimed-half-edge-reads-the-3532-table-does-not-list
kind: issue
title: a half-edge its own edge does not claim is read as its minus half: 27 sites no row lists
status: open
opened: 2026-11-22
priority: P3
cost: M
---


## What

The class of `unclaimed-half-edge-read-as-a-minus-half-*` (PR 3532's
sweep; its table is in each of those rows): an edge claims its two
halves in `he_plus` and `he_minus`, and a test of one slot alone reads
a half neither slot holds as the other half — a **direction** read
(`edge.he_plus == he` calls it the minus half), a **mate** ladder
(`if e.he_plus == he { e.he_minus } else { e.he_plus }` hands back a
half that is not its mate), a **claim filter** (`if edge.he_plus == he
{ push }` drops an edge neither of whose walked halves it claims) or a
**write** ladder (`if e.he_plus == old { e.he_plus = new } else {
e.he_minus = new }` overwrites the wrong slot). On a tier-1 body the
state cannot occur; the site answers wrongly only where the body is
torn, and none of them refuses.

Re-running that sweep on 2026-11-22 (the WALKS E-bundle, which closed
the unowned row) turned up these production sites that no row's table
lists — new since 3532 or moved out of its reach. Line numbers ride
along and may be stale:

- `crates/topo/src/shell.rs` `oriented_points` (~:2899), direction
- `crates/topo/src/shell.rs` `seamed_band` (~:3171), mate
- `crates/topo/src/shell.rs` `moved_walls` (~:5438, ~:5460), direction, two
- `crates/topo/src/region.rs` `boundary` (~:571), direction
- `crates/topo/src/census.rs` `half_edge_tangent` (~:9765), direction
- `crates/topo/src/sector_face.rs` `spline_normal` (~:494), direction
- `crates/topo/src/rim_wedge.rs` `face_boundary_arcs` (~:1034), direction
- `crates/topo/src/spline_trim.rs` `walks_forward` (~:1256), direction
- `crates/topo/src/stands.rs` `across_edges` (~:475), direction
- `crates/topo/src/boolean/reduce.rs` `sheets_part` (~:3931, ~:3933), direction, two
- `crates/topo/src/boolean/join.rs` `seam_germ_side` (~:5677), direction
- `crates/topo/src/boolean/arcs.rs` `arcs_along` (~:61), direction
- `crates/topo/src/boolean/fate.rs` `cut` (~:1955), direction
- `crates/topo/src/boolean/zip.rs` `one_vertex_sense` (~:1121), direction
- `crates/topo/src/boolean/overlap.rs` `half_plane` (~:536), direction
- `crates/topo/src/chord_join.rs` `IslandReading::of_closed_loop` (~:4347), direction
- `crates/topo/src/chord_join.rs` `closed_chord_runs_forward` (~:4813), direction
- `crates/topo/src/carve.rs` the removal walk (~:172), claim filter
- `crates/topo/src/clearance.rs` the edge uses (~:3902), direction
- `crates/topo/src/star/walk.rs` `departure` (~:458), direction
- `crates/topo/src/validate.rs` `mate` (~:9366), mate; `side` (~:9387), direction
- `crates/topo/src/movefac.rs` `trade_mates` (~:923), write
- `crates/sweep/src/blend/surgery.rs` the mate trims (~:5872), mate
- `crates/sweep/src/revolve/full.rs` `unslit_plane_wall` (~:1179), mate

The validator's two read raw records by design, but a validator helper
that defaults a torn claim answers instead of reporting it.

## Repair shape

As the sibling rows: read the pairing through `Edge::claim`
(`crates/topo/src/entity.rs`) — inside `topo`, `crate::live::claimed`
where a miss is a broken premise (it panics naming the half-edge and
the edge), or the site's own typed refusal where the site already
answers corruption as an error. Outside `topo`, `Body::mate` is public
and `None` on an unclaimed half; there is no public direction reading.

## The sweep

`grep -rnE '\bhe_(plus|minus) *[!=]=|[!=]= *[A-Za-z_0-9.()]*\bhe_(plus|minus)\b'`
over `crates/` outside `tests/`, 65 hits. The other hits: on a sibling
row (`cleavetail`: `splitting/finish.rs` `describe_section_boundary`,
`splitting/join.rs` `certify_section_area`'s conic term; `tesslie`:
`mesh` `trimmed.rs`, `walk.rs`; `band`: `blend/surgery.rs` four,
`blend/junction.rs` `plus_on_host`); not this class (`Edge::claim`
itself, the validator's self-pair checks and prose, `null.rs`'s and
`surface_boundary.rs`' two-half comparisons, `step-export`'s flag
ladder and `step-import`'s pair check, which test both slots); test
code (`tier3_tests.rs`, `review_m1_pr4.rs` five, `review_d18.rs`,
`split.rs` prose, `vtxfac_spline_rows.rs`, `validate.rs`
`closed_arc_pinch`, `splitting/finish.rs` `section_edge`,
`euler_kill.rs`, `revert.rs` and `merge_faces.rs`' tear hooks,
`side_reader.rs` `edge_frames`, `sweep` `blend/reach.rs`).

What it cannot match: a slot bound to a local before it is compared,
or a pairing read through `Body::mate` and then defaulted. A second
pass for `let (a, b) = (e.he_plus, e.he_minus)` / `let h = e.he_plus;`
bindings and for `mate(..).unwrap_or` found no site of the class
(`boolean/slit.rs`' two bindings and `split.rs`' read both halves).

## Done when

Every site above reads the pairing through `Edge::claim` (or a door on
it) and refuses an unclaimed half.

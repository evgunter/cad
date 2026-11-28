---
id: main-red-a-rims-centre-is-read-through-circle-frame-at-the-glued-stretchs-end
kind: issue
title: Main red: a rim's centre is read through Circle::frame at the glued stretch's end, which the conic-centre census pins at 0
status: closed
opened: 2026-11-23
priority: P0
closed: 2026-11-24
---


Main's `lint` job is red on the conic-centre census. On main `5fdf61e7be`,
`cargo clippy -p geom-brep --all-targets --all-features --message-format=json -- -D warnings --force-warn clippy::disallowed_methods | python3 scripts/conic_centre_census.py --check`
exits 1:

> crates/geom-brep/src/curve_face/end_root.rs: 1 production read(s) of a conic's centre doors, pinned at 0

The read is `circle.frame().origin()` in the `Curve3::Circle` arm at
`crates/geom-brep/src/curve_face/end_root.rs:223`. HELDRECORDS's elbow-ends
work wrote it (`368b8aa988`, merged with `5deafa798e`). ONEROUND's
`20d3889566` ("conic readers, last pass") made `geom::Circle::frame` a
disallowed door (`clippy.toml:31`) and the census holds each file's
production reads at its pin. The two merged in either order without either
CI seeing the other.

The fix is HELDRECORDS's to choose: read the rim through its readings
(`crates/geom/src/curves/conic.rs`, the census's own recourse), or argue the
read at the site and raise the pin. It is red on every PR whose diff seeds
`geom-brep`, and on the `tier = all` runs.

Found by the FORMWORDS orchestrator while following up a lane's note; not
FORMWORDS's.

## Closed

The read the row names was converted on main by `59892b0ee` ("geom-brep
end_root: a circle's end reads its radial through its axis, not its
centre frame"), another session's, before this row was taken up: on
`origin/main` `0b175aa35` the census reads geom-brep at its pins (69 reads,
17 files) and topo at its pins.

Swept for every disallowed-door read the elbow stack (PRs 7432, 7463)
added, with clippy's `--force-warn clippy::disallowed_methods` over
geom-brep and topo, in `end_root.rs`, `rim_wedge.rs` and `boolean/reduce.rs`
and `boolean/mod.rs`: one, `rim_wedge::rim_of`'s `circle.frame().origin()`,
which carried an `#[expect]` pointing at a closed ONEROUND row. It now
reads the centre through the circle's own readings (its point at the
parameter's zero lies `radius` along `u_ref` from it), and
`scripts/gates/conic-centre-reads.counts` lowers `rim_wedge.rs` from 4
to 3. The other hits in those files are in test code or are ONEROUND's
own, outside the stack.

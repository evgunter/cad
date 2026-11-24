---
id: main-red-a-rims-centre-is-read-through-circle-frame-at-the-glued-stretchs-end
kind: issue
title: Main red: a rim's centre is read through Circle::frame at the glued stretch's end, which the conic-centre census pins at 0
status: open
opened: 2026-11-23
priority: P0
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

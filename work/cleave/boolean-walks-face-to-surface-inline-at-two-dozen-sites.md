---
id: boolean-walks-face-to-surface-inline-at-two-dozen-sites
kind: issue
title: The Boolean walks from a face to its surface inline at about two dozen sites, each with its own refusal
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [boolean-join-resolves-germ-face-surfaces-in-three-closures]
---

Found by PR 5061's review (Q1), on the sweep of
`boolean-join-resolves-germ-face-surfaces-in-three-closures`, which gave
`boolean/join.rs` one walk (`join::face_surface`, pinned by
`face_surface_tests::the_join_walks_to_a_faces_surface_in_one_place`).

## What

Outside `join.rs`, `crates/topo/src/boolean/` walks
`get_face(f) → get_surface(fd.surface)` inline, each site with its own
handling of a face that does not resolve: some refuse with their own
message, some read it as "not this kind" (`Option::is_some_and`,
`matches!(…, Some(…))`), some `unwrap`/`expect`. The crate already has
the walk (`readback::carrier_surface`, behind `readback::face_carrier_kind`
and `query::face_surface_kind`), and `Body::face_surface_linked` reads a
live face's surface with the tier-1 guarantee.

Hit list (a `get_face` within one line of a `get_surface`, outside
`join.rs`, the `*_rows.rs` files and the probes; some may sit in test
modules):

- `boxes.rs:3083`
- `glue.rs:117`, `glue.rs:168`, `glue.rs:215`
- `insert.rs:2153`
- `mod.rs:1523`, `mod.rs:5905`, `mod.rs:6298`
- `ops.rs:4831`, `ops.rs:4927`
- `recl.rs:66`, `recl.rs:350`, `recl.rs:1258`, `recl.rs:1295`, `recl.rs:1500`
- `reduce.rs:1263`, `reduce.rs:1935`, `reduce.rs:5166`, `reduce.rs:5852`
- `rim_wedge.rs:561`
- `solid_contain.rs:2283`
- `vtxfac.rs:876`, `vtxfac.rs:919`, `vtxfac.rs:939`

Blind spot: a walk split across more than one line between the two
calls, or through a local (`let fd = get_face(..)?; … fd.surface`), is
not matched; `grep -n "\.surface)" crates/topo/src/boolean/*.rs` is the
second pass to run.

## Direction

Route each through one walk (a kind read through `query`, a surface
read through `face_surface_linked` behind one refusal), and decide per
site whether a face that does not resolve is a desync or an honest
"no", saying which at the site.


---
id: boolean-walks-face-to-surface-inline-at-two-dozen-sites
kind: issue
title: The Boolean walks from a face to its surface inline at about two dozen sites, each with its own refusal
status: open
opened: 2026-10-14
priority: P1
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

Two more private walks outside `boolean/` (PR 5061's re-review), each a
twin of `readback::carrier_surface` with its own refusal type:

- `chart_region.rs`'s `face_surface` (refuses `ChartRegionError`);
- `splitting/rules.rs`'s `face_surface` (panics on a face that does not
  resolve).

`join::face_surface` now delegates to `readback::carrier_surface`, which
returns the surface key beside the surface and sense for it.

Blind spot: a walk split across more than one line between the two
calls, or through a local (`let fd = get_face(..)?; … fd.surface`), is
not matched; `grep -n "\.surface)" crates/topo/src/boolean/*.rs` is the
second pass to run.

## Direction

Route each through one walk (a kind read through `query`, a surface
read through `face_surface_linked` behind one refusal), and decide per
site whether a face that does not resolve is a desync or an honest
"no", saying which at the site.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical, and it recurs.** It is now 29 sites, up from 24. Keep `Body::face_surface_linked` and repoint the private `face_surface` twins. It recurs the closed row `boolean-join-resolves-germ-face-surfaces-in-three-closures`. The likely cause is that the helper is awkward to call, not architecture: if the canonical helper is easier to call, the copies stop.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: two dozen inline copies of one walk: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

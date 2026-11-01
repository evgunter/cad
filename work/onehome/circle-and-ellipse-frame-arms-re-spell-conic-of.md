---
id: circle-and-ellipse-frame-arms-re-spell-conic-of
kind: issue
title: LoopArc::of and the section outline's Circle and Ellipse arms re-spell geom_brep::Conic::of
status: open
opened: 2026-10-14
priority: P1
cost: E
design: true
---

Found by the class sweep on `cleave/carrier-cross-one-curve`, which
built `carrier_cross.rs`'s and `carrier_speed.rs`'s conic frames through
`geom_brep::Conic::of` instead of re-spelling its Circle and Ellipse
arms.

Two more local frames re-spell those arms field by field:

- `crates/topo/src/ring_path.rs` `LoopArc::of`: a Circle arm and an
  Ellipse arm, each building `Conic { centre, axis, u_ref, a, b, .. }`.
- `crates/topo/src/splitting/section_loops.rs`, the outline walk's
  `OutlineEdge::Conic` arms: `a: u_ref * radius` / `u_ref * major`,
  `b: axis.cross(u_ref) * radius` / `* minor`.

Each can read `Conic::of(carrier)` once and keep only what differs (the
span; the outline's ellipse flag). Doing so also drops their reads of
`Curve3::Ellipse`'s field names, which PR 4861 renames.

`geom_brep::Conic::semi_axis_vectors` (added on the same branch) is the one
spelling of the frame `(semi_u·u_ref, semi_v·v_ref)`; the outline's
`a`/`b` are exactly it.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Design signal, a light one.** **Cluster B, a conic carrier's frame and semi-axis view.** It is spread over geom's `SemiAxes`/`major_frame`, `geom_brep::implicit::Conic` and three topo-private `Conic` structs. It recurs the closed carrier-cross conic-helper row. `ruled_region::Conic::of` is a fourth copy the row missed. Unclear: whether geom or geom-brep owns a conic carrier's frame view, and whether it carries a span. Once that is answered, the rest is mechanical (see `semi-axes-by-size-misses-seven-inline-sites`).

## Slate triage (2026-10-26, CLEAVE)

- Banded P1: two frames re-spell `Conic::of`'s arms: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Priced `E`, the effort the body describes.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

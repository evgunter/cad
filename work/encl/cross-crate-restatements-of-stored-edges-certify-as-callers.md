---
id: cross-crate-restatements-of-stored-edges-certify-as-callers
kind: issue
title: Sweep restates stored edges through the public set_edge_curve, so their span refusal is typed
status: open
opened: 2026-10-19
---


Found by the sweep for
`work/encl/kernel-computed-specs-at-callers-sites-return-a-kernel-bug-typed.md`.

Inside `topo`, a kernel operation that restates a stored edge's carrier
and interval now certifies through `Body::set_edge_curve_from` or
`Body::set_face_surfaces_describing_from` with
`Origin::Kernels(KernelSite::..)`, so a span refusal there panics as a
kernel bug (D2, `[failure]`). Those variants are `pub(crate)` on purpose:
a `pub` door that let its caller choose `Kernels` would let input reach a
panic.

`sweep` cannot reach them. It restates stored edges through the public
`Body::set_edge_curve`, which certifies as `Origin::Callers`, so a span
refusal of a carrier and interval it read off a stored certified edge
returns typed:

- `swept.rs` `describe_wrap_edge` (~1051) and
  `describe_face_rim_at_rest` (~1464, `restated_spec().at_rest_in_chart`);
- `extrude.rs` `upgrade_rim` (~1599, ~1650: the stored curve's carrier
  and `params()` under an `Intersection`);
- `revolve/upgrade.rs` `upgrade_intersection` (~104, ~149: `edge_data`'s
  carrier and interval).

The other cross-crate drivers compute their spec (`extrude.rs`
`sweep_loop`, `loft.rs` `assemble`, `blend/surgery.rs` `attach_contact`,
`rim_phase`, `rim_phase_annulus`; `step-import` `adopt_edges` adopts a
file's edges, which are input) and are not this class.

`Body::describe_at_rest`, which sweep also calls, restates inside `topo`
and already names `KernelSite::AtRest`.

**Repair shape.** A `topo` door whose contract is the restatement itself
(the shape of `describe_at_rest`): it reads the stored carrier and
interval and takes only the new description, so the origin is the
door's to name and no caller chooses it. Each sweep site above moves onto
it.

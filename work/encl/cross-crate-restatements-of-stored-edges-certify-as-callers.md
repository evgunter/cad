---
id: cross-crate-restatements-of-stored-edges-certify-as-callers
kind: issue
title: Sweep restates stored edges through the public set_edge_curve, so their span refusal is typed
status: closed
pr: 5418
opened: 2026-10-19
branch: encl/sweep-restate
closed: 2026-10-20
---


Found by the sweep for
`work/encl/kernel-computed-specs-at-callers-sites-return-a-kernel-bug-typed.md`.

Inside `topo`, a kernel operation that restates a stored edge's carrier
and interval now certifies through `Body::restate_edge` or
`Body::rechart_restating` under `Origin::Kernels(KernelSite::..)`, so a
span refusal there panics as a kernel bug (D2, `[failure]`). Those doors
take only the new description and read the carrier and interval off the
edge's own stored curve, so no caller hands them geometry. They are
`pub(crate)`: a `pub` door that let its caller choose `Kernels` with its
own data would let input reach a panic.

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

- `extrude.rs` `sweep_loop` (~1255 transverse, ~1274 smooth): the
  strut's re-description re-derives `strut_carrier(qs[j], w)` over
  `(0, w_norm)`. The strut was minted in the same call by
  `extruded_strut_spec(.., qs[j], w, w_norm)`, the same expression over
  the same values, and `EdgeCurve::certify` stores a spec's carrier and
  interval verbatim, so it is bitwise the stored strut: a restatement,
  bitwise by construction rather than by a read of the stored curve.

The other cross-crate drivers compute their spec (`loft.rs` `assemble`, `blend/surgery.rs` `attach_contact`,
`rim_phase`, `rim_phase_annulus`; `step-import` `adopt_edges` adopts a
file's edges, which are input) and are not this class.

`Body::describe_at_rest`, which sweep also calls, restates inside `topo`
and already names `KernelSite::AtRest`.

**Repair shape.** A `topo` door whose contract is the restatement itself
(the shape of `describe_at_rest`): it reads the stored carrier and
interval and takes only the new description, so the origin is the
door's to name and no caller chooses it. Each sweep site above moves onto
it. That door is the one `topo` already has, `Body::restate_edge`
(`attach.rs`), which takes the edge and its description only. If it has
to become `pub` for sweep it stays that shape: the caller names no
origin and hands no carrier or interval, so a caller cannot choose
`Kernels` with its own data; the site it certifies under is the door's
to name (one `pub` door per sweep site, or a site the door derives, not
a `KernelSite` argument a caller fills).

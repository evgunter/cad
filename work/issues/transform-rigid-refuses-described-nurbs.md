---
id: transform-rigid-refuses-described-nurbs
kind: issue
title: transform_rigid refuses every NURBS-walled body - the arm matches the Nurbs VARIANT while its reason describes only the placeholder
status: open
opened: 2026-08-31
github: 1346
refs: [1020]
---

## From GitHub issue 1346

opened 2026-08-31, 0 comments.

`topo::transform_rigid` refuses any body carrying a `Surface::Nurbs` or a
`Curve3::Nurbs`, so **no loft, sweep or skinned body in the kernel can be
moved**. The refusal's stated reason is that the geometry is a placeholder
that "evaluates to poison" — which is true of `NurbsSurface::placeholder()`
and false of every described net.

Met in `demos/tour/src/skinned.rs` while placing the second loft beside
the first for the montage-v3 pair cell:

```
thread 'main' panicked at src/skinned.rs:410:
the non-uniform loft is placed beside its twin: NurbsPlaceholder
```

## The gate

`crates/topo/src/transform.rs`:

```rust
Surface::Nurbs(_) => return Err(TransformError::NurbsPlaceholder),   // :308
Curve3::Nurbs(_)  => return Err(TransformError::NurbsPlaceholder),   // :358
```

and its documented reason:

> A `Nurbs` placeholder surface or carrier — unimplemented geometry
> evaluates to poison, so transforming it is refused.
>
> "transform: a Nurbs placeholder surface or carrier evaluates to poison,
> so mapping it is refused"

Both arms match the VARIANT, not the placeholder state. `Surface::Nurbs`
carries `Arc>` — "the universal fallback … a validated
`NurbsSurface` payload" — and it evaluates for real (`Surface::eval` →
`n.eval(u, v)`, `Surface::ders` → `n.ders(u, v)`). The loft walls this
refused are degree-1×2 and 2×2 nets with live control points and rational
weights; they tessellate, they integrate, they export to STEP. Nothing
about them is poison.

**The discriminator already exists** and is public on both halves:

- `NurbsSurface::is_placeholder()` (`geom/src/surfaces/nurbs.rs:295`)
- `NurbsCurve3::is_placeholder()` (`geom/src/curves/nurbs.rs:1150`)

documented as *"Is this payload the placeholder — the 'no description yet'
state — rather than a described surface? … the surface and curve halves
answer it identically."* So the gate can refuse exactly what its own text
says it refuses.

## Why this arm is easier than the one already filed

The sibling arm is **#1020** (*transform: map an Approx face — the
composition law holds; the mapped certificate needs a re-derivation lane*).
That one is genuinely blocked: an approximating surface carries a
certificate, and a certificate is never carried across a geometry change.

A described NURBS under a RIGID map has no such problem. Map the control
points; weights and knots are unchanged; the mapped net is the exact image
of the original, not a re-fit. There is no certificate to re-derive and no
fit door to reach — which is what makes this strictly less work than the
Approx arm beside it.

## What it costs today

Every body whose walls came from `loft_body` / `sweep_body` is immovable:

- the tour cannot place two lofts side by side (met here);
- the lily's six lofted/swept blades are placed by authoring their
  sections in world position, never by moving a built part;
- an assembly instancing a lofted part would refuse at
  `transform_rigid` — the door `wire_placed_union` and every `Transform`
  recipe node lower through.

## Proposed

Refuse on `is_placeholder()` rather than on the variant, and implement the
rigid map as the control-point map for described nets. Keep `ApproxSurface`
refusing as it does (#1020 owns it), and keep the placeholder refusing with
this error's existing text, which will then be accurate.

The demo works around it in the meantime by authoring the second loft's
placements at the offset instead of translating the built body, with a gap
comment citing this issue — recorded, not hidden, per
`memories/demo-purpose.md`.

## Home

`crates/topo/src/transform.rs` is in no open program's territory glob (VERBS owns `offset_*`/`shell`/`replace_face`, S-BOOL the boolean and splitting modules, S-MATE the assembly and rest files), so this lands in `work/issues/` until a program claims the transform door.

---
id: transform-rigid-refuses-described-nurbs
kind: issue
title: transform_rigid refuses every NURBS-walled body - the arm matches the Nurbs VARIANT while its reason describes only the placeholder
status: closed
opened: 2026-08-31
github: 1346
refs: [1020]
branch: fix/transform-rigid-nurbs
pr: 1742
closed: 2026-09-04
---

## From GitHub issue 1346

Opened 2026-08-31; 0 comments.

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
carries `Arc<NurbsSurface<T>>` — "the universal fallback … a validated
`NurbsSurface` payload" — and it evaluates for real (`Surface::eval` →
`n.eval(u, v)`, `Surface::ders` → `n.ders(u, v)`). The loft walls this
refused are degree-1×2 and 2×2 nets with live control points [and rational
weights — **measured false, corrected at close**: a polyline-profile loft's
walls carry weight 1 throughout, and only an ARC-bearing profile skins to a
rational wall. The refusal was never about weights either way]; they
tessellate, they integrate, they export to STEP. Nothing about them is
poison.

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

## Closed

**What landed.** Both arms of `crates/topo/src/transform.rs` gate on
`is_placeholder()` instead of on the `Nurbs` variant, and a described net
maps by its control points with the knot vectors and the weight channel
carried over verbatim. The refusal keeps `TransformError::NurbsPlaceholder`
and its existing text, which is now accurate. The point map itself is
`map_points` on `NurbsSurface` and on the `nurbs_curve!` macro (so both
curve dimensions) — `crates/geom/src/*` is S-CERT's ground and the row is
filed there as `work/cert/nurbs-net-point-map-helper.md`, per this
program's `keep_out`.

The nets are stored **Euclidean** with the weights in a separate channel,
so evaluation is an affine combination of the control points and a rigid
map — being affine — commutes with it. That is what makes the weights
untouched and the mapped net the exact image rather than a re-fit; the
argument, and the converse for a weighted/homogeneous storage, is written
at both doors.

`Surface::Approx` is untouched: #1020 owns that arm and its certificate
re-derivation problem.

**Tests.** `crates/sweep/tests/transform_nurbs_walls.rs` (new suite,
registered in the existing aggregated `all.rs` binary) transforms TWO loft
bodies and asserts the commuting square — `S'(u,v) = M(S(u,v))` on a 9x9
grid per wall and `C'(t) = M(C(t))` at 17 parameters per carrier, plus
bitwise-equal knots and weights. The polyline-profile body (unit weights
throughout) goes under a quarter turn plus a dyadic translation and also
pins tiers 1-3 on the mapped body. The arc-profile body — genuinely
rational walls — goes under a map with no exactly representable entry, and
is the row that can catch a storage confusion at all; it deliberately does
NOT assert tier 3, because that body misses its quadrature budget away from
the origin whether or not a transform is involved (#390, `work/exch/`).
Measured: residual exactly 0 under the dyadic map, worst 1.8e-15 under the
awkward one, against a 1e-12 floor whose only job is to exclude a re-fit. Both
rows fail on the pre-change kernel with `NurbsPlaceholder` (verified by
reverting `transform.rs` alone and re-running). Five gate pins in
`transform.rs`'s own `#[cfg(test)]` module hold both directions: the
surface and carrier placeholders refuse, described nets of each map by
their control points, and the public door refuses one and admits the other
— so the gate cannot silently invert.

**The demo.** `demos/tour/src/skinned.rs` no longer authors the second
loft's placements at the offset. It builds the body at the origin, like its
twin, and moves it with `pncad::topo::transform_rigid`; the gap comment and
the three narration strings that cited this issue are gone.

**A claim in the body above, corrected rather than quietly dropped.** This
issue said the refused loft walls carry "rational weights". Measured, they
do not: the polyline-profile loft that met this bug builds four nets with
every weight exactly 1.0, and unit weights are precisely the case in which
Euclidean and homogeneous storage are indistinguishable — so a suite built
only on that body could not have caught a storage error, which is the error
this whole unit turns on. The acceptance suite therefore carries a SECOND
body, an arc-profile loft whose walls are genuinely rational (measured
`|w − 1|` up to 7.6e-2), with a guard that fails if that fixture ever
reverts to unit weights.

**What I swept for.** The class is *a refusal gated on the `Nurbs` VARIANT
whose stated reason is the placeholder STATE*. Three patterns over
`crates/*/src`, `demos/*/src`, `tools/*/src`:

- `Surface::Nurbs(_)` / `Curve3::Nurbs(_)` / `Curve2::Nurbs(_)` literal
  matches — ~40 hits, all variant-level facts that are true of a described
  net too (no implicit form, no canonical frame, no analytic quadrature, the
  trimmed tessellation lane, `SurfaceKind` classification). None fixed, none
  wrong.
- the same with `(..)` instead of `(_)` — no hits.
- any of `poison` / `placeholder` / `no description yet` /
  `unimplemented geometry` within eight lines of such a match — the only
  hits outside `transform.rs` are `geom-brep/src/implicit.rs`'s four
  `poison()` returns, which are correct: no NURBS surface has an implicit
  form, described or not.

No second instance of the class exists at this merge base.

**What the sweep could not match.** A gate spelled through an intermediary
(`geom_brep::SurfaceKind::Nurbs`, `topo::query`'s `Self::Nurbs`), where the
refusal and its reason sit arbitrarily far from the match. A justification
carried in a doc comment more than eight lines from the match it justifies.
The mirror defect — a site gating on `is_placeholder()` where the whole
variant should be refused — which none of the three patterns is shaped to
find. And the sweep is accurate as of this branch's merge base.

**A finding, filed not taken.**
`work/fix/transform-recertifies-through-the-narrow-lane.md`:
`transform_rigid` re-certifies through the plain `EdgeCurve::certify`, which
refuses a described `Surface::Nurbs` operand of an `Intersection`, while
tier 3 re-certifies through `recertify_nurbs_lane`, which admits it. Loft,
sweep and skin bodies are unaffected — their wall edges are `Chart`
descriptions, which resolve through the iso resolver that admits described
nets — and the new acceptance row proves that path end to end. What is now
reachable and refuses is the M7-8 plane x described-NURBS `Intersection`
class: a body tier 3 calls valid that `transform_rigid` cannot move. The
fix is a `T: EdgeNurbsLane` bound and the lane-wired door, a public generic
signature change out of this unit's scope.

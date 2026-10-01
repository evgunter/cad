---
id: torus-meridian-orient-builds-a-frame-on-an-undecided-normalize
kind: issue
title: torus_meridian_orient builds a hand Gram-Schmidt frame on an undecided normalize
status: closed
opened: 2026-09-15
priority: P0
cost: E
closed: 2026-10-01
---


## What

Found by FRAME-WITNESS's §4 sweep (`docs/FRAME-WITNESS-SPEC.md`, the
frame-witness unit of SCALAR).

`crates/geom-brep/src/props/curved.rs` `torus_meridian_orient` writes
the Gram–Schmidt ladder out by hand, on a normalize that decides
nothing:

```rust
let w = m0.c_c - center;
let rho_hat = (w - axis * w.dot(axis)).normalize();
let tau = axis.cross(rho_hat);
```

That is exactly `OrthoFrame::gram_schmidt(center, axis, w, …)`'s two
steps — `axis` kept, `w` yielding its component along it — with the
second length neither decided nor refused. A meridian centre ON the
torus axis gives a zero residual and `rho_hat` comes out poisoned;
`tau` is then poison too, and what refuses is the `props_meridian_orient`
classify one line further down, under a name that says the orientation
was degenerate rather than that the chart has no radial.

`axis` is a carrier field (`Surface::Torus`'s), so it is unit under the
at-rest rule and not a witness — which is why this is not a one-line
take of the new type: the mint wants a decided axis, and deciding it
here is a decision this function does not make today.

## Why it is filed rather than fixed

The frame-witness unit's fence is its own consumers; this is a
production ladder in PROPS' `geom-brep/src/props/*`, and routing it
through the witness adds a decision (the residual's length) and a K
funnel name that the props lane owns, not SCALAR.

## Pointer

`crates/geom-core/src/linalg/ortho_frame.rs` — `OrthoFrame::gram_schmidt`
is the ladder written once, with both lengths decided and a typed
refusal naming which axis.

## Closed (2026-10-01)

Closed by #3686. The axis is now decided as `UnitVec3`, under
`props_torus_axis`. The radial comes from `OrthoFrame`, under
`props_meridian_radial`.

On main, a meridian centre on the axis refused under the wrong name,
`props_meridian_orient`. A centre within the band of the axis
answered `Ok`. Both cases now refuse typed under the radial's name,
which is D2 row 1.

Orientation is bit-identical to main across 880 f64 patches and the
Interval family, as measured by the review.

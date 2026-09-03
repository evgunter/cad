---
id: general-pcurve-face-props-and-tess-refuse
kind: issue
title: Volume, area and tessellation still refuse typed on a face carrying a General pcurve (P-2 residue)
status: open
opened: 2026-08-29
github: 1179
refs: [498]
---

## From GitHub issue 1179

opened 2026-08-29, 0 comments.

## What

PCURVE P-2 (#498) gave interior-column `Intersection` carriers a home: they derive
a `Pcurve::General` chart image and certify it through `PcurveCache::certify_general`
at the Fitted grade, so such an edge is no longer a construction-killing refusal.

Downstream, volume/area/tessellation of a face carrying one still refuse TYPED,
every one of them citing "the cut-loft unit". That is a real improvement over
"cannot be built at all" and it is narrower than #498's acceptance text, so it is
filed rather than left implicit in a PR body.

## The sites, as they are in the tree today

Located by grep on `main`-as-of-P-2, not copied from the spec:

- `crates/topo/src/props.rs:1125` — `QuadratureUnsupported`, "a NURBS-face pcurve
  endpoint is not exact structure"
- `crates/topo/src/props.rs:1151` — `QuadratureUnsupported`, "a NURBS-face half-edge
  carries a non-iso pcurve — a trimmed NURBS region's quadrature is the cut-loft
  unit's"
- `crates/topo/src/props.rs:1163` — `QuadratureUnsupported`, "a NURBS-face pcurve is
  not axis-aligned — a diagonal trim"
- `crates/topo/src/props.rs:1217` — `QuadratureUnsupported`, "a NURBS-face boundary
  vertex sits strictly inside the UV rectangle"
- `crates/mesh/src/trimmed.rs:975`
- `crates/mesh/src/chords.rs:558`

**Correction to P-2's spec, worth recording.** The spec named six sites as
`mesh/src/trimmed.rs:982`, `mesh/src/chords.rs:564`, `topo/src/props.rs:1147`
and `:1160`, `topo/src/chart_region.rs:1224`, `topo/src/replace_face.rs:1675`.
The count is right and the two `mesh` sites are right (±6 lines), but
**`chart_region.rs` carries no such refusal at all** (no match for `cut-loft`,
`trimmed`, `rectangle lane` or `*Unsupported` in its 3206 lines), and
`replace_face.rs`'s nearest relative is `FittedBoundaryUnsupported` at `:1331`,
which is a different statement. `props.rs` carries **four**, not two. A list of
sites in a spec is worth re-deriving before it is built against.

## What is NOT the blocker

**No new certification class is needed.** `certify_general` at the Fitted grade is
the class, measured working on an interior column (envelope 3.86e-14 m at
ε ∈ {1e-6, 1e-9, 1e-12}). These refusals are about a face whose TRIM REGION is not
an axis-aligned rectangle in its chart — the quadrature and tessellation lanes'
own frontier — not about the pcurve.

## Unmeasured

That these six are *exactly* the set a face carrying a `General` cache hits at
runtime has NOT been measured: the whole-body mint is currently blocked upstream
by the rim arms (see #498's P-2 PR), so no body carrying such a face at rest
exists to call volume/area/tessellate/offset on. The list above is a static
survey of the refusals that name the class, not a trace.

## Home

Named PCURVE exit-walk residue that is explicitly not that (closed) program's; the sites straddle S-CERT's props ground and S-MESH's crate, so it lands unowned under `work/issues/`.

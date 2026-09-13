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

Opened 2026-08-29; 0 comments.

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

## Measured (TRIM-1, `m8_4_intersection_iso.rs::an_interior_column_intersection_mints_a_general_image`)

With the interior-column seam minting exactly (TRIM-1), the P-2 body
mints and validates at rest, and the row calls the two lanes on it:

- `topo::mass_properties` → `MassPropsError::Face { source:
  QuadratureUnsupported { what: "a NURBS-face half-edge carries a
  non-iso pcurve — a trimmed NURBS region's quadrature is the cut-loft
  unit's …" } }` — the `props.rs` non-iso site above, reached by the
  `General` image on the `Intersection` seam.
- `mesh::tessellate` → `TessellateError::UnsupportedNurbsFace { note:
  "degree-1 NURBS direction with interior knots (a C⁰ crease) — the
  interpolation Taylor bound needs C¹; split the face at the crease" }`
  — NOT one of the sites above: the widened chart has interior knots
  in its degree-1 `u` direction and
  `geom_brep::patch_bound::PatchBoundError::Degree1Crease` fires
  before any trimmed-region site is reached.

So on this fixture tessellation never reaches the trimmed-region
refusals; measuring those needs a trimmed chart without a C⁰ crease
(a degree ≥ 2 widening, or a chart whose extra columns carry no
interior knot in a degree-1 direction). Offset (`replace_face`) was
not called by the row.

## Home

Named PCURVE exit-walk residue that is explicitly not that (closed) program's; the sites straddle S-CERT's props ground and S-MESH's crate, so it lands unowned under `work/issues/`.

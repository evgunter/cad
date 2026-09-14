---
id: general-pcurve-face-props-and-tess-refuse
kind: unit
title: Volume, area and tessellation still refuse typed on a face carrying a General pcurve (P-2 residue)
status: dispatched
opened: 2026-08-29
github: 1179
refs: [498]
branch: trim/2-quadrature
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

## The sites — MEASURED, not enumerated (TRIM-2 PR-1, §8 item 4)

The "six static sites" list this item opened with was a grep, and a grep
over refusal TEXT is not a trace. Re-taken by running the two lanes on
the fixture that reaches them — the P-2 body re-widened to a degree-2
`u` chart (`m8_4_intersection_iso.rs::widened_u_chart_deg2`), at
ε ∈ {1e-6, 1e-9, 1e-12}, `f64` lane. **Exactly two sites are on the
trace**, and they are the only two this unit had to open:

- `crates/topo/src/props.rs`, `quad_lane::nurbs_face`'s **non-iso**
  return: `MassPropsError::Face { source: QuadratureUnsupported { what:
  "a NURBS-face half-edge carries a non-iso pcurve — a trimmed NURBS
  region's quadrature is the cut-loft unit's …" } }`, identical at all
  three ε.
- `crates/mesh/src/chords.rs`, `nurbs_tighten`'s **`General`** arm:
  `TessellateError::UnsupportedCurve { note: "NURBS-face half-edge
  carries a GENERAL curve-in-UV pcurve — no certified UV speed bound is
  wired for a spline chart image's chord schedule" }`, identical at all
  three ε. The C⁰ crease gate the degree-1 widening hit
  (TRIM-1's measurement, below) is gone on the degree-2 chart.

The other three `props.rs` sites the original list named are the
RECTANGLE certificate's own (endpoint-not-exact, diagonal,
vertex-inside) and stay: an all-iso loop keeps that lane bit for bit
(TRIM-2 §8.1). `crates/mesh/src/trimmed.rs`'s `General` arm is real but
is NOT reached — the chord pass runs before any face lane
(`tessellate.rs`, `compute_chords` precedes the face loop), so it opens
only once `chords.rs` admits the class, which is PR-2's.

**The offset claim is refuted.** `replace_face_offset` on the bowed face
is not a `General` frontier at all:

| ε | degree-2 body (carries `General`) | ORACLE prism's own bowed wall (no restatement, no `General`) |
| --- | --- | --- |
| 1e-6 | `FittedBoundaryUnsupported { what: "a chart image of a neighbour's chart" }` | the same |
| 1e-9 | the same | the same |
| 1e-12 | `Fit { BudgetExhausted { grid (40, 27), achieved 2.865e-10, tolerance 1e-12 } }` | `FittedBoundaryUnsupported` |

A body with no `General` anywhere earns the same refusal, so the class
is **SHELL's `work/shell/no-approx-faced-body-is-both-movable-and-valid.md`**
(an `Approx` face's boundary has no route), and this measurement is
added there rather than opening a second row. At 1e-12 the widened body
reaches the FIT first and exhausts its budget — a different door of the
same class, and still not a statement about `General`.

## What is NOT the blocker

**No new certification class is needed.** `certify_general` at the Fitted grade is
the class, measured working on an interior column (envelope 3.86e-14 m at
ε ∈ {1e-6, 1e-9, 1e-12}). These refusals are about a face whose TRIM REGION is not
an axis-aligned rectangle in its chart — the quadrature and tessellation lanes'
own frontier — not about the pcurve.

## Measured (TRIM-1) — now the degree-1 CONTROL row

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

Named PCURVE exit-walk residue that is explicitly not that (closed) program's; the sites straddle S-CERT's props ground and S-MESH's crate, so it lands unowned under `work/issues/`. TRIM adopted it as its unit 2; the spec is `docs/TRIM-2-SPEC.md` and the two PRs are `trim/2-quadrature` (props) and `trim/2-tessellation` (mesh).

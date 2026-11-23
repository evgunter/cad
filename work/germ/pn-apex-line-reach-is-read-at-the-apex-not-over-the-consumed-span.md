---
id: pn-apex-line-reach-is-read-at-the-apex-not-over-the-consumed-span
kind: issue
title: plane×cone bounds its apex line outcomes at the apex, so a consumer whose span stays clear of it is refused a line that is within band of its section there
status: closed
branch: germ/pn-apex-reach-over-span
pr: 4646
opened: 2026-10-10
closed: 2026-10-12
priority: P1
cost: M
refs: [pn-apex-line-outcomes-stand-in-for-a-section-off-the-apex]
---


## What

`plane_cone_section` (`crates/geom-brep/src/intersect.rs`) serves
`ApexLinePair` only where `pn_apex_pair_reach` (`|δ|/D`, where the
hyperbola's far branch starts) reads Zero, and `ApexTangentLine` only
where `pn_apex_tangent_reach` (`|δ|/M`) does. Both are read at the apex,
because the arm takes an `extent` and no consumed span. So a consumer is
refused the line outcome even where it is within the band of the true
section over the span the consumer reads. The span may stay clear of the
apex, or lie on one nappe only.

## Where it lands

The refusal is typed and fails loud; it is never a wrong answer. It
lands on all three callers, two of them public verbs. Each refuses where
main built the right answer.

- **`topo::union` of touching solids** (`chord_join`'s cone lane). A
  quarter wedge of a frustum (`y ∈ [y0, 1]`, `y0 ∈ {1/4, 1/512}`,
  `tan α ∈ {1/64, 1/2, 1}`), unioned with a half-space flush on its axial
  cap and overlapping it by `0.5·ε` or `0.9·ε`, in both orders. The cap's
  plane holds the axis, so `D = M = sin α`.
  - Main builds the right union in all 24 cases, at ε = 1e-9, 1e-6 and
    1e-12.
  - Here 20 of the 24 refuse, at every ε: 12 escalate on
    `pn_apex_pair_reach`, and 8 refuse the hyperbola.
  - Row:
    `a_half_space_flush_with_a_frustum_wedge_cap_builds_only_where_its_start_is_served`.
- **The boolean frame** (`pk_germ_frame`, `topo::boolean::join`). A
  `π/6` frustum with its apex outside the body, cut by a box face built
  through the virtual apex with `D = sin(1e-6)`. The plane passes through
  the apex by construction, and rounding leaves `|δ| ≈ 7e-17`.
  - At ε = 1e-12, `|δ|/D ≈ 7e-11` is past the band.
  - Main builds all 6 op/orders right, for both gap signs. Here all 12
    refuse `GermSectionOutsideInventory { Hyperbola }`.
  - Row:
    `a_box_face_through_a_virtual_apex_builds_only_where_its_start_is_served`.
- **`topo::offset_derive`.** The same quarter wedge on a frustum
  (`y ∈ [1/2048, 3/8]`), with a cap moved by `±0.9·ε`. Main planned the
  cap's wall edge, `Ok(None)`. Here it refuses `NeighborPoseUnroutable`,
  naming the hyperbola.

All three rows are in `crates/sweep/tests/a_plane_moved_to_a_cone_apex.rs`.

**Two shapes reach it**, not only a gap placed inside the zero band:
- **Near-coincident placements:** a face moved or placed within the band
  of a plane through the apex.
- **Planes through the apex by construction:** a face built through a
  virtual apex, whose rounding gap is `~1e-16`. At a small ε, a small
  `D` carries `|δ|/D` past the band.

**One nappe.** The cost falls on consumers whose face lies on the
nearer nappe. The PR 4592 review measured the line outcomes main served
and this bound refuses. At ε = 1e-9, 1e-6 and 1e-12, 198, 243 and 81 of
them were within ε of the true section on one nappe, and none on both. A
pair through the apex is two whole lines, and on the far nappe it misses
the section by `|δ|/D`.

## The shape of a fix

Read the bound over what the consumer consumes, as
`plane_cylinder_section` reads its tilt over a `Reach`. That needs a
`Reach` in place of the arm's `extent` at its three callers, carrying
two things.

- **The span.** On a span whose nearest point to the apex is `ρ` away,
  a served line stands within about `max(start − ρ, 0) + |δ|/sin θ` of
  the section, with `θ` the dihedral angle between the planes along the
  generator (`sin θ = √(D·M)/sin α`). The frustum cases need this. With
  `D = M` both nappes start equally far, so the nappe alone does not
  admit them.
- **Which nappe the consumer's face lies on.** A face on one nappe is
  bounded by that nappe's start (`|δ|/M` on the near one) and by the
  asymptote offset `|δ|·sin α/√(D·M)`. That second term is not covered
  by `|δ|/M ≤ ε`, so the nappe-only bound needs it decided too.

## Priority

P1: the arm reads its bound at the apex because it takes an `extent`
where its sibling takes a `Reach`, a special case that should be handled
uniformly. The cost lands on a public verb that main answered right. It
is not P0, because nothing wrong is served and every pose is
near-coincident (an in-band offset, or a rounding gap through the apex).

**2026-10-12, closed (PR 4646, SEQUENTIAL with a second review).** `plane_cone_section` takes a `Reach`, and the pair is bounded over the window the consumer consumes, `min(|δ|/D, Σ) + R`, with each nappe named only where the gap and the axis component clear their own rounding (the first review's MAJOR). The union of a frustum wedge with a flush half-space now builds at every ε. The remaining rounding assumption (an orthonormal cone frame) is filed separately.

**2026-10-25, renamed (PR 5692).** `GermSectionOutsideInventory` above is now `BooleanError::GermSectionOpenConic { conic }`, with no `SectionError` field: `plane_cone_section` mints the parabola and hyperbola as an exact rational-quadratic NURBS arc (`crates/geom-brep/README.md` C1), and the Boolean refuses it because it has no crossing layer for a NURBS carrier. `offset_derive`'s `NeighborPoseUnroutable` above is now `EdgeSection { verdict: OpenConic }`.

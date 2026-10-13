---
id: the-section-table-admits-a-curve-on-its-resolution
kind: unit
title: The section table admits a curve only where its resolution is within eps/K, and refuses the rest as a kernel limit
status: parked
opened: 2026-10-12
priority: P2
cost: M
parent: a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join
blocked_on: [a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex]
---

Part 2 of the design on
`a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join.md`.
It waits for the re-pose (part 1), which removes the parameter term.

## What

An off-axis cut puts a section ellipse's centre at about w/θ. Its
points near the face then hold only about 5·u·|centre − face|: 1.82e-12
at |centre| ≈ 1e4, and 1.16e-10 at 1e6, GERM's numbers. No re-pose
fixes this while an ellipse is a centre plus axes.

## Owed

- A derived `Curve3::resolution(at, window)`, computed with a running
  `Rounded` bound through the construction from the reach's point. It
  is not a stored field. A line resolves to the floor; a circle to
  u·(|c − at| + r); an ellipse over its window.
- It replaces `merge_faces::edge_on_faces_reading`'s `CurveKind::Line`
  test as the one rule. A fitted spline keeps the gate's.
- `plane_cylinder_section` and `plane_cone_section` admit only where
  ρ ≤ ε/K. Everything else refuses before minting, as
  `SectionError::UnsupportedConditioning` (or similar), named per
  D9 row 2.
- That refusal names no move. It ends with D4 ¶1(i)'s last resort
  (loosen ε; report it), and its text may state the cause. Under PR
  4335 it must offer no geometry lever.
- This closes the carrier half of GERM's
  `conic-section-just-past-its-band-fails-certification-downstream`.
  Its chart-image half (the α = 1.55 `MapResidual` table) stays open.
- At the gate: a debug assertion that no carrier with ρ > ε/K arrives
  (D9 row 5).

## Seam note from GERM (2026-10-12): Ev's decision on PR 4812

Ev decided GERM's `conic-section-just-past-its-band-fails-certification-downstream`
on PR 4812, after this unit was designed. The end state: a minted
section ellipse is stored anchored on the arc its edge uses, with the
centre derived and never stored, so the far-centre term (b) is removed
rather than refused; there is **no refusal at the arm**; in the interim
a miss within a carrier's own resolution is relabelled as a kernel
limitation, refusing nothing new; and a certification-time resolution
check waits for D4 ¶4's size range. So this unit's "admit only where
ρ ≤ ε/K, refuse the rest before minting" is superseded; the re-pose
(`a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex`) is
consistent with it as a first step. The decision's text is in GERM's
row.


## Seam note from part 4 (2026-10-13): the kernel-limit ending has a home

Part 4 (`the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`,
PR 4858) needed the same ending for the vertex's section reading,
`δ / sin ψ`, where its own rounding is past `ε/K`. Its home is
`geom_brep::recourse::UnsupportedConditioning`. It carries the clause the
reading decides and the admission's reading, and its `Display` ends in
D4 ¶1 (i)'s last resort with no lever. Its `admit(name, subject, bound,
band)` is the `ρ ≤ ε/K` admission: `K·ρ` decided Zero. Each door wraps it
in its own variant (`SplitReduceError::SectionUnresolved`,
`BooleanError::SectionUnresolved`). A `SectionError::UnsupportedConditioning`
here would wrap the same payload instead of spelling a second ending.

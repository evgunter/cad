---
id: the-section-table-admits-a-curve-on-its-resolution
kind: unit
title: A minted section conic is stored anchored on the arc its edge uses, its centre derived (Ev, PR 4812)
status: dispatched
opened: 2026-10-12
priority: P1
cost: H
branch: oneround/anchored-conic
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

## Re-scoped (2026-10-13): Ev's end state, not an admission

Ev's decision on PR 4812 supersedes this row's "admit only where
ρ ≤ ε/K, refuse the rest". There is no refusal at the arm. The unit is
now the end state. Its decided text is in GERM's row's "Decided"
section:
- A kernel-minted section conic is stored anchored on the arc its edge
  uses: parameter zero there, centre derived and never stored. Its
  resolution is then `u` times the arc distance from the anchor, not
  `u·A`.
- An imported ellipse keeps the file's centre form exactly as read,
  with the anchored form as an internal cache. Export writes the
  original and round-trips bit-exactly; export of a minted ellipse
  writes the derived centre.
- Evaluators read the anchored form either way. No public API changes.
- Where both flanks of a two-flank cylinder cut are used, there is
  one carrier per edge. The germ frames and `Reach::Span` read a
  shared carrier today, so confirm that shape first.
- Every `Ellipse` consumer is kept in step: boxes, chart image,
  envelope, tessellation, certification, STEP export, and the germ
  frames that read `center`.

This follows the re-pose (PR 4861), which removes the parameter term.
The interim relabel belongs to the parent row's part 3 (PR 4810).

## The at-rest frame check (2026-10-14)

An anchored conic still stores a frame. Tier 3's frame margin is
`ε − δ·A`, the frame's own rounding `δ` amplified by the larger
semi-axis `A`, so a near-axial section still refuses at rest at
ε = 1e-12 on rounding alone. That residue is RESTFRONT's
`tier-3s-frame-check-reads-a-section-ellipses-rounding-amplified-by-its-semi-axis`,
measured on PR 4861. This unit's end state has to meet it there.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Evidence from ONEROUND (2026-11-19): the loss is in the stored form, measured

From the phase-1 measurement of `oneround/conic-vertex`, on merge base
`b80352cd9` (local runs; the probe `probe_far_centre_section_misses`
and its commands are on the branch
`analysis/oneround/conic-vertex-probes`, never merged, under
`analysis/oneround/conic-vertex/`).

Random planes against a tilted cylinder (1e-3…1e-6 off its axis) and a
cone (1e-3…1e-7 short of parabolic), through `plane_cylinder_section`
and `plane_cone_section`; carriers whose centre is past 1e3 with a
part inside the box; seamed at the nearest principal vertex; 17 points
over ±80 m of arc; the exact-rational miss off both surfaces. Worst
miss in units of `u·a`, with the point read three ways:

| kind, `a` | samples | from the centre (main) | from the seam vertex | from the vertex summed exactly |
|---|---|---|---|---|
| cone 1e3 / 1e4 / 1e5 / 1e6 | 680 each | 3.39 / 2.51 / 2.53 / 2.31 | 3.61 / 2.35 / 2.22 / 1.88 | 3.18 / 2.22 / 2.15 / 1.79 |
| cylinder 1e4 / 1e5 / 1e6 | 527 / 680 / 680 | 0.94 / 1.04 / 1.12 | 1.16 / 0.86 / 1.16 | 1.16 / 0.85 / 1.16 |

- In metres: 7e-10 at `a ≈ 1e6` (cone), 2e-10 (cylinder), in every
  column. The stored curve, read exactly, is that far from the
  surfaces it was cut from, so no evaluator recovers it: the anchored
  mint is what does.
- The cylinder's in-box part sits 0.42–0.50 rad from its nearest
  principal vertex; the cone's within 1e-3–1e-2 rad of its major
  vertex. So "anchored on the arc its edge uses" cannot mean a
  principal vertex for the cylinder's shallow cuts: the anchor is a
  general point of the arc, with conjugate semi-diameters there.
- PR 7131 (carrier frames as `RigidMotion`, open on Ev) decides what a
  conic's frame and its origin are; an anchored form changes what that
  origin is.
- Readers that need the anchor exact, measured on a far-centred conic:
  `work/oneround/a-far-centred-conics-point-is-decided-against-an-in-box-point-at-the-centres-rounding.md`,
  `work/select/an-endpoint-pin-passes-on-a-far-centred-conics-own-rounded-point.md`.
- A circle has no re-seaming door:
  `work/oneround/a-far-circle-has-no-re-seaming-door.md`.

## Claimed by ONEROUND (2026-11-19)

Moved from HONEPAIR by `git mv`, id and body unchanged, and re-banded P2 → P1: `a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex` (P1) waits on it, and a blocker takes the band of what waits (`work/README.md`, Priority). HONEPAIR had no orchestrator and no lane on it.

- **The spec is Ev's decision on PR 4812** (the seam note above and GERM's `conic-section-just-past-its-band-fails-certification-downstream`, "Decided"), not this file's `## Owed` list, which that decision supersedes: no refusal at the arm and no `UnsupportedConditioning`.
- **What the phase-1 measurement adds** (the appended section above; tables on `analysis/oneround/conic-vertex-probes`): the anchor has to be allowed at a general point of the arc, since a cylinder's shallow cut has no principal vertex near its in-box arc; and the near-gap families, the chords of one carrier and the readings that compare a conic's point with an in-box point all recover once evaluators read a form anchored in the box.
- **Unit:** `oneround/anchored-conic`, letter H, tier dual (a concurrent pair), as logged for `oneround/conic-vertex` on 2026-11-19, whose implementer keeps it. It is stacked on `onehome/carrier-frames` (PR 7131), which makes every carrier hold its frame as a `RigidMotion` (decided, fork 274) and still asks Ev how a stored frame is read at `Interval`.


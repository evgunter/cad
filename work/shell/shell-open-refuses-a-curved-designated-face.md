---
id: shell-open-refuses-a-curved-designated-face
kind: issue
title: shell_open refuses every non-plane designated face at construction, where only the props reading of a curved ringed rim is missing
status: closed
opened: 2026-10-06
priority: P1
cost: H
refs: [shell-offset-three-followups]
pr: 4191
branch: shell/curved-mouth
closed: 2026-10-07
---


`shell::check_designation` refuses any designated face whose surface
is not a plane (`ShellError::OpenFaceRingUnsupported`), so a vessel
cannot be opened through a curved face — a cylinder's side, a dome.
The refusal was drawn when the property inventory had no reading for
a curved face carrying a ring. That premise has moved: `topo::props`
now refuses such a face at the props call
(`MassPropsError::RingOnCurvedFace`) and already measures one shape of
it (a cylinder wall bounded by rims and rulings), so shell's
construction-time gate refuses bodies the rest of the kernel can
build, and a caller who never asks for volume is refused anyway.

The cost is not the gate but the rim stage behind it:
`shell::lift_to` assumes a plane (`unreachable!` on any other
surface), and the closing-mint doc in `shell.rs` notes a curved
designated chart would write rows after the door.

**The open question** (design): move the refusal to the props call,
as the boolean pierce does, and build curved-rim surgery in the lift;
or keep a construction-time refusal and say why. Weighed by the
designer pair before a spec.

Item 1 of the closed `shell-offset-three-followups` (GitHub 1058),
re-filed 2026-10-06 by SHELL's triage. The props reading of other
curved kinds is FLUX's (`flux/an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane`).
Signed (SHELL orchestrator).

## Decided (SHELL orchestrator, 2026-10-06, after the designer pair)

Both designers, independently, rejected the framing above. The
boolean is not a precedent for "refuse at the props call": it builds
the ringed wall and refuses at its own closing tier-3 check 7
(`verbs_germarms`), and `shell_open` already closes on the same
certified validation. So the gate never spared a caller anything. The
real defect is that the rim stage builds every rim in the PLANE's
form. The design they converged on, which is now this row's spec basis:

- **No surface-kind gate.** `check_designation` keeps its
  connectivity gates (they already refuse a capped cylinder's whole
  side wall as `OpenFacesDisconnect`) and loses the kind test.
  `OpenFaceRingUnsupported` retires, along with its editor fold, its
  tests and the Python tag.
- **The rim takes its chart's own form.**
  - An opening that wraps the period of a periodic chart (a dome, a
    cone tip, a zone of a solid of revolution) becomes a SEAMED BAND:
    one face per chart branch, slit along the D1 seam meridian, as
    the full revolve and the fillet annulus already mint it. No ring.
  - A window in a region that does not wrap becomes a RING, as on a
    plane.
  - Prefer keeping the period seam through `canonicalize_chart`
    (split the designated face's seam edge at the counterpart's
    latitude) over un-slitting and re-slitting with `kfmrh` + `mekr`.
    Keeping the seam leaves every rim edge sourced in the operand.
    At a pole-touching chart (two π-bands), `mekr` alone does not
    rebuild the convention.
- **The four planar sites go.**
  - `lift_to`: the lift distance comes from one home, the inverse of
    `geom_brep::offset::offset_surface` (Δradius for cylinder,
    sphere and torus, the apex slide for a cone, a dot product for a
    plane), read along the door's stored-normal convention. Do not
    read it by negating a plane normal.
  - `encloses`/`pair_rings`: the coplanar mean-radius test becomes a
    chart-read test.
  - `canonicalize_chart`: its seam erasure stops on a periodic chart.
  - `validate::ring_outer_contact`'s plane-normal arm checks nothing
    on a curved face; RESTFRONT's
    `check-9-meeting-arms-silent-off-a-plane-…` holds that.
- **What stays unreadable is the readers' frontier, not shell's.**
  - A ringed window on a cylinder bounded by rims and rulings passes
    props.
  - The mesh refuses it until TESS's
    `a-notched-or-ringed-cylinder-wall-does-not-tessellate` lands.
  - Any other ringed curved window refuses at the closing check 7
    (`RingOnCurvedFace`, FLUX's lanes) and builds with no change to
    shell once props reads it.
  - How that refusal is TYPED ("violates" against "cannot yet
    decide") is RESTFRONT's verdict question (see that program's
    `validate-classifiers-and-lower-displays-classify-refusals-differently`).
    Until it lands, the refusal reads through `NotValid`, as the
    boolean's does.

Fixtures the spec owes:
- a dome cap;
- a cone tip;
- a pole-touching sphere zone;
- a primitive cone or torus chart of two half-faces (from
  step-import's normalize);
- a D-section half-cylinder window, which builds and is refused by
  the mesh;
- a sphere window refused at check 7;
- the closing mint's position, pinned now that a curved chart writes
  rows after the void door.

`RimNaming` keeps its shape.

## Closed (SHELL orchestrator, 2026-10-07, PR 4191)

`shell_open` now takes a designated face of any surface kind, and the rim takes the form of its chart:
- A pole-touching periodic designation, on the outer or the void side, opens to a seamed band. The band keeps every operand face and seam under its own key; each seam is re-anchored by a strut collapsed with `kev_describing`.
- A window that does not wrap its period becomes a ring. Wrapping is read from the chart, as a boundary that winds the period.
- The lift reads `geom_brep::offset_distance`, the inverse of the mint.
- `OpenFaceRingUnsupported` is retired.

A document evaluates a band: `emit_shell` names every branch face, pinned by `lib_g17_shell_node::the_capped_vessel_opens_its_cap_into_a_seamed_band`. A planar void designation also stops refusing at name emission.

Reviewed by a concurrent pair, row DR-97. Its document-path MAJOR was fixed in the fix pass.

Residues, each filed:
- `shell-open-band-wrapping-between-two-boundaries`;
- `shell-of-a-cone-tip-refuses-at-the-nappe-decision` (which also holds the lift's unreached cone arm);
- `shell-of-a-tangent-dome-refuses-at-the-axial-corner`.

---
id: check-4-gives-no-arm-verdict-to-a-mixed-or-nurbs-adjacent-edge
kind: issue
title: check 4's material arm gives no verdict to an edge whose samples mix transverse and smooth, or to a NURBS/Approx-adjacent seam, so a wedge end there passes unjudged
status: open
opened: 2026-09-28
---


Found in PR 3362's review (GATHER, `gather/derive-cusp-legality`).
With wedge-0/2π legality derived at rest (D1 tier 3, PR 3317), check 4's
material arm is the only at-rest reader of a wedge end, and two classes
of edge never reach it:

1. **Mixed samples.** An edge that classifies `Transverse` at some
   interior samples and `Smooth` at others falls to the final
   `else { ContactMark::Unmarked }` of the mark ladder
   (`crates/topo/src/validate.rs`, check 4's `let mark = if
   nurbs_adjacent { … }` ladder, the last arm, ~:5638), with `arm`
   left `None`, so `material_arm_error` returns `None`: no verdict at
   all. The first-order pass exempts such an edge from the
   prefer-intrinsic DEMAND by design (the comment ending "a mixed
   sample set is enforced as neither", ~:5390); the material arm
   inherits that exemption for a REFUSAL, which the arm's own split
   rule (`material_arm_outcome`, "escalate, never silent") refuses to
   do for a pairing or end that splits.
2. **NURBS/`Approx`-adjacent edges** are exempt by kind
   (`nurbs_adjacent`, ~:5411; the ladder's first arm, ~:5500): no
   implicit-form gradient, so neither the dihedral nor the material arm
   runs, and the mark is `Unmarked`.

Pinned today as a PASS:
`crates/sweep/tests/a_swept_cusp_is_legal_at_rest.rs`'s
`a_loft_whose_sections_disagree_passes_with_the_seam_unjudged_by_kind`
lofts a section with a `.cusp()`-declared joint against one whose same
joint is a corner, so the seam between those walls is a cusp at one
end and a corner at the other; `validate_geometric` passes it and the
seam is `Unmarked`.

Owed: decide what the arm says on each class — escalate typed (the
split rule's posture) or derive a verdict (for NURBS, from the
description rather than the fit, O5's open conversation) — and pin it.

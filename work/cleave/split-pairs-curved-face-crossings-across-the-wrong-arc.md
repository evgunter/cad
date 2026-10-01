---
id: split-pairs-curved-face-crossings-across-the-wrong-arc
kind: issue
title: a steep split of a cylinder pairs the two crossings on one wall face across the arc outside the face, so the section comes back as an ellipse face plus two coplanar segment faces cancelling it
status: open
opened: 2026-10-01
priority: P1
cost: H
---


Found by the section-rings lane (`cleave/section-rings`): the lane's
hole nesting (`crates/topo/src/splitting/finish.rs`,
`nest_hole_sections`) found a clockwise section polygon it could not
place, and the polygon turned out not to be a hole at all. It is the
same on `main` at `a9f885a3f`.

**Repro.** The unit cylinder of height 2.5 with its seams turned to
`π/2 + 0.05` (`crates/sweep/tests/pis_arc_capped_poses.rs`, the
"valley/ridge at tilt 1.1" rows), split through `(0, 0, 1.25)` with
normal `(sin 1.1, 0, cos 1.1)` (or its flip). The plane crosses both
caps and both seams.

**Observed.** Each half's section is three coplanar faces: the whole
ellipse the plane cuts from the cylinder's carrier (a 2-edge loop
through the two seam crossings, sense `true`), and two 2-gons — the
top cap's chord with the ellipse arc beyond the cap, and the bottom
cap's likewise — sense `false`, cancelling the parts of the ellipse
that lie past the caps. Tier 3 passes both halves and point-in-solid
reads them right (by cancellation); the 2-gons touch the ellipse face's
outline, so they are not holes and stay faces.

**Cause, as measured.** The join (`crates/topo/src/splitting/join.rs`,
`Sweep::take_neighbor`) pairs a new null-edge half with the first loose
end in the same face of opposite sense. On the wall face holding both
top crossings `T1`, `T2` and both seam crossings, the sweep reaches
`T1` and `T2` first (one column of the join order) and pairs them; the
chord `chord_join::chord_spec` mints between them is the arc inside the
face's AZIMUTH window, which is the short arc over `x = −1` — above
`z = 2.5`, outside the face. The book's pairing argument holds for a
planar face, whose crossings lie on one line; on a curved face the
lexicographic order is not the order along the section curve, and
opposite senses do not pick the partner (the material arcs are
`S1–T1` and `T2–S2`, but `T1–T2` is also exit-then-entry). The
correct section is one 6-vertex face.

**Fix shape.** Pair a curved face's crossings by their order along the
face's own section curve, or refuse a candidate partner whose arc
leaves the face (axial extent as well as azimuth) so the half waits
for the right one. `chord_join` is shared with the boolean lane.

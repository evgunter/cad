---
id: topo-smooth-arms-decide-descriptions-outside-the-must-carry-rule
kind: issue
title: topo's two smooth-description arms (boolean D6 ladder, split finish) decide outside geom_brep::must_carry_over_edge
status: open
opened: 2026-10-01
---


## Finding

Found by BAND's cap-rim smooth-arm sweep (every `DihedralClass::Smooth`
arm that stores a description). `geom_brep::must_carry_over_edge` is the
one home of what description a definitely-smooth join carries, and its
doc (`crates/geom-brep/src/dihedral.rs`) scopes that to "every
smooth-join arm in the sweep verbs". After the cap-rim arm moved onto
it, every sweep arm routes there. Two arms in `topo` still decide on
their own:

1. **`crates/topo/src/boolean/ops.rs`, `describe_minted_edges`' smooth
   arm (the "D6 smooth ladder")** hand-rolls the station walk over
   `tangent_jet` / `curvature_lever_arm` and folds an IN-BAND station
   into the conventional description (`_ => { det = false; break; }`,
   the comment: "a zero-side or in-band sample keeps the CONVENTIONAL
   posture"). `must_carry_over_edge`'s contract says the opposite —
   "An in-band verdict is never silently either side, so no caller may
   fold it into conventional" — and the walk does no per-station
   first-order gate. `dihedral.rs` already names this site as one of the
   two hand-rolled siblings left (issue 1439,
   `work/pred/lever-arm-fold-six-hand-rolled-siblings.md`), but as a fold
   duplicate; the in-band policy difference is the part not recorded.
   The comment cites "tier 3's ratified `SmoothUnderdetermined`
   stance" for the in-band fold — whoever takes this should check
   whether that stance covers in-band or only zero-side.
2. **`crates/topo/src/splitting/finish.rs`, the section-boundary smooth
   arm** reads no second order at all: a definitely-smooth section edge
   keeps a pre-existing `TangentIntersection` or chart image when it is
   coherent, and otherwise restates as `chart(s_self)`. The comment
   argues the case ("a curved wall smooth against the section plane is
   either a π seam or a wedge end") rather than asking the rule; a
   jet-determinate π seam whose existing description is a coherent
   chart image keeps it, where prefer-intrinsic would demand
   `TangentIntersection`. Not measured whether any operand reaches that.

Related, not duplicates: `work/boxes/description-staleness-ladder-three-spellings.md`
(the keep-vs-restate ladder these same two arms spell), and
`work/pred/lever-arm-fold-six-hand-rolled-siblings.md` (the fold).

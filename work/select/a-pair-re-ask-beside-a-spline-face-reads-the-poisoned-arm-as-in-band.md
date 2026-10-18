---
id: a-pair-re-ask-beside-a-spline-face-reads-the-poisoned-arm-as-in-band
kind: issue
title: reask_pair asks the must-carry rule beside a spline face, where the lever arm is poison, and refuses the offset and shell doors as an in-band dihedral_arm
status: open
opened: 2026-10-16
priority: P2
cost: E
refs: [onpair-has-no-incidence-lane-on-a-spline-adjacent-pair]
---


Filed by the designer lanes of the plane × spline rim fork (the
`[ev]` PR on `onpair-has-no-incidence-lane-on-a-spline-adjacent-pair`),
off their question.

## Finding

`geom_brep::reask_pair` (`crates/geom-brep/src/dihedral.rs:1076`)
asks `must_carry_over_edge` over any two surfaces. Its sibling
`reask_chart` (`dihedral.rs:1124`) returns `None` first wherever
either side is a spline chart (`dihedral.rs:1131`). That matches D2,
where the must-carry rule needs a definite reading, so "an edge whose
dihedral is not read definitely (in band, or beside a spline face)
falls outside it". `reask_pair` has no such arm.

Beside a spline face the reading has nothing to read:
- `pair_lever_arm` (`dihedral.rs:483`) makes the arm poison for a NURBS
  or approximated surface (its doc, `dihedral.rs:482`);
- `wedge_decided` then decides `"dihedral_arm"` on that poison
  (`dihedral.rs:360`);
- the refusal comes back as an in-band escalation with margin
  `Invalid`.

That is poison read as ill-posedness (DUAL-DESIGN DL6) and a total
evaluator's poison read as a decision (D3: "no reading reads it
without first refusing on kind").

The offset door reaches it through `topo::offset_restate`
(`crates/topo/src/offset_restate.rs:129`), and the attach and shell
doors through `attach.rs:1376`.

## Measured witness

No row on main reaches it, because no plane × spline edge is
`OnPair` yet. Under the fork's probe, the loft door switched to the
pair (`sweep::swept::describe_face_rim_at_rest`, `at_rest_on_pair`):
- `encl_curved_loft_shell::at_interval_the_prism_moves_by_the_shortcut_and_a_slanted_rim_refuses_by_name`
  (`crates/sweep/tests/encl_curved_loft_shell.rs:647`) fails with
  `Escalated { source: Indeterminate { margin: MarginDiag(Invalid),
  predicate: Some("dihedral_arm"), .. } }` from `replace_face_offset`;
- `offd_r1_probes` (4 rows) and further `encl_curved_loft_shell` rows
  failed in the same probe. The other designer's run attributes about
  9 f64 rows to the same margin.

The path through `reask_pair` → `pair_lever_arm` was read off the code
from that payload, not stepped through.

## The fix

Give `reask_pair` the spline-side arm `reask_chart` has. Beside a
spline face the edge falls outside the rule, and it keeps the pair
with `declared`. Then pin the offset row above under a pair-stated
rim. Whatever the fork decides for loft rims, any `OnPair` beside a
spline face (a smooth plane–spline join) reaches this.

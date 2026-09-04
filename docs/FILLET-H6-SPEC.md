# FILLET-H6 — extrude's cap-rim Smooth arm (spec)

**Program:** FILLET (`work/fillet/plan.md`), unit
`extrude-cap-rim-smooth-arm-noop`
(`work/fillet/extrude-cap-rim-smooth-arm-noop.md`). **Track:** kernel
change — the standard v6 unit (binding spec, drawn implementer arm,
cross-model dual review, union fix pass, record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **S**, task-class
**STRUCTURAL**.

- **S** — one match arm in one function, its sentence, and the sharing of
  a rule two sibling arms already spell; the work is the measurement and
  the argument, not the diff.
- **STRUCTURAL** — no predicate, band or margin moves; if Phase 1 finds the
  arm reachable, the treatment it takes is the existing `tangent_second_order`
  rule, reused, not a new decision.

## The claim

`crates/sweep/src/extrude.rs`'s `upgrade_rim` classifies each cap–wall rim
edge's dihedral at the carrier's mid-parameter witness and upgrades
`Transverse` to `Intersection { cap, wall, witness }`; its `Smooth` arm is a
literal no-op (`Ok(DihedralClass::Smooth) => Ok(())`, ~`:1205`) whose doc
says the conventional description "stays valid" because *tier 3's
prefer-intrinsic enforcement exempts definitely-smooth edges*. BOOL-1 (PR
1378, issue 1152) falsified that reasoning on the split's twin arm: the
tier-3 rule that fires on a wrongly-described smooth edge is
`DescriptionNotAdjacent`, and smoothness exempts nothing from it. The two
other `Smooth` arms in this crate — the extrude strut upgrade (`extrude.rs`
~`:881`) and the revolve rim upgrade (`revolve/upgrade.rs` ~`:177`) — apply
OQ7's must-carry: a jet-determinate smooth join (`tangent_second_order`
definite) upgrades to `TangentIntersection`, an under-determined one keeps a
conventional chart image, in-band escalates typed. The cap-rim arm is the
one Smooth arm in the crate that does neither, on a premise that is false.

**What is probably true, to be measured, not assumed:** an extrusion's walls
are ruled along the extrusion normal `n`, so at every rim point the wall's
normal is perpendicular to `n` and the cap's normal is `±n` — the dihedral is
exactly 90°, `Transverse` whenever the classifier decides, and the only
non-`Transverse` outcome is an escalation (a poisoned or collapsed arm →
`SliverRim`). If that argument holds for EVERY caller of `upgrade_rim`, the
arm is unreachable and its sentence must say that argument; if some caller
builds a body whose cap is not perpendicular to its walls, the arm is the
next instance of issue 1152's class and takes the must-carry treatment.

## Phase 1 — measure

1. Enumerate `upgrade_rim`'s callers (`extrude.rs:661`, `:673`; anything in
   `loft.rs`/`swept.rs` that reaches it or re-spells it) and, for each body
   kind that reaches the arm (straight extrusion over every profile leg
   kind — line, arc, circle; a path sweep body; a loft between planes),
   state the geometric relation between the cap's normal and the wall's
   normal along the rim.
2. Try to construct a body whose cap-rim dihedral is definitely smooth:
   oblique loft planes, a path sweep whose end tangent is not the profile
   normal, a profile with a tangent-continuous arc leg meeting the cap (the
   wall is a cylinder whose ruling is still `n` — expect Transverse). Record
   every attempt with the classifier's verdict at the witness
   (`Transverse` / `Smooth` / `Indeterminate`), or the door that refused
   the body first.
3. Report the table in the PR body. **The table decides the unit's shape**
   (§Phase 2 A or B); nothing is a stop.

## Phase 2 — the change

**A — unreachable (the argument holds for every caller).** The arm keeps the
D2 convention of totality but its doc states the TRUE argument (the
ruling-perpendicular-to-cap fact, per caller), and the prefer-intrinsic
sentence goes. For parity and one home, the arm takes the same treatment as
its two siblings anyway — the must-carry rule hoisted into ONE helper
(`geom_brep`'s `tangent_jet` + `curvature_lever_arm` + `tangent_second_order`
spelled once; `extrude.rs:881` and `revolve/upgrade.rs:177` call it) — so a
future caller that reaches the arm gets the honest description rather than
a no-op. The row for this shape: the existing extrude/sweep/loft suites
bit-identical (the dump differential over the extrusion corpus: every
description a rim edge carries at rest, before and after), and the argument
written where the arm is.

**B — reachable.** The arm takes the must-carry treatment through the same
shared helper; the constructed body is a fixture; rows: the smooth rim
carries `TangentIntersection` (jet-determinate) or a chart image
(under-determined) at rest and `validate_geometric` is clean —
`DescriptionNotAdjacent` no longer fires; a mutant (the arm back to a no-op)
reds it through tier 3; the interval twin.

Either way: the `DihedralClass::Smooth` consumer census (the S-BOOL sweep
that found this arm: `topo/splitting/finish.rs`, `census.rs`,
`boolean/ops.rs`, `boolean/rim_wedge.rs`, `validate.rs`, the three in
`sweep`) is re-taken and every arm's disposition stated in the PR body: no-op
with a true argument, must-carry, or refusal — and any OTHER no-op resting on
the prefer-intrinsic sentence is filed (it is another program's ground:
say whose).

## Constraints, binding

- Every existing rim description is bit-identical to the merge base
  (`bitdump` plus a description dump of the extrude/revolve corpus — the
  lane adds the rim descriptions to the dump if it lacks them).
- No new predicate; `tangent_second_order` is the one metered rule, reused.
- Comments state the invariant, not the history (discipline §4).

## Acceptance

The Phase 1 table; the shape taken (A or B) named with its evidence; one home
for the must-carry rule with the two siblings calling it; the consumer census
with dispositions; dump identical; hosted CI green at the drawn point plus the
interval lane asked for.

## Out of scope

The staleness-ladder consolidation issue filed beside 1390; the boolean
rebuild instance (#1382); `topo`'s own Smooth arms (dispositioned, not
changed).

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** The Phase 1 table is true: try to build a smooth cap rim the lane
  did not try (another profile leg kind, a degenerate-but-admitted
  extrusion, a sweep body through a different door).
- **C2** The must-carry rule has one home and the three arms are the same
  rule (diff the helper against both siblings' former spellings; bit
  identity of every existing rim description).
- **C3** The written argument at the arm is true per caller (read the
  callers, not the sentence).
- **C4** The consumer census is complete and each disposition honest.

# SECTARMS — the plan

The section object's remaining arms, cut out of SECT on 2026-10-28 at
Ev's request (SECT was over its budget). Every row
moved from `work/sect/` by `git mv`, with its id and body unchanged.

## What it builds on

SECT's section-first join (fork row 100; `work/sect/plan.md`, "The design"):
- **The section of a surface pair.** It is one value,
  `geom_brep::pair_section`, built at one door and held per operation in
  `topo::boolean::section_table` (step 1a, PR 5872).
- **The carried lane.** A chord is a sub-arc of the seated carrier, with
  `Pcurve::Projected`. Islands close by the chord's own curve:
  `RingClosure::Chord` and `LoopArc::Spline` (step 1b, PR 5894).
- **The torus poses and the critical gap.** One certified sweep decides
  tangency (`each-torus-pair-decides-tangency-on-one-pose`, PR 5924, in
  review at the cut).

Kind dispatch lives only in the value's constructor. An arm here is a new
arm of `pair_section` and is read by the existing lane; a per-pair frame
or lane beside it is a stopgap the design retires.

## The slate

| pri | item | cost | title |
|---|---|---|---|
| P0 | `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm` | H +design | A NURBS face against a cone or sphere has no C5 section arm (klein wall 3, teapot wall 3) |
| P1 | `a-nurbs-face-against-a-torus-has-no-section-arm` | H | A NURBS face against a torus has no C5 section arm (lily wall 8, behind SPLICEFACE's curved-crossing dispatch) |
| P1 | `cone-germ-pairs-against-a-curved-face-have-no-section-frame` | H +design | A cone wall against an oblique cylinder or another cone has no frame |
| P1 | `skew-cylinder-germ-pair-has-no-section-frame` | M | Two cylinder walls with skew axes have no frame |
| P1 | `the-walk-order-is-spelled-twice` | M | The order along a section conic is spelled twice |

## Order

1. **The NURBS × cone/sphere arm (P0).** Its design question (row 100's
   rung 3 for a spline face, and where the tangency along a boundary side
   is read) goes to the designer pair first. SECT's torus-pair designers
   noted the critical-gap principle generalises to NURBS × analytic with a
   2-D sweep of the knot rectangle, but cannot decide a tangency along a
   curve; that is the starting evidence.
2. **Skew cylinders and the walk order**, on the value and matching by foot
   parameter. The walk order may close by retirement.
3. **The cone frames**, once the cone has a metres conversion (SECTRUNG's
   `cone-torus-rung-three-arm-waits-on-the-cone` names the same gap).
The closed edge in a curved face and the C5 plane × torus / cone ×
cylinder residue went to SECTSITE at the cut.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`, so
the program closes without an exit walk.

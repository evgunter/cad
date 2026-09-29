---
id: validate-classifiers-and-lower-displays-classify-refusals-differently
kind: issue
title: validate.rs - about a dozen at-rest classifier arms put a refusal in a different class (defect, not yet, a lever) from the refusal type's own message, and each needs a ruling on which reading is true
status: open
opened: 2026-09-29
priority: P3
cost: M
refs: [validate-classifiers-paraphrase-lower-recourses-with-no-row-comparing-them, validate-own-close-levers-follow-the-d4-recourse-ruling, chart-region-corrupt-spells-its-kernel-defect-ending-by-hand, nappe-spanning-spells-its-kernel-defect-ending-by-hand]
---

**Where this comes from.** S-DUP's design fork on
`validate-classifiers-paraphrase-lower-recourses-with-no-row-comparing-them`
(PR 3387, row 8 of `docs/DESIGN-FORK-LOG.md`). Ev ruled on 2026-09-29.
The at-rest texts stay in `validate.rs`'s `classify_*` functions,
written for the checks window, and are not moved into the refusal
types: *"the choice should ultimately be made based on code quality
rather than drift risk"*. The two texts for an arm may differ on
purpose, because the lever differs by reading. What that ruling leaves
is the arms where the two texts disagree on **what kind of refusal
this is**. That is not wording. One side is wrong, and nobody has
decided which. Both designers found these arms independently. Their
tables are in PR 3387.

**The arms** (`validate.rs` on main at 2026-09-29; re-take at the
lane's merge base):

| classifier | arm | refusal type's own message | classifier at rest |
|---|---|---|---|
| `classify_pcurve` | `SingularChartJoint` | not built yet; nothing to repair (`pcurves.rs` ~:412) | defect |
| `classify_pcurve` | `LoopWraps` | report it | not yet |
| `classify_pcurve` | `OuterSpansPeriod` | describe it as pieces within one period | not yet |
| `classify_pcurve` | `LoopDiscontinuity`, `MissingCache` | re-mint the body | defect (is every at-rest body minted by its door? unmeasured) |
| `classify_mass_props` | `RingOnCurvedFace` (`props.rs` ~:229) | report it | not yet |
| `classify_mass_props` | `NappeSpanning` | report it | not yet |
| `classify_mass_props` | `NotOneChartBranch` | state it as two edges | not yet |
| `classify_mass_props` | `DegenerateFace` | the coincidence menu | defect |
| `classify_mass_props` | `QuadratureBudget` | loosen the tolerance or simplify the trim | loosen the tolerance (both texts miss D4's last-resort qualifier) |
| `classify_chart_region` | `ChartDivergence` | give the pair a structural identity | not yet |
| `classify_chart_region` | `NonPlanarTrim` | structural-identity and re-mint levers | not yet |
| `classify_chart_region` | `RayExhausted` | move the point off the boundary it grazes, or read the pair at a tighter ε | the coincidence menu (the lane offers no declaration) |
| `classify_chart_region` | `DegenerateLoop` | repair the loop or re-mint its pcurves | widen the face |
| `classify_census_cause` | `FaceUnboundable` | repair the face's loop, then check again | defect |

**The unit.** For each arm, decide which reading is true at rest, then
fix whichever text is wrong. That may be the lower message, and the
fix then lands on the type's owner by announced seam. Where the two
texts differ on purpose, the classifier says why in one line. The
per-arm rulings follow D4's recourse ruling (`[ev]` PR 3352) and the
D2-addendum rows. An arm whose true class is unclear is a question for
Ev, not a guess.

**Already filed, fold if they are still open:** the `own_close(…, "lower
the tolerance")` arms (`work/encl/validate-own-close-levers-follow-the-d4-recourse-ruling.md`),
`work/chart/chart-region-corrupt-spells-its-kernel-defect-ending-by-hand.md`,
and `work/props/nappe-spanning-spells-its-kernel-defect-ending-by-hand.md`.

**Fixed already, not members:** `ArcLoopUnsupported` (#3388) and
`classify_offset_fit`'s `Band` arm (#3403).

**Instruments.** `topo::test_support_samples::validation_error_samples`
enumerates every nested variant. `refusal_concision_at_rest` renders
each one through the viewer's standard. That row checks a sentence's
shape, not its truth, so a wrong class stays green. A plant proves only
that the arm is reached.

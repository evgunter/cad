# M5 PR 9c — the banked curved-boolean completions (binding spec, DRAFT until dispatch)

Origin: PR 9 review dispositions (M5-LOG 2026-08-01). Deviations
2+3+5 ruled ACCEPT-AND-BANK as a unit, plus the curved-revert lane
(review MAJ-3 ruling) and the edge×NURBS-face sweep layer (review
F6 aftermath). **Gates PR 12** (die pips = curved SUBTRACT; fillet
trimlines = Fitted pcurves). Schedule after PR 9 + PR 10 merge;
takes the block-13 FABLE remainder. Sections firm up at dispatch;
blocker sizings below are the PR 9 reviewer's executed findings.

## Scope (five banked lanes, one unit)

1. **Cylinder×sphere boolean arm** (PR 9 dev 2): the fitted-chord
   join lane — splitting an SsiBranch at crossing vertices with a
   window-analog selection (fitted carriers have no closed-form
   chart windows; the selection lane needs its own certified
   story), plus sphere-face containment/pierce doors
   (point_in_solid::face_geo supports {Plane, Cylinder} only).
2. **`Pcurve::Fitted` storage variant** (PR 9 dev 3, PR 7 dev 3
   lineage): Copy drops from Pcurve/PcurveCache (Arc payload, the
   Surface-at-PR-3 precedent), rippling through topo storage;
   certification via C2.2 hull bounds in metres (SSI limbs);
   UnsupportedCarrier retires for this class; rung-3 edges at rest
   get real pcurves (today tier 3 refuses honestly rather than
   lies — verified posture to flip).
3. **Curved revert → subtract/intersect** (PR 9 MAJ-3 ruling):
   RevertError::UnsupportedSurface's planar-only lane generalizes;
   the honest union-only front door (landed in PR 9's fix pass)
   retires. Die-pips subtraction is the acceptance anchor.
4. **Cylinder×cylinder equal-radius germs** (PR 9 dev 5): the
   two-cylinder window story for the join dispatch (classification
   arm already in the table).
5. **Edge×NURBS-face sweep layer** (PR 9 dev 4 aftermath): the
   crossing/containment residual machinery for NURBS operand faces
   (curved_face_arm endpoint sides currently poison → Escalated);
   with it, the (Plane, Nurbs) boolean arm goes live end-to-end
   and the shape-(iii) CUT row (loft body × plane, PR 10's
   coordination clause) lands green wherever it is still pinned
   refused.

## Acceptance (to firm up at dispatch)

Cylinder×sphere union AND subtract e2e (both lanes, corpus rows,
certified Fitted pcurves on the seam); die-pips-shaped subtract
smoke row (a sphere bitten out of a slab) ahead of PR 12; the
2-arc-disc union row if PR 9's fix pass shipped the typed arm
rather than arc-aware facing; shape-(iii) cut-loft row flipped
green end-to-end; every refusal this unit retires is re-pinned as
its construction row (the S9 flip pattern); multi-ε honesty per
standing rules.

## Process

Standard: binding spec finalized at dispatch; one implementer
(A/B: block-13 fable remainder) + blinded adversarial review +
fix pass; touched-crate battery + Interval; merge-priority not
expected (nothing here fixes a live wrong-body).

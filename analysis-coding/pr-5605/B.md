# Review B — PR 5605, frozen head ddb44f28

**Verdict: APPROVE-WITH-FIXES** (the MAJOR in the flush arm and the README). Almost all of the location confinement holds and the PR's rows catch a slide back to by-pair. CI green. No glimpse. Probes on a review branch (10 rows, 1 red on purpose).

## MAJOR
**M1 (claims 4, 5): the flush arm treats a refusal as "the interiors do not meet".** `flush_at_rest` turns `Err(TouchingBoundary)` into a `SameOriented` flush row. On the shared-chart path (`declared_pair_overlap` → `declared_chart` succeeds when both faces share a surface → `overlap_on`) TouchingBoundary returns without the interior-witness rung ever running (instrumented). Any two copies with identical carriers overlapping over an area and sharing a boundary segment get a flush row. Probe (a base ∪ sunk fin grafted beside a twin fin; walls overlap 0.75×0.75): 4 flush rows where 0 is right. Controls (two prisms; two half-overlapping blocks) give PositiveArea and 0 rows. Re-derived from the golden: heat_sink's +20 are 4 per pattern fin against the union solid, not "fins on one plane with the base's" as the test doc and PR body say. Product loud anyway, but the census records a wrong decision and "Door 2 must certify no area" is false. Secondary fail-open path (inspection, likely): `interior_witness` maps a `contfp` error to "not inside". Execution · sure.

## MINOR
- m1 (claim 1): tangent-line arm's negative branch pinned by nothing: mutant `Some(dir) => Some(true)` survives 1788 rows. The point arm is sound only if the contact is a single point; sphere seated in an equal-radius cylinder (`pair_contact` → Point, contact a circle) unprobed. Execution · sure (mutant); unsure (sphere case).
- m2 (claims 2, 6): topo README "Confirmed pairs" still states the by-pair rule Ev rejected; names retired rungs; "Vertex-on-edge at rest" names `Declared::ve_face_backed`; a test module doc cites `vv_face_backed`. Inspection · sure.
- m3 (claim 6): contacthold `census-face-rungs-for-a-collinear-overlap-are-unconfined` still parked though this PR closes its subject; `declared-only-meetings-clear-at-the-census-gate-unread` cites four retired rungs. Inspection · sure.
- m4 (claim 4): step-import `gate3` now adds `flush.findings`, so an in-band near-flush pair between instances refuses the import; neither the body nor a row says so. Inspection · likely.

## NOTE
- n1: non-convex faces work (U-shaped base: collinear edge across the notch, tip on the far edge, corner on far corner each own rows; L overlap and a touch at the overlap corner → one face pair; faces meeting at a point → no pair). By-pair and corner mutants each caught by 6 PR rows + 3 probes. Execution · sure.
- n2: incidence lanes sound for area pairs; for a line pair only if the curved trim leaves the tangent line from the vertex (no counterexample named). Collinear `confine` reading the midpoint is sound. Unsure/likely.
- n3 (pre-existing): `ee_cross_backed` lets a pair answer for a third solid's edge crossing inside its overlap closure. Unsure.
- n4: shared-surface opposed planar pairs miss the planar arm by the same shared-chart path (fall back to sweeps, loud). Likely.
- n5: quieting not wired: no check reads `Gap` for flush rows; unclear whether scheduled. Unsure.
- n6: kitchen_sink re-derived (16 OnCarrier → 2 SameOpposite + 9 SameOriented; findings stay 29); heat_sink@10/40/160 0 → 20 each; counts match, heat_sink cause does not.

## Style (Q1–Q6, Q8 partial)
- Q3/Q4 likely: the flush door depends on a comment's premise; two readers give one refusal opposite meanings.
- Q1 likely: line branch re-inlines the `wedge_holds_named` loop beside `corners_meet`.
- Q1/Q7 sure: `pair_contact` returns Area for any two planar faces without reading the carrier relation while `in_overlap_closure` assumes opposed senses.
- Q2 unsure: `Declared::holds`/`ee_bound_backed` push refusals from one pair even when another holds the touch.
- Q5 sure: README promises by-pair behaviour.
- Q6 likely: PR body's "Door 2 must certify no area" overstates.
- Q2 unsure: ~30 lines justifying `ee_cross_backed`'s absorption.

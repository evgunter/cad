# ASM exit walk — criteria vs evidence

**STATUS: RATIFIED — ASM CLOSED at v1 scope (Ev's approval
given directly in-session, 2026-08-23, after resolving his one
concern: the A11 rule-4 question, ratified same-session as the
member-vocabulary rider in ASSEMBLY-DESIGN — see #945). This
document is ASM's done-state of record.** ASM = the assemblies
implementation program (`docs/ASM-PLAN.md`; design
`docs/ASSEMBLY-DESIGN.md`, A1–A13, ratified via #333) at its v1
scope, rungs R1–R2. Criteria are quoted **verbatim** from
ASM-PLAN's exit-shape paragraph, one clause per row, with the
demo-purpose sentence as its own row. Dispositions per the
M5–M8 convention: MET / MET-WITH-RECORDED-HONESTY / CARRIED
(named owner).

## The walk

| # | Criterion (verbatim from ASM-PLAN) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "An assembly document authors, evaluates, validates, and round-trips end-to-end at v1 scope" | MET-WITH-RECORDED-HONESTY | The pipeline is live end-to-end: identity + pins (**#364**, schema v5→…→v14 without ever moving on a demo), A10 roots + the product gather (**#383**), the multi-solid kernel door (**#381**), InstantiatePart (**#414**), sub-assemblies (**#425**), the update door (**#549**), the constructive solve (**#575**), minting + the assembled A5 gate + crossings (**#591**), and the exit demo (**#938**) drives all of it through the `pncad` façade from an on-disk workspace, CI-gated at three tolerance rows and rendered. **The honesty**: "validates" for a MATED assembly reaches the declared direction's frontier — `Uncertified`, every cross-instance declaration declined — not certification. That is the census Door-2 structural-chart-identity gap (two grafted instances share neither a `SurfaceKey` nor a `GeomSource`), steered to M9 on #591's thread before the demo made it visible at user scale (#938 F2). A disjoint assembly validates outright (the demo's layout does); the mated stand's gate honestly reports its frontier and refuses `AtRest` loudly. |
| 2 | "N instances + patterns evaluate to one multi-solid body with instance-qualified names" | MET | **#383 + #381 + #414 + #425**, exercised at user scale by **#938**: 3-instance mated stand and 5-solid patterned layout evaluate to single multi-solid bodies (volumes exact to 1e-12 against hand derivation, re-derived independently by both ordinal-66 reviewers); 130/78-name tables; the natural lookup ("pattern instance 2's post cap") resolves through the nested `Instance(2)` → `InPart` → cap name to a world frame. Selector depth is the recorded cost of the nesting vocabulary (#938 F11 — not a defect, stated). |
| 3 | "pins move only by recorded update-edits with mate re-verification" | MET-WITH-RECORDED-HONESTY | **#549** (A13's four clauses executable; same-pin refuses, staged update-all, mixed-pin lint, memo re-key hardened) + **#938**'s update door: a part edited on disk moves nothing until `update_reference` is recorded; the migrated assembly re-verifies and — on the deliberately shorter part — the fit gate refutes both declarations, each naming its mate. **The honesty**: the mixed-pin state A13 declares legal is not evaluable against the one-file-per-id workspace (#938 F5) — the lint permits what the store cannot serve. That is AQ1 (the document store) arriving at user scale; carried on AQ1's register. |
| 4 | "split/inline hold their acceptance" | MET | **#525** (the recorded refactorings + acceptance harness) + **#938**'s round trip over the whole name table, both directions — where the ordinal-66 review sharpened the claim itself: `inline` mints fresh host ids, so the acceptance property is name-resolution identity through the two RECORDED node maps (split's ∘ inline's), exactly A4's contract ("arena-key identity not required"), now enumerated per-name with the crossing count pinned (26) and the hoisted cluster frame bit-equal on restoration. The patterned-post second cut is probed rather than assumed, and both arms report. |
| 5 | "constructively-solvable mate chains place and verify" | MET-WITH-RECORDED-HONESTY | **#575** (mate chains place clusters; UNDER/CONTRADICTORY refuse naming subgroup and clash; Δc ≡ 0 by construction) + **#591** (declarations mint into the record set; the assembled gate runs; crossings re-verify) + **#938** (a 2-mate chain seats the shelf and far post, both `Determining`; the solved pose asserted — translation AND rotation — against an independently hand-composed frame; solved poses provably absent from authored placements per A11 rule 2). **The honesty, three named carries**: "verify" reaches the frontier, not certification (row 1's F2); a face-pair declaration does not yet back the edge/vertex contacts the same seat induces — **#943**, reframed on Ev's steer to extend the boolean lane's EXISTING face-backed closure (`vf_face_backed`, census D4) to the at-rest lane rather than re-minting the table-with-legs machinery as mates; and mates × patterns do not compose in v1 (**#945** — RULED at this walk's ratification: A11 gains the member-vocabulary rider, no algebra change, `Instance(i)` heads canonical, parameters never solved; #945 is now the banked implementation unit). |
| 6 | "everything outside v1 refuses typed with recourse text naming its rung" | MET-WITH-RECORDED-HONESTY | The refusal walls landed unit by unit (ASM-4's nine arms all tested naming subjects; R2-a's UNDER/CONTRADICTORY; R2-b's mate-attributed gate refusals and `CrossingUnverified`; the workspace's typed resolution refusals) and **#938** walks four as a user hits them: under-determined (residual in class vocabulary + recourse), contradictory (both mates + predicate + measured clash), outside-the-at-rest-vocabulary (`Tangent` → `NoAtRestRecord` quoting the class table's reason), wrong pin (both pins + recourse verbatim). **The honesty**: `Contradictory` and `NoAtRestRecord` carry no recourse *sentence* — the library's text gap, not the demo's — recorded with the doubled-recourse defect in **#947** (whose demo assertion goes red when the fix lands, by design). `Fit` is not yet a kernel variant, so its refusal is unreachable by a user (#938 deviation 4, disclosed). |
| 7 | "Demos demonstrate real usage per the standing demo-purpose rule." | MET | **#938** is authored the way a user would write it (both ordinal-66 reviewers judged the model real and the one geometry accommodation — inset posts — legitimate modeling, reproducing the flush-seat refusal it avoids); every awkwardness met is gap-commented at its site and scheduled: **#943–#948** filed from the walk, F2 on the #591→M9 steer, F5 on AQ1, and the whole Python answer — the assembly surface is unreachable from `pncad-py`, `evaluate` takes no resolver — deposited into `docs/LIB-LOG.md` as a dispatchable bindings series with the demo named as its coverage oracle. The demo also repaired the seam it exposed: the A5 gate was authoring-reachable but not validation-reachable through the façade, adjudicated an accident and re-exported (#938 fix pass), restoring the tour's every-door-through-`pncad::` invariant. |

## Walk evidence beyond the criteria

- **The A/B record**: every unit row recorded AT MERGE with
  per-phase tokens and wall (ASM-1 through ASM-DEMO), spanning
  the v3→v4→v5 protocol transitions; the program contributed
  cross-model pairs #5 (TESS-SPAN, ordinal 54), and #21
  (ASM-DEMO, ordinal 66 — carrying the #952 ordinal-collision
  correction, resolved by dispatch order with the two-ended
  claim discipline adopted). Dual labels diverged / substance
  converged remained the stream's stable calibration signature
  through the program's last row.
- **Schema discipline**: v5 (ASM-1) → v14 (ASM-R2b) across seven
  bumps, two mid-flight collisions caught only by the by-eye
  constant read; the exit demo round-trips v14 UNCHANGED — the
  walk's own evidence that the surface is closed at v1.
- **Side lanes discharged en route** (Ev-assigned, not
  R1/R2 scope): MESH-PROBEGATE (#579), TESS-SPAN (#594,
  leaf_a 261,780 → 84,524 triangles), and TESS-SPLIT — spec
  reconciled with the sliver lesson (#936) and **in flight as
  PR #951 at this walk's writing** (review ordinal 67 running;
  its state is the ASM-LOG tail's, not this document's).
- **Banked, per the plan's own scoping**: **ASM-XSPLIT** (the
  AQ8 conversion door — crossing mates passed explicitly at
  split; spec not yet written; the one follow-on unit the
  program hands its successor), R3 (rides M9/C7 per the plan's
  "Out of this program"), R4 (instanced evaluation, mirror,
  import-as-assembly post-AQ1), rungs (c)/(d) as their own eras.
- **Open with named owners at walk time**: #943 (census
  closure — design conversation), #944 (alignment-frame-from-
  face door), #945 (mates × patterns — RULED at ratification,
  the A11 member-vocabulary rider; now a banked implementation
  unit), #946 (sub-assembly declarations at the seam), #947
  (recourse text), #948 (parametric loop helper), #950
  (TESS rim-chord residual), F2/census Door-2 (M9), AQ1 (the
  store), the LIB-LOG bindings series.

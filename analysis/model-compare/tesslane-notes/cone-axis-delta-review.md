DELTA RE-REVIEW — PR 7049 @ 58565e3f0d. Final. Differential base: the merge-base 0309707d31 (main as this head merged it; main has since taken PR 7050, which touches no mesh file).

**Verdict: NOT-MERGEABLE-AS-IS**, narrowly. The fix pass did what it says and the `SeamTwice` cure is sound as far as I could push it. What blocks is ruling C: its premise is false, so a default-ε regression is in the tree unruled, and the PR, the filed row and the pinned row state something untrue.

## Claims
1. HELD. One-face full-turn cone walls, seam split at 1, 2, 5 points and near each end, all with check_mesh, resolved outward winding, sampled deviation and volume:
   - half-angles 0.5°, 2°, 10°, 25°, 29°, exactly 30° (sector = half turn), 31°, 35°, 45°, 60°, 80°, 88°, outer and reversed inner, two scales (δ up to 15× the radius), δ 0.3–0.001: 1727 cells mesh;
   - tall walls whose narrow sector holds the chord between a seam point's two copies (5°–29.9°, slant ratio 15–150, δ 1.0–0.03): 672 cells mesh; 1° likewise (114).
   Not built: a ring on such a face (the boolean refuses my brick cuts `GermSectionOpenConic`, a hyperbola); the apex on the seam (an axis-touching revolve builds half faces); a partial turn that walks a seam (no door). Mutant `SeamTwice` restored: the two rows named go red. No suite body changed verdict, so the cure touches no existing mesh.
2. HELD. `got_enclosing` answers `Meshed` only after `check_mesh`, then winding, volume, outward and deviation, per cell. Reading the volume off the unsplit solid hides nothing: `split_edge` moves no geometry, and a missing or doubled patch fails `check_mesh` first. 9/9 at ε 1e-6 and 1e-12; 503/503 with the slow set at the default.
3. Attribution HELD, extent CORRECTED (MINOR-A).
4. FALSIFIED (MAJOR-A). Mechanism as stated. Reach is not.
5. HELD by reading and by survey. In `chords::tighten` a line's count rises only on a side with a lattice: planar none, trimmed cylinder none, cone arm `hv = ∞` on a ruling, sphere/torus/spindle bound no line; only the spline arm. My own survey off final boundaries: line edges above one chord 224 in `mesh` (222 plane|spline, 2 spline|spline) and 78 in `sweep`'s 62 meshing modules (60, 18); no line edge at all between a ruled face and a spline face in either. Profiles are lines and arcs only, so the doors are import, a cut loft, a blend. I built none.
6. HELD. Door dropped → the notched row; guard deleted → the side row; chord pass blind → the counts row and the class row; route removed → the PR's four among seven in my subset. The count pin works but pins numbers, not the relation (NOTE-C).
7. HELD. G is two comparisons, as stated. The hair-split fold appears in no base-meshing cell of either sweep.
8. HELD. Per-patch digests, first head against fix head: `mesh` 10455 common bodies, 0 patches moved, 0 verdict changes; `sweep` 3016 common, 0 and 0. Only the retired row's and the new rows' bodies differ.
9. CORRECTED in three places: "at ε 10⁻¹² only" (false); "Two thin-annulus cells" as the list of what meshes on the base and refuses on the head (incomplete); survey counts 236/80 against my 224/78 (a different instrument; the conclusion holds). The mutant table, the density table and "nothing else moved" are true.
10. Below.
11. Hosted CI on 58565e3f0d: change filter, lint, test (kernel), test (app), corrupt input, viewer wasm32, step import, demos, tools, mesh budget meter, interval backend: success; interval oracle and cache prime skipped; python suite still in progress when I last read it, so the gate is not yet closed. Slow set, run here: `mesh` 503/503; `sweep` meshing modules 578/578; the two viewer rows pass. Not run: step-import, stl and tour slow rows.

## Findings
- **MAJOR-A, DEMONSTRATED.** `trimmed` development cut, via the route. `DevelopedMerge` at the DEFAULT ε: 408 cells mesh on the base and refuse on the head, δ 0.1/0.01/0.001. Bodies: `revolve` of (1,0),(2+tan α,0),(2,1),(1,1) and of (0,0),(2+tan α,0),(2,1),(0,1), full and half turn, one cone ruling split, oblique.
  - α 0.01°: a split 1e-5 of the range from either end, or two splits 1e-5 apart mid-ruling, and every finer gap `split_edge` admits;
  - α 0.1°: 1e-6; α 1° and 3°: 1e-7; α 26.6°: two splits 3e-8 apart.
  The cut is 2⁻²⁷ of the slant radius r/sin α, so a draft-angle cone resolves far less than a 45° one. Neither "only at ε 1e-12" nor "only a hair from an end" is true.
- **MINOR-A, DEMONSTRATED.** Thin planar neighbour. 373 base-meshing cells refuse `Triangulation(Plane)` on the head with a ruling split: plain cylinder walls, tapered walls and stepped rods, widths 0.01–0.05, radii 0.3–4, full, half and 1 rad turns. Every one has δ > the face's width (largest width/δ 0.67); none at δ ≤ width; the base refuses the same faces the same way with a rim split (405 cells). The PR and the thin-arc row name one body.
- **MINOR-B, BY-INSPECTION.** `split_rulings::a_ruling_split_within_the_developments_resolution_refuses` asserts at the default ε that the body cannot be built. True of its 45° body; it reads as the class's answer and is not.
- **NOTE-A, BY-INSPECTION.** `trimmed.rs` census after emit: the fan re-derivation the cure cites is a `debug_assert`, so a miss is a panic, not a typed refusal. The release profile keeps debug assertions, so it does run.
- **NOTE-B.** The ring, apex-on-seam and partial-seam cases of claim 1 are unbuilt, not passed.
- **NOTE-C, BY-INSPECTION.** `split_rulings::a_routed_face_is_chorded_for_the_chart_lane` pins eight triangle counts. Any density change re-pins it, the strip unit certainly; what it means to hold is that a routed face's rim is chorded at the target its lane sizes at.

## The three rulings
- **(a) thin annulus: ACCEPT WITH A CONDITION.** It is the filed class, and a face narrower than δ at tolerance δ is a fair refusal for an interim unit. The condition: the PR and the row state the rule (a planar neighbour narrower than δ beside a routed face, cylinder walls included), not two cells. Holding the base's chord count on a shared rim would cure it, but that is the mismatch the chart lane's headroom exists to make; I would not ask it of an interim.
- **(C) `DevelopedMerge`: REJECT as ruled.** Three of its four reasons survive (the base's lane had no theorem; the cure is a design choice; the strip unit removes it). The first does not, and it carried the weight. A 1° draft cone with two vertices 1e-7 of its ruling apart at the default ε is rare but ordinary input, and it meshed clean on the base. I would accept a residual only if re-ruled on the true reach: the row, the PR and the pin corrected to 2⁻²⁷·r/sin α, a pin at the default ε, and the priority reconsidered above P3.
- **(F) the guard: ACCEPT.** One expression, true text, a typed refusal on the right edge, a row that reds without it, and no body in three suites. If a body arrives it is the filed row's, not this unit's.

## First findings
- MAJOR-1 (`SeamTwice`): fixed. MAJOR-2 (`DevelopedMerge`): declined as a residual; reason not accepted (MAJOR-A).
- MINOR-1: pinned and filed, accepted. MINOR-2: fixed. MINOR-3 a, b, c: fixed.
- NOTE-1: ruled and filed, accepted. NOTE-2: fixed.
- S1: fixed differently (one expression; the two predicates' difference stated and filed), accepted. S2, S3, S4, S5, S6, S7, S9: fixed. S8: declined to PR 7041, accepted. S10: not addressed, yours at closing. S11: filed. S12: the count pin now moves under the mutant, accepted with NOTE-C.

## Executed, and read only
Executed: own probes on both trees at the default ε (seam 1920 cells, thin neighbours 6885, hair splits 1440), tall walls on the head (954); whole `mesh` suite with the slow set and my digest hook on the head; `sweep`'s meshing modules likewise; five mutants over the split-ruling, `curved::tests`, T1 and D9 rows; the unit's rows at ε 1e-6 and 1e-12; the two viewer rows; `work.py lint` (ok). Read only: `chords` for claim 5, the emit pass and census, the four filed rows, step-import. Nothing committed, pushed or posted; both trees restored.

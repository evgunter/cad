# FORK-S5Q — what quiets an at-rest finding (designer B)

## For Ev

**Recommendation (likely): one rule, entailment at the finding's cells.** A finding is quiet exactly when a *holding* assertion *entails* it: the assertion's two selections are the finding's two cells, it reads a measure's output directly, and every value its relation and bound admit lies in the finding's stratum of that measure. The three phrases D10 turns on are not three rules; they fall out of this one.

**Premise check.** "The same measure" presumes a finding has a measure. It does not: the census decides contact by a zero residual and interference by a vertex in the other copy's material, and measures nothing. What a finding has is a *stratum*: contact is "these two cells coincide" (C5's `g = 0`, structural or not), interference is "these two materials overlap" (`g < 0`). An assertion speaks to a finding when its holding implies that stratum, whatever primitive it reads. "Does not straddle zero" is the wrong test for the same reason: `gap ≥ 0` admits `[0, ∞)`, which does not straddle zero, yet says nothing about touching; D10's own example (`= 0` for contact, a bound on one side of zero for interference) is the stratum test, not the straddle test. The text should say what its examples mean.

**Definitions.**

- **Cell.** `(copy, name)`: a `Body` output of a world placement and one `StableName` of that copy (a face, an edge, or a vertex where the record names one).
- **Stratum table.** Each `MeasurePrimitive` states which of its values mean clearance, contact and interference, beside `dim()`: `Gap` (`> 0`, `= 0`, `< 0`); `Distance` and `MinClearance` (`> 0`, `= 0`, none: both are magnitudes); `Angle` (none: `0` means parallel, not coincident). It is one table in `measure.rs`, extended with each new primitive and never consulted by evaluation.
- **Admitted set.** `{x : x relation b}` for the bound's value `b` in the same evaluation, `b`'s sign decided against zero through the one margined decision assertions already use (a bound inside the band reads as zero).
- **Entails.** The admitted set lies inside the finding's stratum of the assertion's primitive.

**The rule, as a table.** Quiet needs: verdict `Holds` in the same evaluation (`Violated`, `Unevaluated`, poisoned, or a primitive with no `f64` value such as `MinClearance`, quiets nothing); `value` reads one `Measure`'s output directly; the measure's two selections equal the finding's two cells as a set; and entailment.

| assertion | contact finding | interference finding |
|---|---|---|
| `Gap = 0`, `Distance = 0` | quiet | — (never holds there) |
| `Gap = b`, `Gap ≤ b`, `b < 0` | — (never holds) | quiet |
| `Gap ≤ 0` | loud (admits `< 0` too) | loud (admits `0` too) |
| `Gap ≥ 0`, `Gap ≥ b` (`b < 0`), any `Angle`, any bound on a defined variable over a measure | loud | loud |

No primitive is special-cased: an unsigned primitive cannot quiet an interference because its admitted set under any bound that could hold is never inside an empty stratum, and a mis-roled `Gap` (outer and inner swapped) at an interference reads positive and is `Violated`, so the verdict carries the role check.

**Sites.**

- **Contact.** The finding is stage 4's value-decided coincidence record at rest, and its site is the two cells that record names, of two copies. An assertion matches when its selections are those two cells. Not an incident face of them: a corner or a tilted edge resting on a face has no measure that could hold anyway, and widening the match would only let a holding assertion on a neighbouring face answer for a cell it does not measure.
- **Interference.** One finding per connected component of the two copies' overlapping material. Its site is the faces of each copy that bound that component; an assertion matches when it names one bounding face of each copy. The component is found by intersecting the two copies (the boolean, `f64`, at the census's band), on interfering pairs only. A pair whose intersection the kernel refuses (a sliver, a carrier pair without a join arm) is a **could-not-look** finding of the check — it could not say *where* — not a loud interference: DS6 already has that category, and it refuses at every severity but Off, which is the safe direction. Localizing by the census's witness instead (vertex-incident faces) is rejected: a pin in a bore has no vertex of either copy in the overlap except a seam vertex, so the witness names the wrong faces or none.

**Worked example.** A pin (radius 5.01) pressed through a plate's bore (radius 5), the pin also cutting into a rib of the plate. The overlap has two components: the annular tube between the bore wall and the pin wall, bounded by `{bore wall, plate top, plate bottom} × {pin wall}`, and the rib notch, bounded by rib faces and the pin wall. `Gap(bore wall of the plate copy, pin wall of the pin copy) ≤ −0.005 mm`, holding, names a bounding face of each copy of the first component and quiets it; the rib notch names no bore face and stays loud. The same assertion over the unplaced bodies' faces matches no cell (the cells are the copies') and quiets nothing. With `≤ 0` it quiets nothing; with `≤ −0.02 mm` it is `Violated` and quiets nothing. Two copies resting face to face by values: a contact finding on the two faces, quiet under `Gap = 0` or `Distance = 0` over them, loud under `≥ 0`.

**What it makes representable, and not.**

- An assertion cannot hide a second overlap: a distinct component has distinct bounding faces. One connected overlap with several mating pairs (a block wider than its pocket in both axes, one ring-shaped overlap) is quiet under an assertion on any one of its pairs; the report lists the component's faces, so the others are visible, not hidden. Tightening this to "every opposed same-kind pair bounding the component" is a later, stricter, reversible step.
- A contact whose cells no primitive measures (a vertex on a face, an edge lying in a face it is not parallel to, a curved face against a plane) is loud until one exists or the contact is constructed. The cure is a primitive with that arm, which adds a row to the stratum table and changes no rule. After stage 2 E no vertex is selectable, so a vertex-on-face contact cannot be quieted at all today.
- `gap ≤ 0` quiets nothing. A press fit states its minimum interference, as every real fit does; "no clearance" is not an intent about overlap, and admitting both strata is exactly the indifference the lint exists to report.
- An assertion on `min(gap₁, gap₂)` or `2·gap` quiets nothing: entailment through arithmetic is not decided. Two assertions say it.

**Cost at rest.** Contact: an index of assertions by cell pair, one bound-sign decision per candidate; nothing geometric. Interference: one boolean intersection per interfering pair, which is the only new geometry; the quieting itself is the same index. Nothing evaluates a measure for the rule; it reads verdicts and written bounds.

**Ratified text to change (D10, Assertions).** The sentence "A finding is quiet exactly when an assertion on the same measure at the same site has a bound the observation meets and that does not straddle zero: a contact finding under an assertion that the gap is zero, an interference finding under a bound on one side of zero." becomes:

> A finding is quiet exactly when a holding assertion entails it: its measure's two selections are the finding's two cells, its value is that measure's output, and every value its relation and bound admit lies in the finding's stratum of that measure, `{0}` for a contact (`= 0`) and the negative values for an interference (`≤ b` or `= b` with `b < 0`). Each measure states which of its values mean clearance, contact and interference; one whose values cannot mean the finding quiets nothing. A contact finding's cells are the two the census decided coincident. An interference finding is one connected component of two copies' overlapping material, its cells the faces of each copy that bound it; a component the kernel cannot bound is a finding that the check could not look.

**Rejected final states.**

- *The literal straddle rule* (`≥ 0` quiets contact, `≤ 0` quiets both). It contradicts D10's own example, and it quiets the one hazard the contact finding exists for: a point coincidence under variation that nothing said should hold. Sure.
- *Site = the copy pair.* One holding fit assertion quiets every overlap between two copies, including the rib notch above. Sure.
- *Site = an incident face of the record's cell.* Buys nothing today (no measure holds there) and lets a neighbour answer. Likely.

**Confidence.** Entailment as the principle: sure. The stratum table as its home: likely. Per-component interference sites by intersection: likely (cost is the open question; the fallback is the witness, which is worse, not a different rule). Could-not-look for a refused intersection: likely.

## For the orchestrator

- The embargo on §11 leaked: §1 (items 1–3 of the quieting rule, "FORK-S5-1/2/3 … recommended") and §3 restate the recommendations inline. I built the rule from the entailment principle before comparing; it converges with §1 on stratum containment and per-component sites, and differs on three points: no special case for `MinClearance`/`Distance` (derived from the table), the site is a cell pair of whatever kind stage 4's record names (the spec's `FaceSite` assumes faces), and a refused localization is could-not-look rather than a loud unquietable interference (FORK-S5-4 should decide which severity path it takes).
- Provenance: `git log -S` on the sentence hits only graft commits in this clone; fork-log row 62 says D10 was ratified at PR 3990 (2026-10-03) in three decisions, none of which is the quieting wording, so the sentence is agent-written text Ev approved as part of the whole. Unsure; I could not read the PR.
- Spec test 6's pin-in-bore depends on the census having a lane: the containment arm decides it only when a seam vertex of one copy lands strictly inside the other (a pin protruding past the plate puts the bore's seam vertices inside the pin); with the pin's ends flush, no vertex is strictly inside and the curved×curved backstop refuses undecidable. Construct the row with a protruding pin, and note the seam dependence is a census matter, not this rule's.
- After stage 2 E, measure operands read `Face`/`Edge` selects; today `carrier_of` also reads vertices. Whether stage 2 E keeps a vertex operand decides whether any vertex-on-face contact can ever be quieted; `distance` has no line×plane arm either, so an edge lying in a face has no measure. Belongs to INTENT as a measure-vocabulary row, not to this fork.
- The census's band and `ASSERT_BOUND` are two margined decisions on one geometry: a contact the census calls Zero can read `Violated` under `= 0` near the sliver. Existing hazard, not this rule's; the rule reads the verdict and reports the disagreement as a loud finding beside a violated assertion.
- The bound-sign decision reads the bound variable's value in the same evaluation; when a bound is defined over an observed variable, that value is measured, which the rule tolerates (it reads bound values, never measure values).

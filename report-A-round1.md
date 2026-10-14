# Designer A, round 1 (proven-zero), as delivered 2026-10-14

(Dispatched twice. The first A was lost to a container restart before it delivered. This report is from the re-dispatch, which got the same brief and was told not to read analysis/ branches.)

## For Ev

**Recommendation (likely): leave the class split as it is.** A zero the symbolic tier proves keeps ending as its in-band and exactly-zero siblings do. `StoredDefinite` is not the wrong concept and needs no per-sign form. The one real defect is narrower: `CertCheck::ParamSpan`'s whole refused side means "kernel or file defect" at every door, but only its prose note says so, and its ending opens with a lever the note itself rules out. Fix that one decision, and clarify what "definite" means in D4 ¶1 (iv).

Terms:
- *No-size class* (`Shows::NoSize`): a reading in or near the band that shows no size the user could intend. That covers an exact zero, a proved zero, and an in-band reading wholly on the refused side.
- *Refused class* (`Shows::Refused`): a reading past the band, wholly on a sign the decision refuses.
- *At rest*: a reading over a stored body, that is, the validator and the import door past ε_in.

### Premise check: three corrections

1. **A proved zero is never past the band** (sure).
   - The symbolic tier attempts a proof only when the numeric answer was Zero or undecided. A definite nonzero answer skips proof (`Sym::decide_sign`). So a proved zero is always a near-zero reading.
   - D4 ¶1 (i) governs that region: "user-facing messages and recourse never fork on exactly-on vs in-band below ε_input … kernel semantics keep the distinction (message policy, not predicate policy)".
   - A proof is kernel semantics. It says *how* the zero was decided, not that the user should do something different.
   - Routing proved zeros to the at-rest defect ending would split one zero story by which tier answered: exactly-on would read differently from exactly-on.

2. **`StoredDefinite` is already per-sign in effect** (sure).
   - Each `SizedPass` refuses at most one nonzero sign. `Positive` and `NonNegative` refuse Negative. `NonZero` refuses only zero, which can never be past the band.
   - Zero always goes to the no-size class, which never reads `StoredDefinite`.
   - So "one value per decision" equals "one value for the one refused nonzero sign".
   - `DIHEDRAL_ARM` is already correct. A zero arm (cone point, short edge) ends in the lever plus the note. A definitely negative arm (a stored radius below zero) ends as a defect at rest.
   - The brief's hypothesis would break this. At a cone point the radius is identically zero, so the tier can prove it zero (likely). Under that hypothesis an ordinary cone point would be called a damaged file.

3. **Reachability** (likely).
   - Proved zeros occur under the parameter-box drive, where the symbolic tier is default-on (`SymbolicDials::default`). About 65% of drive samples are proved zeros.
   - Almost all of those are on decisions that *pass* at zero: residuals and coincidences such as `carrier_matches_mapped_source`, `carrier_on_surface_*` and the endpoint checks (K-REPORT, M10-7 addendum).
   - On a sized decision that *refuses* at zero:
     - `DIHEDRAL_ARM`: a proved zero is plausible at a cone point.
     - `ParamSpan`: a proved zero needs a span no construction mints, so it is reachable only through a defect.

### What the issue actually points at: `ParamSpan`

`CertifyError::IntervalNotForward`'s doc says a span m ≤ 0 "has no size to tighten below, and no kernel construction mints one … at a build it is a kernel defect, over stored geometry a defect of the kernel or the file". Every no-size or refused reading of this decision means exactly that, at every door. Today it is told three ways:

- **No-size class, at a build or at rest** (exact, proved, or in-band reversed): "Recourse: move the geometry so this edge is not vanishingly short; … this is a kernel defect [or a damaged file], worth reporting". The lever contradicts its own note.
- **Refused class at a build:** the same lever plus the note.
- **Refused class at rest:** `KERNEL_OR_FILE_DEFECT_ENDING`.

The per-decision meaning ("this refused side is unminted") lives in prose, so the type cannot keep the three in step. This is the only such decision. `DIHEDRAL_ARM` and `Meter::NormalFloor` have *mixed* no-size notes ("an edge of no length **or** … a radius below zero is a defect"; "**if** it has none …"), and for them the lever plus the note is the honest single story.

### Final states

**A. No change** (class split and `StoredDefinite` as they are).
- Makes true: one zero story per decision, whichever tier decided it.
- Leaves possible: `ParamSpan`'s lever-plus-defect-note ending, and its two-way at-rest split by class.
- Reversible: fully.

**B. A, plus one concept, "unminted"** — leaning this way (likely).
- A decision may declare its refused side one that no construction mints. Its no-size and refused classes then end in the door's own defect ending: `KERNEL_DEFECT_ENDING` at a build, `KERNEL_OR_FILE_DEFECT_ENDING` at rest. Its in-band size offer and its unresolved and floor arms keep the lever.
- It lives as the third value of the one choice `SizedWords::refused` already carries, read by `Shows::NoSize` as well as `Shows::Refused`. geom-core still spells the endings, so no caller can inject text.
- `StoredDefinite` gains `Unminted` beside `Contradiction` and `Lever`. Since `Unminted` is no longer at-rest-only, `RefusedSide` is a better name.
- Makes true: every `ParamSpan` reading of m ≤ 0 (exact, proved, in-band reversed, past the band) ends one way at each door. That is D4 ¶1 (i) and (iv) applied together. The lever its note denies is gone, and the build door matches the error's own doc.
- Import door within ε_in: unchanged. The door's sentence and the lever win there, as they should for a sliver edge in a file.
- Cost: one more variant, used by one decision today.
- Reversible: yes.

**C. The brief's hypothesis: a proved zero reads `StoredDefinite`, or `StoredDefinite` becomes a per-sign table.** Against (likely):
- it forks proved zeros from numeric ones, against Ev's own ratified D4 ¶1 (i);
- it misreports a cone point on `DIHEDRAL_ARM`;
- it encodes a sign split the classes already make.

**D. Split `ParamSpan` into a sized check plus a "not forward" invariant (`Unsized::Defect`).** Against (sure): choosing between them would have certify branching on the reporting margin's sign outside the counted door (`MarginDiag` is reporting-only; D4 ¶3).

### Ratified text

- **D4 ¶1 (iv) rider** ("A surface reading a body at rest may end a definite contradiction … as a defect").
  - Provenance: agent-written in `2341414335` (PR 3352). Ev's comments there address only the tolerance-lever question, and the rider was approved in passing ("this sounds good!").
  - Recommend clarifying: "definite" means decided past the band on a nonzero sign the decision refuses. A zero, even a proved one, tells its in-band story per (i).
  - Under B, also say that a decision whose refused side no construction mints ends as a defect at every door, not only at rest.
  - This changes ratified text, so it goes to you.
- **D4 ¶1 (i)'s one-message-for-exactly-on-and-in-band** is your own wording from #129, per PR 3352's provenance section. I have not re-verified that here. It is the reason to reject C.

### Confidence

- A over C: likely.
- B over A: likely.
- "A proved zero is never past the band": sure.
- "Each pass set refuses at most one nonzero sign": sure.
- "A proved zero is reachable at a cone point on `DIHEDRAL_ARM`": likely.

## For the orchestrator

**Brief errors.**
- (a) "Within one decision meaning differs by sign while `StoredDefinite` is one value per decision" is not a mismatch. Zero never reads `StoredDefinite`, and there is at most one refused nonzero sign. `DIHEDRAL_ARM` ends correctly today.
- (b) The `ParamSpan` example is real, but proof is not what causes it. An exact numeric zero, or an in-band reversed span, ends in the same lever-plus-defect-note.

**Not checked.**
- Whether any production at-rest reader actually runs a refusing sized decision at `Sym<_>`. The likely path is `validate_geometric` as the shell verb's last act, under the drive's default-on symbolic replay; the validator is generic over the scalar.
- Off-question: that path would read a just-built body "at rest" and name "a damaged file" during a build. Worth an issue if confirmed (`crates/topo/src/shell.rs`, the `validate_geometric` call in the shell verb).
- Whether a cone point's radius is actually proved zero under the shipped rules.

**Possible second defect of `ParamSpan`'s shape.** At a build, `DIHEDRAL_ARM`'s and `ParamWinding`'s refused class ends in the lever plus the note even where no construction mints the reading (a negative radius). PR 5023 and 5101's "Ruled" sections chose lever plus note at a build for every decision. Under B, a decision with `Unminted` changes that at the build door too. Flag it to the implementer as intended, not drift.

**Implementing B.**
- It touches geom-core's `RefusedReading`, which only `Shows::ending` reads. `NoSize` would read it for the unminted value only.
- The build-door defect text means geom-core must know the door. Pass the door, never the text.
- Update the fourth list in `scripts/gates/reporting-margin-door.sh`.
- Re-baseline:
  - `recourse::tests::one_definite_reading_ends_one_way_whichever_arm_routes_it` (its `ParamSpan` rows);
  - `predicate::tests::only_a_refused_reading_reads_what_the_caller_knows_of_it` (its title becomes false);
  - certify's at-rest/import-door parity tests.

**Leave alone.** Fork-log row 9's ruling: `Reading` stays in geom-brep, and there is no nested `ending(Reading)`. B adds neither. Nothing here touches the D10 hold.

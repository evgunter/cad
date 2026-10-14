# Designer B, round 1 (proven-zero), as delivered 2026-10-14

## For Ev

**Recommendation (likely).** Two small changes, and `StoredDefinite` stays one value per decision.

1. **A zero the symbolic tier proved classes as `Shows::Refused`, not `Shows::NoSize`** (`MarginDiag::shows`, `crates/geom-core/src/predicate.rs`). `Refused` is defined as "a decided sign the decision cannot use"; a proved zero on a decision that refuses zero is exactly that, decided by proof instead of by the band. Everything downstream then follows with no new mechanism:
   - At a build it ends in the lever plus the note (unchanged).
   - At rest a `Lever` decision ends the same (unchanged).
   - At rest a `Contradiction` decision ends in `KERNEL_OR_FILE_DEFECT_ENDING`.
   - The import door within ε_in already ends `NoSize` and `Refused` alike (its sentence plus the lever, no note), so it is unchanged.

   The payload word "a zero the symbolic tier proved, which no tolerance makes a size" moves to a `proven()` guard on the `Refused` arm of `Indeterminate::reading`. An exact f64 zero (`Value(0.0)`) stays `NoSize`: it is a rounding, and D4 ¶1 (i) is right that it tells the in-band story.

2. **`DIHEDRAL_ARM` becomes `StoredDefinite::Lever`, and its note drops "a radius below zero is a defect"** (`crates/geom-brep/src/dihedral.rs`). The brief's sign split inside this decision is real in the text but not in the tree:
   - Its Zero reading (a cone's point) is sound geometry the lever reaches.
   - Its Negative reading needs a stored radius below zero, which nothing mints: a profile refuses a nonpositive circle radius before any sweep (`profile/src/path.rs`), and the at-rest battery refuses it in its check 1, before edge re-certification, as `UnrepresentableSurfaceDatum` (`topo/src/validate.rs`).
   - The folded arm is `min(edge extent, radii)`. The extent is at least the chord and sits behind `ParamSpan`, so a zero extent cannot reach the arm either.
   - The note was reworded to describe the negative arm on 2026-10-14 (`dda54034fb`, agent text, not ratified) for a test row that mints a negative arm by rounding. That row can keep its Negative and expect the lever.

   **`StoredDefinite` is one value per decision because what a definite refused reading means is one thing per decision once the unreachable arm is not written down.**

**Premise check.** The brief is right that a proved zero is definite and that `Shows` is the place where that is lost. It is wrong that the answer needs a per-sign concept: the one decision whose definite readings differ by sign differs only on a reading nothing produces. "Definite" today is spelt by placement (`Past`), when it means "decided on a refused side", whether by band or by proof. Change 1 says that.

**Reachability (sure).**
- The only production minter of a proved zero is `Sym<T>::decide_sign` (`geom-core/src/sym.rs`).
- The only production symbolic sessions are editor-core's leaf replay at `Sym<Interval>` (`drive.rs`) and the coincidence door (D10 hold). The kernel's own builds and the checks window run at `f64`.
- In that replay, a proved zero at `ParamSpan`, `Transversality` or `TangentSecondOrder` is a stored description the document's own parameters contradict identically, which is the (iv) case. At `DIHEDRAL_ARM` it is a cone's apex on an edge sample.
- So today's ending is wrong only on a path the hold covers, and the fix is in geom-core and one table row, not on the held surfaces.

**Ratified text.** D4 ¶1 (iv)'s at-rest rider was written in PR 3352 (`2341414335`, agent text in an `[ev]` PR). Ev's comments there were on the ε split, so it was approved in passing. No change is needed: a proved zero is "a definite contradiction between a stored description and its stored geometry". If anything is added, it is one clause in `Shows::Refused`'s doc, not in DESIGN.md: *decided on a refused side, past the band or by proof*.

**Options weighed as final states.**
- **A (recommended): changes 1 + 2.**
  - True after: every definite refused reading ends by (class, `StoredDefinite`), with "definite" meaning decided; no decision carries a note about an unreachable arm.
  - Leaves possible: nothing new.
  - Reversible: one match arm and one table row.
  - Consequences:
    - `predicate::tests` rows for `(Positive, v(5e-10).as_proven())` move from `NoSize` to `Refused`.
    - `dihedral::tests::a_gates_rejection_ends_by_its_reading_on_either_arm`'s negative row expects the lever plus the note at every door.
    - `recourse::tests::one_definite_reading_ends_one_way_whichever_arm_routes_it` gains a proved-zero route per `Contradiction` decision.
- **B: nothing changes.** Defensible under a reading of (i) that a proof is "exactly-on". Against:
  - The ending then offers "move the geometry so this edge is not vanishingly short" for a span proved zero in the document's own parameters, which no move reaches.
  - `Shows::NoSize`'s doc has to keep saying a proved zero "shows no size", while `Refused`'s says the same reading "ends as it does, save where it contradicts stored data". Two classes for one meaning.
- **C: `StoredDefinite` per sign** (`stored_zero`, `stored_negative`). Against (sure): the only decision that would use it does so for an arm nothing mints. It adds a representable state no decision needs and two fields to keep in step.

**Worked example.**
- A revolve whose stored arc has `t1 − t0 ≡ 0` in the document's parameters, read at rest in the replay.
  - Today: "Recourse: move the geometry so this edge is not vanishingly short; an edge of no length … is a kernel defect or a damaged file, worth reporting".
  - After A: "There is no way through: this is a kernel defect or a damaged file; report it", which is what its reversed sibling already says.
- A cone's apex on an edge, arm proved zero, at rest. Today and after A: the arm's lever plus "an edge of no length or a cone's point leaves no angle to measure".

**Confidence.** Recommendation: likely. Proved-zero reachability: sure. Negative-arm unreachability: sure at a build, likely at rest (check 1 reports first; I did not confirm it short-circuits check 2).

## For the orchestrator

- **Assumed:** "definite" in the brief means "decided on a refused side", matching `Shows::Refused`'s own doc. If it means "past the band" literally, option B is the answer and nothing moves.
- **Not checked (no build):** whether `tier3_local_checks` skips edge re-certification on a face whose datum failed check 1, or pushes both rows. Either way the representability row names the negative radius first; the claim in For Ev is phrased for both.
- **Off-question, worth a filed issue on ENCL:**
  - `topo::validate::classify_certify`'s at-rest reason for `IntervalNotForward { Zero }` is "its length is within the tolerance of zero". That is true of a band-decided zero and weak for a proved one. With change 1, the reason could read the margin's `proven()` the way `Indeterminate::reading` does.
  - `CertCheck::ParamSpan`'s `no_size` note and its `Contradiction` say the same thing twice. After change 1 the note is read only on band-decided readings, so its "or a reversed one" clause becomes dead text and can go.
- **Gate and pins:** the gate `scripts/gates/reporting-margin-door.sh` counts `ContradictsStoredData` spellings; change 1 adds none. The `Shows` test table in `predicate.rs` (`(Positive, v(5e-10).as_proven(), Shows::NoSize)`) is the one pin that moves in geom-core.
- **D10 hold respected:** nothing is proposed on the replay, the coincidence door or records. The proved-zero path runs through them, but the change is in `shows` and one table row.
- **Files read for the argument:**
  - `crates/geom-core/src/predicate.rs`: `Shows`, `shows`, `SizedWords`, `RefusedReading`, `Indeterminate::reading`
  - `crates/geom-core/src/sym.rs`: `decide_sign`, the one `as_proven` mint
  - `crates/geom-brep/src/recourse.rs`: `StoredDefinite`, `SizedDecision::recourse`/`words`
  - `crates/geom-brep/src/certify.rs`: `CertCheck::ending`, the span gate, `edge_extent`, the arm mint
  - `crates/geom-brep/src/dihedral.rs`: `DIHEDRAL_ARM`, `wedge_decided`
  - `crates/geom-brep/src/implicit.rs`: `curvature_lever_arm`
  - `crates/topo/src/validate.rs`: check order, `classify_certify`
  - `crates/editor-core/src/drive.rs`: the `Sym<Interval>` replay
  - `work/encl/a-lever-decisions-definite-arm-at-rest-drops-its-note.md`
  - the PR 3352 and PR 5101 bodies

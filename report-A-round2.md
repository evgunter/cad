# Designer A, round 2 (proven-zero), as delivered 2026-10-14

Shown: B's round-1 report.

## For Ev (summary of the revised recommendation)

**Recommendation (likely).** At rest, a reading that shows no size ends one way, whether a proof or a number decided it. That takes three changes and no new concept.

1. **At rest, `Contradiction` covers the whole no-size class.** For a `StoredDefinite::Contradiction` decision, every reading that shows no size at rest ends in `KERNEL_OR_FILE_DEFECT_ENDING`:
   - a proved zero,
   - an exact numeric zero,
   - an in-band reading wholly on the refused side,
   - a past-band refused reading.

   Readings that do show a size, and the offer, keep the lever. Builds are unchanged.
2. **`DIHEDRAL_ARM` becomes `StoredDefinite::Lever`**, and its note drops "a radius below zero is a defect".
3. **Clarify D4 ¶1 (iv):** "definite" means the reading certainly shows no size on a refused side, whether a band, a number or a proof decided it.

**Premise check.**
- The cause is not proof. An exact numeric zero and an in-band reversed span end in the same lever-plus-note today. A fix keyed on proof forks exactly-on readings by tier, which D4 ¶1 (i) forbids.
- No per-sign `StoredDefinite` is needed. Each pass set refuses at most one nonzero sign. `DIHEDRAL_ARM`'s negative reading needs a datum that profiles refuse and that at-rest representability reports first. Its real meaning is "lever".

**Why the whole no-size class (likely).** Every Contradiction decision passed decided at this run's tolerance when the edge was certified. At rest, a reading that shows no size cannot come from lane width alone, since a wider enclosure reads as unresolved. So it contradicts the stored certificate however it was decided. `ParamSpan`'s own error doc says this, and today its lever contradicts its own note.

**Final states.**
- **A (recommended): changes 1–3.** One at-rest story per decision for "shows no size". Proof never changes a message. A cone point keeps its lever. The import door within ε_in is unchanged. Reversible: one match arm and one table row.
- **B: reclass only a proved zero, plus DIHEDRAL→Lever.** Against (likely): it leaves exact and in-band zero spans self-contradictory, and forks exactly-zero by tier, against (i).
- **C: nothing changes.** Against: `ParamSpan`'s at-rest ending stays self-contradictory.
- **D: per-sign `StoredDefinite`, or "unminted".** Against: under A, only `ParamSpan` at a build remains, which is reachable only through a defect and whose note already says so.

**Worked example.** A span reading 0 at rest, whether proved, computed, or −5e-9 in band.
- Today: "Recourse: move the geometry so this edge is not vanishingly short; an edge of no length … kernel defect or a damaged file".
- After A: "There is no way through: this is a kernel defect or a damaged file; report it".
- A cone point arm proved zero at rest keeps the lever plus its note.

**Ratified text.** The D4 ¶1 (iv) at-rest rider is agent text (`2341414335`, PR 3352), approved in passing. Change 3 states its principle and does not alter what it decides.

**Confidence.** A over B: likely. "No-size at rest contradicts the stored certificate": likely. "A negative arm needs a datum already refused": likely.

## For the orchestrator

- **What moved me.** B's `DIHEDRAL_ARM` reachability evidence, which I checked: the `validate.rs` datum block says a refused datum is "the one reported first", though checks 3–5 may still escalate, so it does not short-circuit them. The negative-radius clause is agent text from `dda54034fb`, written for a test row. Also B's point that a proved zero on Transversality or ParamSpan is a stored contradiction, which I extended to the whole no-size class. That drops my "unminted" concept.
- **What did not move me.** "Definite = decided by band or proof" while an exact f64 zero stays no-size, which forks exactly-on readings by tier. And B's claim that `ParamSpan`'s note becomes dead: the note is still read on in-band reversed and exact-zero readings.
- **Implementation notes.**
  - `predicate::tests::only_a_refused_reading_reads_what_the_caller_knows_of_it` inverts.
  - PR 5101's ruled "an in-band reading at rest keeps lever + note" is an orchestrator ruling, and changes only for no-size in-band readings.
  - The `reporting-margin-door.sh` counts are unaffected.
  - The `RefusedReading` docs need rewording to "a reading that shows no size".
  - `ParamSpan`'s note will be read only at a build.
- **Not checked.**
  - Whether adoption assigns `Transversality` to tangent-continuous STEP edges. If it does, such an edge at rest past ε_in reads as a defect under A.
  - Whether any at-rest reader runs at the symbolic lane outside the drive replay.
- **Off-question.** `classify_certify`'s at-rest reason for `IntervalNotForward { Zero }` should be re-read under A.

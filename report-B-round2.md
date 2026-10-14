# Designer B, round 2 (proven-zero), as delivered 2026-10-14

Shown: A's round-1 report.

**What moved me:** all three of A's points.
1. The D4 ¶1 (i) objection holds against my change 1. A proved zero is never past the band, because `Sym::decide_sign` proves only when the numeric answer was Zero or undecided. Classing it as Refused forks it from an exact f64 zero by tier, against (i), which is Ev's text from #129.
2. The `ParamSpan` diagnosis is right: an exact zero and an in-band reversed reading have the same lever-versus-note contradiction, so the proof is incidental and change 1 treated a symptom.
3. With change 1 dropped, `DIHEDRAL_ARM` is correct today.

**What did not move me:** the negative arm of `DIHEDRAL_ARM` is still unreachable, so the clause "a radius below zero is a defect" describes nothing that is produced. That is text cleanup, not design, and I withdraw it from the recommendation.

## For Ev (summary)

**Recommendation (likely): A's round-1 option B.**
- The class split and `StoredDefinite` stay as they are. A proved zero keeps ending as its exact and in-band siblings do.
- Let a decision declare that its refused side is *unminted*: no kernel construction produces it. `CertCheck::ParamSpan` would be that decision. Its no-size and refused classes then end in the door's own defect ending: `KERNEL_DEFECT_ENDING` at a build, `KERNEL_OR_FILE_DEFECT_ENDING` at rest.
- The size offer, floor, unresolved and straddle arms keep the lever.

**How it is carried:**
- It rides as a third value of `SizedWords::refused`, which `Shows::NoSize` reads for that value only.
- geom-core spells every ending, and the caller names the door's outcome.
- Fork-log row 9 holds.
- `StoredDefinite` gains the value and is better renamed after the refused side.

**Makes true:**
- Every `ParamSpan` reading with m ≤ 0 ends one way at each door.
- The lever its note denies is gone.
- The build door matches the error's own doc.

**Unchanged:** the import door within ε_in.

**Reversible:** one variant and one row.

**Cost:** the lever-plus-note-at-a-build rule from PRs 5023 and 5101 changes for this one decision, on purpose.

**Rejected:**
- the brief's hypothesis;
- my round-1 change 1;
- splitting `ParamSpan` into a sized check plus an invariant;
- doing nothing (defensible, but it leaves a lever beside a note that says there is none).

**Ratified text:** D4 ¶1 (iv)'s at-rest rider was agent-written (`2341414335`, PR 3352) and approved in passing. Recommend clarifying it so that "definite" means decided past the band on a refused nonzero sign, and a zero, proved or not, tells its in-band story per (i). Under B, add that an unminted refused side ends as a defect at every door. D4 ¶1 (i) is unchanged.

**Worked example:** a stored arc with t1 − t0 ≡ 0, proved zero at rest. Today it ends in the lever plus the note. After B it reads "There is no way through: this is a kernel defect or a damaged file; report it", the same as its reversed sibling and an exact f64 zero. A cone point keeps its lever.

**Confidence:**
- B over nothing: likely.
- Rejecting the brief and change 1: likely ((i) is sure).
- A proved zero is never past the band: sure.
- The negative arm is unreachable: sure at a build, likely at rest.

## For the orchestrator

- **Concordance:** I now agree with A's round-1 A-over-C and B-over-A, and withdraw both of my round-1 changes. `DIHEDRAL_ARM`'s "radius below zero" clause (`dda54034fb`) is text only.
- **Open residue:** `CertCheck::Transversality` and `TangentSecondOrder` (Contradiction, no note) end a zero verdict at rest in the lever alone (`RefusedArm::Zero` → `NoSize` → `lever_recourse(lever, None)`). PR 5101's body says `NotTransverse` at rest "was already the defect ending", but the code does not show that. Whether a stored tangency is a contradiction at rest and a lever at a build is a per-door shape that B's single value does not express. It is worth its own issue; don't fold it into B.
- **Unchecked:** whether `validate_geometric` runs under the drive's default-on symbolic replay as a build's last act.
- **`classify_certify`'s at-rest reason** for `IntervalNotForward { Zero }` should follow B and become the mismatch reason.
- **Implementing B:**
  - geom-core learns the door, as a variant and never as text;
  - update the gate's fourth list;
  - re-baseline the `recourse` and `predicate` tests and certify's parity tests.

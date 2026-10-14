# Round 3 (proven-zero): both designers converged, 2026-10-14

Each designer was shown the other's round-2 report and asked four things: what moved each of them, the property question beneath (per decision), the Transversality residue, and DIHEDRAL_ARM.

## Both recommend: one three-valued kind per sized decision

The kind replaces `StoredDefinite { Contradiction, Lever }`. Both the no-size class (`Shows::NoSize`: an exact zero, a proved zero, or an in-band reading wholly on the refused side) and the past-band refused class (`Shows::Refused`) read it.

**The three values:**
- `Lever`: the lever plus the note at every door.
- Contradiction at rest (A calls it `ContradictsAtRest`, B `StoredContradiction`): the lever plus the note at a build; `KERNEL_OR_FILE_DEFECT_ENDING` at rest (the validator, and the import door past ε_in).
- `Unminted`: `KERNEL_DEFECT_ENDING` at a build; `KERNEL_OR_FILE_DEFECT_ENDING` at rest.

**Unchanged by the kind:**
- Readings that may show a size (offer, floor, unresolved, straddle) keep the lever.
- The import door within ε_in is unchanged.
- A proved zero ends as its exact and in-band siblings do (D4 ¶1 (i)).

**Rows:**

| Decision | Kind |
|---|---|
| ParamSpan | Unminted (both, sure) |
| Transversality | Contradiction at rest (both) |
| TangentSecondOrder | Contradiction at rest (both) |
| DIHEDRAL_ARM | Lever (both); the note drops "a radius below zero is a defect" |
| ParamWinding | A: Contradiction at rest. B: Unminted, likely, resting on `profile/src/path.rs` `ArcSweepNotShortOfFullTurn`, with not every sweep traced; the implementer to confirm or leave it Contradiction at rest. A reachability fact, not a design split. |

**Ratified text.** D4 ¶1 (iv)'s at-rest rider (`2341414335`, PR 3352) is agent-written and was approved in passing. Both designers say the clarification is Ev's call:
- "definite" means certainly on a refused side, whether a band, a number or a proof decided it;
- a refused side that no construction mints ends as a defect at a build too.

## Points both checked

- **PR 5101's claim was wrong.** It said `NotTransverse` at rest "was already the defect ending". In fact a Transversality or TangentSecondOrder zero verdict at rest ends in the lever alone (`RefusedArm::Zero` → `NoSize` → `lever_recourse(lever, None)`), while `classify_certify`'s reason says "tangent where its description says they cross".
- **STEP adoption** assigns `TangentIntersection` separately from `Intersection` (`step-import/src/adopt.rs`), so tangent STEP edges are not misread.
- **A withdrew one round-2 rationale:** "certified once" proves too much. The extension rests on (i) instead: no fork by distance from zero on the certain side.

## Unchecked by both

Whether an at-rest reader runs at the symbolic lane outside the drive replay. Whether `validate_geometric` at a shell verb's end reads a just-built body as at rest.

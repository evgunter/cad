# ASM-4 — split and inline (binding spec; R1's closing unit)

Binds ASSEMBLY-DESIGN A4 (split/inline as first-class recorded
refactorings; acceptance at structural + name-resolution identity,
Evan's ruling), A10 (roots move with the cut), A11 (cluster
placements move with the cut). Pre-logged **L / structural**.
The codebase's first multi-document OPERATION (ASM-1's store is
read-side; this unit needs the write side). Deviations reported,
never absorbed.

## D-1: the write side of the workspace (minimal)

`Workspace` gains exactly what split needs: create a new document
file from a `Doc` (id freshly minted via `derive` from a caller
label or `random_document_id` at the authoring layer), and re-save
an existing document by id. Duplicate-id refusal unchanged. No
general mutation API — split/inline are the only writers.

## D-2: `split` — cut a subtree out into a new document

Signature at the document layer (`pncad`): given a Doc, a set of
node ids closed under the cut rule, and a new-document label →
(remainder Doc, new part Doc, the recorded edits on both sides).
Semantics per A4:
- The cut set must be a union of whole placement clusters and
  ancestor-closed within the recipe DAG (a cut that would sever a
  consuming edge refuses typed naming the edge). v1 scope: NO mate
  nodes exist yet, so "every mate edge crossing the cut becomes
  the interface record" is vacuous — state this and leave the
  interface-record hook named for R2 (the seam is the crossing
  declarations; the type goes in now as an empty record so R2
  extends rather than retrofits).
- The new document receives the cut nodes (ids remapped
  deterministically in document order), their roots (A10 list
  order preserved), their placements (A11), and params/witnesses
  they reference (copied; refuse typed if a cut node references
  an uncut param — no silent sharing).
- The remainder receives an `InstantiatePart` node per cut-out
  cluster, pinned to the NEW document's content pin, placed at the
  cluster's old frame; roots and placements update by the A10/A11
  maintenance rules through the recorded edits.
- Both sides' changes are ordinary recorded `DocEdit`s (a compound
  refactoring edit is acceptable if atomic undo demands it —
  implementer's choice, reported).

## D-3: `inline` — the inverse

Given a Doc and one `InstantiatePart` node: resolve the pin,
splice the referenced document's recipe into the host (ids
remapped, names re-anchored from `InPart`-wrapped to local — the
inverse of the bridge), place via the instance's cluster frame
composed onto the part's placements, delete the instance node.
Refuses typed when the referenced product is not what the recipe
can express locally (none known in v1 — state if found).

## D-4: acceptance — A4's ruling, executable

**split-then-evaluate ≡ unsplit evaluation at structural +
name-resolution identity**: same product topology and geometry
semantically (censuses equal, volumes bit-equal), and every stable
name that resolved before resolves after — to the corresponding
entity (the split side reaches it through the instance
qualifier). Arena-key/bit identity NOT required (D9's per-arena
convergence precedent) — taken if free, asserted only as the
weaker semantic identity. `inline(split(d)) ≡ d` at the same
identity level. Round-trip through persistence on both sides.

## Acceptance rows

1. Split a two-cluster assembly's one cluster out → remainder
   instantiates the new doc; the A4 identity holds (census,
   volumes, all names re-resolve; probe several).
2. Inline it back → identity with the original (same level);
   undo of BOTH refactorings restores exactly.
3. Refusals: severing cut (names the edge); uncut-param reference;
   duplicate-id at create; inline of a stale pin = PinMismatch.
4. Roots/placements: maintenance rules produce the expected lists
   on both sides (each its own assertion).
5. Interface-record hook: the empty crossing-declaration record
   exists on the remainder and round-trips (R2 extends it).
6. D9: split twice in fresh processes → byte-identical pair of
   documents (modulo the minted id, which the caller supplies
   deterministically in tests).
7. Cold clippy: CI scope + pncad-py python feature + interval
   graph. k-lint fires → report, never silence.

## Standing brief lines

As ASM-2B-SPEC's, verbatim (OUTPUT DISCIPLINE; foreground rows;
poll harness-backgrounded output files; kill by recorded PID only;
local-scripts/ tooling; merge-before-open; confirm checks start;
invariant comments; commit+push per unit).

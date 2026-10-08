# FORK-S4P — what a carrier's provenance is read from, and what the coincidence record is (designer A)

## For Ev

**Recommendation** (likely). There are two parts.

1. **Provenance is the document's.** The door reads where a recorded cell came from: the operand read it entered the deciding operation through, then its name in that operand's table, then the document's definitions back to the operation that minted it. That chain is all the door needs to build the cell's canonical form. `GeomSource` goes completely, with nothing left in the kernel: the stamps, `SourceExpr::Placed`, the `Recipe` arm of `GeomOrigin`, `AxisSource`, the bit witnesses and `wire.rs`'s stamping and `compose_placed`. N6 is retired and replaced; it does not survive as a rung of the door.
2. **Two records, one citing the other.** A **coincidence** records a *decision*: the operation decided from values that two cells are one, and the lint proves or reports it. A **contact** (`ContactRecords`) records a *touch*: cells of the result touch, and the result's census reads the record to excuse the touch. Every contact cites the coincidence it came from. Only coincidences leave the kernel into `NodeValue`. The lint reads only coincidences, and the census reads only contacts.

**Premise check.** The question treats `GeomSource` and `StableName` as two competing sources of provenance. Neither is provenance on its own terms:
- A `StableName` alone is not enough. A pass-through operation adds no name segment (N1), so an extrude's top cap and the same cap after a `Transform` have the same name. Comparing names alone would "prove" a part flush with its own moved copy, which is false. Placement lives in the document's chain of reads, not in the name.
- `GeomSource` is that chain of reads copied into the kernel. The document layer stamps it (`stamp_minted`, `compose_placed`), and every kernel operation then carries it through by hand: the row carry, `revert`, the merge, `transform`. That is one fact represented twice and kept in step by hand. Its only content is "same minting node, same mint index, same placement chain". Canonical-form equality (unit C) proves everything that proves, because one construction read twice gives equal forms by definition.

So the real question is: does the kernel keep a copy of recipe structure that no kernel decision reads? My answer is no.

### Part 1: the final state (P1, recommended)

- **The row's cells.** A row names each cell as `NamedCell { read, name }`. `read` is the deciding node's operand read, or the node itself for a cell born in the operation (a split's section edge). For an at-rest row, `read` is the copy's world placement.
- **The door's walk.** It starts at `read`, steps through the read's definition, and composes each placing operation into the frame as it passes. It strips the segments that do not change the carrier (`FromA`/`FromB`/`FromMember`, `SplitFragment`, `Fragment`, and a `Merged` reduces to the kept constituent). It ends at the minting node and role, and that role's `CarrierFlow` gives the form.
- **Rungs.** Unit B's rung 1 is this same walk with the leaf compared by identity (same minting node and role, same placement chain). Unit C swaps the leaf for the canonical form, after which `Rung::SameSource` has nothing left to prove and goes. The final rungs are `CanonicalForm` and `PolynomialIdentity`.
- **Imported and opaque carriers.** An imported face is named by its Import node and role, and its flow is `Opaque`, so two imported faces never prove. That matches D1's import paragraph: their contacts are findings, quieted by an assertion.
- **The guard against drift.** C's witness checks every stated form against the built carrier at the f64 environment, across the whole corpus in CI. That replaces N6's bit witness as the check that the document's account matches the kernel's geometry. It covers the "kept description" rule for merged faces too.

What P1 makes true, and the others cannot:
- **The layering is strict.** The kernel decides margins and records arena cells. It holds no recipe-derived data, so N6's "recipe vocabulary lives in editor-core, topo only compares it" becomes "topo holds none of it".
- **One source of truth.** The document's chain of reads is the only record of how a cell was built.
- **Stage 3 needs no stamp migration.** When placement becomes a bundle of mates, only the walk's placement step changes.

### Part 1: the alternatives

- **P2: keep `GeomSource` as the door's lowest rung.** Unit B ships exactly as specified, and the rung is sound. But two representations of provenance stay, kept in step by hand, and they can disagree: the stamp's `Placed{node, instance}` has to be redefined when stage 3 changes what a placement is. Every kernel operation keeps carrying data none of them reads. Once C lands, the rung proves nothing C does not. Easy to undo later, but it is dead weight.
- **P3: read provenance from the name alone.** Rejected as unsound (the `Transform` example above).
- **P4: lower canonical forms into the kernel.** Rejected. It puts `VarId`-shaped data into topo and is still a second copy.

### Part 2: the final state (R1, recommended)

```text
topo::Coincidence  { cells, relation, site, margin }   // a decision; operand or op-born cells
topo::ContactRecords entries gain  decided_by: RowIx    // a touch in the result; cites its decision
NodeValue.coincidences: Arc<[NamedCoincidence]>         // the only one that leaves the kernel
```

What R1 makes true:
- **Each type says one thing.** A decision is about the *operands*: a flush union's two caps are glued and then vanish from the result, the mitre and a profile junction are not contacts at all, and a merge leaves a continuation, not a touch. A contact is about the *result*: its cells exist in the result arena, and the census needs its granularity there (C3's witness edge and patch overlap).
- **D1 (ii) becomes a type invariant.** D1 (ii) says result touching arises only from decided coincidences; with R1, a contact with no deciding row cannot be constructed.
- **Rows never need re-keying across operations.** Once a row is named at its operation, nothing re-keys it again. Contacts keep their descendant-map carry inside the kernel, as they do today.

The alternatives:
- **R2: one type.** Every row would need optional result cells, because a glued cell has no image in the result. It would also need room for the census's certification artefacts. That is a type that means two things.
- **R3: widen `ContactRecords` to hold decisions.** It cannot hold the mitre, a profile junction, or a decision whose cells were merged away. This is what spec §6's "ContactRecords gains the Zero-glued pairs" reads as.

### Worked example

`E` extrudes a box of height `h`. `T` translates `E` by `h` along the extrude direction. `U` is the union of `E` and `T`, which glues E's top cap to T's bottom cap.
- **The row.** `U` records one row: `SameOpposite`, with cells `(read A, E/Cap(Top))` and `(read B, E/Cap(Bottom))`.
- **The walk.** Cell A reaches `plane(f, d, h)`. Cell B passes through `T`, so its form is `plane(f, d, 0)` shifted by `h` along the normal. The plane's symmetry folds that shift into the offset, giving `plane(f, d, h)`. The two forms are equal, so the row is **proven**. This holds once unit H lands; before it, C keeps `T`'s placement as an opaque atom, and the row is unproven.
- **Contacts.** `U` has no contact record, because the caps are interior and removed.
- **What the name alone gives.** If `T` translated by `h' = 10 mm` while `h = 10 mm`, the names are the same as before, but the row is unproven with residual `h − h'`. Name-only provenance would have proven it, wrongly.

### Ratified text that changes (quoted)

- **N6** (names README, ratified #74) is retired whole. Its theorem "same `GeomSource` ⇒ bit-identical descriptions; the converse is not claimed, so equal bits without a shared source stay unglued. The declared coincidence rung is this lookup" is replaced by:
  > N6 — A cell's construction is read from the document. A recorded cell is named by the read it entered the deciding operation through and its name there; the door follows that read through the document's definitions to the minting operation and role. The kernel carries no recipe provenance.
- **N3 and "A union's face is named for its PARENT"** read N6 too: "merges only structural or declared-coincident faces, which share a recipe source" and "share a recipe source (N6)". Both become "faces its margins decide a continuation, each recorded".
- **D1 (ii).** "the result carries machine-checkable contact records (…), each checked by the `unproven-coincidence` lint" becomes "…, each citing the coincidence the op decided and recorded, which the lint checks".
- **C3.** `PatchContact`'s "carrier identity by the structural or declared rung" becomes "carrier identity decided Zero by its margin, cited from its recorded coincidence". C3's granularities themselves stand.

All of these fall under D10's retirement list or are wording that follows from it. N6's replacement states a principle, not a rewording, so it is the one I would put to you.

### Does unit B rework? Little

- **Unchanged.** The record, the emission sites and the `NodeValue` plumbing all stand.
- **Two changes.**
  - The named cell carries its operand read explicitly. The spec implies this ("the operand's table").
  - Rung 1 is written as the document walk, not as a read of the body's stamps. If B is already written over stamps, only that rung's body changes. C would have discarded it either way.
- **Later units.** `decided_by` can land with B, or with E when E adds the Zero-glued contacts. `GeomSource` is deleted once nothing reads it: kernel decisions stop reading it in E, and the door stops in C. That pulls the deletion of the `Placed` arm forward from H to C.

**Confidence.**
- "C's forms subsume same-source": likely. The only gap I can see is an `Opaque` flow shared across roles, which is closed by keying `Opaque` by carrier role.
- "Name alone is unsound": sure.
- "No kernel decision reads `GeomSource` after E": sure, from the spec's E and the readers in the tree.
- R1 over R2: likely.

## For the orchestrator

- **Not traced.** I could not trace who wrote N6, D1 (ii) or C3, because the checkout is shallow and `-S` lands on merge grafts. I took the brief's attributions: N6 ratified #74, and D1 (ii)'s lint clause from D10's integration.
- **Gap in the spec's docs list.** Spec §6 lists N6 but misses N3 and the union-parent paragraph of the names README. Both cite N6 and declared coincidence. `emit_union.rs:2389`'s comment says the same.
- **`NodeValue.contacts`** carries only the mate-minted records ("Empty for every op but instantiate"). After unit I it has no producer and should go; a boolean's contacts ride its payload.
- **An assumption to confirm.** For a decision made in a mid-operation body (the merge stage), I assumed the emitter can map its cells to operand cells through the internal grafts, which spec §1 also assumes. If some site cannot, the row names the operation's own role.
- **Not checked.** Whether every face role maps to exactly one carrier, so that keying `Opaque` by role is sound. Sweeps build one wall per carrier run, which suggests yes. A revolve's two half-bands on one carrier would give equal forms, not an `Opaque` clash.

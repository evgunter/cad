# FORK-1 — the output signature of an operation (designer A)

## Round 2 — For Ev

Both designers agree on the shape: fixed, named, typed ports per variant; `Profile` and one multi-body kind (`Instances` / `Bodies`, one name to pick) as new kinds; a split as two `Body` ports with DM3's split arm retired and its index pick kept; `Transform` taking its operand's kind; assertions, mates and gauges defining nothing; D10's "one or more" reworded. I adopt the other report's grouping of the kinds, which is better than mine: **value kinds** (free or defined: the scalars, the discrete kinds, `Point` … `Frame`), **product kinds** (defined only by an operation: `Body`, `Bodies`, `Profile`) and **selections of a product** (`Face`, `Edge`). Two differences remain.

**1. The revolve's axis: a kind, or a check on the definition.** The other report types `Datum::AxisInPlane` as an `Axis` and keeps "written in the profile's frame" as a check that reads the axis variable's *definition*. Its strongest point is sharing: an in-plane axis is also a world line, and D10's coaxiality ("one `Axis` variable read twice") and a circular pattern about the revolve's axis need it to be readable as one. That moved me on the slot, not on the kind:

- **Revised (likely): `AxisInPlane` stays a kind, and every `Axis` slot admits it**, reading it as the world line through its frame. A slot already states the set of kinds it admits (`Body | Bodies` at a placer), so this is the same mechanism, one-way: a revolve admits only `AxisInPlane`, an axis slot admits both. No lift operation, nothing to declare, and sharing works as the other report wants.
- Why not the definition check: D10's operations read values; a reader that looks through a variable to how it was defined forks on provenance, not type, which is D3's silent-dispatch trap one level up. The failure it leaves representable is concrete: once stage 3 gives `Axis` a free arm, or a mate defines one, a well-kinded `Axis` sits in a revolve's slot and refuses only at evaluation, and FORK-4's re-pointing would have to re-run a definition walk at every write. With the kind, "in the profile's frame" is read off the value (the frame variable it carries), equal ids and no band, the shape the code has today.

**2. An instance of a part: one folded `Body`, or the product mirrored.** The other report types `InstantiatePart` as one `Body`, as the evaluator folds it today, so the insert door needs no part resolver. Its strongest point is locality: a signature that depends on another document means a document's own variable table cannot be checked without that document in hand. What does not move me is the fold itself:

- The fold hides a multiplicity in a value. An instance's `parts` count is value-dependent today (a pattern in the part's product makes it the count's value), a boolean fed an instance refuses `ProductOperand` only at evaluation, and no reader can take one body of a two-body part at all: an in-context cut through one instantiated bracket, the assembly feature D10's "an assembly is a recipe of the same formalism" invites, is unwritable. A `Bodies`-typed instance would allow it but makes the common one-body part pay an index pick at every read.
- **Revised (likely): mirror the product, with the signature recorded on the node.** `InstantiatePart` carries its output signature as node data beside `interface.crossings`, written when the `DocRef` is minted (the caller holds the part then: the pin is a content hash of it) and verified against the resolved part on every evaluation, exactly as the crossings are. The mint and the load walk read the record; no door needs the resolver; a re-pin whose product shape differs refuses at evaluation as a crossing that no longer resolves does. That answers the locality point in the code's own existing shape. A one-body part is one `Body` port, so today's bare-id spelling still works for the common case, and `AmbiguousOutput` fires on a multi-body part as on a split.

**On the FORK-3 coupling the other report raises** (if a selection becomes set-valued, `Bodies` and `Faces` should be one collection kind over an element kind): agreed, provided the collection is ordered and indexed, since a pattern's order is data (N1's instance index) and a fillet's selection is canonically sorted today, so it can be.

Everything else in my round-1 text stands. Confidence: the shared shape, sure; `AxisInPlane` as a kind admitted by every `Axis` slot, likely; the instance mirroring its product with a recorded signature, likely; the folded `Body`, rejected, sure.

## Round 2 — For the orchestrator

- The two reports differ only on the two points above; neither needs a third round unless you want one on naming (`Instances` vs `Bodies`).
- If mirroring is taken, the spec's PR A gains: `InstantiatePart { signature: Vec<(Role, VarKind)> }` as recorded node data (wire and preimage), the façade's instantiate door filling it from the part it pins, and an evaluation check `SignatureMismatch` beside `CrossingUnverified`. PR C's product then lists `Body | Bodies`, and an instance's `Bodies` port is the part's pattern root.
- `written_against` (`eval/wire.rs`) compares node ids; with the kind it compares the `Frame` variables the profile and the axis read, as the other report also says. Test 3 of the spec should cover a world `Axis` at a revolve refusing `SlotVarKind` at the door.


## Round 1 — For Ev

**Recommendation (likely).** An operation's outputs are a **signature**: an ordered list of (role, kind), fixed when the node is inserted, each entry minting one variable of that kind. The signature is a function of the node's variant and, for two variants, of the kinds it reads. D10's kinds list gains `Profile`, `Instances` and `AxisInPlane`; the datum kinds are the ones already listed; a split's halves are two ports, so DM3's projection node narrows to instances. In full:

| operation | reads | defines |
|---|---|---|
| `Datum::Plane` / `Axis` / `Point` / `Frame` | scalars | one `Plane` / `Axis` / `Point` / `Frame` |
| `Datum::AxisInPlane` | a `Frame`, four scalars | one `AxisInPlane` |
| `Datum::FaceFrame` | a `Face` (stage E), a spin | one `Frame` |
| `Profile` | a `Frame`, its step scalars | one `Profile` |
| `Extrude`, `Revolve`, `Tube`, `HollowTube`, `Loft`, `Sweep` | profiles, an axis, scalars | one `Body` |
| `Fillet`, `Chamfer`, `Shell`, `Boolean`, `Union`, `PlacedUnion` | bodies, selections, scalars | one `Body` |
| `Split` | a `Body`, a `Plane` | two `Body`: roles `above`, `below` |
| `Pattern` | a `Body` or `Instances`, a `Count`, a rule | one `Instances` |
| `Transform` | a `Body` or `Instances` | one of the kind it read |
| `Instance` (today `Part::Instance`) | an `Instances`, a `Count` | one `Body` |
| `InstantiatePart` | nothing (a leaf) | the pinned part's product list, kind for kind |
| `Measure` | selections | one scalar |
| `Assertion`, `Mate`, `Gauge` | — | nothing in stage 2 |

Definitions:

- **`Profile`**: a planar region, loops of pieces drawn on a `Frame`, with each piece's step identity. Constructed, never selected; read by the sweeps and the loft list.
- **`Instances`**: an ordered family of bodies indexed by a `Count`, whose length is a variable's value. The family is one variable because the length is not static, so there cannot be a port per body. A pattern's output; a nested pattern's placement-major order is its order.
- **`AxisInPlane`**: a line in a sketch frame, authored in that frame's 2-D coordinates and carrying which `Frame` variable it lives in. Distinct from `Axis` (a world line) so a revolve's slot refuses a world axis at the door, and "in the profile's plane" is "the same `Frame` variable", an identity with no band, the shape D10 already uses for coaxiality.
- **Role**: the name of a port within a signature (`above`, `below`), authored as `split.above`; stored as the port index `SplitHalf::output_body` already fixes.
- An empty half or an empty boolean result is a value of kind `Body` (the typed absence today), not a kind; a reader that needs material refuses it as now.

**Premise check.** Of the four cases in the brief, three dissolve once "defines one or more" is read as a per-variant signature; the instances are the one genuinely new kind.

- **Datums** are not a gap. D10 lists `Point`, `Axis`, `Plane`, `Frame`; a datum defines the kind it constructs. Stage 3 changes how a *free* `Frame` is spelled, not its kind, so stage 2 types the slots now. The only kind missing is the in-plane axis.
- **A split** is "one or more" exactly: two ports. The `Part::SplitHalf` row was a workaround for one value per node.
- **The spec's static `outputs() -> &'static [(port, VarKind)]`** is wrong for two variants. A `Transform`'s output kind is its operand's (`Body → Body`, `Instances → Instances`, as the evaluator already says), and an instance's outputs are the pinned document's product. The signature is `outputs(&self, kinds of its reads) -> Vec<(Role, VarKind)>`, computed once at insert; the minted kinds are then fixed (VR3). `Transform` retires in stage 3, so its polymorphism is short-lived; the instance's is permanent and right: an instance is a copy of a product, so it defines what the product lists.
- **A fifth case the brief omits** and the answer must cover: an instantiated part's product is today one `Body` value with a `parts` count, refused at a body seat only at evaluation (`ProductOperand`). That is "several bodies" spelled a second way beside `Instances`. Mirroring the product list makes the bad state unrepresentable: a boolean reads one port; `parts` and `ProductOperand` go.

**Ratified text that changes.**

- **REFERENCES DM3** (built PR 1860; agent-ruled, no comment from you on its PR): "a part of a multi-body value is selected by a projection node" narrows to instances. A split's halves are read as `split.above` and `split.below`; the remaining node is an operation, `Instance { of: Instances, index: Count }`, keeping DM3's verbatim name pass-through. The viewer loses the split-half row DM3 counted as its cost.
- **D10's Operations paragraph**: "defines one or more" becomes "defines the variables its signature states, possibly none" (an assertion defines nothing), and the kinds list gains a fifth group, the constructions' own values: `Profile`, `Instances`, `AxisInPlane`. The paragraph is agent-written text in PR 3990; you ratified the model at the level of "nodes read and define variables", and the kinds list was not a decision you took separately.
- **Slot admission**: three slots admit two kinds, `Pattern.input`, `Transform.input` and the product list's entries (`Body | Instances`; a pattern in the product expands in order, as today). A slot states the set of kinds it admits and the door checks membership. Every other slot admits one kind.

**Alternatives weighed.**

1. *One list kind for every multi-body value, split included* (a split defines one `Instances` of two). Rejected: the halves are a fixed, role-named pair; a list loses the roles and makes a reader index a constant. Ports are the general form the sentence already provides.
2. *Keep `Part` for the split and give `Split` one `Split`-kind output.* Rejected: a kind with one reader whose only job is to take it apart is a tuple pretending to be a value, and the projection row is ceremony.
3. *Fold the in-plane axis into the revolve as four scalar slots in the profile's own frame*, the build-less option: in-plane by construction, no kind, no node. I lean against it: a revolve axis is shared (two revolves, a circular pattern about it, stage 6's coaxiality through one axis variable), and a slot cannot be read twice. Reversible either way: the kind can be added later without moving a body. If you want less, this is the one place I would take it.
4. *A pattern's instances as one `Body` of several lumps*, the instantiate path's spelling. Rejected: a pattern's copies may overlap, which is not a solid, and lumps would need the fuse's disjointness certificate the pattern deliberately does not claim.
5. *An instance defines one `Instances`* (no resolver needed at insert). Rejected on final state: the product's length is data the node pins (`DocRef` with its identity stamp), so the signature is static per node, and a `Count` index into a fixed list is a weaker type than a port. The cost, stated apart from the ranking: the insert and load doors must see the referenced document to mint; evaluation already requires it.

**Consequences.** Typed at the door: a boolean fed a profile, a split, a pattern or a whole multi-body instance refuses `SlotVarKind` before the document changes. `WrongOperand`, `ProductOperand` and the viewer's by-variant `denotes_body` (which today admits a transform of a pattern that then fails at evaluation) retire into one kind check. The authored sugar "a node id means port 0" refuses `AmbiguousOutput` on a split and on an instance of a multi-body part. A `Sweep`'s `path` stays a `Profile` read (its first loop's chain); that is a pun, a path being a curve and not a region, and it is not widened here.

**Reversibility.** Every kind added is additive. Ports can be collapsed into a list later; the reverse rewrites readers. The two-kind slots can be narrowed if stage 3 gives `Instances` a placer-free form.

Confidence: a signature per node fixed at insert, sure; `Instances` as a kind, sure; the split as two ports and DM3 narrowed, likely; `AxisInPlane` as a kind rather than folded in, likely; an instance mirroring its product, likely.

## Round 1 — For the orchestrator

- **Provenance.** The checkout is a graft at 2026-10-03, so `git log -S` on D10's sentences hits the graft; I used PR 3990's body and comments instead. Ev's verbatim transcripts (commits `5f7a1c71e3`, `3d70e5de72`, `fae23dbc71`, `c4158a079c`) are not in this checkout, so I could not check whether Ev said anything about multi-output nodes or patterns. PR 1860 (DM3) has no comments.
- **Spec amendments if this is taken.** §1's static `outputs()` and §2's "a node's signature is fixed by its variant" need the two exceptions; Q5's `AmbiguousOutput` also covers an instance of a multi-body part; PR A needs the `PartResolver` at the insert and load doors. A re-pin of an instance's `doc_ref` that changes the product's shape must refuse or strand readers; that belongs to FORK-4's ruling on re-pointing. If the resolver at the insert door is judged infeasible for A, alternative 5 is the fallback for instances only, with a row filed to retype at stage 3; the PR should say which was taken.
- **Off the question.** `denotes_body` in `crates/viewer/src/combine.rs` lists `Transform` as body-denoting unconditionally, so the viewer's seat admits a transform of a pattern that refuses at evaluation. B retires the function, so no row is filed.
- **Names** (`Instances`, `Instance`, `AxisInPlane`, roles `above` / `below`) are yours to settle; the split's port index must be `SplitHalf::output_body`, whose doc already says nothing else may restate the mapping.

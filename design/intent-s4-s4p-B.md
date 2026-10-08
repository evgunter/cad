# FORK-S4P — a cell's provenance is its selection; a contact record cites its decision

## For Ev

**Recommendation** (`likely`, both parts as one design):

1. **A carrier's provenance is read from the document, not from a stamp.** A recorded
   cell is a *selection* in D10's own sense: the `Body` port it was read from plus its
   `StableName`. The door derives the construction from that alone: the port's read is
   walked back through the pass-through placements (Transform, instance, pattern) to the
   name's minting node, composing their frames; the minting node's verb states the role's
   carrier (`CarrierFlow(verb, role)`) over that node's slot variables. The kernel carries
   **no provenance of a description at all**: `GeomSource`, `SourceExpr`, `GeomOrigin::
   Recipe`/`Cleared`, `stamp_minted`, `compose_placed`, `AxisSource`/`AxisRecord` and
   `ParamSource` retire (after unit E nothing in the kernel reads any of them). N6 is
   rewritten, not amended.
2. **Two typed things, one reference.** A `Coincidence` is a *decision*: what an op decided
   Zero, between which of its inputs' cells, at which site, with which margin. One row per
   decision, cells in the inputs' key spaces, never re-keyed by later surgery. A
   `ContactRecords` row stays what C3 says it is, a pair of *result* cells whose interiors
   meet, re-keyed through grafts as today, and **gains `backing: CoincidenceIx`**, the
   decision it is a survivor of. The lint walks decisions; the census walks records; a
   record without a decision cannot be built. D1's own sentence ("a pair of cells … plus
   its backing: a coincidence an op decided Zero, recorded") becomes the type.

**Premise check.** The brief asks "what is a carrier's provenance" as if something must
hold it. The code shows the stamp holds nothing the door can use: `GeomSource::minted(node,
index)` numbers descriptions in arena order (`stamp_minted_from`), and no reader decodes
`index` — it is identity, not construction. The construction already has a home: N1 defines
a `StableName` as "the minting recipe node plus the combinatorial role the entity plays
there; it denotes a construction role, not a point set", and the name table is total over
live entities (N4). So question 1 is not "which of two stamps", it is "stop stamping". On
question 2, `ContactRecords` and the coincidence record answer different questions (which
result cells touch, vs. what was decided from values), in different key spaces (result keys
vs. operand keys), with different cardinalities (one plane decision backs many vertex and
edge survivors; a merge, an `EqualAngles` mitre or an `EqualRadii` decision backs none). One
type would have to flag which rows the census may look for; two unlinked types lose D1's
backing. The link is the design.

**Definitions.** *Port*: one typed output of a node (D10's "named, typed ports").
*Selection*: `(Body port, StableName)`, D10's `Face`/`Edge` variable shape. *Pass-through
placement*: an op that mints no geometry and adds no name segment (N1: Transform, a Part's
projection) or wraps one (`Instance { i, of }`, `InPart { of }`); the walk composes their
frames, as C's opaque pose atom before stage 3 and as the composed `Frame` after H. *Slot
subject*: a coincidence between a cell and the op's own input pose (the split's pinch is a
vertex ON the plane the node reads); the spec's `CellPair` cannot say this,
`Subject::{Operand(which, cell), Born(cell), Slot(role)}` can. *Decision site*: the closed
enum of places a Zero verdict glues, merges or makes pieces touch.

**What this makes true that the alternatives cannot.**
- *Structure is decided in one place from one source of truth.* The DAG and the name table
  are already a function of (recipe, structural parameters, verdicts) (N4's CI invariant);
  the door reads only them. A stamp would be a second encoding of the same path, kept in
  step by `stamp_minted`/`compose_placed` at ten call sites.
- *The kernel owns no document vocabulary.* `ParamSource` smuggled lowered slot identity into
  `Body` as opaque bytes "this crate compares and never reads"; after E it compares nothing.
  The kernel body becomes geometry + topology + D5 birth provenance, and the layering note
  N6 states today is satisfied by having nothing to lower.
- *An unrecorded value coincidence is unrepresentable.* Every record names its backing, so a
  site that glues and emits survivors without a row fails to compile, which is D10's
  "every coincidence the kernel infers from values … is recorded" as a type rather than a
  review item.
- *Rows are stable.* A decision's cells are the operands' cells, which the operands' tables
  resolve directly (the boolean emitter already chases grafted keys back to B's keys for
  `FromB`). Only `Born` subjects follow the op's own descendant map, as names do. The
  spec's "re-keyed through every graft" applies to records, not rows.
- *The finding, the assertion and the recourse share a currency*: selections, and the
  minting node a selection already identifies.

**Worked example.** Block A extruded `depth = h`; block B extruded from a sketch on A's top
cap, placed by Transform T. Union glues B's bottom cap to A's top cap on a Zero margin. Row:
`{ cells: (A.port, Cap(Top) @ nodeA), (T.port, Cap(Bottom) @ nodeB); SameOpposite;
PlaneLadder; margin }`. Door: A's cell → `plane(frameA, dirA, h)`; B's cell → walk T (one
frame) to nodeB → `plane(frameB, dirB, 0)` composed with T; frameB is the sketch's plane,
which stage 2 B makes a read of A's cap, so both reduce to the same `PoseForm` modulo
`Planar`: `Structural(CanonicalForm)`. The union's `ContactRecords` has the ON-set survivors
(A's rim vertices on B's cap, …) each with `backing = 0`. The census checks the survivors
against the body; the lint checks row 0; neither re-derives the other.

**What it leaves possible that should not be** (`likely`): a `CarrierFlow` statement can
drift from the verb's code and prove a falsehood; C's corpus witness is the guard, and the
retired N6 theorem lives on only as that witness's obligation. **Reversibility**: high; a
stamp is one side table re-added, `backing` one index per record.

**Alternatives.** *Keep `GeomSource` as identity and table forms by it*: `(node, index)` is
not a role, so the table is built from the name table anyway — two identities kept in step
for one cell. *Stamp the canonical form on the body* (as `ParamSource` does for radii):
document variables in the kernel as bytes it cannot read, and stale by construction, since
a form changes under an edit that moves no geometry. *One type*: the three mismatches
above, and a merged pair becomes a record of cells that no longer exist, which the census
refuses as stale unless flagged. *Two unlinked types*: D1 (ii)'s "each checked by the lint"
becomes a convention; a glue that emits survivors without a row still compiles.

**Ratified text this changes** (quoted; each is a redesign of what D10 replaced):
- **N6** (`names/README.md`, #74), whole clause from "Every surface, curve and point
  description carries `GeomSource { node, expr, orient }`" to "Identity holds per
  evaluation against the current document only" → *"A carrier's construction is read at
  the document's door from the cell's selection: its port is walked back through the
  pass-through placements to the name's minting node, and that node's verb states the
  role's carrier over its slot variables (`CarrierFlow`). The kernel carries no provenance
  of a description. That identical construction yields identical bits (D9) is the
  `CarrierFlow` witness's obligation, not a gluing rung."* The code-map row "N6 `GeomSource`"
  goes. N1's pass-through sentence stands and is what the walk relies on.
- **topo README preamble** (#178, #965): "a **record** (`ContactRecords`: …) is the
  verified form a result body carries" → *"a **decision** (`Coincidence`) is what an op
  decided Zero between its inputs' cells, with its site and margin, proven at the
  document's door; a **record** (`ContactRecords`: …) is a pair of result cells whose
  interiors meet, citing the decision that backs it."* The ladder sentence as unit E has it.
- **C3**: "carrier identity by the structural or declared rung" → *"backed by a
  `SameOpposite` decision"*; add *"every granularity cites its backing decision."*
- **D1 (ii)**: "each checked by the `unproven-coincidence` lint" → *"each citing the
  coincidence (D10) that backs it, which the `unproven-coincidence` lint checks"*.
- **DESIGN standing outcome**: "same source ⇒ same bits by D9, converse deliberately
  unclaimed" → strike; no source exists.
- **D10 Coincidence paragraph**: unchanged. Add after "a split's ON verdict": *"between a
  cell and the construction's own input pose as between two cells"* (`unsure` whether Ev
  wants the slot subject stated in D10 or only in the record's type).

**Unit B rework** (plainly): yes, contained. `Rung::SameSource` reading `surface_source`
becomes "same name, same placement chain", computed by the port walk B must write anyway (C
reuses it for the opaque atom). `CellPair` becomes `Subject` pairs in operand space with
`Slot`; the kernel `Coincidences` remap shrinks to `Born` cells; each `ContactRecords` row
gains `backing`. Nothing in `NodeValue`, the lint or the surfaces changes. The kernel-side
deletion of `GeomSource` stays in E, where the spec already deletes the kernel's rung 1.

**Confidence.** Recommendation `likely`. Load-bearing: the minted index is role-less and
undecoded (`sure`, grep); a `StableName` plus port determines the construction up to
placement (`sure`, N1/N4); pass-through placements are recoverable by walking the port's
reads (`likely`: Transform adds no segment while `Instance`/`InPart` wrap — both are
walkable, but the asymmetry is a smell outside this fork); nothing in the kernel reads
`GeomSource`, `AxisSource` or `ParamSource` for a decision after E (`likely`, from the spec's
E list plus the grep of readers: `plane_eq`, `carrier_eq`, `merge_faces`, `reduce` F7,
`chart_region`'s diagnostics, `field_source_evidence`).

## For the orchestrator

- **Contamination, declared.** A grep for `FORK-S4` over the spec returned §14's option lists
  and recommendations for S4-1 and S4-3 (lines 571–594) before I could avoid them. I formed
  the view from the code first and the above differs from §14 on S4-3 (not one type; two
  linked) and sharpens S4-1 (the cell is a selection, provenance is a DAG walk, and
  `ParamSource`/`AxisSource` retire with `GeomSource`). Weigh accordingly.
- **Unverified.** No unit-B branch exists on origin (only `intent/stage4-spec`), so I
  designed against the spec text. The clone is shallow, so `git log -S` for N6/C3/D1 returned
  grafts; provenance is from the companion table (#74; #178, #965).
- **Spec gaps off the question**, for B/E rather than Ev: (1) `CellPair` lacks a cell-vs-own-
  input-pose arm (the split pinch); (2) rows are not graft-remapped, only `Born` subjects
  follow the op's descendant map; (3) a vertex classified ON a glued face is a survivor of
  the face decision, not its own row — spell which at each emission site or the lint reports
  N rows per unproven plane; (4) after E `ParamSource` and `AxisSource` have no kernel reader
  and should retire with `GeomSource`, `GeomOrigin` collapsing to `Imported | KernelDirect`;
  (5) STEP import mints `ContactRecords` and will need a backing (`DecisionSite::Adopted`) or
  none — F's or D7's, not this fork's; (6) Transform adds no name segment while
  `Instance`/`InPart` wrap, two spellings of "placed" that stage 3's `Frame` should make one.

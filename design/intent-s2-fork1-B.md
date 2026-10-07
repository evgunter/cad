# FORK-1 — the output signature of an operation (designer B)

## For Ev

**Recommendation** (likely). An operation's outputs are a **fixed list of
named, typed ports**, set by its variant. Kinds are split by what can
make them:
- **value kinds** may be free or defined: the scalars, the discrete
  kinds, and `Point` … `Frame`;
- **product kinds** are only ever defined by an operation: `Body`, and
  two new ones, `Profile` and `Bodies`.

`Face` and `Edge` stay what D10 says: selections of a product.

The four cases:

| Today | Typed as |
|---|---|
| a profile | one `Profile` output (new kind) |
| a datum | one output of D10's own geometric kind: `Plane`, `Axis` (`AxisInPlane` too), `Point`, `Frame` (`FaceFrame` too). No new kind. Stage 3 adds the free arm |
| a pattern's `Instances` | one `Bodies` output (new kind: an ordered list of bodies whose length is a `Count`) |
| a split's two halves | **two `Body` ports**, `above` and `below`. DM3's `SplitHalf` arm retires |

The full signature table:

| Operation | Outputs |
|---|---|
| `Extrude`, `Revolve`, the tubes, `Loft`, `Sweep`, the blends, `Shell`, `Boolean`, `Union`, `PlacedUnion`, `InstantiatePart` | `Body` |
| `Split` | `above: Body`, `below: Body` |
| `Pattern` (reads a `Body` or a `Bodies`) | `Bodies` |
| `Transform` | the kind it reads: `Body` → `Body`, `Bodies` → `Bodies` |
| picking one body out of a `Bodies` (DM3's `Instance` arm), by a `Count` index | `Body` |
| `Measure` | one `Length` or `Angle`, set by its primitive |
| `Assertion`, `Mate`, `Gauge` | nothing |

- **A `Body` may be empty** (a boolean's ∅, F8; a split's empty side).
  Emptiness depends on values, so it cannot be a kind. A seat that needs
  material refuses at evaluation, as it does today.
- **The product** (stage 2's PR C) lists `Body` and `Bodies` variables.
  A pattern of bolts is listed whole, as the gather takes `Instances`
  today.

### Premise check

1. **The datum case is not a question.** D10 already names
   `Point`/`Axis`/`Plane`/`Frame`. Stage 2 adds them as kinds that only
   an operation defines, and stage 3 adds their free arm. No kind is
   added and none moves. (sure)
2. **"Defines one or more" is false of three nodes.** `Assertion` and
   `Mate` define nothing, and neither does `Gauge` until stage 3 retires
   it. D10's own text treats an assertion and a mate as something other
   than an operation. The sentence should say "an operation defines one
   or more; an assertion or a mate defines none". (sure)
3. **The brief misses a fifth case.** `Transform`'s value takes its
   input's shape: one body gives a body, `Instances` give `Instances`.
   No static per-variant signature can type it. Its output kind is fixed
   when the node is inserted, from what it reads; kinds are fixed at
   minting (VR3). Re-pointing it at a different kind refuses at the door
   (`SlotVarKind`) rather than changing the kind of a variable that has
   readers. Stage 3 retires `Transform`-as-placement, so this exception
   is temporary. (likely)
4. **What the list is missing.** D10's list mixes two things: kinds a
   person can type, and kinds only construction produces. Its
   "references" group holds `Body`, but a body is not a reference to
   anything: it is what an operation makes. A profile and a pattern's
   bodies are in the same position. Naming that group ("product kinds:
   no free arm, no unit, no distribution, never an analysis axis") gives
   the new kinds an obvious place, and it is the rule stage 2's spec
   already states for reference kinds. (likely)

### Ratified text this changes

- **D10, Variables**: the list gains `Profile` and `Bodies`. "The
  references (`Face`, `Edge`, `Body`)" becomes "the products (`Body`,
  `Bodies`, `Profile`), defined only by an operation, and the selections
  of a product (`Face`, `Edge`)". D10, Operations: "one or more" is
  reworded as in premise 2.
- **REFERENCES DM3**: the `SplitHalf` arm retires. A split half is read
  through a port. The `Instance` arm stays as the one way to take one
  body from a `Bodies`. Its stated reason for being a node was that an
  operand struct would fork every consumer's operand door. Once every
  operand is a read, the port lives in the read (`Output { node, port }`),
  and that reason no longer applies to fixed-arity outputs.

### The answers, as final states

**A (recommended): fixed named ports; `Profile` and `Bodies` kinds.**
- What it makes true:
  - every output's existence follows from the edit sequence alone (D9),
    never from a value;
  - a split half is an ordinary read, so a bare split at a body seat
    cannot be written, where today it is refused;
  - a body seat cannot be given a profile, a datum or a pattern: the
    door refuses it by kind, where today `WrongOperand` refuses it at
    evaluation.
- What stays possible: an index past a pattern's count, which is a
  value, so it refuses at evaluation as DM3 says today.
- Worked example: six bolts patterned, then subtracted from a plate.
  - The pattern defines one `Bodies`. The product lists it whole.
  - `Boolean { b: <the pattern's output> }` refuses by kind. The person
    unions the bolts (a `Union` over `Bodies` is a separate decision,
    not taken here) or picks one by index.
  - Change the count from 6 to 4: no variable appears or disappears.
    A pick at index 5 refuses at evaluation, typed.
- Reversible: kinds and ports are enum arms plus a persisted port index.

**B: one port per pattern instance**, so a pattern of N defines N `Body`
variables.
- The variable table would depend on a value. Editing the count mints or
  deletes variables and strands their readers, and a parameter sweep
  over the count changes the document's set of identities. This breaks
  D9 and VR1 (an id is a function of the edit sequence).
- Rejected. (sure)

**C: a split defines one value of a new `SplitPair` kind, projected by
DM3's `Part` node.**
- This is a fixed list of ports written as a type. It keeps a tree row
  whose only job is projection, and a refusal that ports make
  unrepresentable.
- Rejected. (likely)

**D: a profile as a sheet `Body`, or as a `Face`.**
- A profile carries step identity, its frame and each edge's authored
  radius, none of which a body or a face has.
- `Face` is a selection of a body by name (D10), which a profile is not.
- Every body seat would have to refuse sheets by value.
- Rejected. (likely)

**E: a pattern as one multi-lump `Body`.**
- Its copies may overlap, so the result is not a valid body.
- D3 says a pattern does not fuse. The fused form already exists:
  `PlacedUnion`.
- Rejected. (sure)

### Coherence with the other forks

- **FORK-3.** "One body of a `Bodies` by index" has the same shape as
  "one face of a `Body` by name", so the two belong in one home:
  - if a selection is a node, the pick stays a node (DM3's `Part`, now
    with only its `Instance` arm);
  - if a selection is a variable definition, the pick becomes one too.

  If FORK-3 makes a selection set-valued, `Bodies` and the selection set
  should be one collection kind over an element kind, not two separate
  ones. (likely)
- **FORK-2 and PR C.** The product admits `Bodies`, so
  `ProductFault::NotABody` becomes "not a body or bodies". (sure)
- **The `RecipeNodeId` shorthand.** A node id written where an operand
  is expected means the node's port 0. `Split` is the only operation
  with two ports, so the spec's `AmbiguousOutput` refusal fires there
  and nowhere else. (sure)

Confidence:
- on the recommendation: likely;
- on rejecting B and E: sure;
- on `Profile` being a kind of its own rather than a body: likely.

## For the orchestrator

- **Signatures.** The spec's `Node::outputs() -> &'static [(port, VarKind)]`
  cannot type `Transform`. It needs either an output kind of "the kind of
  slot `input`", resolved at insert, or a `&self` signature. Ports should
  be named in the table (`above`/`below`), with the index as the wire and
  preimage form.
- **Revolve's same-frame rule** (`wire.rs` `written_against`) compares
  node ids today. After B it compares the `Frame` variable the profile
  reads with the one the `AxisInPlane` reads. The axis slot is kind
  `Axis`, and in-plane-ness stays a check on the axis's definition.
  Nothing new is needed, but test 3 should cover a world `Axis` given to
  a revolve.
- **Sweep's `path` is a `Profile`** ("a profile whose first loop's chain
  is the trajectory"). It is typed `Profile` here. Whether a path is a
  kind of its own is off-question, and worth a row if anyone wants one.
- **`MeasureUnavailable`** (a measure with no value at this scalar) gives
  a scalar output a "no value" state. PR D has to give that state a home
  in the variable's value domain. It is not FORK-1's.
- **`InstantiatePart`** folds a multi-body part product into one `Body`
  at the seam (`wire_instantiate_part`, `place(&part.body, …)`), so it
  is typed `Body`. I did not check whether a part product listing a
  `Bodies` still folds the same way. PR C should confirm it.
- **Provenance.**
  - DM3 is "Built: DOCM-2 (PR 1860)". I could not trace who wrote it:
    the history is shallow, and `-S` finds only merges.
  - D10 is treated as ratified per the brief. Its "one or more" also
    reaches only merge commits here.
- **Assumption.** `PlacedUnion` takes one body, not a `Bodies` (the
  `ValuePayload::Instances` doc says the prototype "takes ONE body"),
  which matches today's operand door.

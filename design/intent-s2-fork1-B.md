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

| Operation | Outputs |
|---|---|
| `Profile` | one `Profile` (new kind) |
| `Datum` | one of D10's own kinds: `Plane`, `Axis` (`AxisInPlane` too), `Point`, `Frame` (`FaceFrame` too). No new kind; stage 3 adds the free arm |
| `Split` | **two `Body` ports**, `above` and `below`. DM3's `SplitHalf` arm retires |
| `Pattern` (reads a `Body` or a `Bodies`) | one `Bodies` (new kind: an ordered list of bodies whose length is a `Count`) |
| picking one body of a `Bodies` by a `Count` index (DM3's `Instance` arm) | `Body` |
| `Transform` | the kind it reads: `Body` → `Body`, `Bodies` → `Bodies` |
| `Extrude`, `Revolve`, the tubes, `Loft`, `Sweep`, the blends, `Shell`, `Boolean`, `Union`, `PlacedUnion`, `InstantiatePart` | `Body` |
| `Measure` | one `Length` or `Angle`, set by its primitive |
| `Assertion`, `Mate`, `Gauge` | nothing |

- **A `Body` may be empty** (a boolean's ∅, F8; a split's empty side).
  Emptiness depends on values, so it cannot be a kind. A seat that needs
  material refuses at evaluation, as it does today.
- **The product** (PR C) lists `Body` and `Bodies` variables; a
  pattern is listed whole, as the gather takes `Instances` today.

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
3. **A fifth case the brief misses.** `Transform`'s value takes its
   input's shape, so no per-variant signature types it. Its output kind
   is fixed at insert from what it reads (kinds are fixed at minting,
   VR3); re-pointing it at another kind refuses (`SlotVarKind`). Stage 3
   retires `Transform`-as-placement, so this is temporary. (likely)
4. **What the list is missing.** It mixes kinds a person can type with
   kinds only construction makes. A `Body` is not a reference to
   anything; it is what an operation makes, and so are a profile and a
   pattern's bodies. Naming that group (no free arm, no unit, no
   distribution, never an analysis axis — the spec's rule for reference
   kinds) gives the new kinds their place. (likely)

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
- Still possible: an index past the count (a value), refused at
  evaluation as today.
- Worked example: six bolts patterned (one `Bodies`, listed whole in
  the product). `Boolean { b: <pattern> }` refuses by kind; the person
  picks one by index or unions them (a `Union` over `Bodies` is not
  decided here). Count 6 → 4 mints and deletes nothing; a pick at
  index 5 refuses at evaluation, typed.
- Reversible: kinds and ports are enum arms plus a persisted port index.

**B: one port per pattern instance** (N `Body` variables). The variable
table would depend on a value: a count edit mints or deletes variables,
and a sweep over the count changes the document's identities (breaks D9,
VR1). Rejected. (sure)

**C: a split defines one `SplitPair` value, projected by DM3's `Part`.**
A fixed port list written as a type; it keeps a projection-only tree row
and a refusal that ports make unrepresentable. Rejected. (likely)

**D: a profile as a sheet `Body` or a `Face`.** A profile carries step
identity, its frame and each edge's authored radius; a `Face` is a
selection by name (D10); every body seat would refuse sheets by value.
Rejected. (likely)

**E: a pattern as one multi-lump `Body`.** Copies may overlap; D3 says
a pattern does not fuse, and the fused form is `PlacedUnion`. Rejected.
(sure)

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

Confidence: recommendation likely; rejecting B and E sure; `Profile` as
its own kind rather than a body likely.

## For the orchestrator

- **Signatures.** The spec's `&'static [(port, VarKind)]` cannot type
  `Transform`: it needs an output kind "same as slot `input`", resolved
  at insert. Ports are named (`above`/`below`); the index is the wire and
  preimage form.
- **Revolve's same-frame rule** (`wire.rs` `written_against`) compares
  node ids; after B it compares the `Frame` variables the profile and the
  `AxisInPlane` read. The slot is `Axis`; in-plane-ness stays a check on
  the axis's definition. Test 3 should cover a world `Axis` at a revolve.
- **Sweep's `path` is a `Profile`** (its first loop's chain). Typed so
  here; whether a path is its own kind is off-question.
- **`MeasureUnavailable`** gives a scalar output a "no value" state; PR D
  must give it a home in the value domain. Not FORK-1's.
- **`InstantiatePart`** folds a multi-body part product into one `Body`
  (`wire_instantiate_part`), so it is typed `Body`. Unchecked: that a
  part whose product lists a `Bodies` folds the same way (PR C).
- **Provenance.** DM3 ("Built: DOCM-2, PR 1860") and D10's "one or
  more" trace only to merge commits in this shallow history; D10 taken
  as ratified per the brief.
- **Assumed** `PlacedUnion` takes one body (the `Instances` doc says its
  prototype "takes ONE body").

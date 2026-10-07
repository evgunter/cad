# INTENT stage 2, FORK-3: what a selection is in the document (designer B)

## For Ev

**Recommendation (likely).** A selection is a **variable definition**, not a recipe node:
`VarDef::Select { body, names }`, beside `Free`, `Defined` and `Output`. Its arity is in the
kind: `Face`/`Edge` hold exactly one name, and new set kinds `Faces`/`Edges` hold an ordered,
duplicate-free list. A fillet reads **one** `Edges` variable and loses its `target` field,
because the body is the selection's. A selection authored at two sites is **two variables**,
and passing the variable is how two readers share one, as VR8 rules for scalars. `Rebind`
stays addressed by name and rewrites every selection that lists the name; changing one
selection is the ordinary define edit.

### Premise check
- *The three sub-questions are not independent* (sure). Asked separately, "many variables
  or one set" looks like a count question. What it actually decides is **where the body is
  stated**. Today `Fillet { target, selection: Vec<StableName> }` resolves names against
  `target`. If every edge is its own `Edge` variable, and each carries its own body read (as
  D10 says a selection does), then the body is stated N+1 times: once in `target`, once per
  edge. Nothing ties those copies together, so a fillet of body X that targets body Y can
  be represented, and it has to be refused at the door and again at evaluation. With one
  set-valued variable the body is stated once, and the mismatch cannot be represented.
- *The empty case settles it* (sure). `Shell.open` may be empty: that is the sealed hollow,
  which is legal by the node's own contract. As a list of singleton selections, an empty
  shell has no selection left to carry the body, so `target` must stay as a field, and the
  duplication comes back for every non-empty shell. A set selection with no names still
  carries its body.
- *Node versus definition is mostly settled by what a node is* (likely). In this design a
  node mints geometry and names: `RecipeNodeId` is the D5 naming substrate. It is also
  memoized by content key, because it is expensive. A selection mints nothing. It is a
  table lookup on a body that is already built. That makes it a pure function of a variable
  and stored data, which is exactly what a definition is.

### Ratified text I would change
- **D10, Variables.** "defined, by an `Expr` over other variables or as an output of an
  operation" becomes "…, by an `Expr`, by a selection of a `Body` variable, or as an output
  of an operation". **D10, Operations:** "A `Face` or `Edge` variable (or a set of them) is
  a selection…". Both are completions, not reversals. The D10 sentence was ratified on PR
  #3990. I could not open Ev's transcript commit in this checkout, so I cannot tell whether
  the selection wording is Ev's own or agent text approved in passing.
- **SELECT-DESIGN §4, the one-type rule, restated:** "A GUI selection is the same value as
  the names a selection definition stores: `Vec<StableName>`." The rule survives almost
  word for word under this design. A GUI pick set is committed by writing it into a select
  definition, with kinds filtered at the door and a mixed set refused. The GUI selection
  itself stays heterogeneous and outside the document, as §4 already says.
- **No other ratified text moves.** The materializer doctrine stands: what is stored is a
  name, never a query, and `select_where` returns the `Vec` the caller writes.

### The answers as final states
**A (recommended): a definition, with set kinds.**
- **The variable.** `Var { kind: Edges, def: Select { body: VarId, names: Vec<StableName> } }`.
  `Face`/`Edge` admit exactly one name, checked at the door like any kind check.
- **The ladder.** It runs once per name, inside the select's evaluation, and nowhere else.
  One vanished name fails the select, and every reader refuses with that diagnosis, as a
  fillet does today.
- **Who reads which kind.**

  | Reader | Kind it reads |
  |---|---|
  | `Fillet` / `Chamfer` | `{ edges: Edges, size }` |
  | `Shell` | `{ open: Faces, thickness }` (empty = sealed) |
  | `Measure`, `FaceFrame`, a mate side | one `Face`/`Edge` each |

  The multi-entity readers are exactly the ones that need their body to agree with their
  selection, so they are the ones the set kind serves.
- **Tree.** A select has no row. `Doc::upstream` expands through its definition to the
  body's operation, so a fillet's tree edge goes to the body it blends, as today.
- **Panel.** The selection is shown at its reader: "edges (8) of the body of Extrude
  'base'". A select the user names (say "mounting faces") also appears in the variables
  panel, like any named variable.
- **Count.** A typical part with four fillets of eight edges holds four selection
  variables, not thirty-two.
- **Order.** The list is ordered because `Shell` reads designation order: the rim inherits
  its first face's identity. A blend ignores order.
- **Reversibility.** Easy to extend: a set built from `Edge` variables can be added later as
  one more definition.

**B: a node per selection, a tree row.**
- **Gains.** A select gets memoization and the node machinery for free.
- **Costs.**
  - It mints a `RecipeNodeId`, from the naming substrate, for something that names nothing.
  - It adds tree rows that are noise: one fillet becomes one row plus its picks.
  - It puts something that is not an operation into the operation graph, which D10 reserves
    for things that build.
  - Memoization is wasted on a lookup.
- **Reversibility.** Harder to undo, because node ids reach the pinned hashes.

**C: one `Edge` variable per entity, list slots `Vec<S>`.** This is the spec's lean.
- **Gains.**
  - Each entity is individually addressable.
  - Sharing an edge between a fillet and a measure is free.
- **Costs.**
  - The body is duplicated N+1 times.
  - A mismatch is representable and has to be refused.
  - An empty shell needs `target` back.
  - The variable count scales with picks.
  - SELECT-DESIGN's `Vec<StableName>` stops being any stored value.
- **What it is still good for.** Its one real gain (sharing a single edge across a fillet
  and a measure) is a narrow need. A, extended later, covers it.

### The shared/distinct sub-question (sure)
- **Authored apart means distinct.** A select authored at two sites is two variables. The
  argument is VR1/VR8's: identity is minted, never deduplicated by value. Deduplicating
  would make a definition edit at one site silently move the other.
- **The GUI offers the match.** It offers an existing selection of equal value (same body,
  same names), as D10 does for a typed value. Declining is what makes the two distinct.
- **Distinctness never affects geometry.** Coincidence reads a select as its definition,
  so two equal selections are one construction for D10's structural test.
- **The one edit that crosses sites is `Rebind`.** Its job is to repair a name that
  topology broke, so it rewrites the name in every selection that lists it, plus the
  declared pairs and appearance keys, as today. Changing what one fillet picks is
  `SetSelection { var, names }`, the reference kinds' "define" door from VR7. Picks
  become edits of that door.

### Confidence
- The body-stated-once argument: sure.
- Definition over node: likely.
- The set kinds (`Faces`/`Edges`) over a single kind with a runtime arity check: likely.
- Distinct-by-authoring: sure.
- `Rebind` stays name-addressed: likely.

## For the orchestrator
- **Brief.** It was adequate. The spec's §1 already removes `at` from
  `Datum::FaceFrame` "because the body is the select's". That is the same argument I make,
  and it should be applied to `Fillet.target`, `Chamfer.target` and `Shell.target` too. The
  spec's test 13 ("rebinding mints no selects; the fillet's slot ids are unchanged") holds
  under A. Its sugar (`Vec<StableName>` given to a slot) lowers to one set select instead of
  N singleton selects.
- **Unchecked.**
  - Ev's #3990 transcript (commit `5f7a1c71e3`): this shallow checkout does not hold that commit.
  - Whether a single name can legitimately land on several entities. `Tied` is refused
    today, and `Fragment` paths name the pieces individually. I assumed one name means one
    entity.
- **For the spec.**
  - **Canonical order.** A fillet's selection is stored sorted today, so that its content
  key is stable; a shell's is ordered. Under A the select stores the authored order. The
  fillet's content-key preimage should spell its select's names sorted, so a reorder is not
  a memo miss. Only `Shell` reads the order.
  - **Empty sets.** An empty `Edges` read by a blend refuses at the door, as today ("a blend
    of nothing is an unfinished recipe").
- **Defects off the question.** None found.

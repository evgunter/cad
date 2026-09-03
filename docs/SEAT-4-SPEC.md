# SEAT-4 — the Verb substrate, carried by the blend pair (unit spec)

Executes `docs/VERB-SEAT-DESIGN.md` §2 (V1–V4) for the first two
verbs. This spec elaborates the ratified ledger recommendations
VS-Q1/VS-Q2/VS-Q5 **as recommended** — no deviation from the
ratified doc is taken here, so the spec self-merges per the standing
rule; any implementation-time deviation from THIS spec that touches
a ledger answer is Ev-gated, not disclosed-and-carried.

## The prime directive: substrate in, behavior pinned

SEAT-4 introduces the kernel-side verb declaration and re-plumbs the
blend pair through it. Nothing observable moves:

- **The wire format is untouched.** `Node::Fillet`/`Node::Chamfer`
  keep their serde shapes; no schema bump; a saved document reads
  back byte-identical and re-saves byte-identical.
- **Content keys are untouched.** The memo tags for both node kinds
  stay numerically identical; the memoization behavior of every
  existing document is pinned unchanged.
- **Evaluation results, names, refusals are untouched.** The name
  tables, `BlendRefusal` wrapping, `NodeErrorKind` arms and Python
  tags all read the same before and after.

The unit's whole value is where the NEXT verb's cost lands; §6 of
the design doc says that cost is measured at the next door, not
claimed here.

## S4-1 — the `verbs` crate (VS-Q1 as recommended)

A new small crate `crates/verbs`, above `sweep` and `topo`, below
`editor-core`; `editor-core` gains the dependency, kernel crates
gain none. No serde, no `Expr`, no names, no `RecipeNodeId` — the
§0 lowered line verbatim. If the crate's manifest cost measurably
bites (the GENERICS-BUILD-COST ledger's concern), the fallback is a
module in `sweep` — that is a ledger-answer deviation and therefore
Ev-gated: STOP and report, do not take it silently.

Contents, for the blend pair:

```rust
pub enum Verb<T> {
    Fillet  { edges: Vec<EdgeKey>, radius:   T },
    Chamfer { edges: Vec<EdgeKey>, distance: T },
}
```

with one impl owning:

- `run(&self, operand: &Body<T>, tol: Tol) -> Result<VerbOut<T>, VerbError>`
  — dispatching to `fillet_edges`/`chamfer_edges`. `VerbOut` carries
  the body and the birth record (`BlendNaming`), by value, exactly
  as `Blended<T>` does today (it may BE a thin re-wrap; do not
  restate the record). Operand arity is one body for both of these
  verbs; the operand comes in by reference, never in the payload
  (design V1).
- **The parameter→field flow, as data** (design V1's provenance
  obligation; SEAT-6's substrate): a `fn param_flow(&self) ->
  &'static [ParamFlow]` describing which parameter reaches which
  stored scalar field of which minted-role family — for the fillet,
  `radius` → the band/corner carriers' radius fields; for the
  chamfer, `distance` → no stored field (the setback positions
  planes; say so as an explicit empty-flow row, never by omission).
  `ParamFlow` is plain data (param index + a closed field-role
  enum); NO consumer is wired in this unit — SEAT-6 is the consumer
  — so the acceptance for this piece is that the declaration exists,
  is exhaustive per verb, and is asserted against the birth record
  by one test per verb (every flow row names a role the record
  actually mints).

`VerbError` wraps `BlendRefusal` for these two; a closed enum with
room to grow at later migrations (D3, no wildcard arms anywhere).

## S4-2 — commitments live with their owners (design V2)

- The **content tag** match moves to a small exhaustive
  `fn content_tag(&Verb<…>) -> u64`-shaped match IN `editor-core`
  beside the memo machinery, returning the EXISTING numeric tags for
  Fillet and Chamfer (pin them by test against the pre-change
  constants). The kernel declares nothing about memoization.
- The **wire spelling** stays exactly where it is (`Node`'s serde
  derives); no new match needed this unit because the spelling is
  untouched.
- Python constructors, viewer labels: untouched (no new verb).

## S4-3 — the per-verb correspondence in `editor-core` (design V3)

One module per verb (`editor-core/src/verbs/fillet.rs`,
`…/chamfer.rs`, or one `blend.rs` for the twins if their sharing is
total — implementer's structural call, stated), each declaring the
correspondence as data/functions: which `SlotId` feeds which `Verb`
parameter, which payload selection feeds the key list, which
emitter mints names. `wire_fillet`/`wire_chamfer` collapse onto ONE
generic lowering driven by that correspondence: resolve the frozen
selection through the N5 ladder (unchanged code), evaluate the slot
to `T` (unchanged), build `Verb<T>`, `run`, emit names via the
existing `name_fillet`/`name_chamfer` doors. The `node.rs`
traversal arms are NOT restructured in this unit — the design's
cost reduction is claimed at the next NEW verb, not by rewriting
what exists.

## S4-4 — VS-Q5: the `RimSide`/`RimSupport` twin KEEPS, recorded

As recommended: not load-bearing for two variants, and `RimSide`
belongs to the birth record, not the `Verb` payload — collapsing it
is orthogonal to this unit. The one action: the naming.rs twin
paragraph gains a sentence naming `Verb` as the canonical owner
that a future collapse would target, so the disposition is recorded
where the twin lives.

## Acceptance

- Workspace + demos suites green, BOTH feature graphs; hosted CI
  green on the final head with the gated point stated.
- **Pin rows, red-first where cheap**: content tags equal the
  pre-change constants; a v-current saved document with a fillet
  and a chamfer round-trips byte-identical; one existing blend
  document's evaluation is bit-identical (body + name table)
  through the new lowering.
- `param_flow` exhaustiveness test per verb against the birth
  record's role families.
- No serde/`Expr`/name/node-id types in `crates/verbs` (a
  layer-guard test in the crate, the LB13 shape).
- The PR body states the measured shape of "adding a verb after
  this" (which files a hypothetical `Verb::Shell` would touch),
  as the baseline design §6's next-door measurement will be read
  against.

## Out of scope

Boolean and other verb migrations (SEAT-5+); any `ParamFlow`
consumer (SEAT-6); the `RimSide` collapse; issue 1527; any `Node`
or schema change whatsoever.

# M4 PR 1 binding spec — recipe substrate + editor-core birth

Orchestrator-authored binding spec (2026-07-23). Deviations must be
REPORTED, not improvised. Charter: M4-PLAN PR 1 under ratified forks
F1 (dimension lattice), F2 (result-DAG shape — types only referenced
here), F4 (node vocabulary), F7 (expression AST, no conditionals).
Branch: `ev/m4-1-editor-core`.

## Scope

IN: crate `crates/editor-core` — `Doc` as a value, `RecipeNodeId`,
the v1 node vocabulary AS DATA, structural-vs-continuous parameters,
the expression sublanguage v1 (typed quantities, dimension checking,
generic evaluator), `ExprPath`, the DocEdit vocabulary + pure
`apply`. NO geometry evaluation (PR 2), NO serde/persistence (PR 6),
NO name resolution (PR 3/4).

## D1 — Crate and layer boundary (binding)

`crates/editor-core`, depending on `geom-core` ONLY (for
`Real`/`Decide` — the expression evaluator is scalar-generic from
day one, a ratified banked principle). It must NOT depend on
topo/sweep/profile in this PR; PR 2's evaluation service adds those
behind the evaluator, keeping G1's layering honest.

## D2 — Doc is a plain value (binding)

`Doc` = recipe DAG (node map + insertion-ordered node list) +
document metadata (recorded ε per H4's future landing; empty
metadata map). Cheap-clone plain Rust (Vec/BTreeMap; NO
persistent-data-structure dependency — document scale does not
justify one; revisit only with corpus latency data).
`apply(doc, edit) -> Result<Doc, EditError>` is PURE (returns a new
value; the input is untouched). Undo/redo = keeping prior values;
no edit is ever destructive of history at this layer.

## D3 — RecipeNodeId + node vocabulary as data (binding)

- `RecipeNodeId(u64)`: minted from a monotone counter stored in
  `Doc`, never reused (deletion does not free IDs), never
  positional. This is N1's substrate — treat its stability as a
  contract with tests.
- Node payload enum per M4-PLAN F4, DATA ONLY (no evaluate methods):
  `Datum{Plane|Axis|Point}`, `Profile` (programmatic: the existing
  profile-crate description carried opaquely as a value — do NOT
  re-model profiles; wrap), `Extrude`, `Revolve`, `Split`,
  `Boolean{Union|Intersect|Subtract}`, `Transform` (rigid
  placement), `Pattern{Linear|Circular}` with a Count-typed
  structural index expression (A8), `Declare` (coincidence-intent
  pairs BY StableName type — import the name TYPE shape from
  NAMING-DESIGN N1 as a placeholder struct in this crate; resolution
  semantics are PR 3/5).
- Upstream references: nodes hold input `RecipeNodeId`s; the DAG's
  edges are those references. `apply` rejects unresolvable refs and
  cycles with typed errors.
- **StableName-carrying payloads (Declare) — carve-out, ruled at the
  PR 1 review (2026-07-23)**: names are REFERENCES, not DAG edges.
  `apply` validates at edit time that a name's node EXISTS (a
  never-existed id is a typo, not a dangling reference — refuse at
  the best-diagnostics door), but a later `DeleteNode` MAY strand a
  name: that is NAMING-DESIGN N5's ratified dangling-reference
  semantics (loud `NodeGone` at resolution; `Rebind` is the repair;
  blocking the delete would force cascade-or-pre-repair, worse than
  the typed-failure flow). Rustdoc on Declare states this.
- Structural vs continuous (D8, binding shape): structural
  parameters are Count-typed expression slots whose edits are
  `SetStructuralParam` (a distinct DocEdit arm from `SetParam`);
  continuous are Length/Angle/Scalar slots. The type system makes
  the distinction unlosable, per DESIGN.md's "stated, not emergent".

## D4 — Expression sublanguage v1 (binding, F1+F7)

- Quantity dimensions: `{Length, Angle, Count, Scalar}`. AST:
  literals (dimensioned), parameter refs (document-level named
  params), arithmetic (Add/Sub/Neg/Mul/Div), trig
  (sin/cos/tan/atan2 on Angle → Scalar; atan2 → Angle), min/max
  (same-dimension). **No conditionals, no iteration, no
  user-defined functions** — total by construction; case analysis
  belongs to structural parameters (ratified F7 rationale).
- Dimension checking at expression CONSTRUCTION time (typed
  `DimensionError`), per the F1 lattice: same-dimension add/sub/
  compare/min/max; Mul requires ≥1 dimensionless operand; Div
  requires a dimensionless divisor. Same-dimension ratios
  (Length/Length) are REFUSED in v1 — relaxation is additive; pin
  the refusal with a test and a doc note saying exactly that.
- Count: integer-valued; Count arithmetic closed over
  add/sub/mul/min/max; Count→Scalar promotion explicit, never
  implicit.
- Evaluator: `fn eval<T: Real>(expr, params) -> Result<T, EvalError>`
  — generic over `Real`, no raw comparisons on control-flow paths
  (there are none: the AST has no branches), pin an Interval
  instantiation test. Units erase at this boundary (GQ5): the
  evaluator returns raw `T` in kernel units.

## D5 — ExprPath (binding)

`ExprPath { node: RecipeNodeId, slot: SlotId, path: Vec<u8> }` —
`SlotId` a per-node-type named enum (never an index), `path` =
AST-child indices. Pin with tests: editing a DIFFERENT expression,
or unrelated AST subtrees of the same expression, leaves an
ExprPath's referent intact (this stability is what GeomSource will
lean on in PR 5).

## D6 — DocEdit vocabulary v1 (binding)

Arms now: `InsertNode`, `DeleteNode`, `SetParam`,
`SetStructuralParam`, `SetExpression`, `SetDocParam` (document-level
named params). Arms RESERVED by doc comment (landing later, listed
so the enum's evolution is planned, not ad hoc): Rebind (PR 4),
ReWitness (PR 4, types only), SetTolerance (PR 6), appearance edits
(PR 7). `apply` validation: refs resolve, no cycles, dimension
checks re-run on touched expressions, structural edits flagged in
the returned edit record. `EditError` typed and specific — no
stringly errors.

## D7 — Replay + diff (binding acceptance machinery)

- Replay: `Doc::replay(edits: &[DocEdit]) -> Result<Doc, EditError>`
  from the empty document; applying a recorded edit list reproduces
  the Doc BIT-IDENTICALLY (float fields bit-equal; test pins).
- Structural diff: `Doc::diff(&self, other) -> DocDiff` listing
  node-level adds/removes/changes — the primitive SetTolerance's
  audit and the naming layer's edit-diagnosis will later consume;
  keep it node-granular, no expression-level cleverness yet.

## D8 — Acceptance (binding)

`crates/editor-core/tests/m4_pr1_doc.rs` (+ siblings):
- The DIE as a document: author the 21-pip die recipe as DATA
  through `apply` calls (datums, profile wraps, extrude, pattern or
  21 explicit subtract nodes — your choice, report it), diff two
  variants (changed pip depth), replay-identity green.
- Dimension-checker refusals: Length+Angle, Length×Length,
  Length/Length, implicit Count→Scalar — each a typed error test.
- ExprPath stability tests per D5; RecipeNodeId never-reused test
  (delete then insert); cycle and dangling-ref rejection tests;
  Interval evaluator instantiation test.
- rustdoc on every public type stating its ratified source (F-fork
  or N/W-decision) — one line each, no essays.

## D9 — Process (binding)

OUTPUT DISCIPLINE per M3 conventions (≤~150 lines/tool call,
skeleton first, chunked reads, report ≤120 lines). Branch
`ev/m4-1-editor-core` from origin/main; push after EVERY commit.
Gate: `scripts/gate.sh <merged sha>` — note the workspace gains a
crate, so clippy/fmt rows cover it automatically; verify
`cargo test -p editor-core` joins the workspace default. NEVER
export RUSTFLAGS. Open the PR (title "M4 PR 1: editor-core — recipe
substrate, expression sublanguage, DocEdit/apply"), do NOT merge —
adversarial review follows.

# PORT-DIMS-1 — the dimension refusals at the Python boundary (spec)

**Unit:** `work/port/PORT-DIMS-1.md`, carrying two rows:
`load-path-stringifies-structured-refusals` and
`python-dimensionerror-names-the-quantity-check-not-the-dimension-check`.
**Branch:** `port/dims-1-load-door-and-name`. **Deleted at merge** with
its `docs/DOC-LEDGER.md` row in this PR.

Read `docs/prompts/implementer-discipline.md` in full first.

## The row is four weeks old and the tree has moved under it

`load-path-stringifies-structured-refusals` was written 2026-08-20. I
re-read it against the tree on 2026-09-15 and **two of its statements
are now false**. Verify both corrections yourself — this spec is a
hypothesis, and specs from this orchestrator have carried confidently
stated errors that lanes caught by reading the tree.

1. **"Debug-formatted into the message" is stale.** The site now reads
   `D::Error::custom(format!("ill-dimensioned expression refused: {e}"))`
   — `{e}`, Display, not `{e:?}`. PR #2482 changed it. The message is
   now the refusal's own prose rather than a Debug dump, which is a real
   improvement and **not the defect**. The defect is that a structured
   `DimensionError` becomes a `String` in a serde message at all.
2. **The row's first sweep names instances that are gone or were never
   instances.** It lists `py/value.rs:203`, `:710` and `py/flush.rs:166`
   as `format!("{err:?}")` user-facing messages. At today's head, every
   surviving `{:?}` in `crates/pncad-py/src/py/` that I found is inside a
   `__repr__`, which is correct Python idiom and not an error message.
   **Do not chase the named instances. Re-derive the class.**

What the row gets right, and I confirmed: `WireExpr::rebuild()` returns
`Result<Expr, crate::expr::DimensionError>`, the document layer's own
dimension checker (`crates/editor-core/src/expr.rs`); the serde message
becomes `PersistError::Parse`, whose Python tag is `"parse"`; and
`crates/pncad-py/src/errors.rs`'s header promises *"typed exceptions
carrying the structured error, never strings"*.

**And there is a second site the row never names.** `wire.rs`'s
`WireMeasureExpr` Deserialize does the same thing — `wire.rebuild()`
returning `Result<MeasureExpr, DimensionError>`, stringified into
`Error::custom`. One row, two doors. Expect more; `persist/` holds
thirteen `Error::custom` calls and you should ask each the same question.

## Part A — the load door keeps the structure

The class is **a structured kernel refusal destroyed at a serde
boundary**, not "a Debug format string". Derive it. Each of `persist/`'s
`Error::custom` sites either (a) stringifies a structured kernel error,
which is this defect, (b) reports a genuine parse or schema fault, which
is what `Error::custom` is for and is correct, or (c) is something else
you should describe. **Put the disposition of all thirteen in the PR
body**, one line each. A hit list is a receipt; "swept clean" is a claim.

The hard part is not finding them, it is that serde's `Deserialize`
signature gives you one error type and it is the format's. So the
question the unit actually answers is: **where does the structure go so
that `pncad.load` can raise it?** That is a design call and it is yours.
Sketch of the space, not a constraint:

- A typed arm on `PersistError` that carries the structured refusal,
  with the serde message kept as the human half.
- A side channel that survives the serde round trip.
- Refusing earlier, before serde is in the picture, where the wire type
  can be validated against the document layer without `Error::custom`.

Argue for what you pick and say what it costs. If the honest answer is
that one site is fixable and the rest need a shape this unit should not
invent, **land that one and file the rest** (§6) rather than widening.

**The reachability class is the more useful half of the row, and it
stands.** #689 argued that the kernel's `DimensionError` reaches Python
through exactly one door because *"no bound door binds the operator
builders"* — true of the **authoring** doors and false of the
**deserialization** doors, because every `Deserialize` in `wire.rs`
re-runs a smart constructor. Ask `tags.rs`'s `persist_error_tag`,
`edit_error_tag` and `path_error_tag` the same reachability question
from the load path as well as the authoring path, and say what you find.

## Part B — the name

Ev ruled `S107` a defect on 2026-09-15, on the principle that **Python
should always match Rust where it can**. The Rust type is
`QuantityOpMismatch`; the Python class it is published as is
`DimensionError`; the document layer's real `DimensionError` reaches
Python as `LiteralError` and as `PersistError`. So the name denotes the
thing that is not the dimension checker.

The rename is small. The **re-documentation is the bulk** — the row
lists the sites, and they exist to reconcile the two spellings, which is
Q2's shape in the reviewer brief. The ruling licenses deleting them
rather than maintaining them.

**The shape question, which Parts A and B answer together.** Renaming
the Python class frees the name `DimensionError`. Whether the document
layer's real type then takes it is **not ruled** — Ev ruled the mismatch
a defect, not what the vacated name is used for. If it does take it,
that is the class Part A's structured refusal should arrive as, and the
two halves land as one story. If it does not, Part A still has to name
something. Decide it once, for both, and put the reasoning in the PR.

## Verification

This unit's defect was found **by execution**, not by reading — that is
why it earns the second arm. Follow suit: the row's own reproduction
drove `pncad.load(text)` over a hand-edited save file and read the class
and tag back. **Do that again at your head**, before and after, and put
the observed before/after in the PR body. A Rust test that the arm
exists is not evidence that a Python caller receives it.

Hosted CI is the verification of record. Push, mark ready for review,
poll the run's jobs API in the foreground until it concludes, report in
the same turn. The python suite is the row that matters most here. No
`CI-Config:` trailer.

## Review

**Full review — correctness arm and style arm.** This unit is the one
`work/port/plan.md`'s posture names, on triggers 2 and 3: the guarantee
has no mechanical guard, and the defect is invisible from Rust. Write
the PR so a reviewer can find each decision and its reasoning.

Note for your own reading, because it has caught two units on this
slate: this diff is a fix for a structural finding — a refusal losing
its structure at a boundary — so check whether the fix mints a fresh
instance. A new typed arm that carries a formatted string, or a second
tag vocabulary beside `tags.rs`, would be exactly that.

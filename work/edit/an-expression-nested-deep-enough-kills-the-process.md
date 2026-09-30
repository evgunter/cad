---
id: an-expression-nested-deep-enough-kills-the-process
kind: issue
title: editor-core: parse_expr, eval and the drop of an Expr recurse once per nesting level with no bound, so a deep expression kills the process
status: review
opened: 2026-09-30
pr: 3510
branch: edit/expr-nesting-bound
priority: P1
cost: M
---


(EDIT, found by the recursion sweep of `edit/part-depth-bound`.)

## What

`Expr` (`crates/editor-core/src/expr.rs`, `ExprKind`) is a tree of
`Box<Expr>` with no depth bound, and every walk over it recurses once
per level: the parser's `Parser::unary` and its parenthesised
sub-expression (`crates/editor-core/src/parse.rs`), `eval_inner`,
`eval_count`, `param_refs`, `literal_bits`, `with_replaced`, the
memo's `feed` (`crates/editor-core/src/eval/memo.rs`), the derived
`Drop`, `Clone`, `PartialEq` and serde impls, and `unparse`'s
`write_nested`. `MeasureExpr` (`crates/editor-core/src/measure.rs`,
`eval_measure_inner` in `eval/measure.rs`) has the same shape.

The fix pass's mutual-recursion sweep names the rest of the cycle:
the parser's `sum` → `product` → `unary` → `primary` → `call` (and
the parenthesised sub-expression) → `sum` (`crates/editor-core/src/parse.rs`),
and the parameter-source key's `encode` ↔ `binary` ↔ `unary`
(`crates/editor-core/src/param_source.rs`).

So an expression nested deep enough kills the process with a stack
overflow instead of refusing typed. The text an author types into a
parameter field reaches `parse_expr` as written, so this is a crash
from user input, not only from the API.

Loading is bounded by accident: `persist/mod.rs` reads with
`serde_json::from_str`, whose default recursion limit (128) refuses a
nested file typed. Saving is not: `serde_json` serialises without a
limit, so an expression built through the API crashes the save.

## Evidence

Release build (the workspace profile), on a thread with an 8 MiB stack,
each input parsed with `parse_expr(src, &BTreeMap::new())` and then
evaluated with `eval(&e, &ParamEnv::default())`:

| input | outcome |
|---|---|
| `1+1+…+1`, 10⁵ terms | parses and evaluates |
| `1+1+…+1`, 10⁶ terms | stack overflow, SIGABRT |
| `((…(1)…))`, 10⁵ deep | stack overflow in the parser |
| `--…-1`, 10⁵ deep | stack overflow in the parser |

The probe (a scratch binary depending on `editor-core`):

```rust
let src = format!("{}1{}", "(".repeat(n), ")".repeat(n));
let parsed = editor_core::parse_expr(&src, &BTreeMap::new());
let v = editor_core::eval(&parsed.unwrap(), &editor_core::ParamEnv::<f64>::default());
```

The thresholds above are release-only. The wheel CI builds and tests
(`maturin build` with no `--release`, so the dev profile) dies far
sooner. `Doc.parse_expr` on CPython 3.12's main thread (8 MiB), with
the review's probe (`review-probes/depth-rev/expr_probe.py` on
`review/depth-rev`, not merged), re-run on `edit/part-depth-bound`'s
wheel:

| input | outcome |
|---|---|
| `((…(1)…))`, 2000 deep | parses |
| `((…(1)…))`, 2500 or 3000 deep | SIGSEGV in the parser, exit 139 |
| `--…-1`, 10⁴ deep | parses |
| `1+1+…+1`, 3·10⁴ terms | parses |
| `1+1+…+1`, 10⁵ terms | parses, then SIGSEGV at teardown (the `Expr`'s drop), exit 139 |

```python
import sys
from pncad import Doc
n, kind = int(sys.argv[1]), sys.argv[2]
d = Doc("expr-probe")
if kind == "paren":
    src = "(" * n + "1" + ")" * n
elif kind == "neg":
    src = "-" * n + "1"
else:
    src = "+".join(["1"] * n)
d.parse_expr(src)
print("parsed", kind, n)
```

## What would close it

A nesting bound checked where an expression is built (the parser and
the smart constructors), refused typed with a recourse, and set so
every walk above fits the smallest stack a door runs on (the wasm32
build's 1 MiB); or walks that do not recurse, including `Drop`. A row
that parses, evaluates, saves and drops an expression one past the
bound on that stack and reads the refusal back.

## Built (2026-09-30)

One bound, `expr::MAX_NESTING` = 128 levels, read by every door that
mints an expression: the smart constructors cache each node's nesting
and refuse past it (`DimensionError::NestedTooDeep`; `Expr::neg` and
`MeasureExpr::neg` are fallible for it alone), the text door builds
through them (brackets alone nest nothing, and a minus sign directly
before a number is the literal's own, so every tree the constructors
admit reads back from its `unparse` text node for node), and the load
door (`persist::nesting`) scans a body against the nesting a save can
reach (271 JSON brackets, pinned against a census of every expression
position in the corpus) before serde_json reads it with its own fixed
limit off, reading an expression's children through a counter that
refuses past the bound. The parser, `eval`, `eval_count`, the
measurement evaluator and `Drop` on `Expr` and `MeasureExpr` keep their
own stack (`crate::tree`). Every other walk recurses at most 128
levels; the tallest, the load door, needs 647 KiB of the 1 MiB wasm32
stack in the dev profile. A flat chain of more than 128 terms refuses,
and the docs where a user meets it say so. Rows:
`editor-core::all expr_nesting_bound` (ten, on a 1 MiB thread),
`u8a_parse`'s signed-literal row and random-tree round trip, four unit
rows, and `test_expressions.TestNestingBound` on a `threading.Thread`.

Correction to the premises above: the round trip broke from 62 levels,
not only past the save's own recursion: serde_json's 128 counts JSON
levels, and an operator is two, so a 62-term sum saved and then
refused to load.

Filed: `a-metadata-value-nested-deep-enough-kills-the-process` (P2) and
`a-flat-chain-of-more-than-128-terms-refuses` (P4, the two routes that
would lift the bound). The load door's limit now also admits a
`StableName` nested up to 65 `InPart` levels where serde_json refused
past about 31; that is the name row's to walk, and its row says so.

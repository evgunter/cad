---
id: corpus-result-node-loops-skip-silently
kind: issue
title: m4_pr8_corpus.rs's result-node loop can skip every document with nothing counting it
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## What

`crates/editor-core/tests/m4_pr8_corpus.rs`, in the corpus loop that
runs the body pass, opens with

```rust
let Some(result) = d.result else { continue };
```

and nothing counts how many documents took the `else`. Three of the 28
registered documents carry `result: None` today. If a corpus edit,
a builder change, or a `result` wiring regression turned that 3 into
28, every assertion under the `let` would stop running and the row
would stay green: the loop still iterates, the test still passes, and
no number moves.

## Why it is this program's

*A row that cannot go red is not a test.* This is the same defect
SUITE/D114's reviewer found in that unit's own mass-properties arm,
which was gated on the identical `d.result` and guarded by nothing;
D114 closed its instance with a counter asserted positive
(`props > 0`) and prints it in the row's census line. The same
three-line fix applies here.

## Fix

Count the documents that reach the body pass, assert the count
positive, and print it. A floor on the exact number is not wanted —
the corpus grows — but zero must be loud.

`crates/*/tests/*` is S-TINT's and S-TCOST's shared ground; filed here
because the question is a claim that cannot fail, not a cost.

Filed by SUITE/D114.

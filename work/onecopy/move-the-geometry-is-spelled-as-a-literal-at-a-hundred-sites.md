---
id: move-the-geometry-is-spelled-as-a-literal-at-a-hundred-sites
kind: issue
title: "Recourse: move the geometry …" is spelled as a literal on 103 lines, beside its home (MOVE_RECOURSE, move_recourse!("so …"))
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Left by `work/onecopy/carry-refusal-copies-and-hand-spelled-move-recourses.md`. That row gave the move-recourse class its spelling (`MOVE_RECOURSE` the whole ending, `COINCIDENCE_RECOURSE` the fragment, `geom_core::move_recourse!("so …")` a move named more closely), and put the gate on the named pair. The gate reads constants, not words. The words themselves are still written by hand under `crates/*/src` (counted on 2026-11-10, at `onecopy/carry-copies`):

| spelling | lines |
|---|---|
| `"… Recourse: move the geometry"`, the whole ending | 13 |
| `"Recourse: move the geometry so …"` | 81 |
| `"Recourse: move the geometry …"`, other continuations | 9 |
| `"Recourse: move <another subject> …"` (`the parts` 42, `the point` 8, `the split` 6, …) | 111 |

Many of the 13 are test pins (`refusal_routes.rs`, `order.rs`, `tier3_tests.rs`, `certify.rs`), which may rightly spell the text they pin. The production ones are copies of `MOVE_RECOURSE`.

**What is owed.**
1. Route the production copies of the whole ending, and the `so …` literals that are `&'static str`, through the home.
2. Decide what a `format!` site with interpolation after "so" writes. `concat!` cannot take it; `{MOVE_RECOURSE} so {…}` is the likely spelling.
3. Then decide whether the gate should read the words `Recourse: move the geometry` as a literal.

The "another subject" lines are a different lever ("move the parts" is an operation's own) and are not the move arm's.

`grep -rn 'Recourse: move' crates/*/src` lists them all.

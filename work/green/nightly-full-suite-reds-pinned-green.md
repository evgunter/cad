---
id: nightly-full-suite-reds-pinned-green
kind: unit
title: every red row in the nightly full suite outside the slow-set row is re-blessed or pinned green, each with its fix row on its owning program
status: open
opened: 2026-10-28
priority: P1
cost: H
---


## What

Nightly 74 (run 40637814390, main `a1cfd1cfdb`) failed 141 test rows
across the three ε in `full suite (every eps row, slow set included)`.
`main-reds-editor-core-slow-set-and-the-k-probe-sweep` covers some of
them; most sit on rows that kernel programs filed (`grep -rl
'main-red' work/`) and have not greened.

## Owed

Per GREEN's charter (`program.md`, "A red is greened, not fixed,
here"): for each red row in tonight's run that is neither in the
slow-set row nor on an open PR that already greens it (e.g. #5935),
re-bless an explained deliberate move, or pin what main produces (or
`#[ignore = "pinned red: work/<program>/<row-id>"]`) and make sure a
fix row exists on the owning program, naming the bisected merge, the
old and new values and the reproduce command. Done when the nightly's
full-suite job is green, or every row left red is named with the open
PR that greens it.

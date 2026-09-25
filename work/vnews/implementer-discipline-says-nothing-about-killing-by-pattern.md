---
id: implementer-discipline-says-nothing-about-killing-by-pattern
kind: issue
title: implementer-discipline says nothing about killing processes by pattern, and a lane's pkill took down another lane's build
status: open
opened: 2026-09-24
needs_ev: true
---


## What happened

On 2026-09-20 the `is-instance` lane cleared its own stuck runs with
broad `pkill -f nextest` and `pkill -f cargo doc` on a box shared with
other lanes. Another VNEWS lane (the tone unit, its own target dir) was
building at the time and the lane disclosed that it may have killed that
run; it restarted. The lane reported the harm itself, unprompted.

## Why it survives Ev's register test

This is the one rule that survived applying Ev's #3024 ruling to
`work/vnews/plan.md`'s dispatch rules (#3209): it reports an actual
problem, a prompt could actually fix it — an advance warning is exactly
what stops a lane typing `pkill -f` — and it is written nowhere in
`docs/prompts/` or `memories/`. Grepped both for `pkill` before filing.

## The ask

A bullet in `docs/prompts/implementer-discipline.md` §2, beside the two
`CARGO_TARGET_DIR` bullets, since owning your target dir is what makes
your processes attributable. The `[ev]` PR carries the text.

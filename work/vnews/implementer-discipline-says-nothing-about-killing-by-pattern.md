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

## A second shared-box hazard, added 2026-09-25

The same PR now carries a second bullet, for the same reason and the
same class. Two VNEWS lanes each wrote `body.md` to the session's shared
scratchpad; the `frame.rs` prose-pass lane's `gh pr edit` then published
the gated-controls lane's PR body onto #3215, and restored it within
minutes from a namespaced copy. The old VIEW lane register carried a
*namespace the scratchpad* rule and it went with #3024; this is the
first recurrence since, and it produced a wrong public artifact rather
than a near-miss. It passes Ev's test on the same grounds as the first
bullet: an actual problem, prevented by an advance warning, written
nowhere in `docs/prompts/` or `memories/` (grepped for `scratchpad`).

The orchestrator's own share: VNEWS's wave-1 briefs gave each lane a
scratchpad prefix, and the briefs were trimmed after #3024 on the
reasoning that standing obligations live in `docs/prompts/`. This one
did not live there, so trimming it from the briefs removed it entirely.

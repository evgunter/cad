---
id: python-cannot-set-options-structs
kind: issue
title: Python callers cannot set import/STL/eval options: four kernel options structs cross the FFI with no exposure and no census anchor
status: open
opened: 2026-09-01
github: 1495
refs: [1493, 730]
---

## From GitHub issue 1495

Opened 2026-09-01; 0 comments.

(SMELL-UV orchestrator) Filed from PR 1493's review as the durable home for its disclosed-but-unscheduled siblings — the same class that PR closed for `StepOptions` (issue 730), one door over each.

**The tooth**: `crates/pncad-py/src/py/value.rs:999` hardcodes `ImportOptions::default()`, so a Python caller cannot override `eps_in` on STEP import — a caller-visible ε they cannot set, the exact asymmetry issue 730 named for export, now the import door's.

**The class**: four kernel options structs cross into `pncad-py` with no per-field exposure and no destructure anchor, so a field added kernel-side is silent in Python:
- `step_import::ImportOptions` (the tooth above);
- `stl::AsciiOptions` and `stl::BinaryOptions` (the STL doors expose `solid_name`/`header` as kwargs but carry no census anchor tying them to the structs);
- `EvalOptions` (the evaluation door).

**The fix shape, established**: PR 1493's `step_string` pattern — one keyword per field, `None` forwarding to the Rust default via `unwrap_or(defaults.X)` (no re-spelled constants), plus the `surface_census.rs` destructure anchor (a probe extra field reds E0063 at the door and E0027 at the census, measured there). PR 1493's review also flagged that any `NotBound` roster entry needs the decay half (a bound-later spelling must red) — its fix pass is adding that machinery; reuse it.

Track U fence (SMELL-UV's); takeable as one lane.

## Home

`work/code-quality/` — the issue places itself inside Track U's fence (SMELL-UV's) and calls it takeable as one lane; the register owns Tracks K–X.

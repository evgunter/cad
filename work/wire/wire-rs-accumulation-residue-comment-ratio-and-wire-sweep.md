---
id: wire-rs-accumulation-residue-comment-ratio-and-wire-sweep
kind: issue
title: wire.rs's two accumulation findings survive the header fix: 41% comment ratio, and wire_sweep exists to fail
status: open
opened: 2026-09-12
---



## Finding

`wire-rs-module-header-describes-five-sixths-of-the-file` carried **three**
findings, not one. PR 2480 addressed the first and closed the row; the
delta review of that PR (NOTE 4) caught it. The other two are here so
they are scheduled rather than deleted with the row.

Both were measured by the style review of PR 2376, reading
`crates/editor-core/src/eval/wire.rs` end to end — which nothing in this
project's process otherwise does. Confidence `likely` on each, the
reviewer's.

- **The file is 41% comment** — 1885 comment lines against 2712 code
  lines at that reading, excluding the inline test module. `unit`
  carries ~35 doc lines over 4 lines of body; `mod ladder` carries ~90
  over ~60. Each is individually defensible, which is the point: no
  diff ever sees the ratio, so nothing ever decides it. (The file has
  since grown; a taker re-measures rather than quoting these numbers.)
- **`wire_sweep` exists to fail** — it evaluates two structural slots
  into `_stations`/`_v_degree`, calls `section_of` twice into `let _`,
  and unconditionally returns `CurvedSolidFrontier`. Deliberate and
  documented, and it still reads as dead on every future encounter.
  The recipe doors it runs first are real (a Sweep on a datum must read
  as a recipe error, and `wire_operand_door`'s `section_of` sweep row
  now pins that), so the question is how to say "these doors run, the
  geometry does not exist" without the body looking abandoned.

## What PR 2480 did settle

The header itself. It now names four jobs — the wiring, the
mid-evaluation name ladder, the declaration routing, and the
placement-rule arithmetic — so the claim that falsified itself is gone
and the open structural question is visible. Splitting the file is
still not proposed by either row.

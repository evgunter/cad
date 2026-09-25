---
id: wire-rs-accumulation-residue-comment-ratio-and-wire-sweep
kind: issue
title: eval/wire.rs is about 47% comment by line and nothing decides whether that is wanted: a ratio budget or an editorial pass
status: open
opened: 2026-09-12
priority: P4
cost: D
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

## Re-measured 2026-09-15 (style review of PR 2681)

The row asks a taker to re-measure rather than quote 41%. The review of
PR 2681 read `crates/editor-core/src/eval/wire.rs` end to end — the
same thing nothing else in this project's process does — and measured:

- **5173 lines**, non-test span **4806**, **2177 comment lines against
  2498 code lines — 46.6%.** The ratio has moved 41% → 46.6% since the
  PR 2376 reading, on a file that grew by roughly 2100 lines.
- Re-run with the same instrument on PR 2681's merge head (which adds
  ~65 lines of door and doc to the file): 5238 lines, non-test span
  4872, **2222 comment against 2517 code — 46.9%**. The drift is the
  file's, not that PR's.

`wire_sweep` still reads as dead in the body sense this row means —
`let _` bindings, an unconditional `CurvedSolidFrontier` — and is NOT
dead in the reachability sense: the `Node::Sweep` arm of the wiring
dispatch calls it, which is what makes the recipe doors it runs first
real. The row's question is unchanged.

## Read against the tree (2026-09-24)

**The `wire_sweep` half is done in PR 3141.** Its recipe doors now run as
statements (`need_count(...)?;`, `section_of(...)?;`) under one comment
that says each runs for its refusal alone, with no reader for what it
answers. The `let _`/`_stations` bindings that made the body look
abandoned are gone. The behaviour is unchanged.

**The comment-ratio half is left open, and it is not an E fix.**
Re-measured with the row's instrument, on the span before the first
`#[cfg(test)]`, counting a line as comment when its trimmed text starts
with `//`: the span is 4959 of the file's 5708 lines, with **2320 comment
lines against 2639 code lines, or 46.8%**. That is flat against the
2026-09-15 reading (46.6 to 46.9%). No single site is wrong. The row's
question is whether a ratio budget, or an editorial pass over about 2300
lines of `eval/wire.rs` commentary, is wanted at all. That pass is well
over E, and the budget is a choice about process, not a code fix. The
row should be re-priced (D or H) or deferred with a cited ruling rather
than dispatched as E.

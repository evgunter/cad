---
id: wire-surgery-header-split-is-spelled-nine-more-times-by-line
kind: issue
title: The wire's header/body split is spelled nine more times, by line, outside its one home
status: open
opened: 2026-09-20
priority: P4
cost: E
---


## The finding

A saved document is an id header followed by a JSON body, and
`crates/editor-core/tests/wire/mod.rs` is the one home for telling the
two apart: `doctored` (the surgery) and `wire_body` (its read-only
half) both come off one `split_body`, which cuts at the first `{`.

Every suite that cuts by BRACE now imports that home — the four
`text.find('{')` copies (`asm_r2b_interface_wire`, `asm_r2a_mate_solve`,
`docm7_union_declare`, `r2_m10_2_probes`) were folded on
`edit/crossing-drops-mate-id`. The copies that cut by LINE
(`text.split_once('\n')`) were not. Grepped over
`crates/editor-core/tests/*.rs`, nine suites cut a save that way, at
sixteen sites:

- `bool13_r1_probes.rs` — a local `split` helper answering
  `(header, Value)`, a body-only comparison, a duplicate-key splice
  that must stay TEXT, and a read-only parse (four)
- `msolve6_part_extent.rs` — one read-only parse and two that rewrite
  the body (three)
- `m4_pr6_refusal.rs` (two), `m10_1_distribution_wire.rs` (two
  read-only parses), `bool13r2_probes.rs` (a local `split` helper and
  a header strip)
- `m4_pr6_review_probes.rs`, `unreadable_by_this_build.rs`,
  `wire_rv_unknown.rs`, `asm_r2a_mate_solve.rs` — one each (the last
  a body-text comparison, beside the brace-cut site already folded)

The two spellings do not disagree today: the header is one line and
the body's brace opens the next. That is why nobody notices the tree
holds two rules for one question — a header that grew a second line,
or a body that did not start on its own line, would move them apart,
and each of the sixteen sites would then be its own repair.

**What the grep could not match**: a site that cuts by index, by
`lines()`, or by a `find` on some other anchor; and the same pattern
in any crate's tests but `editor-core`'s (searched there because
`tests/wire/` is editor-core's alone and deliberately not reachable
from `crates/viewer/tests/`).

## What closing it would decide

Whether the line cut and the brace cut are ONE rule with one home
(and which cut that home makes), or two rules each with its reason
stated. If one: the sixteen sites import `wire_body`/`doctored`, and
`tests/wire/mod.rs` says where a save's two parts divide. The
duplicate-key splice in `bool13_r1_probes` is the one site that cannot
use a parsed value at all — `serde_json::Value` holds no duplicate key
— so it wants the header and the body as TEXT, which is a third thing
that home could answer for rather than a reason to keep a copy.

Ground: `crates/editor-core/tests/*` (TCOST/TINT); the duplication
question is S-DUP's by its own charter.

Found by the EDIT lane on `edit/crossing-drops-mate-id` (PR #2906),
folding the brace-cut copies.

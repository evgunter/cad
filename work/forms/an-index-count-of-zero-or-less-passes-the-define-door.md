---
id: an-index-count-of-zero-or-less-passes-the-define-door
kind: issue
title: An index's count of 0 or a negative count passes the define door
status: open
opened: 2026-11-03
---


Found by FORMS' review of PR 6414. It predates that PR, but that PR's count row makes it reachable from a widget.

`DocEdit::DefineVar` with `VarDecl::Index { count, .. }` accepts a literal count of `0` or a negative count. The viewer builds that edit in `crates/viewer/src/props.rs:1662` (`index_count_edit`). Typed through `SetVariableText` on a pattern's index:

- `"0"` is accepted and the family has 0 members.
- `"-1"` is accepted, and the family then fails to evaluate.
- `SetVariable` with `Count(0)` and `Count(-2)` are accepted too.

The door does refuse `"2.5"` and `"1 mm"`, with a message naming the index and its dimension, and it refuses an unknown name at the parse. A formula (`"3 + 1"`) is accepted and gives 4 members.

The ruling is open: should the define door refuse a literal count below 1 (a formula can only be refused at evaluation), or is a refusal at evaluation, badged on the family, the design? Either way, the case needs a row.

Moved from `work/issues/` to `work/forms/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

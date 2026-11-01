---
id: a-per-member-slot-reading-lists-every-member-eagerly
kind: issue
title: A slot reading per member evaluates and holds every member eagerly, uncancelable
status: open
opened: 2026-11-01
---


Found by `wire-members` (WIREHOLD, `a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it`) while sweeping for loops over a family's members.

## Finding

`crates/viewer/src/props.rs`, the slot reader for an expression that reaches an index (the arm after `doc.expr_reach(expr)` returns a non-empty `Reached::Chain`), maps `doc.member_envs::<f64>(&chain)` to each member's index values and value and collects them into a `Vec` before it shows a span.

`Doc::member_envs` (`crates/editor-core/src/repetition.rs`) is lazy: one member's environment at a time. The viewer makes it eager. An index admits a count up to `u32::MAX` (`MemberCountFault::ADMITS`, `repetition.rs`), and nested indices multiply, so a reading of `k` under `k = index(4e9)` evaluates the expression 4e9 times and holds 4e9 rows (a `Vec<i64>` and a value each), on the thread that draws the properties pane, with no cancel.

The evaluator's own family loop now yields to cancelation before each member (WIREHOLD PR, `eval::members::eval_family`). This reader does not go through the evaluator, so that does not reach it.

## What it needs

The span the pane shows is a least and a greatest value (`MemberValues::span`). It could be folded over the lazy iterator without holding every member. Whether the pane should also stop after a budget is a policy question for whoever owns the properties pane.

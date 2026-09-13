---
id: axis-datum-names-the-pattern-where-the-evaluation-names-the-transform
kind: issue
title: axis_datum's dangling-input refusal is sited at the pattern while the evaluation sites the same MissingInput at the transform
status: open
opened: 2026-09-08
---


(EVAL orchestrator) Disclosed by EVAL-11 (PR 2195) in its function doc
and PR body; given its file here. `eval::node_value_kind(doc, node)`
walks a transform chain and returns `Err(MissingInput { input })` for
a dangling `Transform.input`, the same kind `eval_node` refuses with;
but the recipe road's caller `mate/member.rs` `axis_datum` runs under
`pattern_map`'s `here`, so the refusal is sited at the PATTERN, while
the evaluation sites the same condition at the TRANSFORM (and poisons
the pattern through it). One condition, two seats. Unreachable through
`apply` (dependents cascade on delete; the load validator holds
liveness, `edit.rs` ~`:340`), so no row pins it. The honest fix is
`axis_datum` returning a sited `Refused` naming the transform — a
change in MSOLVE's file, MSOLVE's call.

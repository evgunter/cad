---
id: shelled-result-discards-its-own-closing-verdict
kind: issue
title: Shelled::body is a Body: the verb validates its result and discards the verdict, where the Boolean's result keeps it
status: open
opened: 2026-10-06
priority: P3
cost: E
---


`topo::shell_open` (`crates/topo/src/shell.rs`, "One validation") ends
in `validate_geometric` on the body it returns, then hands it back as
`Shelled::body: Body<T>`, dropping the verdict it just derived. The
Boolean's result carries its body as an `AtRestBody`
(`crates/topo/src/boolean/ops.rs`, `BooleanBody::body`), and since
PR 4112 the shell's operand is one too. So a caller that chains a
shell into a finished-body door (another shell, a Boolean, a split, or
editor-core's `finished_operand`) re-runs tier 3 on bits the verb has
already passed.

Measured by the PR 4112 review lane on `vessel(1, 2)`, t = 0.2 (debug
build): the gate takes 3.3 ms, the shell 27.4 ms, and re-gating the
shelled result 16.0 ms. That is a cost, not a correctness hole.

Owed: return the result through `AtRestBody::validate` (it runs the
same `validate_geometric`), and follow the type through `verbs::run_shell`'s
`VerbOut` and its consumers. Left out of PR 4112 on the orchestrator's
ruling.

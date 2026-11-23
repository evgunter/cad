---
id: blend-reconcile-says-the-commit-door-admits-only-single-body-targets
kind: issue
title: BlendTool::reconcile's survival step does not flag a name surviving only on another member of a pattern target, and the commit door admits one
status: open
opened: 2026-11-22
priority: P2
cost: E
---

Found by FORMWORDS' `denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one`
sweep, which retired `combine::denotes_body` into the body arm of
`session::refuse::admits`.

**What.** `BlendTool::reconcile`'s doc (`crates/viewer/src/blend.rs`)
said its survival step's membership test reads the target NODE's edge
names, "not the drawn body's", and that the resulting gap — a name that
survived on another body of a multi-body node is not flagged — "cannot
reach a document — the commit door admits only single-body targets
(`crate::session::admits`)". The body gate is a kind check on the read
a node named alone takes, and a pattern defines a `Body`-kind variable
reached by its index (D10 `[repetition]`): `admits(doc, pattern,
NodeKindWanted::Body)` is true, measured on a box patterned × 2
(`combine_ops::box_and_pattern`), and `DocSession::add_blend` gates
its target through `require_kind(node, NodeKindWanted::Body)`. The same
doc says, a few paragraphs up, that `all_edges` "asked about a pattern"
answers "every instance". So a blend target that is a pattern is a
multi-body node the commit door admits, and the stated reason the gap
cannot reach a document does not hold for it. The doc was corrected
in the PR that filed this row (it now says the commit door admits a
pattern target and points here); the gap itself is this row.

**What a taker decides.** Whether a blend of a pattern can reach the
commit with a name that survived only on another member (then the gap
is real and wants the pick-index narrowing the doc describes, or a
member read at the blend's target), or whether something else stops it
(then the doc names that instead of the body gate).

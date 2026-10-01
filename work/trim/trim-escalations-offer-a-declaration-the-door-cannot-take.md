---
id: trim-escalations-offer-a-declaration-the-door-cannot-take
kind: issue
title: trim: an iso-row escalation forwards the declare menu to the shell op, which takes no declaration
status: closed
opened: 2026-09-29
priority: P2
cost: E
closed: 2026-09-30
pr: 3525
---


(CHROME, from the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.)

## What

`IsoRowError::Escalated` (`crates/geom-brep/src/nurbs_iso.rs`, the
variant near :239, its `Display` near :265) renders
`iso_boundary_row escalated: {source}`: a function name as its
subject, and the whole `Indeterminate`, so it ends in
`COINCIDENCE_RECOURSE`.

Its one user-facing door is the shell op. `ReplaceFaceError::IsoRow`
(`crates/topo/src/replace_face.rs` near :664) wraps it whole, and so do
`ShellError::Face`/`ShellError::Lift` (`crates/topo/src/shell.rs` near
:594/:598) and `NodeErrorKind::Shell`
(`crates/editor-core/src/eval/mod.rs` near :2332). The shell op takes
no declaration (`Node::Shell`, `crates/editor-core/src/node.rs` near
:2007, has no `declare`), so "declare the coincidence" has no object
there.

## Repair shape

Render `source.payload()` with a subject in plain words and a routed
recourse, either here or at the `ReplaceFaceError::IsoRow` wrapper
(SHELL's ground; see `shell-refusals-short-of-the-shape-guard`). See
`sweep::blend::BlendError::Escalated`'s `Display` and
`profile::validate::decision_subject` for the shape. The guard is
`test_utils::refusal::subjectless_escalations`.

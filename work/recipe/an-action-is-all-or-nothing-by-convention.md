---
id: an-action-is-all-or-nothing-by-convention
kind: issue
title: A viewer action is all or nothing only because every closure ends on its first refusal with ?; Recording lets an action go on past a refusal
status: open
opened: 2026-10-03
priority: P2
cost: M
---


## The finding

PR 3931's second review (N2, likely). The viewer's `stage_run` promises an action is all or nothing, and the minted-slice generator it replaced made "a refusal ends the run" structural. `Recording` (`crates/editor-core/src/edit.rs`) deliberately lets an action carry on after a refused edit (the refactor and the rows rely on that), so a viewer closure that writes `let _ = run.apply(..)` would commit a partial action. Every closure today uses `?`, and the viewer suite reds a `commit_action` that swallows a refusal (the reviewer's M6), but nothing in the types holds it.

## What would close it

Make the all-or-nothing door unable to continue past a refusal — e.g. `stage_run`'s closure receives a recorder whose `apply`/`insert` return `Result` and whose refusal ends the run (a wrapper over `Recording`, or a `Recording` mode), so a partial action cannot be committed by construction. Weigh against `Recording`'s other callers that legitimately continue.

Filed by the RECIPE orchestrator from PR 3931's second review.

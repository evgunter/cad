---
id: a-needle-cavity-test-red-on-main
kind: issue
title: topo shell_role_reader's needle-cavity test is red on main
status: open
opened: 2026-11-01
---


`crates/topo/tests/shell_role_reader.rs`
`a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`
fails on origin/main (reproduced on main in a separate worktree by the WIRE
`wire-carries` lane, 2026-11-01, while gating PR 6268). Not bisected; the
cause is unknown.

Moved from `work/issues/` to `work/flush/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Closed as a copy (ONECOPY, `onecopy/main-reds-triage`, 2026-11-09)

Same test, same day, as `work/flush/main-red-a-needle-cavity-12eps-channel-escalates-at-bool-contact-vertex.md`, which names the failure; that row is the home.

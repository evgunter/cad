---
id: a-subtract-tokens-chart-agreement-reconciles-revert-by-hand
kind: issue
title: the Boolean's one-record token reconciles revert's plane convention by hand when it reads how two charts face under subtract
status: open
opened: 2026-11-11
---


Filed by the onedoor fix-3 lane on PR 6505 (rv3 review S5).

## What

`crates/topo/src/boolean/ops.rs`, `OneRecordTokens::of_result`, reads
whether a glued pair's two charts face one way through the one
spelling `one_record::charts_agree`, but it feeds B's sense in as
`fb.sense != turned`, where `turned` is computed by hand: under ∖ a
reverted plane turns its chart where every other kind turns its faces'
senses. That convention belongs to revert (`transform::OrientationClass`)
and is restated here from outside it; a change to how revert treats a
kind would leave this site silently wrong.

## What closing it takes

Read the senses where revert has already applied its convention: the
moved and kept faces' senses in the result, on the records they wear
there, so the token needs no knowledge of revert. That needs a live
result face on each side of every pair (a fully covered decided face
has none), or a revert-owned door that says how a reverted record's
chart faces against its source's.

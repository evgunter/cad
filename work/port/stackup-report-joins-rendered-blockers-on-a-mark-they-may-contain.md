---
id: stackup-report-joins-rendered-blockers-on-a-mark-they-may-contain
kind: issue
title: the stackup report joins rendered Unavailable blockers flat on a "; " the elements may carry
status: open
opened: 2026-09-16
priority: P3
cost: E
---


Found by the sweep of
`work/view/startup-notices-join-on-a-mark-a-prefs-notice-contains`
(VIEW, #2710), whose population was every flat join of already-rendered
sentences on a mark the elements are free to write themselves.

## The site

`crates/editor-core/src/stackup.rs`, the report's `rss` row: the
`Rss::UnavailableBecause` arm renders each `Unavailable` blocker
through its own `Display` and joins them with a bare `"; "`. There are
two such renderings in the file — the one in the report body and a
second in the summary above it — so a change to the mark has to find
both.

A blocker whose own sentence writes a `"; "` makes the list read as one
item more than it has. Not audited arm by arm here; the finding is that
nothing holds it either way, which is the part a type could carry and
does not.

## The shape of the two answers already taken next door

VIEW hit the same class twice. `display::AdmissionFault` (#2693)
narrowed the element type so the sentences the join can reach are a
closed population a census covers; the startup line moved its elements
up a level so a boundary mark a door strips applies instead.
`crates/viewer/README.md`'s *"The line is composed at two levels and
they are two marks"* section is the written-up version of both.

## Home

PROPS's: `crates/editor-core/src/stackup.rs`.

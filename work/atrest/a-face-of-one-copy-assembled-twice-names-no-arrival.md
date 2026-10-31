---
id: a-face-of-one-copy-assembled-twice-names-no-arrival
kind: issue
title: A ⊔ A's interference finding lists the copy's faces once for two arrivals, so it cannot say which arrival a face is on
status: open
opened: 2026-10-27
priority: P3
cost: M
---

Filed from stage 5 C's review (PR 5810, lane wjfxCC, N5).

`A ⊔ A` (one copy read twice by a `⊔`,
`crates/editor-core/tests/intent_s5_c_at_rest_check.rs`
`one_body_assembled_twice_refuses_nothing`) is one interference
finding whose `Overlap::Bounded { faces }` lists six faces, though the
overlap is bounded by twelve, six of each arrival. The two arrivals are
one copy, so each `FaceSite` is `{ copy, face }` with the same `copy`
and the same name for both arrivals' faces, and
`crates/editor-core/src/checks/at_rest.rs` `localize` sorts and
deduplicates them. The finding cannot say which arrival a face is on,
and nor can a selection: it reads the copy by its variable, so it names
a face of the first arrival and never of the second (which is why the
finding is not `quietable`, and refuses at no severity).

Done when an arrival read twice is told apart in an interference
finding's sites (by its place among the `⊔`'s reads, say), or the
finding says it is one copy read twice and names its faces once on
purpose.

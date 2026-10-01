---
id: the-shape-guard-misses-the-boolean-merge-stage-label
kind: issue
title: tint: stage_prefixes does not see 'coplanar-merge output stage refused:', the label BooleanError::Merge puts before every merge refusal
status: open
opened: 2026-09-30
---


(TOPO, the fix pass of PR 3506: found by that PR's review.)

## What

`test_utils::refusal::stage_prefixes` (`crates/test-utils/src/refusal.rs`)
does not report "coplanar-merge output stage refused:", the label
`topo::BooleanError::Merge`'s `Display` (`crates/topo/src/boolean/mod.rs`)
puts in front of every `MergeCoplanarError` the Boolean's merge stage
forwards. The clause carries the wrapper verb "refused", which
`SENTENCE_WORDS` counts as a sentence's word, and the guard's two
exceptions for a wrapper verb (`WRAPPER_VERBS`) read it as a label only
when the clause opens with the verb or its subject is a gerund
(`joining the operands' sections refused:`); here the stage is named by
a noun phrase ("coplanar-merge output stage"), so neither fires.

So a row that runs the guard on a wrapped merge refusal passes over the
label it exists to catch: PR 3506's
`refusal_routes::tests::a_plane_orientation_refusal_tells_one_story_and_offers_no_declaration`
did (its fix pass now compares the wrapped text directly), and
`refusal_routes::tests::a_declared_pair_of_meeting_faces_is_contradicted_at_the_merge`
still does.

## Repair shape

Teach `stage_prefixes` the noun-phrase stage (a clause ending in
"stage refused", or any clause whose last word is a wrapper verb with
no article or auxiliary before it), then either admit
`BooleanError::Merge`'s label as that wrap's filed stage or drop the
label there (TOPO's `crates/topo/src/boolean/mod.rs`), and re-run the
rows above.

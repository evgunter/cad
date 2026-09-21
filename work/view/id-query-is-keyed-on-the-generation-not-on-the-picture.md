---
id: id-query-is-keyed-on-the-generation-not-on-the-picture
kind: issue
title: The id query is re-asked on a new generation, so a scene rebuilt at the same generation leaves a stale GPU answer to be compared
status: review
opened: 2026-09-15
branch: view/id-query-key
pr: 2622
priority: P1
cost: D
---


## The second producer of "the two picking paths disagree"

`IdQueryLog::step` keys its outstanding question on
`(cursor, generation)` (`crates/viewer/src/frame.rs`, `step`), and
`pane::viewport` feeds it `self.index.map(PickIndex::generation)`. So a
still cursor over an unchanged generation answers `IdStep::Hold`: no new
query, no new id pass, and `id_answer` keeps whatever the GPU wrote for
the last serial.

**The generation is not the picture.** `ViewerApp::sync_scene` rebuilds
the scene on a display-revision change and on a focus-set change as well
as on a new index, and both of those hold the generation still. Hiding a
part is the plain case: the drawn ids lose the hidden part's patches,
the ray path is handed the same `DisplayView` and answers *nothing*
there, and `idpass::disagreement` compares the ray's fresh *nothing*
against an `id_answer` the GPU wrote for the picture that still had the
part in it. That is a disagreement between two pictures, reported as a
disagreement between two picking paths — and issue #1097 §4 tells an
operator to read that sentence as an `R32Uint` clear fault.

**Not the missing co-guard, which is why it has its own file.**
`index-reads-without-the-evaluation-co-guard` is about resolving an id
through an index that did not mint it; `drawn_index` closes that one and
does nothing here, because in this case the index IS the drawn index and
it is the ANSWER that is stale.

## The shape of the fix, and what it costs

`ViewerApp::revision` bumps on exactly the event this question is about —
every successful scene rebuild — and `ViewerBehavior` already carries it,
so `step(cursor, revision)` states the question the log is actually
asking. It changes `IdQueryLog`'s stored key from
`Option<([f64; 2], Option<Generation>)>` to a `u64` one and rewrites the
rows that drive it, which is `frame`'s vocabulary rather than the
viewport's, and is why the guard unit left it here rather than widening
into it.

Check before taking it: a generation change with no scene rebuild behind
it. If one exists, `revision` alone under-asks where `generation` did
not, and the key is the pair.


## Closed

Taken by `view/id-query-key`. **The check the item asked for found one:
a generation change with no scene rebuild behind it exists**, so
`revision` alone under-asks where `generation` did not and the key is
the pair. `ViewerApp::revision` is written in exactly two places
(`app.rs`, the `1` at assembly and the `wrapping_add(1)` in
`sync_scene`'s success arm), and the generation the log is handed is
`self.picks.index()`'s — which `sync_scene` can leave newly landed over
a rebuild it refused, since the refused arm writes `scene_fault` and
nothing else. So the key is `idpass::IdSubject { revision, generation }`,
and `crates/viewer/README.md`'s *A pick id is one index's word* states
it.

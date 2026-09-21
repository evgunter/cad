---
id: the-picture-key-never-became-a-type
kind: issue
title: The (generation, delta) picture key is spelled five ways and its two cache fields have no stated invariant
status: review
opened: 2026-09-05
branch: view/picture-key
pr: 2670
priority: P1
cost: D
---


## What

Found by VIEW-6b's style review (S4, with S20 folded in — they are
one finding: both are the key failing to be a value).

VIEW-6b made `(Generation, DisplayTolerance)` the thing that decides
whether an index describes the picture on screen. It never became a
type, so "is this the same picture?" is asked five ways, in
`crates/viewer/src/pickindex.rs`, `crates/viewer/src/pickcache.rs` and
`crates/viewer/src/evalseam.rs` (three files since the seam split — the
first of the five is `PickIndex::current_for`, `pickindex.rs:773`; the
next three are `PickCache`'s, in `pickcache.rs`):

- `PickIndex::current_for(Some(generation), delta)`;
- `self.outstanding == Some(wanted)` and `self.attempted == Some(wanted)`;
- `self.attempted != Some((done.generation, done.delta))` in `land`;
- `(next.generation, next.delta) != (done.generation, done.delta)` in
  `<IndexRequest as Job>::supersedes` — `ThreadIndexer::poll` until the
  three threaded seams were folded onto one coalescing handle, which
  moved the comparison without changing it.

Each is correct today. What is missing is the one place that says what
the key IS, so a sixth site cannot be written with one half of it —
which is exactly the defect the review found and this unit fixed in the
fifth, where the comparison had been by position instead.

**And the fold that moved the fifth site added a sibling worth naming**:
`<FitRequest as Job>::supersedes` compares `(generation, requested)` by
exactly the same shape. It is NOT a sixth spelling of this key — a fit's
δ is the δ someone asked for and not the δ a picture was built at — but
it now sits one trait impl away from one that is, spelled identically,
which is the search-and-replace hazard the near-miss below is about.

**Note the near-miss**: `idpass::IdQueryLog::step` keys on
`idpass::IdSubject`, which is the scene revision and the generation
**without** δ. That is currently right — a δ change reaches it as a new
index, because `PickCache::sync` nulls the held index at the submit and
only `land` installs one, so an index cannot change δ without the key
seeing `None` in between — but it is the site most likely to be wrong if
the key ever becomes a type and this one is migrated by
search-and-replace.

## The two fields

`PickCache::outstanding` is always either `None` or exactly
`attempted`. It is a boolean wearing the key's clothes; `land` reads
one and clears the other, and no sentence anywhere states that they
move together. A key type would make the pair expressible as one value
with a state (`asked` / `answered`), which is what the two fields
actually encode.

## Cost

Cosmetic today. Filed because the unit that introduced the key also
introduced the one bug it prevents, one review round apart.

## Closed

`PictureKey` is the type — the landed generation and the δ its roots
were tessellated at, one value, private fields, `PartialEq` over the
pair, and `PictureKey::of` the only door. Every site that asks *is
this the same picture?* now holds one of these: `PickIndex::key` and
`PickIndex::current_for`, `IndexRequest`/`IndexDone`'s `key` field and
`<IndexRequest as Job>::supersedes`, `PickCache`'s one `Attempt`, and
`ViewerApp::scene_key` through `pane::viewport::drawn_index`.

The two fields collapsed. `PickCache::outstanding` was always either
`None` or exactly `attempted` — verified by reading every write to
both, of which there are four — so the pair is one value with a state,
`Attempt::Asked(key)` and `Attempt::Answered(key)`. A cache waiting on
a picture other than the one attempted is now unrepresentable rather
than merely absent.

Both named non-members stayed non-members:
`<FitRequest as Job>::supersedes` still compares
`(generation, requested)` and now says at the impl why it is not this
key; `idpass::IdQueryLog` and `idpass::IdSubject` were not touched at
all.

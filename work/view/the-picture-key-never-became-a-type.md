---
id: the-picture-key-never-became-a-type
kind: issue
title: The (generation, delta) picture key is spelled five ways and its two cache fields have no stated invariant
status: open
opened: 2026-09-05
---


## What

Found by VIEW-6b's style review (S4, with S20 folded in — they are
one finding: both are the key failing to be a value).

VIEW-6b made `(Generation, DisplayTolerance)` the thing that decides
whether an index describes the picture on screen. It never became a
type, so "is this the same picture?" is asked five ways, in
`crates/viewer/src/pick.rs` and `crates/viewer/src/evalseam.rs`:

- `PickIndex::current_for(Some(generation), delta)`;
- `self.outstanding == Some(wanted)` and `self.attempted == Some(wanted)`;
- `self.attempted != Some((done.generation, done.delta))` in `land`;
- `(next.generation, next.delta) != (done.generation, done.delta)` in
  `ThreadIndexer::poll`.

Each is correct today. What is missing is the one place that says what
the key IS, so a sixth site cannot be written with one half of it —
which is exactly the defect the review found and this unit fixed in
`ThreadIndexer::poll`, where the comparison had been by position
instead.

**Note the near-miss**: `frame::IdQueryLog::step` keys on the
generation **without** δ. That is currently right — it asks "has the
picture changed under a still cursor", and a δ change reaches it as a
new index — but it is the site most likely to be wrong if the key ever
becomes a type and this one is migrated by search-and-replace.

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

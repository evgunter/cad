---
id: index-seam-vocabulary-sits-in-the-wrong-module
kind: issue
title: evalseam and pick import each other because the index seam's trait lives beside the evaluation seam rather than beside its payload
status: open
opened: 2026-09-05
refs: [index-request-and-index-inputs-are-one-concept-twice, viewer-session-god-module-split]
---


## What

Found by VIEW-6b's style review (S3).

`crates/viewer/src/evalseam.rs` imports `crate::pick::{PickIndex,
PickIndexError}`; `crates/viewer/src/pick.rs` imports
`crate::evalseam::{Generation, IndexDone, IndexRequest, IndexService,
InlineIndexer}`. Both are vocabularies under
`crates/viewer/README.md`'s module rule, which the cycle does not
break — but it is a cycle, and the evaluation seam beside it has none:
it depends on the kernel and on nothing in this crate.

The asymmetry has a cause. **`IndexService`'s whole payload lives in
`pick`** — the request is what `PickIndex::build` takes and the answer
is what it returns. The trait is in `evalseam` only because the OTHER
seam is, and because that file is where this crate agreed to keep its
threads.

## Why it is not obviously a bug

The threads rule is real and was worth keeping: one file owning every
join handle and channel is what makes "no source change above this
boundary" checkable. Moving `IndexService` into `pick` would put a
worker thread in a module whose job is answering questions about a
tessellation.

So the choice is between the cycle, a third module for the index seam
alone, and hoisting the payload types. It is a placement question with
three defensible answers, which is why it is filed rather than fixed
inside a unit that had a ruling to build.


## The three answers costed, and the finding that none of them works
## (VIEW orchestrator, 2026-09-06 — for Ev)

Written against `origin/main`. `crates/viewer/src/pick.rs` is 2,792
lines; `evalseam.rs` is 901.

### The cycle, exactly

Two edges, and they are not the same kind of thing.

- `evalseam.rs:112` — `use crate::pick::{PickIndex, PickIndexError}`.
  This is the index seam needing its own PAYLOAD: what `build_index`
  returns and what `IndexDone` carries.
- `pick.rs:64` — `use crate::evalseam::{Generation, IndexDone,
  IndexRequest, IndexService, InlineIndexer}`.

**The second edge is two different dependencies wearing one `use`
line**, and separating them is what unlocks everything below:

1. **`Generation` alone**, read at `pick.rs:689` (a FIELD of
   `PickIndex`), `:758-765` (`current_for`, `generation`) and `:2231`
   (`IndexInputs`). It is `pub struct Generation(u64)`
   (`evalseam.rs:126`) with `FIRST`, `next` and `get`, and it depends
   on **nothing** — not the kernel, not this crate.
2. **The seam itself** — `IndexRequest`, `IndexDone`, `IndexService`,
   `InlineIndexer` — read only at `pick.rs:2296-2499`, which is
   `PickCache`.

### None of the three answers this item offered breaks the cycle

**(a) Keep it.** Costs nothing today. Module cycles are legal Rust and
the ratified module rule (`crates/viewer/README.md`, *Module
boundaries*) is about VOCABULARY vs DRIVER and says nothing about
cycles between vocabularies — so the cycle is not a violation of
anything, which is worth saying plainly: this is a new question, not an
enforcement gap. What it costs is that neither module can be read or
moved without the other, in a crate whose whole architecture programme
has been about making files readable whole.

**(b) A third module for the index seam alone.** Move `IndexRequest`,
`IndexDone`, `IndexService`, `InlineIndexer` (and `ThreadIndexer`) into
`indexseam.rs`. **This does not break the cycle — it relocates it.**
`IndexDone` carries a `PickIndex`, so `indexseam` → `pick`; `PickCache`
builds an `IndexRequest`, so `pick` → `indexseam`. It also costs the
threads rule ("one file owning every join handle and channel", which
is what makes *no source change above this boundary* checkable) unless
that rule is restated as "the seam modules own the threads". Strictly
worse than (a): same cycle, one more module, a weakened rule.

**(c) Hoist the payload types.** As stated — move the seam's request
and answer types down — it does not reach either, for the same reason:
`IndexDone` needs `PickIndex`, and `PickIndex` is in `pick`.

**So the item posed a question and offered three answers, none of which
answers it.** That is the finding, and it is why this went to Ev rather
than to a lane.

### What the cycle is actually a symptom of

**`pick.rs` holds two layers**, and the seam runs between them:

- an **index data structure** and its queries — `PatchId`, `EdgeId`,
  `IdMap`, `PartWindows`, `PickIndex`, `PickIndexError`, `EdgePick`,
  `Highlight`, `highlight`, `edge_overlay`, `focus`,
  `cursor_projection` (roughly `:78-2230`). This half needs
  `Generation` and `DisplayTolerance` and nothing else in the crate;
- a **pick policy / cache** — `IndexInputs`, `PickCache`, `CacheStep`,
  `IndexLanding`, `NotIndexed`, `unindexed` (roughly `:2230-2792`).
  This half drives the seam.

The seam sits *above* the first and *below* the second, and both halves
are in one file, so any edge to either is an edge to both. That is the
whole cycle.

### (d) The answer that does work, and it is two moves

1. **`Generation` moves to a leaf.** It is a request counter, not part
   of the evaluation seam's machinery, and it is read by six modules
   (`pick`, `app`, `lib`, `frame`, `evalseam`, `session`). It depends
   on nothing, so it can sit below everything — its own small module,
   or beside `DisplayTolerance` in `scene`, which likewise imports
   nothing from this crate.
2. **`pick.rs` splits at the layer boundary above**, the index data
   structure into its own module and the policy staying in `pick`.

The result is acyclic and each module keeps a coherent job:

    generation  ←  pickindex  ←  evalseam  ←  pick

`evalseam` keeps BOTH seams and therefore both sets of threads, so the
threads rule is untouched — which is the property (b) had to give up.

**Move 1 without move 2 is not enough**: `PickIndex` would still be in
the same file as `PickCache`, so `evalseam` → `pick` → `evalseam`
survives on the seam types. **Move 2 without move 1 is not enough
either**: `pickindex` needs `Generation`, so `pickindex` → `evalseam` →
`pickindex`. They only work together, which is the thing this write-up
exists to say.

### What it costs

Move 1 is small: one type, six import sites, no behaviour.

Move 2 is a real split of a 2,792-line file — mechanical in kind (unit
1c did exactly this to `session.rs` and `app.rs` and changed no
assertion), but it is the larger half of the work and it touches a file
`crates/viewer/tests/*` reaches into. Unit 1c's precedent says the cost
is bounded and the shape is known.

**The honest alternative is (a).** The cycle harms nothing today; it is
a legibility cost, not a defect, and this program has larger items open
against `frame.rs` (2,481 lines, eight concerns) than against this. If
the answer is "not now", the useful outcome is that this write-up
replaces the item's three non-answers so the next reader does not cost
them again.

### What I am NOT proposing

Nothing about `IndexRequest` and `IndexInputs` being one concept twice
— that is `index-request-and-index-inputs-are-one-concept-twice`, and
its own analysis says the two types are **not** redundant, because
`IndexRequest` owns its copies so the worker holds nothing borrowed
from a session. The cheap answer there (state the relationship at each
type) is independent of this and can be taken by any lane.

### The question for Ev

Take (d), or take (a) and close this as answered-not-fixed? I lean (d)
if the split lands as its own unit rather than riding another, and (a)
if the answer is that VIEW has better things to do — but the one thing
that should not survive is the item's current text, which offers three
answers that do not work.


## RULED: (d) (Ev, PR 2076, 2026-09-06)

> (d) sounds good!

So the unit is the two moves, and they land together because neither
breaks the cycle alone:

1. **`Generation` to a leaf** — its own module, or beside
   `DisplayTolerance` in `scene`; the lane picks and argues.
2. **`pick.rs` splits at the layer boundary** — the index data
   structure and its queries out, the policy (`PickCache`,
   `IndexInputs`, `CacheStep`, `IndexLanding`, `NotIndexed`,
   `unindexed`) staying in `pick`.

Target shape: `generation ← pickindex ← evalseam ← pick`, acyclic, with
`evalseam` keeping BOTH seams and therefore both sets of threads.

**It is a move, so no assertion changes.** Unit 1c is the precedent and
the standard — it split `session.rs` and `app.rs` with no test file
touched and no assertion changed, and
`docs/prompts/implementer-discipline.md` §3 forbids a behaviour change
smuggled through a mechanical one. A `pub use` shim left behind is what
`session-shims-and-test-imports` is still open about, so this unit
re-points its callers rather than leaving two spellings of every moved
path.

`crates/viewer/tests/*` is VIEW's territory now (Ev, in-chat,
2026-09-04), so the test-side re-pointing is this unit's to do rather
than announce.

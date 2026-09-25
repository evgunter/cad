---
id: folded-moved-true-arm-covers-a-fold-that-did-not-move
kind: issue
title: frame::folded_moved answers true for a fold that refused and did not move, under a name and a doc line that say only moved
status: open
opened: 2026-09-19
priority: P1
cost: E
---

Found by the sweep `is-instance-collapses-absent-and-wrong-kind`
carried, which had `frame::folded_moved` on its list of four
unadjudicated siblings. It is **not** that row's class — the type is
not hiding news a reader wants — so it is filed rather than fixed
there, and `frame.rs` was off limits to that lane besides.

## The mismatch

```rust
/// Whether a folded event stream actually moved the camera.
pub fn folded_moved(folded: &Folded) -> bool {
    !folded.applied.is_empty() || folded.refused.is_some()
}
```

`crates/viewer/src/frame.rs`, `folded_moved` (~`:2096`). The `true` arm
covers two states: a fold that applied camera operations, and a fold
that applied none and carries a refusal. **The second did not move the
camera.**

**The collapse is deliberate and asserted**, which is why this is a
prose row and not a behaviour one.
`a_refused_fold_is_news_about_the_camera_and_apply_overwrites_with_it`
(`frame.rs`, the module's own tests) asserts `folded_moved` of a
refused fold with the reason inline — *"a refusal is a camera event
too"* — and `frame_policy.rs`'s `folded_moved` row pins both arms the
same way. The module doc states the question correctly too
(`frame.rs:25`, *"What a folded event stream amounts to"*). So the
value is right and two of the three places that describe it are right.

**What is wrong is the name and the function's own first line**, which
both say *moved* over an arm that did not move. The one production
caller (`pane::viewport`, the `land` guard) wants both states, because
`land` both applies the move and delivers the refusal, and the doc
comment says the guard exists to keep *"`land` is documented as the one
place a camera MOVE becomes application state"* true. A predicate whose
`true` arm includes a fold that did not move is a weaker guarantee than
that sentence claims, so the next reader building on it gets the weaker
thing without being told.

## The fix

Re-word the first doc line to the question the value actually answers —
*whether a folded stream is a camera event at all* — and rename if the
lane taking it judges the rename worth the churn (three production
sites plus two test files). VNEWS's own rule is that a rename nobody
reads is not news, so the doc line is the part that has to move and the
name is the part that is arguable.

## Home

VNEWS's: `crates/viewer/src/frame.rs`. It takes with whichever
`frame.rs` lane is next in the serialized cluster — this is a doc
comment and possibly a name, not a build of its own.

## Adjudicated 2026-09-20 (`vnews/frame-cluster-order`)

### The premise holds and the citation is exact

`folded_moved` is `crates/viewer/src/frame.rs:2096`, the body is
`!folded.applied.is_empty() || folded.refused.is_some()` at `:2097`,
and the first doc line — *"Whether a folded event stream actually moved
the camera."* — is `:2082`. The rest of that doc comment
(`:2084-2095`) is the *"what this buys is a statement, not a fix"*
argument, which is true and stays. The `land` guard is
`crates/viewer/src/pane/viewport.rs`, and the two assertions the row
names are `crates/viewer/src/frame.rs:2353` (*"a refusal is a camera
event too"*) and `crates/viewer/tests/frame_policy.rs`'s `folded_moved`
row.

### The rename: struck on 2026-09-20, RESTORED the same day

This section first declined the rename, citing the Charter's *"a rename
nobody reads is not news"*. **That was a misreading and the decline is
withdrawn.** `work/vnews/plan.md` says of that clause, three sections
from where it is stated, that it *"is a scoping test — does this row
belong to a NEWS program at all"* and that *"the consequence is about
ORDER, not merit"*. Using it to delete work is the one thing it does
not do. Recorded rather than silently reversed, because an
adjudication that strikes a deliverable with a rule that does not
license striking is exactly the shape this unit was convened to catch.

**On the merits, re-derived.** The two questions are different and both
are real:

- *Does an end user see it?* No. `folded_moved` is crate-internal with
  one production caller.
- *Does a reader of the code pay?* **Yes**, and that is the cost the
  row is about. The name asserts *moved*; the `true` arm includes a
  fold that moved nothing. `pane/viewport.rs:439-442` guards `land` on
  it under a comment saying `land` is *"the one place a camera MOVE
  becomes application state"* — so the next person building on that
  guard gets a weaker guarantee than the name states, and the doc-line
  repair alone leaves the name still saying it. A false name is read
  more often than the doc line under it.

**The churn, re-derived — the row's own figure is short.** It says
*"three production sites plus two test files"*. The actual population
is **nine sites in four files**:

| site | what |
|---|---|
| `crates/viewer/src/frame.rs:2096` | the declaration |
| `crates/viewer/src/frame.rs:25` | the module header naming it |
| `crates/viewer/src/pane/viewport.rs:442` | the one production call |
| `crates/viewer/src/pane/viewport.rs:439` | the comment naming it |
| `crates/viewer/src/frame.rs:2163`, `:2353` | the module's own rows |
| `crates/viewer/tests/frame_policy.rs:1379`, `:1390` | the policy rows |
| `crates/viewer/README.md:674` | the `frame` charter row names it |

**That last one is the cost the row did not price.** `README.md` is
VDOC's by `work/vnews/program.md`'s `keep_out`, which says a defect
there *"is filed on vdoc and never fixed across that fence"* — and a
rename does not falsify that sentence by accident, it is the lane's own
diff doing it. `work/vnews/plan.md`'s dispatch rules already adjudicate
this exact collision and give the carve-out the win: the named
`keep_out` beats the general *your-own-diff-falsified-it* rule. So a
rename **files a VDOC row for `README.md:674` and announces**.

### Disposition: a rider for the doc line, a judgement for the name

The two halves separate cleanly and should not be forced together:

1. **The doc line** (`frame.rs:2082`) — a rider on group A's carrier,
   `work/vnews/frame-rs-says-the-per-subject-line-is-a-question-for-ev`.
   `frame.rs` only, no other file touched. This is what `rides_with:`
   names.
2. **The rename** — restored to the deliverable and left to the lane,
   as the row always said. It widens past `frame.rs` to
   `pane/viewport.rs` (VNEWS's and VGEOM's and VSEAM's: announce) and
   to a VDOC row for `README.md:674`. **If the lane takes it, this row
   stops being a pure rider** and group A's shape changes; that is a
   consequence to weigh, not a reason to strike it.

A candidate the row does not name, since the first doc line has to be
rewritten either way: `folded_is_a_camera_event`, which is the question
`frame.rs:25`'s module header already asks (*"what a folded event
stream amounts to"*).

### No Ev gate

`folded_moved` is not named in `crates/viewer/GUI-DESIGN.md`, and the
sentence being repaired is a doc comment in this program's own file.
The ruling and its searches are in
`work/vnews/rank-one-discards-the-frames-other-news`, the canonical
home; it is cited here rather than restated.

## The doc line landed; the rename did not, 2026-09-24 (`vnews/frame-rs-prose-pass`)

**The doc line.** `folded_moved`'s first line now asks the question the
value answers — *whether a folded event stream is a camera event at
all: it applied a camera operation, or it refused one* — says a refused
fold moved nothing and still answers `true` because `land` both applies
a move and delivers a refusal, and says outright that the name is
narrower than the value. The *"what this buys"* paragraph's *"calling
it on frames where nothing moved"* became *"with no camera event"*, the
only reading under which the guard keeps the sentence it protects.

**One more sibling, fixed in the same pass:** the assertion message in
`frame.rs`'s `a_clean_fold_keeps_a_message_it_did_not_write` said *"a
fold that moved nothing never reaches the line at all"* — false for a
refused fold, which moved nothing and reaches the line. It now says *"a
fold that is no camera event"*.

**The rename was not taken, and this row stays open on it.** The lane's
judgement: the rename is worth doing on the merits the adjudication
gives — the name is read more than the doc line, and it now sits over
a doc line that says the name is too narrow — but it is not a prose
pass. It reaches `pane/viewport.rs` (VNEWS's, VGEOM's and VSEAM's),
`crates/viewer/tests/frame_policy.rs`'s two `folded_moved` rows and
`crates/viewer/README.md`'s `frame` charter row (both VDOC's), where a
serialized one-file doc slot reaches one file. So it is left as its own
unit rather than widened into this one.

**The rename's population has one prose member the table above does
not list as prose:** `pane/viewport.rs`'s comment over the `land`
guard (~`:436-442` at this base) says *"Landed only when the fold
actually MOVED something"* — the same claim the doc line made, over the
same call. It is fixed with the rename, or re-worded on its own by
whichever lane next holds that file; it is not `frame.rs`'s, so this
pass left it.

The carrier this rode on is closed, so `rides_with:` is removed.

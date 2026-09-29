---
id: ranked-and-unranked-verdicts-are-one-type
kind: issue
title: A ranked verdict and a policy's verdict are the same type, so which door a call site must use is carried only by prose
status: closed
opened: 2026-09-06
refs: [2026]
priority: P1
cost: M
branch: vnews/a-ranked-verdict-is-its-own-type
closed: 2026-09-28
pr: 3391
---

Found by #2026's style review, on the unit that created the second
door.

## What this is

`crates/viewer/src/frame.rs` has two doors onto the status line and the
rule for choosing between them is written down in three doc comments
and in nothing else:

- `frame::apply(status, update)` — the verdict goes to the field.
- `frame::deliver(notices, status, update)` — a `Show` joins the
  frame's notices and meets `frame_status`'s ranking; the other three
  arms go to the field.

**Both take `StatusUpdate` and both are correct for some callers**, so
picking the wrong one compiles, passes every row, and reintroduces the
defect the sweep was for. `crates/viewer/src/pane/viewport.rs` has both,
ten lines apart, in one file:

```rust
// :46, in `land`
frame::deliver(notices, status, frame::fold_status(folded));
// :178, in the id pass
frame::apply(self.status, frame::cursor_status(step));
```

Nothing at either call site says which is which. The rule is *"can this
policy ever answer `Show`?"* — a fact about the callee's arms, invisible
from here, enforced by no type. Give `frame::cursor_status` a `Show` arm
one day (the news vocabulary has been extended twice already) and
`:178` silently becomes a writer that puts a sentence on the line the
ranking never saw. There is no diff at which that looks wrong: the
diff that breaks it is in `cursor_status`, and the site that breaks is
in another file and unchanged.

That is `status-line-writers-bypass-the-ranking`'s own defect class,
moved up one level. That item swept eighteen sites where a writer could
skip the ranking; the sweep's own fix leaves ONE site where a writer can
skip the ranking, and it is spelled the same as the site that must not.

## The asymmetry is real, so "one door" is not free

The two doors are not redundant. `app::ViewerApp::perform_batch`
(`crates/viewer/src/app.rs`) computes `frame::frame_status(...)` — the
RANKED answer, a `Show` that has already been weighed against
everything the frame said — and hands it to `apply_status`. That `Show`
must reach the field. Routed through `deliver` it would be pushed back
onto `notices` to be ranked a second time, against the very list it was
the winner of.

So there genuinely are two kinds of `Show` here:

1. **A policy's `Show`** — one candidate sentence, must be ranked.
2. **The ranking's `Show`** — the frame's answer, must not be.

## The proposal (reviewer's, #2026)

**Make them different types.** A ranked verdict arrives as something
other than `StatusUpdate` — the natural shape is for `frame_status` to
return a distinct `LineVerdict` (or for `StatusUpdate` to lose its
ranked role entirely and the field-writing door to take the new type)
— at which point:

- one door suffices for policies, and it is `deliver`;
- `apply` becomes private to `frame`, or takes the ranked type;
- the choice a call site now makes in prose is made by the compiler,
  and a `Show` arm added to `cursor_status` fails to build at
  `viewport.rs:178` instead of silently changing behaviour.

## What has to be decided, and by whom

- Whether the ranked type is a new enum or a newtype over `Message` —
  `frame_status` can only answer `Keep`, `Clear` or `Show` today
  (`Expire` is a policy's answer and never the ranking's), so the
  ranked type is a strictly smaller vocabulary and that is worth
  stating in it.
- What happens to `apply`'s public surface: it is called from
  `app::ViewerApp::apply_status` and from `pane::viewport`'s cursor
  path, and the second of those is exactly the site this item is about.
- Whether `frame::deliver`'s `Clear` arm survives the change. **No
  policy that reaches `deliver` answers `Clear` at all now**:
  `fold_status` is the only one that does reach it — `deliver`'s single
  call site is `pane::viewport`'s `land` — and it answers `Show` or
  `Expire`; `dialog_status`, the other policy of that shape, is deleted
  (the ruling
  `was-the-status-route-supposed-to-fire-for-an-absent-chooser`).
  `Clear` has exactly one production CONSTRUCTION, `batch_status`'s
  `(true, None)` arm (`frame.rs:474`) — `frame_status` answers `Clear`
  too, but only by forwarding that verdict unchanged when
  `notices.is_empty()` (`frame.rs:521-523`), which is the ranking's
  output and reaches the field through `apply_status`, never through
  this door. `deliver`'s own header argues the arm stays
  regardless — a wildcard there would route a variant added later to
  the field by default — and that argument is about the compiler
  carrying the rule, not about the arm having a producer.

Sequence after `frame-module-has-eight-concerns-and-no-holds-row`'s
split question is answered, or independently of it — the two touch the
same file but not the same argument.

## Adjudicated 2026-09-20 (`vnews/frame-cluster-order`)

### The sequencing gate has fired

`work/view/frame-module-has-eight-concerns-and-no-holds-row` is
**closed**, and so are the three rows this one refs or argues from:
`status-line-writers-bypass-the-ranking`,
`camera-fold-clears-status-line` and
`was-the-status-route-supposed-to-fire-for-an-absent-chooser`. Nothing
sequences this row any more.

### Citations re-derived against the tree

The claims hold; every number in the body has moved.

| the body says | the tree today |
|---|---|
| `frame::apply(status, update)` | `crates/viewer/src/frame.rs:542` |
| `frame::deliver(notices, status, update)` | `crates/viewer/src/frame.rs:512` |
| `viewport.rs:46`, in `land` | `crates/viewer/src/pane/viewport.rs:93` |
| `viewport.rs:178`, the id pass | `crates/viewer/src/pane/viewport.rs:473` |
| `batch_status`'s `(true, None)` arm, `frame.rs:474` | `crates/viewer/src/frame.rs:586` |
| the forward when `notices.is_empty()`, `frame.rs:521-523` | `crates/viewer/src/frame.rs:667` |

**One claim in the body is now false as stated.** The two doors are not
*"ten lines apart, in one file"*: they are **380 lines apart** in
`pane/viewport.rs`, at `:93` and `:473`. The finding is unaffected and
arguably sharpened — a reader who could see both doors in one screen at
least had the comparison in front of them.

**Two facts re-checked because the row rests on them, and both hold.**
`frame::deliver` has exactly one production caller,
`pane/viewport.rs:93`; `frame::fold_status`
(`crates/viewer/src/frame.rs:1125-1133`) answers `Show` or
`Expire(Subject::Camera)` and never `Clear`, so the row's
*"no policy that reaches `deliver` answers `Clear`"* is still true.
`frame::apply` has two production callers outside the module —
`crates/viewer/src/app.rs:1342` and `crates/viewer/src/pane/viewport.rs:473`
— so *"`apply` becomes private to `frame`"* is not available without
the second half of the row's own proposal (`apply` taking the ranked
type). `crates/viewer/src/pane/viewport.rs:959` is a third `apply` site
and is inside `#[cfg(test)]` (`:828`), so it is not a production door.

### This row's fix deletes `a-fold-row-composes-a-producer-with-a-dead-door`'s instance

That row names two dead compositions in `frame.rs`'s test module, both
`apply(status, fold_status(refused))`
(`crates/viewer/src/frame.rs:2188` and `:2370`). Under the proposal
above — `apply` taking a ranked type rather than a policy's
`StatusUpdate` — **both stop compiling**, because `fold_status` returns
the policy type. So the sibling row is not a question this program has
to answer ahead of this one, and it must not be spent as a serialized
`frame.rs` lane before it: taking this row first removes its subject.

### No Ev gate

The doors, the type and the ranking they feed are stated in
`frame.rs`'s doc comments and in `crates/viewer/README.md`, which the
`docs/DESIGN.md:33` companion row calls *"the implementation record,
which the program maintains itself"*. `crates/viewer/GUI-DESIGN.md`
says nothing about either door. The finding and its searches live once,
in `work/vnews/rank-one-discards-the-frames-other-news`'s adjudication
section; this row cites it rather than restating it.


## Closed 2026-09-28 (`vnews/a-ranked-verdict-is-its-own-type`)

Citations re-derived against main after #3235 before building. The
table in the adjudication had rotted again, but every claim held.

**The fix.** `frame::frame_status` answers a new enum,
`frame::RankedVerdict { Keep, Clear, Show(Message) }`, and
`frame::apply` takes that type and nothing else. `frame::StatusUpdate`
is now only a policy's verdict, `{ Keep, Expire(Subject), Show(Message) }`,
and `frame::deliver` is the only door that takes it.
`pane::viewport`'s id pass now calls
`frame::deliver(self.notices, self.status, frame::cursor_status(step))`.
With the old `frame::apply(self.status, frame::cursor_status(step))`
spelling, the crate fails to build with E0308. A `compile_fail,E0308`
doctest on `frame::deliver`, paired with a twin that does compile,
holds that fact.

**The three sub-questions, decided.**

1. *New enum or newtype over `Message`:* a new enum. The ranking
   answers `Keep` and `Clear` as well as `Show`, so a newtype over
   `Message` cannot carry its answer. The two vocabularies also differ
   in both directions, and each type states that by what it lacks:
   `Clear` belongs only to the ranking, `Expire` only to a policy.
2. *`apply`'s public surface:* it stays `pub` and takes `RankedVerdict`.
   `app::ViewerApp::apply_status` (production) and two test sites need
   it, so it cannot be made private. The cursor path, which was the
   site this row was about, no longer reaches it.
3. *Does `deliver`'s `Clear` arm survive:* no. The variant is gone
   from `StatusUpdate`. Its one construction was `batch_status`'s
   `(true, None)` arm, and that is the ranking's vocabulary, so it
   moved to `RankedVerdict`. `deliver`'s header argument (no wildcard)
   still holds for the arms that remain.

**Something the row did not name: `batch_status` is now private.** It
was `pub` and returned the same type as `frame_status`. On either type
it would have been a wrong door that compiles. As a `StatusUpdate` it
would need an unreachable `Expire` arm in `frame_status`. As a public
`RankedVerdict`, `apply(status, batch_status(ops, refusal))` skips every
notice the frame produced, which is this row's defect one level down.
Its only production caller was `frame_status`. The integration rows
that called it now call `frame_status(&[], ops, refusal)`, which
returns the same answer. `frame_policy.rs`'s loop asserting
`frame_status(&[], ..) == batch_status(..)` became a tautology and was
deleted. The empty-batch case it covered was moved into
`a_hover_only_batch_leaves_the_status_line_alone`.

**`a-fold-row-composes-a-producer-with-a-dead-door` and its rider
were already closed** (2026-09-20 and 2026-09-24). What this unit
does to their subject: both dead compositions no longer compile, and
their two rows were rewritten. `a_clean_fold_retires_the_camera_refusal_it_did_write`
now runs the live order (`deliver`, `frame_status`, `apply`,
`deliver`), which makes the rider's disclosure paragraph moot, so it
was removed. `…_and_apply_overwrites_with_it` became
`a_refused_fold_is_news_about_the_camera`. The residue that stays open
is VDOC's crate-wide rule row,
`work/vdoc/a-fixture-may-compose-a-dead-door-and-nothing-says-when`.
It got an evidence section saying its instances are now zero.
`folded-moved-true-arm-covers-a-fold-that-did-not-move`'s citation of
the renamed row was fixed in place.

**Residue, filed:**
`work/vnews/the-status-field-is-lent-bare-so-a-pane-can-write-around-both-doors`.
The types close the wrong-door mistake but not the field.
`ViewerBehavior` still lends the panes a bare `Option<Message>`, and
`RankedVerdict`'s variants are public. So a pane that assigns the field,
or builds its own `RankedVerdict::Show` and hands it to `apply`, still
bypasses the ranking. No production site does either today, so that
row is P3: a missing guarantee with no live site.

**Why the two verdicts stay two types (for the next sweep).**
`StatusUpdate { Keep, Expire, Show }` and
`RankedVerdict { Keep, Clear, Show }` are two three-state enums one
hop apart, and they share `Keep` and `Show`. They stay separate
because the shared `Show` means opposite things. In a `StatusUpdate`
it is a candidate that must be ranked; in a `RankedVerdict` it is the
winner, which must not be ranked again. The type is what decides which
door a verdict takes. Merging the two enums, or giving them a shared
core, would put both meanings back behind one spelling, which is the
defect this row closed. `RankedVerdict`'s own doc states the same
argument.

**What the cursor path gains, stated exactly.** `cursor_status` has
never had a `Show` arm, so moving the id pass to `deliver` changes
nothing a reader sees today. What it changes is what a future `Show`
arm would do there: it joins the notices, and the site is not left
writing a sentence the ranking never saw.

**Where I disagree with the row.** The row says *"one door suffices
for policies, and it is `deliver`"*. That is true. Its framing of
`apply` as possibly *"private to `frame`"* was never available: the
ranked answer is computed in `frame` but applied in `app`. The
adjudication already said this. Separately, the row's *"Clear has
exactly one production CONSTRUCTION"* was the decisive fact, and it
settles the third sub-question outright rather than leaving it to
`deliver`'s header argument.

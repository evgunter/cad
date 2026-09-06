---
id: status-line-writers-bypass-the-ranking
kind: issue
title: Eighteen writers reach the status line without the frame's ranking, so it decides nothing they say
status: closed
opened: 2026-09-04
closed: 2026-09-06
pr: 2026
refs: [camera-fold-clears-status-line, the-news-vocabulary-has-no-expiry, stale-file-citations-after-the-split, startup-notices-need-holding-to-badge]
---

## What this is

`frame::frame_status` ranks a frame's news — refusal, then every notice
the frame produced, then the batch's own verdict — and `frame::apply`
is the one door a `StatusUpdate` becomes the field through. **Nineteen
writers reach the field without asking the ranking.**

Each is the shape `camera-fold-clears-status-line` fixed at one site: a
writer with no way to say "I have nothing to add", whose sentence is
then kept or lost by whichever writer in the frame happens to run last.
`perform_batch` runs AFTER the panes have drawn
(`crates/viewer/src/app.rs`, the frame loop), so a message a pane wrote
this frame is erased by that frame's `StatusUpdate::Clear` before it is
ever painted — the defect `frame_status` was extracted to stop for tool
notices, still live for everything else.

## The arithmetic

`grep -rn "status = " crates/viewer/src/` matched 22 lines at this
branch's merge base. Two are `apply_status`'s own arms, which are the
door and not writers, leaving **20**. This branch removed two:
`app.rs`'s product-fault assignment, which became a badge, and `land`'s
own unconditional assignment, which became `frame::apply` +
`frame::fold_status`. **18 matched writers remain**, plus one the grep
could not match (below), for **19**.

**What the pattern cannot match**, restated from what it missed: a
**struct-literal field initializer** (`status:` with a colon, not
`status =`) — that is how `app.rs:580` was missed; a writer that
mutates the line through a helper; a `*self.status = …` split across
lines by rustfmt; and any writer reaching the field under another
binding.

## The hit list, classified

**News** — belongs on the line, but must reach it through
`frame_status`'s ranking:

- `crates/viewer/src/app.rs:697` — a δ that `DisplayTolerance` refused.
- `crates/viewer/src/app.rs:860` — the preferences store could not be
  written. The outcome of an act.
- `crates/viewer/src/pane/view.rs:93` — the δ field's text is not a
  number.
- `crates/viewer/src/pane/create.rs:162,167` — the mate tool's refusal,
  and its no-landed-evaluation arm.
- `crates/viewer/src/pane/create.rs:375` — add datum refused.
- `crates/viewer/src/pane/create.rs:600,601` — add profile: no frame
  picked, and the placement refusal.
- `crates/viewer/src/pane/create.rs:808` — extrude refused.
- `crates/viewer/src/pane/create.rs:1139,1171,1176,1217` — the blend
  tool's event wording, its two refusal arms, and the seated tools'
  shared one.
- `crates/viewer/src/pane/viewport.rs:172` — a cursor action the pick
  index refused.
- `crates/viewer/src/pane/viewport.rs:363` — the two picking paths
  disagree. Already a frame product (`frame::disagreement`); only its
  delivery bypasses the ranking.

**Standing facts** — reads of held state, so per
`crates/viewer/src/frame.rs`'s header they want a badge, not the line:

- **Landed** (`news-and-standing-facts-are-orthogonal-axes`): the
  pick-index refusal, the scene refusal and the projection refusal are
  `frame::index_badge`, `frame::scene_badge` and
  `frame::projection_badge`, and their three assignment sites are
  gone. Three fewer writers for this sweep.
- `crates/viewer/src/app.rs:580` — **the writer the grep missed**, and
  the one standing fact still on the line. The startup preferences
  notices, written as a struct-literal initializer. "Your preferences
  file has a key I do not understand" is a standing fact about the
  file; it sits on the news line, where the first acting batch of the
  session silently deletes it. Under the ruled rule it is a read of
  held state — of the file as it stands — so it badges, which means
  the notices have to be HELD rather than rendered once into the
  field.

**The sort test is settled, and it is PROVENANCE** (Ev, 2026-09-06, on
`unindexed-refusal-is-an-outcome-not-a-read`). A writer is sorted by
**what caused its sentence to exist** — an act puts it on the line —
and not by what the sentence is about. The two axes disagree on every
refusal that reports a seam, and provenance won because it is the only
one a reader can see: it decides whether the sentence exists on a frame
where nobody acted.

Two entries above are that class and **stay news** under it:
`frame::unindexed_refusal`, which `pick::unindexed` raises for a
`Select` and for nothing else; and `crates/viewer/src/pane/viewport.rs`'s
cursor action the pick index refused. Under the rival test both would
have moved, and with them the whole class *"a refusal about a seam"* —
the largest coherent group on this news list after the tool refusals.
Re-sorting the list against the settled test is this unit's work.

**A policy that reaches the field without the ranking**, which is
this item's own module being one of its subjects:

- `frame::fold_status`, applied at `crates/viewer/src/pane/viewport.rs`'s
  `land`. It answers in the right vocabulary and goes through
  `frame::apply`, so it is not an assignment — but it does not go
  through `frame_status`, so a camera refusal raised in a frame that
  also carries a clean acting op is overwritten by that batch's
  `Clear` before it is painted. Fixing it is the same work as the rest
  of this item and not a separate change.

## Why it was not swept with the fold

`crates/viewer/src/pane/` is CHROME-adjacent: CHROME's items cite
`crates/viewer/src/app.rs` (e.g. `work/chrome/drag-tick-has-three-homes.md`),
and the 1c split moved that code into `pane/*`, so CHROME owns items
over code now living in these files even though its citations still
name the old path (`stale-file-citations-after-the-split`). A
nineteen-site sweep landing there is a merge conflict bought for
nothing.

## What a fix looks like

Each news site pushes onto the frame's `notices` rather than assigning
— the door `Tools::reconcile` and `Tools::feed` already use, which puts
it in `frame_status`'s rank 2 and stops the same frame's batch verdict
from erasing it. **`pane` modules cannot reach `notices` today**:
`ViewerBehavior` carries `status` and not `notices`, so that field
moves with the sweep. Each standing fact gets a badge function in
`frame` beside `product_badge` and a toolbar read beside the existing
badges — see `four-badges-five-spellings` for what that family should
look like before four more members are added to it. The pane sites are
the bulk and are independent of each other, so this splits cleanly.

## Closed

Every sentence the line can hold now comes through the ranking.
Sixteen assignments and `frame::fold_status` push onto the frame's
`notices` instead of writing the field, so they meet
`frame_status`'s rank 2 and the same frame's accepted batch can no
longer erase them before they are painted. `ViewerBehavior` carries
`notices`, which is the one structural change; it also keeps `status`,
because a RETIREMENT is the one thing a notice cannot express.

`frame::deliver` is the new door for a policy that may or may not have
something to say: a `Show` joins the notices, a `Keep`/`Expire`/`Clear`
reaches the field. `fold_status` needed exactly that — its refusal is
news and its clean arm retires the camera sentence — and
`ViewerApp::deliver_status` is the `&mut self` shorthand beside
`apply_status`, which stays for the RANKED verdict.

## The census, re-derived — and it is eighteen, not twenty

The number has now been wrong four times, and the reason is the same
each time: the membership test was "reaches the field outside the
ranking" when it should have been **"can put a SENTENCE on the line
that the ranking never saw"**. Applying a verdict outside the ranking
is not the same as writing one, and a retirement must not be ranked.

The eighteen: sixteen direct assignments (`app.rs` ×2, `pane/create.rs`
×10, `pane/view.rs` ×1, `pane/viewport.rs` ×3), one struct-literal
initializer (`app.rs`'s `status: frame::startup_notices(...)`, the site
the `status = ` grep could not see), and `frame::fold_status`, whose
`Show` arm reached the field through `apply`.

**Two the previous counts included and should not have:**

- `frame::cursor_status` returns only `Keep` or `Expire`. It can never
  put a sentence on the line, so it was never one of these writers —
  it is the well-behaved shape the item is asking every writer to
  become. The README counted it as one of "two more".
- `frame::dialog_status` has a `Show` arm, and it is **unreachable at
  both call sites**: the Open… and Save As… buttons are
  `add_enabled(chooser.usable(), …)`, and the `Show` arm is
  `(chose: false, usable: false)`. A click implies usable. It is
  latent, not live; it now goes through `deliver` anyway, so if the
  guard is ever removed the sentence lands in the notices.

`crates/viewer/README.md` and `frame.rs`'s header are corrected to
eighteen with the distinction stated, so the next reader inherits the
test rather than the number.

## What the pattern could not match

The census was derived by shape over `crates/viewer/src/**`: assignment
to any binding ending in `status` (`X.status =`, `*self.status =`,
bare `status =`), struct-literal `status:`, every `frame::apply(` and
`apply_status(` call, and `status.take()/replace()/insert()`. What it
still cannot see: a writer that reaches the field through a helper
taking `&mut Option<Message>` under another parameter name (only
`land` does today, and it is in the list by hand); a field reached
through a `struct` update syntax (`..other`); and — the blind spot that
matters — **reachability**, which is why `dialog_status` needed reading
rather than counting. A grep can find every site that CAN write; only
reading says which ones can be reached.

## Not taken, and filed

`app.rs`'s startup notices badge under the rule but cannot badge
without being HELD, and what retires them is a design question:
`startup-notices-need-holding-to-badge`.

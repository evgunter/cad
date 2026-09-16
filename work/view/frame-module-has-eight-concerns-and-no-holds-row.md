---
id: frame-module-has-eight-concerns-and-no-holds-row
kind: issue
title: frame.rs holds eight concerns, its header describes three, and it has no Holds row in the viewer README
status: closed
opened: 2026-09-04
refs: [opoutcome-superseded-has-no-production-reader, four-badges-five-spellings]
closed: 2026-09-16
---

Found by the VIEW-6 review (2026-09-04), which added the ninth
concern's worth of surface and had no rule to write it against.

## What is in there

`crates/viewer/src/frame.rs` is **2,536 lines** on this tree (`wc -l`;
the growth ledger below is where the 984 it was filed at lives) and its
public surface covers eight unrelated things:

1. the status-line vocabulary and its ranking — `StatusUpdate`,
   `apply`, `acts`, `batch_status`, `frame_status`, `fold_status`,
   `NOTICE_SEPARATOR`, `Withdrawal`;
2. the toolbar badge for the landed product — `product_badge`;
3. draft/offer chrome — `creation_offer`, `retype_draft`;
4. the file-chooser backend probe — `ChooserBackend`,
   `chooser_backend`, `chooser_backend_of`, `NO_CHOOSER_BACKEND`;
5. the XDG preferences path — `prefs_path`, `prefs_path_in`;
6. WSL detection — `running_under_wsl`;
7. camera-fold bookkeeping — `folded_moved`;
8. the id pass and the picking disagreement — `IdStep`, `IdQueryLog`,
   `Disagreement`, `disagreement`.

**The module header describes the first, the seventh and the eighth**,
and states the module's charter as "three decisions that used to sit
inside `app`". Five of the eight are not covered by that sentence at
all. A reader deciding whether a new pure policy belongs here has the
charter to go on, and the charter is wrong about the file.

## The second half, which is why this is filable and not a taste note

**`frame` is the one viewer vocabulary with no `Holds` row in
`crates/viewer/README.md`.** That file has two module tables — the
session's vocabularies (`session::select`, `refuse`, `op`, `author`,
`delete`, `probe`) and the app's (`forms`, `drafts`) — plus the driver
roster, which is explicitly *the* roster and is enforced by
`viewer-module-kinds.sh`. `frame` appears in the README only in prose,
as "a vocabulary built unconditionally".

So there is no field a new concern must be written into, and nothing
notices when one is added. That is how eight accumulated, and it is
what makes "does this belong in `frame`" unanswerable rather than
merely unanswered.

## What a fix looks like

Two moves, and the second is cheap and independent:

- **Give `frame` a `Holds` row** in the app's vocabularies table,
  written the way `forms`' row is — with the argument for what the
  module is for, not a list. The list is what has to be justified
  against it.
- **Then split what the row cannot honestly cover.** The status
  vocabulary is one thing and the environment probes (chooser, XDG,
  WSL) are plainly another; `IdQueryLog`/`Disagreement` are the
  picking seam's bookkeeping and read as a third. The header's own
  argument — "each is a pure function or a small value with typed
  steps, and the frame loop no longer decides what they mean" —
  justifies extraction from `app`, not co-location with each other.

Not urgent, and deliberately not scheduled against a lane that is
touching the file: the row comes first, because a split with no rule
to split on is a rename.

## The file grew again, and the row still does not exist (#1886, 2026-09-05)

`frame.rs` was 984 lines when this item was written. At #1886's head it
is **1,131** — the unit added `dropped_hide_notice` and `render_causes`
beside `supersession_notice`, and its PR body does not mention this
item.

**The reviewer's independent Q8 read, which is the useful part:** this
extended concern 1 (the status-line vocabulary), it did **not** add a
ninth. So the file is not more miscellaneous than it was. What it did
was turn a single function into a three-function sub-family — which is
precisely the accumulation this item names, arriving in the concern
that was already the largest, with no `Holds` row to write it against
and no diff at which it looked unreasonable.

That is the item's own thesis reproducing itself one unit later, and it
is the argument for taking the cheap half — **the `Holds` row in
`crates/viewer/README.md`, which has Holds tables at `:319` and
`:336`** — before another lane touches this file.

## It grew again, and the prose section is the wrong shape (#1933, 2026-09-05)

`frame.rs` was 984 lines when this item was written, 1,131 at #1886's
head, and **2,037** at `view/news-and-badges`'s. That unit built
the two ratified vocabularies (`Subject`, `Message`,
`StatusUpdate::Expire`, `Withdrawal`, `Badge`/`Tone`/`Affordance`, the
four badge constructors and the ten subject-assigning doors), so the
growth is the concern that was already largest growing again — this
item's thesis reproducing itself a second unit running.

**The unit answered the README obligation in the shape this item names
as the problem.** `crates/viewer/README.md` gained a prose section,
*"The status line's two lifetimes"*, placed beneath the vocabulary
tables rather than as a `Holds` row in one. Prose beneath a table is
exactly how a module's concerns accumulate without any diff at which
the accumulation looks unreasonable: there is no row to make longer, so
nothing counts.

The cheap half asked for *"before another lane touches this file"* has
now been skipped twice. Whether to take the `Holds` row now — and
whether that section collapses into it or stays beside it — is the
orchestrator's call, not a lane's.

## The row is taken (#1957, 2026-09-05)

`crates/viewer/README.md`'s app-vocabularies table now has a `frame`
row, written the way `forms`' row is: the argument for what the module
is for, and an explicit statement that **the charter justifies taking
each of these out of `app` and not their being one module.** A new
concern now has a field to be written into, and the row points here for
the split.

**What deferring it three times cost, since the item asks.** `frame.rs`
was 984 lines when this was filed, 1,131 at #1886, 2,037 at #1933 and
**2,298** at this unit's head — so the row is taken and the file still
grew — and each of the three took the expensive
half. The specific cost is visible in this unit's own review: the
README obligation was answered a second time as prose beneath the
table, which grew that section from 29 lines to ~46 before the row
existed to carry any of it. Prose beneath a table has no field to make
longer, so there was no diff at which any of the three looked
unreasonable — which is this item's thesis, reproduced a third time by
the units that read it.

**What the row does not do.** It does not split the module, and it does
not pretend to cover the eight concerns honestly — it says so in the
row itself. The second move above is still open and is still not a
lane's call.

## It grew again, and the ledger stopped being written (#2026, 2026-09-06)

984 lines when this item was filed, 1,131 at #1886, 2,037 at #1933,
2,298 at #1957 — and **2,475** at this unit's head. #2026 is the fourth
unit in a row to grow the file and the first not to write the row at
all: the count above stops at #1957 because the two units since simply
did not add to it, which is the accumulation this item names arriving
in the ledger kept to watch for it.

## The ledger's first fall, and it is one line (2026-09-09)

2,538 before the ruling on
`was-the-status-route-supposed-to-fire-for-an-absent-chooser` was
built, **2,536** after. The unit deleted `dialog_status` whole — Ev
ruled its `Show` arm misclassified rather than merely unreachable — and
concern 4 above lost a member without losing the concern. **A ledger
that finally moves the other way by ONE line is this item's argument,
not its answer**: the growth between #2026 and here is unrecorded
because the intervening units did not write the row, so 2,475 → 2,538
is the honest gap and the fall out of it is noise. The eight concerns
are still eight and the module is still unsplit.

**What #2026 added, and to which concern.** Concern 1, the status-line
vocabulary — already the largest — gained a second DOOR (`frame::deliver`
beside `frame::apply`, splitting a policy's verdict into news for the
frame's notices and retirement for the field) and a 45-line row
asserting the two halves against each other. Neither is a ninth concern
and neither is unreasonable on its own; that is the point. The `Holds`
row now exists to write it against, and this unit's first draft did not
extend it — the row was extended in the fix pass, with `deliver`.

## Evidence, not work: ~80 lines of environment probing in a per-frame module

Recorded here because it belongs to the split this item owns and to
nothing else on the board. `crates/viewer/src/frame.rs:1671-1878` —
`ChooserBackend`, `chooser_backend_of`, `chooser_backend`,
`zenity_on_path`, `session_bus_hinted`, `prefs_path`, `prefs_path_in`
and `running_under_wsl`, a 208-line span of which ~80 are code — is
startup environment probing: a `PATH`
walk for zenity, two `DBUS_SESSION_BUS_ADDRESS`/`XDG_*` reads, an XDG
config-directory resolution and two `WSL_*` variable reads.

The module's first line is **"The per-frame policies the viewport runs
— as values, so they are replayable."** Environment probing is neither
of those things: it runs once at startup rather than per frame, and it
reads ambient process state, which is what not-replayable means.

**This is not a proposal to move it, and the ruling it would fight is
not the one it looks like.** `scripts/gates/no-ambient-env.sh` ratifies
that the viewer's ambient reads have ONE home and names this file as
that home, and `prefs_path`'s own doc argues it. That ruling settles
*where the ambient door is*. What it does not do — and never claimed to
— is make the charter sentence at the top of the module true of what
sits under it. Being the sanctioned home for a concern is a reason the
concern is here; it is not a reason the module is one module.

So this is the cleanest available argument for the second move: two
things co-located by two separate good reasons, with a header sentence
that can only describe one of them. A split honours both — the ambient
door stays one door and stops being filed under "per-frame policies,
replayable".

Evidence for the split, not a task. Nothing here asks a lane to move
it.


## The split is taken, and the ledger's last entry (2026-09-16)

984 lines when this was filed, 1,131 at #1886, 2,037 at #1933, 2,298 at
#1957, 2,475 at #2026, 2,536 after the `dialog_status` deletion, 2,996
when the rule above was written. **After the cut: `frame.rs` 2,583,
`platform.rs` 257, `idpass.rs` 254** — 3,094 over three files against
2,996 over one, the difference being two module headers, two `use`
blocks and three doc links that had to be spelled across the new
boundary. Nothing was deleted and no behaviour changed.

**The membership, checked rather than taken.** The ruling's two OUT
sets are right about what leaves; the item's own lists under them were
not complete, and one of its numbers was never true.

- **The span is wrong at the merge base, not merely stale.**
  *"`crates/viewer/src/frame.rs:1671-1878`"* above names
  `/// stands for exactly as long as the policy holds the refusal.`
  through `/// arm, and the one the chrome disables the dialogs over.`
  on `origin/main` at `c0648077ce`. The probes run `1856-2076`: the
  span's END lands inside `ChooserBackend`'s variants and its START is
  two hundred lines short. A range that names the wrong opening line
  reads as a measurement of the concern's size and is not one.
- **Concern 4 had two members the ruling's list does not name**, and
  they are the ones the README argues about: `Zenity` and `SessionBus`,
  the two named readings `chooser_backend_of` ranks. They moved with
  it; a probe's typed reading is not a per-frame policy either.
- **`PREFS_DIR` and `PREFS_FILE`** are concern 5's and appear in
  neither list. They moved with `prefs_path_in`, which is their only
  reader.
- **`NO_CHOOSER_BACKEND` is concern 4's by this item's own list and the
  ruling's OUT list omits it. It moved**, and the reason is its own
  doc: *"there is no status-line route beside it"* — it is the text a
  disabled control carries, not a `Message`, so the status-line
  vocabulary is not what it belongs to. It reads `ChooserBackend` and
  nothing else in `frame` reads it.
- **Concern 2 is not one function.** *"the toolbar badge for the landed
  product — `product_badge`"* was true when written and has been false
  since #1957: `Badge`, `Tone`, `Affordance`, `SeamSubject` and eight
  badge doors. All stay, all are pure functions of a frame.
- **`Progress`/`progress` belong to none of the eight.** They arrived
  at #2055, after the list was written, and no ledger entry recorded
  them. A pure function of `(Outstanding, bool)`; it stays, and it is
  now named in the charter.

**It closes.** Both moves this item asked for are taken — the `Holds`
row at #1957, the split here — and the test the ruling set is met: the
module's first sentence is now true of everything under it, and
`crates/viewer/README.md`'s `frame` row states the exclusions rather
than confessing it cannot cover what is there. The concerns that remain
co-located (1, 2, 3, 7, plus `progress`) are the ones the ruling holds
the charter true of, so there is nothing left here to keep open for.

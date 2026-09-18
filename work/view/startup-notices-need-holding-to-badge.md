---
id: startup-notices-need-holding-to-badge
kind: issue
title: the startup preferences notices are rendered once into the status line and must be held before they can badge
status: open
opened: 2026-09-06
---


Disclosed by the status-line sweep, which routed every other writer
through the ranking and could not route this one without answering a
design question first.

## What happens

`ViewerApp::new` builds the field directly —
`crates/viewer/src/app.rs`, `status: frame::startup_notices(&notices)`
— a struct-literal initializer, which is why the sweep's original
`status = ` pattern missed it. The notices are what the preferences
store said when it loaded: a key it did not understand, a theme name
it could not resolve.

They are rendered ONCE, into the line, and then the notices themselves
are dropped. The first batch the user acts on clears the line, and the
sentence is gone with no way to get it back.

## Why the sweep did not take it

Under the ruled sort it does not belong on the line at all. Provenance:
nothing just happened — the file said this when the session opened, and
it says the same thing now. It is a read of held state, so it badges.

**But it cannot badge without being held.** A badge is a read, and
there is nothing to read: the notices were consumed at construction.
Making it badge means `ViewerApp` keeps them — a new entry in the frame
state inventory GQ6's toolkit decision rests on — and then the question
is what retires them. A preferences notice is true until the file
changes, and nothing in the viewer watches the file, so the badge would
stand for the life of the session with no event able to retire it.

That is a design question and not a mechanical move, which is why this
is filed rather than swept.

## What resolving it looks like

Three shapes, and the choice is the item:

- **Hold and badge, retired by nothing.** Honest — the fact is true
  for the session — but it is the first badge with no retiring event,
  and `frame.rs`'s header is explicit that a fact nothing can retire is
  the shape it distrusts.
- **Hold and badge, retired by a re-read.** Give the store a re-load
  the user can ask for, so the badge has an event that clears it. More
  machinery than the fact is worth unless something else wants it.
- **Leave it on the line and accept the erasure**, saying so where it
  is written. The cheapest, and the only one that needs no new held
  state — but it means the line keeps one writer that the ranking
  cannot help, because the sentence must exist before the first frame
  and the ranking runs per frame.

Not urgent: a mis-keyed preferences file is rare and the sentence is
advisory. What is not acceptable is the current silence about it,
which this file removes.

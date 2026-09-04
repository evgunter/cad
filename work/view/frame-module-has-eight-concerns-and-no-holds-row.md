---
id: frame-module-has-eight-concerns-and-no-holds-row
kind: issue
title: frame.rs holds eight concerns, its header describes three, and it has no Holds row in the viewer README
status: open
opened: 2026-09-04
refs: [opoutcome-superseded-has-no-production-reader, four-badges-five-spellings]
---

Found by the VIEW-6 review (2026-09-04), which added the ninth
concern's worth of surface and had no rule to write it against.

## What is in there

`crates/viewer/src/frame.rs` is 984 lines and its public surface covers
eight unrelated things:

1. the status-line vocabulary and its ranking — `StatusUpdate`,
   `apply`, `acts`, `batch_status`, `frame_status`, `fold_status`,
   `NOTICE_SEPARATOR`, `supersession_notice`;
2. the toolbar badge for the landed product — `product_badge`;
3. draft/offer chrome — `creation_offer`, `retype_draft`;
4. the file-chooser backend probe — `ChooserBackend`,
   `chooser_backend`, `chooser_backend_of`, `NO_CHOOSER_BACKEND`,
   `dialog_status`;
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

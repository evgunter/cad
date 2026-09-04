---
id: camera-fold-clears-status-line
kind: issue
title: A camera fold clears the status line, so a message raised on Open never survives the re-frame
status: review
opened: 2026-08-29
github: 1253
branch: view/status-lifetimes
---

## From GitHub issue 1253

Opened 2026-08-29; 0 comments.

## What happens

`app::land` is the one place a camera move becomes application state, and it assigns the status line unconditionally:

```rust
fn land(camera: &mut Camera, status: &mut Option<String>, folded: &camera::Folded) {
    *camera = folded.camera;
    *status = folded
        .refused
        .as_ref()
        .map(|(op, error)| format!("camera: {error} (from {op})"));
}
```

When a fold applies cleanly the `map` is `None`, so the line is **cleared** — whatever it held, whoever wrote it. `viewport_ui` calls `land` whenever `frame::folded_moved` is true, which includes the re-frame an Open books through `fit_on_scene` → `pending_fit`.

So a message written by `sync_scene` on the frame a new document's scene lands is erased later in that same frame, by the fit that document also booked.

## What it costs today

`sync_scene`'s product-fault message, which is the one thing that reports a malformed product:

```rust
self.status = self
    .session
    .product_fault()
    .map(|fault| format!("product: {fault}"));
```

A naming collision across roots is not a node failure, so no tree badge carries it — this line is its only home, and on an Open it is written and immediately cleared. The comment above it says the viewport "would otherwise draw a product nothing says is malformed"; after the re-frame, that is what happens.

Found while wiring the display budget's verdict. That verdict went to a toolbar badge instead, which sidesteps the problem rather than fixing it — the badge is the right home for a standing fact independently, but the choice was forced.

## Why this is a design question and not a patch

The status line has at least four writers with different lifetimes, and the bug is that they share one `Option<String>` with no notion of which outlives which:

- **transient refusals** from `perform_batch`, which `frame::batch_status` already gives a considered policy (rank the refusals, clear only on a clean acting batch, ignore hover)
- **camera verdicts** from `land`, which clear on every clean fold — the fastest-moving writer, and the one that wins by accident
- **per-landing facts** from `sync_scene` (the product fault, the pick-index refusal)
- **frame reports** from `viewport_ui` (the two picking paths disagreeing)

Candidate shapes, none obviously right:

1. `land` clears only a message it wrote — needs the line to carry its writer, i.e. a typed status rather than a `String`.
2. Per-landing facts become badges, like at-rest, checks and the display budget already are, and the line is left to transients. Says what the line is FOR, but the product fault is a fault and a weak badge is a quiet one.
3. A ranked status like `Refusal::preferred`, extended across writers rather than only within a batch.

(2) and (3) are not exclusive. The reason to file rather than pick: `frame` was extracted precisely so these rules would be values a row can execute instead of app-gated code nobody can reach, and whichever shape wins should land there with rows, not as a condition bolted onto `land`.

## Reproducing

Open any document whose product has a fault (two roots colliding in the name table). The message appears for no frames; commenting out the `*status = ...` line in `land` makes it appear and stay.

## Home

`work/issues/` — `crates/viewer`'s app/frame seam is GUI-era ground and the GUI program is closed.

## What landed, and the rule it landed as

**The status line carries per-frame NEWS, and `frame` owns its
ranking. A fact that stays true after the frame ends is not news and
does not belong in the line.** That sentence is now
`crates/viewer/src/frame.rs`'s header, and both halves of it are
values there rather than conditions at a call site:

- `frame::fold_status` answers `StatusUpdate::Keep` for a clean fold
  and `Show` for a refused one. A camera arriving where it was sent is
  not news; clearing stays the acting batch's verdict alone
  (`batch_status`), because an action the document accepted is the one
  event that makes a standing complaint stale.
- `frame::apply` is the one door a `StatusUpdate` becomes the field
  through, so `Keep` is spelled as a decision instead of as the
  absence of an assignment. `land` and `ViewerApp::apply_status` both
  go through it.
- `frame::product_badge` renders the gather's verdict, filtering
  `NoBodyRoots` (an empty document is not malformed — the blank
  viewport says so). The toolbar draws it beside the at-rest and
  checks badges, in the unresolved colour rather than the weak one the
  budget advisory uses: a fault is a fault, and the home it moved to
  must not be quieter than the line it left.

Of the shape's three candidates, this is (2) and (3) together and not
(1): the line does not carry its writer, because a writer that has
nothing to say now says so.

`sync_scene` no longer writes the fault at all. Two consequences worth
recording. The badge is a read of held state, so it can never be stale
or erased by anything else in the frame — where the old line was
written only inside the successful-rebuild arm. And it therefore
appears for faults the line could not reach: a failed root refuses the
pick index, and `sync_scene` returned at that arm before ever reaching
the product-fault assignment, so `RootFailed` and `RootPoisoned` badge
now where before only the tree row did.

### Rows

`crates/viewer/src/frame.rs` (default lane, so all twelve CI test jobs
run them) and `crates/viewer/src/pane/viewport.rs` (`--features app`).
Each was driven red by mutating the thing it names:

- `a_clean_fold_keeps_a_message_it_did_not_write` — red when
  `fold_status`'s clean arm answers `Clear`.
- `a_product_fault_survives_the_re_frame_the_open_that_raised_it_books`
  — the composition this item filed: a session whose landed product
  does not gather, and the `CameraOp::Frame` an Open books on the same
  frame. Red under either mutation.
- `the_gather_verdict_badges_every_fault_but_an_empty_document` —
  builds the item's own repro as a value (`ProductError::Naming`) and
  asserts `NoBodyRoots` stays silent. Red when `product_badge` filters
  everything.
- `landing_a_clean_fold_does_not_clear_a_message_it_did_not_write` —
  calls `land`, which is the one wiring a `frame` row cannot reach:
  red when `land` goes back to assigning the line itself.

### What did not land

The other eighteen direct writers of the line, which bypass
`frame_status` entirely. Censused and classified in
`status-line-writers-bypass-the-ranking`; not swept here because
`crates/viewer/src/pane/` is shared with CHROME.

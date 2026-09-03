---
id: camera-fold-clears-status-line
kind: issue
title: A camera fold clears the status line, so a message raised on Open never survives the re-frame
status: open
opened: 2026-08-29
github: 1253
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

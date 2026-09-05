---
id: progress-takes-three-positional-bools
kind: issue
title: frame::progress takes three positional bools, two of them adjacent and swappable with no type or test to catch it
status: open
opened: 2026-09-05
---


## What

Found by VIEW-6b's style review (S19).

`crate::frame::progress(busy, running, indexing) -> Option<Progress>`
(`crates/viewer/src/frame.rs`) is the one place the viewer decides what
the toolbar says about work outstanding. Its three arguments are bare
`bool`s, and the first two are **adjacent and differently defined**:
`DocSession::busy` is "the picture is older than the document",
`DocSession::running` is "the seam has work". Swapping them at the call
site type-checks and produces plausible chrome — a spinner where a
cancel should be, or a cancel over a live run.

**The row cannot catch it.** `the_chrome_has_one_progress_state_…`
calls the same function with the same positional convention, so a
swapped call site and a swapped test agree with each other. The one
caller is `app.rs`'s toolbar, which is `app`-gated and unexercised by
any test — so nothing in the tree would go red.

## What a fix looks like

Either three named types or one struct the session hands out. The
second is the more interesting version: `busy` and `running` are both
`DocSession`'s answers about one moment, and a `Progress` computed from
a value the session mints cannot be given them in the wrong order.

## Not urgent

The call site is one line and is currently correct. It is filed
because the argument for a value here is the same one
`crates/viewer/README.md` makes for every other chrome decision in
`frame`, and this function is the newest of them.

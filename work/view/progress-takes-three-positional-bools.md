---
id: progress-takes-three-positional-bools
kind: issue
title: frame::progress takes three positional bools, two of them adjacent and swappable with no type or test to catch it
status: closed
opened: 2026-09-05
closed: 2026-09-06
branch: view/progress
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

## Closed (2026-09-06)

**Verified first.** Exactly one caller, as filed: `app.rs`'s toolbar
was the only site naming `frame::progress`, and the only test naming it
was `frame_policy.rs`'s
`the_chrome_has_one_progress_state_and_evaluation_outranks_indexing`,
which repeated the same positional convention. `busy()` and `running()`
were paired at that one call site and nowhere else. Away from it the
two are read singly and often — `session.busy()` at nineteen sites and
`session.running()` at five, mostly wait loops in `tests/eval_seam.rs`,
`tests/review_gui3_r*.rs`, `tests/review_gui4_r2.rs` and
`examples/r1_e2e.rs` — which is what decided the shape below.

**The shape taken: one value the session mints** — the item's second
option. `session::Outstanding` is `Current | Evaluating | Canceled`,
minted by `DocSession::outstanding()`, and `frame::progress` now takes
`(Outstanding, bool)`. Three points:

- The three variants are what the chrome actually distinguishes, so
  the pair is not handed to anyone: it is read inside `outstanding()`,
  **by name in an if/else chain**, where there is no argument position
  for either read to occupy and so nothing to transpose. A struct with
  two `bool` fields would have moved the swap to its constructor; an
  enum removes it.
- `progress`'s two remaining arguments have different types, so a
  transposed call site does not compile.
- `busy()` and `running()` stay, because each is a real question
  on its own and `busy()` is the tree's wait predicate. What is gone is
  any signature that takes both.

**Why the test can no longer agree with a swapped caller.** The
`frame_policy` row now names states (`Outstanding::Canceled`) instead
of spelling a position, so there is no convention left for a caller to
mirror — the swap it used to hide is a type error. What that row cannot
say is which session state produces which variant, so the fold is
covered where it can be executed without the `app` feature:
`tests/eval_seam.rs`'s cancel row drives a real `DocSession` through
`Evaluating` → `Canceled` → `Evaluating` → `Current` and asserts
`outstanding()` at each. That is the coverage the old arrangement had
nowhere: `app.rs` is `app`-gated and untested, so the mapping from
session state to chrome state was asserted by no one.

**Rejected: three named types** (the item's first option). It types the
signature but leaves the session minting three values a caller still
assembles in order, and it says nothing about `busy`/`running` being
one fact — the eighth combination (`!busy && running`) stays
expressible and stays unreachable, so the function goes on being total
over a domain wider than its subject. The enum makes the unreachable
state unrepresentable instead of documented.

**Rejected: putting `Outstanding` in `frame`.** `frame` is a
vocabulary, so the session driver may name it; but the value is what
the session says about itself, not a per-frame policy, and `frame`
already carries eight concerns
(`work/view/frame-module-has-eight-concerns-and-no-holds-row.md`). It
sits in `session` beside `Landing` and `AtRestBadge`, which are the
same thing — values the session mints and a vocabulary consumes — and
`crates/viewer/README.md`'s session paragraph now carries the rule.

**Rejected: `vocabulary!`.** Not a closed enumeration anything
iterates; nothing here wants an `ALL`.

No residue.

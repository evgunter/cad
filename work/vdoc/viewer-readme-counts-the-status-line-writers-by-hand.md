---
id: viewer-readme-counts-the-status-line-writers-by-hand
kind: issue
title: crates/viewer/README.md counts the status line's writers as eighteen/seventeen by hand, and the population is a grep no type bounds
status: closed
opened: 2026-09-24
priority: P4
cost: E
closed: 2026-10-26
---


Filed 2026-09-24 by VNEWS's `frame.rs` prose pass
(`vnews/frame-rs-prose-pass`), which removed the same counts from
`crates/viewer/src/frame.rs`'s module header under
`work/vnews/hand-maintained-counts-in-frame-rs-prose-have-no-guard`.
That row said the README's mirror *"is filed there"*; no `work/vdoc/`
row carried it, so this is that filing. `README.md` is VDOC's by
`work/vnews/program.md`'s `keep_out`, so it is filed and not fixed.

## The sentences

`crates/viewer/README.md`, the status-line section (~`:802`, `:842-852`
at this filing's base):

- *"the sweep that sorted eighteen writers"*
- *"**Seventeen of the eighteen writers that can put a sentence on the
  line now come through the ranking.** All eighteen used to reach the
  field without it — sixteen assignments, one struct-literal
  initializer at startup, and `frame::fold_status` …"*
- *"Seventeen now push onto the frame's `notices`"*
- *"**The eighteenth is the startup initializer**"*

## Why they are stale, and why updating them is not the fix

The writers onto the frame's notices are every write to `ViewerApp`'s
`notices` (which `ViewerBehavior` lends the panes) under
`crates/viewer/src`. `frame::tool_news` alone has fourteen production
callers today (`pane/create.rs` ten, `app.rs` two, `pane/profile.rs`
two), so *"seventeen"* no longer counts the population it names. The
*"sixteen assignments"* half is history — a description of the tree
before the ranking sweep — and is true of that past, not of anything a
reader can check now.

A new number re-mints the defect on the next writer. `frame.rs` now
says *"every writer but one"*, names the rule that derives the
population, and says at the site why no row guards it: the population
is a grep, not a type. The README's `frame_policy.rs` guard
(`the_readme_counts_its_two_populations_correctly`) counts two
populations a type bounds; this one is not of that kind, and a guard
hand-updated beside each writer would be the same defect under a
`#[test]`.

## The fix

State the rule and drop the numbers, matching `frame.rs`'s module
header; keep the startup initializer as the named exception.

## Same section, one citation more

The paragraph's last line cites
`work/view/startup-notices-need-holding-to-badge.md`; that row is now
`work/vseam/startup-notices-need-holding-to-badge.md`. Recorded on
`work/vdoc/crates-cite-work-view-rows-that-moved-in-the-rescope`,
which owns that class.

**Correction, 2026-10-13 (`vnews/tool-news-is-a-value`):** the
`tool_news` census above is stale. `app.rs`'s two sites now go through
`frame::tool_notice`, so `tool_news` has eleven production callers
(`pane/create.rs` nine, `pane/profile.rs` two), each handing it a
`tools::PanelRefusal`.

## Closed (2026-10-26, tracker sweep)

**Fixed in this sweep**, as this row asked: state the rule, drop the numbers, keep the startup initializer as the named exception. `crates/viewer/README.md`'s status-line section (~`:1000-1016`) now reads *Every writer that can put a sentence on the line but one now comes through the ranking*. It names the population (every write to `ViewerApp`'s `notices` under `src/`, a grep rather than a type) and defers to `frame`'s module header (`frame.rs:145-166`) for why no counting row guards it. *The one exception is the startup initializer*. *the sweep that sorted eighteen writers* (~`:885`) is now *the sweep that sorted the writers*. The section's `work/view/startup-notices-need-holding-to-badge.md` citation now points at `work/vseam/`. `grep -n 'eighteen\|eventeen' crates/viewer/README.md` returns nothing.

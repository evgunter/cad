---
id: viewer-src-files-no-successor-claims
kind: issue
title: Eleven crates/viewer/src files are claimed by none of the four re-scope successors, and three of them are where live rows have to land
status: open
opened: 2026-09-19
---


Found by the VNEWS orchestrator at that program's first dispatch
(2026-09-19), deriving each wave-1 row's fix site against the four
successors' `paths`.

## What is true

VIEW's re-scope of 2026-09-17 split `crates/viewer/src/*` across
`vnews`, `vgeom`, `vseam` and `vdoc` by naming files rather than by
globbing. Fifty-two files are tracked under `crates/viewer/src`; the
union of the four successors' `paths` covers forty-one. **Eleven are
claimed by no successor:**

    bin/viewer.rs   blend.rs       drafts.rs     matetool.rs
    pane/profile.rs parts.rs       platform.rs   prefs.rs
    revolvetool.rs  theme.rs       tree.rs

Re-derive rather than trusting the list: it is the set difference of
`git ls-files crates/viewer/src` against the four `paths` lists, taken
at this filing, and a successor claiming a file closes its entry
silently.

## Why it is not merely untidy

Nothing here is UNOWNED in the tracker's terms — VIEW's own
`crates/viewer/src/*` still covers all fifty-two and VIEW is open. But
**VIEW is `NOT DISPATCHING`** by its plan's own status line, so a row
whose fix lands in one of the eleven has an owner that will not
dispatch it and a dispatching program that does not claim the file.
That is a gap the split created, not one it inherited.

Three of the eleven are live sites for rows the split HANDED OUT:

- **`tree.rs`** — `work/vnews/tone-is-a-value-in-frame-and-a-comment-in-two-panes`
  proposes `RowStatus::tone()` beside the existing `badge()`, and
  `RowStatus` lives here. *Claimed by `vnews` in the same commit that
  files this row*, because that row is dispatching now; the other ten
  are left rather than swept up, since claiming a file nobody needs is
  how a second unrecorded overlap gets minted.
- **`platform.rs` and `prefs.rs`** —
  `work/vnews/environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere`
  is about `platform::ChooserBackend::usable` against
  `prefs::PrefsStore::unusable`, and its sweep rule ranges over
  `platform::Zenity`, `platform::SessionBus`, `platform::prefs_path`
  and the WSL probe. Every named site is in the two unclaimed files.
- **`pane/profile.rs`** — `work/vnews/viewer-preview-names-a-verb-by-its-variant-identifier`
  cites `pane/create.rs:582-586` as the site that renders
  `PreviewError` to screen. That citation is stale: `PreviewError`
  rendering is now `pane/profile.rs` (`sketch::tip_state_words` at
  `:346`), which no successor claims, and `create.rs:582-586` today is
  the `ShapeKind::Path` notation block.

## Why it is VIEW's

The allocation was VIEW's act and its completion is a precondition of
VIEW's exit walk, in the same way and for the same reason as
`the-lane-register-has-no-home-after-views-directory-goes`: when
`work/view/` goes, the glob that currently makes these eleven owned
goes with it, and the gap stops being a dispatch inconvenience and
becomes eleven genuinely unowned files.

## What resolving it looks like

Not "give them all to someone". The charter test each successor states
is what sorts them, applied per file — `theme.rs` and `drafts.rs` are
plausibly two programs' each, and `bin/viewer.rs` may be nobody's. What
this row asks for is that the sort be DONE and written into the four
`paths` lists, with both sides' `keep_out` naming the other wherever
two claim one file, before VIEW's directory is deleted.

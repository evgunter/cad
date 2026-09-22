---
id: viewer-src-files-no-successor-claims
kind: issue
title: Eleven crates/viewer/src files are claimed by none of the four re-scope successors, and three of them are where live rows have to land
status: closed
opened: 2026-09-19
priority: P4
cost: E
closed: 2026-09-21
branch: view/cut-residue
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

## Closed — 2026-09-21, `view/cut-residue`

**Re-derived rather than trusted, and the list had moved twice.** The
difference is `git ls-files crates/viewer/src` against the union of the
four successors' `paths`, matched with `fnmatch.fnmatchcase` exactly as
`scripts/work.py:459` matches a territory glob. The script is printed in
this branch's PR body and was run as printed; on the merged tree it
gives **ten**, not eleven:

    bin/viewer.rs   blend.rs   drafts.rs       matetool.rs
    pane/profile.rs parts.rs   platform.rs     prefs.rs
    revolvetool.rs  theme.rs

`tree.rs` left the set exactly as this row predicted it would — VNEWS
claimed it in the commit that filed this row, and nothing announced
that the entry had closed.

**And one of the ten was never in the gap.** This row's second claim is
that an unclaimed file has *"an owner that will not dispatch it and a
dispatching program that does not claim it"*. That is a claim about
EVERY open program, and the difference above ranges over four of them.
Widening it — the same match against every `work/*/program.md`, ignoring
`view` and `chrome`, whose `crates/viewer/src/*` globs are the ones that
die with VIEW — puts **`drafts.rs` in AUTHOR's `paths` already**, where
it has been since CHROME's cut of 2026-09-20 sent the authoring-door
rows there. AUTHOR dispatches. So the gap was **nine** files, and the
row's own instrument was narrower than its claim, which is this
register's proxy rule with the row holding the proxy.

### Where the nine went, and the charter sentence that decides each

| file | claimed by | the test that decides it |
|---|---|---|
| `bin/viewer.rs` | `vnews` | VNEWS: *the word a fact is spelled in*, and *a row belongs here only if a reader would see the difference*. Sixty of its 113 lines are `report_to_page` and the three status-overlay ids — the only surface a startup refusal has on a phone — and the comment at the call site argues about not doubling the "failed to start" prefix, which is `the-new-document-button-states-its-refusal-twice` one layer out. |
| `blend.rs` | `vseam` | VSEAM: *the op vocabulary — what the viewer may author at all, and whether one op is one act*. The accumulator holds an open-ended edge set across frames and commits exactly one `SessionOp`; the file names `egui` nowhere. |
| `matetool.rs` | `vseam` | Same sentence: two face picks held in tool state, one committed `DocEdit`. VIEW's own open P1 `face-selection-carries-a-bare-stable-name` is the typed-door row on `FaceSelection`, and `session/select.rs` is VSEAM's already. |
| `revolvetool.rs` | `vseam` | Same sentence: two node picks in tool state, one `SessionOp::AddRevolve`. The live VNEWS row that cites it (`seat-line-takes-a-hand-built-slice-…`) says in its own Home section that `revolvetool.rs` is *"named as evidence, not as a fix site"*. |
| `parts.rs` | `vseam` + `vnews` | **Split, and both sides written.** VSEAM: *an answer cached across a generation* — the catalogue is a snapshot *"taken once, not per frame"*, against the directory the session opened. VNEWS: `PartEntry::refusal` and the `Add part…` chooser's disabled sentence, two live rows. |
| `pane/profile.rs` | `vseam` + `vnews` | **Split, and both sides written.** VSEAM: the file exists because VSEAM's own `editing-a-profile-does-not-share-the-create-forms-interface` lane wrote it (PR #2862) as the one editor both of a profile's doors use — an authoring door. VNEWS: the verdict and tip-state words, and three further live VNEWS rows cite it. |
| `platform.rs` | `vnews` | VNEWS: `environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere` is live there and **every site it names is in this file** — `platform::Zenity`, `platform::SessionBus`, `platform::prefs_path`, the WSL probe. |
| `prefs.rs` | `vnews` | The same live row's other half, `prefs::PrefsStore::unusable`, plus `document-news-has-no-home`. |
| `theme.rs` | `vgeom` + `vnews` | **Split, and both sides written.** VGEOM: `mixfraction-has-two-constructors-where-one-would-do` names `crates/viewer/src/theme.rs` as its **Where**, and `the-shader-encodes-a-mark-strength-nothing-bounds` is the Rust/WGSL parity half. VNEWS: `tone-doc-argues-from-a-site-that-now-reads-the-value` holds one member open in as many words — *"stays open on this row — `theme.rs` is claimed by no dispatching program"*. |

Nothing was invented to empty the list, and no file went to VDOC: VDOC
claims no `crates/viewer/src` file but `lib.rs` by design, and its
`keep_out` says so.

### The one thing the sort could not place, said rather than papered over

`bin/viewer.rs`, `platform.rs` and `prefs.rs` are one subject — **the
viewer's PROCESS boundary**: what the run commits before the first
frame, what the environment offers, what survives between runs. No
successor charter has that subject. VNEWS's test is that *nothing it
touches survives the frame that produced it*, and a preference survives
the run; VSEAM's is what the viewer holds *on behalf of the document*,
and neither a `zenity` probe nor a colour scheme is the document's. The
three are claimed for the news- and word-shaped rows that are live on
them today, which is honest and which the `keep_out`s state. **A row
about whether a preference is KEPT, or about when a probe is re-read,
has no charter among the four** — it goes to the VIEW orchestrator as a
report, not into a program whose own test excludes it. That is written
into `work/vnews/program.md`'s `keep_out` rather than left here, because
this file stops being read at close.

### What could not be done from here, and is already filed

All ten stay CHROME's under `crates/viewer/src/*`, and CHROME's
`keep_out` names `view` and cannot name the successors —
`work/vdoc/the-four-view-successors-are-a-one-sided-double-claim`
already owns that and takes the nine new files with it unchanged.
`python3 scripts/work.py territory --overlaps` counts **58 overlap
lines before and 58 after**, with no successor-to-successor pair in
either list (a pair recorded on both sides is not reported, which is
the condition this row asked for); the `chrome`-side counts grow
11 → 17 (`vnews`), 16 → 17 (`vgeom`) and 21 → 27 (`vseam`), and those
are that row's to close.

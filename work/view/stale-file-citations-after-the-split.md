---
id: stale-file-citations-after-the-split
kind: issue
title: 24 open files cite app.rs:NNNN or session.rs:NNNN for code the split moved, five of them wrong about the file
status: open
opened: 2026-09-04
---


The 1c split moved most of `app.rs` and `session.rs` into thirteen new
modules. **24 open `.md` files across five programs and `docs/` cite
`app.rs:NNNN` or `session.rs:NNNN`** for code that has moved, most of
them past the new files' last line (`app.rs` is 1,752 lines and
`session.rs` 1,500; citations run to `app.rs:5384` and
`session.rs:3030`).

A number that no longer resolves is cheap to re-find. The costly ones
are where **the CLAIM, not just the number, went stale** — the
sentence names the wrong file:

| File | What is now wrong |
|---|---|
| `docs/DOCM-IDENTITY-DESIGN.md:112` | cites `session.rs:3030` and `:2321` past the file's last line; both doors are still in `session.rs`, but the probe the second one is about is now split with `session/probe.rs`, so the subject is in two files and the citation names neither |
| `docs/BOOL-10-SPEC.md:62,159` | names "`app.rs`'s tool palette" as the viewer's arm for the verb; `PathVerb` and its `ALL` table are in `forms.rs`, and what draws them is in `widgets.rs` and `pane/create.rs` |
| `work/code-quality/viewer-pathverb-all-hand-written-seventeen.md:19` | "`app.rs:514` declares `enum PathVerb`" — it is `forms.rs` now |
| `work/chrome/viewer-const-all-tables-have-no-exhaustiveness-guard.md:14` | the three `const ALL` tables cited in `app.rs` are in `forms.rs` |
| `work/chrome/app-rs-doc-comment-merge-scars.md:19` | scar #1 is `tip_mark`, which moved to `sketch.rs`; #2 (`perform_batch`) is still in `app.rs` at a new line |

## Why no gate caught it

`scripts/doc-gate.sh` fails only on rustdoc's **bracketed** intra-doc
links, and only inside Rust doc comments. Every citation above is an
unbracketed code span in a Markdown file, which rustdoc never sees.
That is why the gate is green over two dozen stale references, and it
is the same hole as `boundary-rule-has-no-mechanical-check`: the
prose is checked by readers only.

## Disposition

**Only `work/view/*` is this program's to fix**, and its own
citations are corrected as they are touched. The rest —
`work/chrome/*`, `work/code-quality/*`, `work/docm/*`, `work/fix/*`
and both files under `docs/` — belong to other programs; they are
announced, not edited (`docs/prompts/implementer-discipline.md` §6).
The right owner of the general case is probably whoever takes the
guard above: a citation of the form `<file>.rs:<line>` is as
mechanically checkable as a `use` block, and nothing checks either.

## VIEW's half, paid (VIEW orchestrator, 2026-09-04)

Five files in `work/view/` carried pre-split citations. All are
re-pointed against the tree at `d799235e`, each with a note saying a
re-point happened so a reader who remembers the old numbers can tell
one from a claim change:

| File | Was | Now |
|---|---|---|
| `opoutcome-superseded-has-no-production-reader.md` | `session.rs:1469`, `app.rs:1745`, `session.rs:2700`, `:3081` | `session/op.rs:633` and `:646`, `app.rs:800`, `session.rs:1056` and `:1418` |
| `revolve-tool-unreachable-no-axisinplane-form.md` | `session.rs:2867`, `session.rs:425`, `app.rs:742` | `session.rs:1196`/`:1200` and `seats.rs:161`, `session/refuse.rs:61`/`:65`, `forms.rs:52` and `pane/create.rs:354-363` |
| `save-is-not-gesture-guarded.md` | `session.rs:2712`, `:2750` | `session.rs:1070`, `:1105` — **and see below** |
| `two-gestures-can-be-in-flight-together.md` | `session.rs:1558` | `session.rs:153` |
| `blamed-mates-lost-its-exhaustive-arm.md` | `app.rs:2880` | nothing: there is no `MateFault` in `app.rs` at all now |

### The one that was not a re-point, which is this item's whole point

`save-is-not-gesture-guarded.md` describes 23 `if self.gesture.is_some()`
guards at 23 call sites and reasons from `open` carrying one where
`save` does not. **VIEW-1b deleted that mechanism**: the rule is one
exhaustive table (`crates/viewer/src/session/op.rs:586`) checked once
in `perform`, and two `is_some()` reads survive in `session.rs`,
neither a dispatch guard. Correcting the two numbers would have left a
file whose citations resolve and whose sentences are false — **more
dangerous than the broken numbers**, because a resolving citation
reads as checked.

That is the costly case this item was filed on, found in this
program's own directory, while paying the cheap half. It is evidence
for the item's closing argument: a `<file>.rs:<line>` citation is as
mechanically checkable as a `use` block and nothing checks either —
but a machine that only resolved the numbers would have passed this
file, so the guard the general case wants is weaker than the reading
that found this.

### Announced, not edited

The rest is other programs' (`docs/prompts/implementer-discipline.md`
§6). Named here so the announce is a list and not a sentence:
`docs/DOCM-IDENTITY-DESIGN.md:112` (DOCM),
`docs/BOOL-10-SPEC.md:62,159` (S-BOOL),
`work/code-quality/viewer-pathverb-all-hand-written-seventeen.md:19`,
`work/chrome/viewer-const-all-tables-have-no-exhaustiveness-guard.md:14`
and `work/chrome/app-rs-doc-comment-merge-scars.md:19` (CHROME), plus
the remainder of the 24 across `work/docm/` and `work/fix/`.

**What stays open here** is the general case only: whether a citation
gate exists, and if it does, that resolving a number is not the same
as checking a claim.

## The class has a second member, and this one is not about file paths (#2026, 2026-09-06)

Found by #2026's style review. **Closing one item falsified a premise
cited as live in seven sibling files**, and none of them names a path
or a line number — so the "re-point the citation" half of this item
does not reach them and the "check the claim" half is the whole of it.

The premise: `status-line-writers-bypass-the-ranking` is an open census
of writers that reach the status line outside `frame::frame_status`'s
ranking, and the count is nineteen or twenty. It closed at #2026 with
the census re-derived to **eighteen** — the membership test had been
wrong all four times it was counted — and seventeen of those routed
through the ranking.

| File | What it said | Disposition |
|---|---|---|
| `work/view/plan.md:75` | the sweep is residue of #1849 and "open", "(19 sites)" | corrected |
| `work/view/plan.md:170` | the three forks gate a sweep that is still to run | corrected |
| `work/view/news-and-standing-facts-are-orthogonal-axes.md:38,45,148` | "nineteen writers", "the twenty-writer sweep", "has nothing to sort by" | corrected (item is `review`, not closed) |
| `work/view/one-line-one-subject-loses-a-mixed-frames-expiry.md:27` | "from that unit onward" — future tense about a landed unit | corrected, and the heading above it (`Why it is not reachable yet`) with it |
| `work/view/four-badges-five-spellings.md:65-68,132` | "sorts nineteen writers", "Four of the nineteen are standing facts" | **left as written** — the item is closed |
| `work/view/camera-fold-clears-status-line.md:157-161` | "The nineteen remaining direct writers" under *What did not land* | **left as written** — the item is closed |
| `work/view/unindexed-refusal-is-an-outcome-not-a-read.md:61,115` | "the twenty-writer sweep" | **left as written** — the item is closed |
| `work/view/log.md:1362-1363` | "all three are inputs to the same nineteen-site sweep" | **left as written** — the log is append-only; a new entry carries the correction |

### What the split between those two columns is, since it is the interesting part

A **closed** item is a record of what was believed and done at a
moment; editing it to agree with a later tree destroys the only thing
it is for. A **live** item is a claim about the tree as it stands, and
one that has gone false is read as checked. So the rule this program
applies, and which is worth stating here because this item is where the
class lives: **correct live rows, never closed ones, and append to the
log rather than rewriting it.** The cost is that a reader grepping for
"nineteen writers" finds four hits that are all historical and nothing
marks them so at the grep — which is the same hole as the numeric
citations above, one level up. A machine that resolved `file:line`
citations would not have caught any of these eight.

### Why no gate caught it either

`scripts/work.py lint` resolves *ids*, so it knew perfectly well that
`status-line-writers-bypass-the-ranking` had gone `closed` — and every
one of the eight files above still parsed, because none of them makes
the claim in a field. The claim is in prose, and the prose is what went
stale. That is this item's closing argument arriving a second time from
a different direction: resolving a reference is not checking a claim,
and here the reference resolved perfectly.


## A third member: the `pick.rs` split (2026-09-06)

`pick.rs` split at its layer boundary — the index and every query over
it into `pickindex.rs`, the policy staying — and `Generation` left
`evalseam.rs` for `generation.rs`. Every `crates/viewer/src/pick.rs:NNNN`
citation in the tracker is therefore either in the wrong file or at the
wrong line, on the same mechanics as the 1c split above: the numbers
below `:2212` moved to another file entirely and everything above it
shifted by 2,167.

**VIEW's half, paid in the same PR**, corrected against the tree at the
split's own commit. The first five were found by the file-and-line
sweep; the last two were **missed by it and found by the style review**
(`pick-split-sweep-missed-two-live-view-rows`), which is the part of
this entry worth reading:

| File | Was | Now |
|---|---|---|
| `adjacent-same-typed-arguments-are-the-same-swap.md:55` | `pick.rs:408`/`445` | `pickindex.rs:405`/`442` |
| `ui-thread-work-after-the-index-seam.md:31` | `pick.rs:932` | `pickindex.rs:928` |
| `outstanding-and-progress-are-two-three-state-enums-one-hop-apart.md:49` | `pick.rs:2525` | `pick.rs:359` (same file — `PickCache` stayed) |
| `new-document-owes-the-reframe-open-gets.md:54` | `pick.rs:2270-2288` | `pick.rs:103-121` (same file) |
| `pick-priority-filter-vocabulary.md:17` | `viewer::pick::EDGE_PICK_RADIUS_PX` | `viewer::pickindex::EDGE_PICK_RADIUS_PX` |
| `focus-marking-is-per-node-not-per-segment.md:15,21,26,54` | `pick::focus` x3 and *"Viewer ground (`crates/viewer/src/pick.rs`)"* | `pickindex::focus`, `pickindex.rs:2076` — the item's whole subject moved |
| `the-picture-key-never-became-a-type.md:18` | *"in `pick.rs` and `evalseam.rs`"* — a two-file list | a three-file list; `PickIndex::current_for` is `pickindex.rs:756` |

Closed VIEW rows citing the old paths — `pick-and-parts-name-the-session
-driver`, `pick-index-built-on-ui-thread`, `unindexed-refusal-is-an-
outcome-not-a-read` — are **left as written**, under the rule this item
already states: a closed row is a record of what was believed at a
moment.

### Announced, not edited

Other programs' (`docs/prompts/implementer-discipline.md` §6):
`work/chrome/mispaired-ids-exempts-the-empty-window.md:32` (the unit
tests it names moved with the structure, into `pickindex.rs`),
`work/chrome/pickindex-per-part-window-twins.md:18,40` (`pick.rs` named
twice as the file the seven twins live in — they are `pickindex.rs`'s
now, and the item's own id already reads as if it knew),
`work/chrome/edge-cost-claims-name-a-search-that-is-gone.md:13,19`
(`pick.rs:1979-1983` and `pick.rs:600-607`, both now `pickindex.rs`),
`work/chrome/viewer-first-light-on-real-hardware.md:110`
(`pick::PickIndex`), and
`work/code-quality/run-on-whitespace-in-message-literals.md:9,23`
(`crates/viewer/src/pick.rs:1771`, now `pickindex.rs:1774`), and —
found only by the re-sweep, not by the first pass —
`work/fix/error-types-with-no-display-class.md:73,87`, which names
`crates/viewer/src/pick.rs` as the home of `PickError`'s missing
`Display` (it is `pickindex.rs:1784`) and cites `pick.rs:676` for a
comment about `HitTestError` that the line number had already outrun
before this split.

### What this member adds to the item's argument

**The first draft of this section got it wrong, and the correction is
the contribution.** It read: *"this split produced no rows where the
claim went stale, only numbers, because a move that changes no
behaviour cannot falsify a sentence about behaviour."* The premise is
true. The conclusion does not follow, and the style review of #2079
produced three counterexamples
(`a-pure-move-falsifies-sentences-about-structure`):

- `crates/viewer/README.md:323` — the `session::op` row named `pick` as
  a `SessionOp` reader. Zero hits in `pick.rs` after the split; seven
  in `pickindex.rs`, which the row did not name. In a file this unit
  re-read and corrected two other rows of;
- `the-picture-key-never-became-a-type.md:18` — a two-file list that
  is a three-file list;
- `focus-marking-is-per-node-not-per-segment.md:54` — a *"Viewer
  ground"* naming the wrong file.

**A tracker is not mostly sentences about behaviour.** It is mostly
sentences about where things are and what names what, and those are
exactly what a pure move falsifies. So the durable statement is:

> A behaviour-preserving move cannot falsify a sentence about
> behaviour, and reliably falsifies one about STRUCTURE — which is the
> half a behaviour-preserving unit is least primed to look for, because
> its whole discipline is aimed at the other one.

That also relocates this item's two halves. They do not come apart on
"numbers versus claims"; they come apart on **what a machine can
reach**. A `<file>.rs:<line>` gate would have resolved every numeric
citation in this member and caught none of the three above — the README
row least of all, since it names a module in prose and cites nothing.

### And the sweep's stated blind spot was not the one that bit

#2079 disclosed its pattern and named its blind spot as *"a citation
that names neither a path nor a moved symbol"*. Both missed rows name
one: `pick::focus` and `crates/viewer/src/pick.rs` are inside the
pattern. What actually happened has two causes, and the second is the
one to carry:

1. the first pass's `grep` over `work/view/*.md` was **malformed** —
   an unescaped `|` in the shell, which printed `command not found`
   above output that was read as a result. A grep that could not run
   reads exactly like a grep that found nothing;
2. even with it running, the **disposition** step only acted on the
   `pick.rs:NNNN` shape. Bare paths and `module::symbol` citations
   matched and were not carried through.

So the honest blind spot is not a pattern gap: **the pattern was
right and the reading of it was not.** The re-sweep that found the
third announced row above ran three separate greps — `pick::`-any-symbol,
bare `crates/viewer/src/pick.rs`, and each moved type name — and
checked each hit's item status before deciding. What it still cannot
reach is a row that names the module in prose without a path or a
symbol, which is precisely the `session::op` README row's shape.

## Three classes the line-number sweep cannot fix, from #2103's fix pass

#2103's fix pass re-derived every `<file>.rs:<line>` citation in
`work/view/`'s open items into the five files that PR changed:
**25 citations corrected across 12 items, and 20 of the 25 were
already wrong at the merge base** — `frame.rs:1440` was `:1537` and
`frame.rs:1652` was `:1784`, both off by roughly eighty lines. So the
general case this row names is not a residue of one split; it is the
tracker's steady state between sweeps, and a sweep that runs only when
a PR moves code will always find more than that PR moved.

The same pass hit three citations it could not repair, and each is a
different reason:

**1. A citation whose SUBJECT is gone, not moved.**
`work/view/cursor-projection-is-f32-in-a-module-whose-matrices-are-f64.md:16`
cites `camera.rs:878-881` for a quoted passage — *"the matrix it
transforms is the one `Camera::view_projection` produces…"* — that
exists nowhere in the repository, at `e42cb5e46` or at head, checked
over `crates/` and `docs/` at both. Re-deriving it needs a judgement
about what the author meant, and guessing is #2083's defect in the
other direction: a citation repaired to point somewhere plausible is
worse than one visibly broken.

**2. A citation into a file the sweeping PR did not touch.**
`work/view/free-move-drag-dissolved-by-open.md:18` cites
`session.rs:1319` for `clear_for_new_document`, which is at
`session.rs:1583`. Nothing was wrong with the sweep — `session.rs` was
simply outside its five files. **A citation sweep scoped to a PR's own
diff cannot converge**, because the rows it leaves are the ones no
future PR has a reason to look at either.

**3. Text that is not a citation at all.**
`work/view/vocabulary-macro-bodies-are-outside-rustfmt.md:27` embeds a
pasted `cargo fmt` diff header (`Diff in crates/viewer/src/blend.rs:173:`).
It is a captured tool transcript recording what the tool said at the
time, and repointing it would fabricate output. A sweep that matches on
`<file>.rs:<line>` cannot tell this shape from a citation, so it must
be able to leave one alone — which means the sweep needs a disposition
step and not only a matcher.

Class 3 is the one that constrains the fix: any mechanical repointer
built for this row will match transcripts, and a repointer that edits
them is worse than no repointer.

## Two rows found wrong and deliberately left as written (#2272, 2026-09-09)

#2272 enumerated every `app.rs:NNN` citation in `work/view/` and read
each at its base line (`1d29a8eeb`). Two OPEN rows were wrong for
reasons the sweep could not repair, and both are recorded here rather
than repointed — **not because a sweep may leave a row it found, but
because these two are the shapes this item already says a repointer
must be able to leave alone.**

| File | What it cites | What is actually there | Disposition |
|---|---|---|---|
| `free-move-drag-dissolved-by-open.md:53-55` | *"`frame::supersession_notice` already renders it, `app.rs:785`"* | **the symbol does not exist**: `frame::supersession_notice` has zero definitions anywhere in the tree, and its only occurrence anywhere under `crates/` was a doc comment at `crates/viewer/src/session/op.rs:742`, which #2272 corrects in the same PR as this row (`doc-comments-name-symbols-that-do-not-exist`). `app.rs:785` at `1d29a8eeb` was `match index.scene_focused(…)` inside `sync_scene`; at #2272's head it is `let Some(index) = self.picks.index() else {`. The real renderer is `frame::Withdrawal::superseded` (`frame.rs:699`) through `Withdrawal::notice` (`frame.rs:714`), reached at `app.rs:932-935` | **left as written.** Repointing a wrong SYMBOL makes it more dangerous, not less — this item's own class-3 argument, and the `save-is-not-gesture-guarded` case above: a citation that resolves reads as checked |
| `new-document-owes-the-reframe-open-gets.md:18,20` | `app.rs:778` for `let opened = matches!(op, SessionOp::Open(_));`, and `:794-798` for the `None if opened` arm it quotes | both name the wrong subject at the base and at head. `let opened` was `app.rs:918` at `1d29a8eeb` and is `:925` at #2272's head; `app.rs:778` at base was `let Some(index) = self.picks.index() else {`. The quoted `None if opened` block was `945-948` at base and is `952-955` at head; `794-798` at base was `sync_scene`'s `Ok(mesh)` arm | **left as written.** The row is outside #2272's own diff, which is this item's class-2 case: *a citation sweep scoped to a PR's own diff cannot converge*. The correct numbers are in this row, so the next touch of that file re-points from a record rather than a re-derivation |

### What this member adds

The first is a **fourth** class the line-number sweep cannot fix, and
it is not any of the three above: a citation whose FILE and LINE are
repairable but whose **named symbol has never existed**. Class 1 is a
subject that is gone; this is a subject that was never there. The two
look identical to a matcher and come apart under a reader, and the
disposition is the same for a different reason — class 1 cannot be
repaired without guessing what the author meant, this one *can* be
repaired and must not be, because repairing the number leaves a
sentence naming a symbol that does not exist and now points somewhere
real.

The second is the one that says why this section exists at all. #2272
found both, judged both correctly, and wrote neither down: its PR body
said only that *"most were already stale"*. `docs/prompts/implementer-
discipline.md` §6 is explicit that a residue disclosed inside a
program's own fence owes a file in the same PR, and that a PR-body
sentence is not one. This item is the file, and *"only `work/view/*` is
this program's to fix, and its own citations are corrected as they are
touched"* (`:40-41` above) is the clause that makes it this item and
not another.

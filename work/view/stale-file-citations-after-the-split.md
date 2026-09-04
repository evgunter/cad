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

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

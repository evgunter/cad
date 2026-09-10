---
id: named-not-linked-is-a-silent-disposition-at-eleven-of-thirteen-sites
kind: issue
title: eleven bare own-module spans are deliberate and say so nowhere, in a crate with three module headers that explain the same choice
status: open
opened: 2026-09-10
---


Filed by `doc-comments-name-symbols-that-do-not-exist`, which created
eleven of these and could not afford to annotate them in the same PR.
See `rustdoc-posture-test-names-one-axis-of-three` for why the sites
exist at all.

## The shape

A bare code span in a doc comment reads two ways and the reader cannot
tell them apart: **nobody thought to link it**, or **it must not be
linked**. In this crate the second is a real and load-bearing category
— a name in an `app`-feature module cannot be linked from a
renderer-free one, because `scripts/doc-gate.sh` documents `viewer` at
DEFAULT features too — and the crate has a written convention for
saying so:

- `theme.rs:9-12` — *"(Named, not linked: both modules sit behind the
  `app` feature, so an intra-doc link to either breaks rustdoc exactly
  in the headless pass this header celebrates — issue #1330.)"*
- `vocab.rs:51-52` — *"plain code spans and not links, because `forms`
  is behind the `app` feature and a link to it does not resolve in a
  default-feature build"*
- `forms.rs:18-20` — the same move for a different reason
  (`pub(crate)` items).

**The convention exists. It is applied at 2 of 13 sites.**

## The population, and the rule that produces it

Rule: every unbracketed backtick span in a `///` or `//!` line under
`crates/viewer/src` whose whole content is `<mod>::<path>` with `<mod>`
one of this crate's own modules (`lib.rs`'s `mod` list). Read
2026-09-10 at `origin/main` plus `view/dead-symbols`: **20 spans**, in
two categories.

**Category A — cannot be linked, feature axis (13 spans).** The target
is in an `app`-only module and the citing module is rendered at default
features:

| Site | Span | Note? |
|---|---|---|
| `frame.rs:6` | `app::ViewerBehavior::viewport_ui` | silent |
| `frame.rs:85` | `pane::viewport` | silent |
| `frame.rs:216` | `app::ViewerBehavior::viewport_ui` | silent |
| `frame.rs:385`, `:554`, `:1802` | `pane::viewport::land` | silent |
| `frame.rs:1766` | `app::run` | silent |
| `pickindex.rs:12`, `:13` | `pane::viewport` | silent |
| `props.rs:40` | `app::FieldWriting` | silent |
| `tree.rs:278` | `app::indeterminate_wording` | silent |
| `vocab.rs:50` ×2 | `forms::BOOLEAN_OPS`, `forms::MATE_PRIMITIVES` | **noted at `:51-52`** |

**The sharpest is `pickindex.rs:12-13`**, where `[`crate::marks`]` and
a bare `pane::viewport` sit in one sentence with nothing saying which
is deliberate. A reader repairing the "inconsistency" reds the gate.

**Category B — rendered by no pass (7 spans).** Doc comments inside
`#[cfg(test)]` modules, which `cargo doc` never renders:
`frame.rs:2001`, `:2233`, `:2237`, and `pane/viewport.rs:525`, `:562`,
`:563`, `:568`. A note here is worth less — nothing renders the prose
either — and the honest fix may be the other one, since ten bracketed
links remain in `cfg(test)` comments elsewhere in the same two files.
`frame.rs:2232-2233` spells both rules inside one comment.

## Why it is not folded into the PR that made it

Every candidate site for the note is a module header or an item doc,
so the note **adds lines**, and these are among the most heavily cited
files in the tracker: `frame.rs` **71** `file:line` citations across
`work/` and `docs/`, `pickindex.rs` **37**, `props.rs` **30**,
`tree.rs` **17** — 155 in four files. A note costs the census of every
band it shifts, and `plan.md` is emphatic that a re-point derived
wrongly is worse than a number left stale, because a fresher wrong
number hides the breakage. That census is this item's real content and
it is a unit, not a paragraph.

**The cheap shape, if a unit wants one:** a single module-header
sentence per file in the theme.rs voice, four files, one census. Not a
per-site comment — the disposition is a property of the module pair,
not of each sentence.

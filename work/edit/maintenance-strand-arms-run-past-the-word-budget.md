---
id: maintenance-strand-arms-run-past-the-word-budget
kind: issue
title: Maintenance's Strand and StrandedAppearance arms render 54 and 62 words
status: open
opened: 2026-10-01
---


(PROPS's recourse-grammar lane, from the re-derived census
`work/props/props-refusal-prose-outgrows-the-viewer` describes: every
`impl Display for` block in `crates/*/src`, split into match arms,
counting each arm's string literals with a `{…}` placeholder as one word
and named recourse constants NOT expanded. The standard — 75 words on the
RENDERED text, with its test — is stated once in
`work/chrome/error-and-check-text-overflows-its-region.md`, section "The
standard a refusal is rewritten to".)

## What

In `crates/editor-core/src/edit.rs`, `Maintenance`'s `Display`:

| words | arm |
|---|---|
| 62 | `Self::StrandedAppearance` |
| 54 | `Self::Strand` |

Both are over the 50-word line the census lists at, before any payload.

## Why it is filed here

`crates/editor-core/src/edit.rs` is `edit`'s by territory.

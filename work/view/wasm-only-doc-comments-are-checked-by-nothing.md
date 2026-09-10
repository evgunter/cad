---
id: wasm-only-doc-comments-are-checked-by-nothing
kind: issue
title: a doc comment on a wasm-only item is rendered by no doc pass this repo runs, so nothing checks it
status: open
opened: 2026-09-10
---



Disclosed by `viewer-docs-do-not-build-at-wasm32`'s closing unit, which
ruled the viewer's rustdoc a host artifact. That ruling names this blind
spot as its stated cost; this file is the schedule for it.

## What

A doc comment attached to an item behind `#[cfg(target_family = "wasm")]`
is read by **no** rustdoc pass this repo runs:

- `scripts/doc-gate.sh` builds at the host target, where the item does
  not exist, so the comment is never parsed and its links are never
  resolved.
- The browser pass that would parse it is not run by anything, and
  `crates/viewer/README.md`'s GQ6 ruling says so deliberately.

The receipt, at `ac4a69dd5` plus this unit's two de-links:
`/…/doc/viewer/app/fn.run.html` exists in the host build and
`fn.run_web.html` does not; at `wasm32-unknown-unknown` it is the other
way round. So `run_web`'s doc is rendered only by a pass nobody takes.

## Why it is worth a file rather than a shrug

Two of the nine links the closing unit measured were in exactly this
class — `[`run`]` twice inside `run_web`'s own doc — and they were dead
in every configuration for as long as they existed. Nothing found them:
the host gate could not see the comment, and the browser pass that could
had never been run until the row that filed it. The population is small
and enumerable today — `crates/viewer/src/app.rs` is the only file with
wasm-only items carrying doc comments, and `crates/viewer/src/bin/viewer.rs`
has six wasm-only sites and no intra-doc link at all — but the checker
count is zero, and a class with no checker grows silently.

## The shape that would close it, and its price

`#[cfg_attr(target_family = "wasm", allow(rustdoc::broken_intra_doc_links))]`
on each item whose both-target doc links a host-only item — four such
items today, the `evalseam` and `app` module docs and `app::evaluator`
and `app::indexer` — makes the browser pass GREEN without weakening the
host pass on those items, because the `allow` is conditional. A CI row
could then hold it, and what that row would gate is precisely this
class: a wasm-only item's doc links would be resolved for the first
time.

The price is why it was not taken here. The `allow` is a per-site tax
levied forever on a crate whose architecture is one seam with two arms,
so it mints host-only links by construction; every future one costs an
attribute or a de-link, and the ratchet's own cost lands on the docs a
reader actually reads. The judgement was that four attributes plus a CI
row is too much standing machinery for a population of one file — but it
is a judgement about today's population, and the population is what
changes.

## Fence

The CI row is `.github/workflows/ci.yml`, which is **CIW's**. Anything
here that reaches the gate is a handoff, not a VIEW edit.

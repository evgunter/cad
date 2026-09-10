---
id: rustdoc-posture-test-names-one-axis-of-three
kind: issue
title: the host-page test brackets links the gate then reds: it asks about the item, and two of the three ways a page goes missing are about the target
status: open
opened: 2026-09-10
---


Found by `doc-comments-name-symbols-that-do-not-exist` while applying
the ruling it was told to apply, on the day that ruling merged.

## The ruling, and what it says to do

`crates/viewer/README.md`'s **Rustdoc posture: the host pass is the
gate** decides which code spans in this crate may become intra-doc
links. Its test:

> Ask whether the host pass renders a page for the item the doc
> comment sits on […] **It does** — the host pass holds that link, and
> a browser-pass error on it is by construction. Permitted.

Read as written, *"it does → bracket freely"* is a sufficient
condition. **It is not**, and following it reds `scripts/doc-gate.sh`.

## What happened

The closing pass bracketed 64 spans, checked every one against the
host pass at `--all-features` — the configuration the ruling's own
instrument names — and got zero errors. The gate then reded on **13**
of them.

The reason is that the gate documents `viewer` **twice**. CI's row is
`scripts/doc-gate.sh --pr --scope … --skip-viewer-toolkit`, and that
flag documents `viewer` at **DEFAULT features** as well, at
`-D warnings`; its own error message says so — *"rustdoc rejected the
viewer pass at DEFAULT features — its renderer-free modules are gated
on every run"*. `mod app`, `forms`, `pane`, `drafts`, `widgets` and
`gpu` are all `cfg(feature = "app")`, so at default features they do
not exist, and a link from a renderer-free module into any of them
resolves at `--all-features` and nowhere else.

The 13 were in `props.rs`, `tree.rs`, `vocab.rs`, `pickindex.rs` and
`frame.rs` — every one a module that is documented at default features
— pointing at `app::*`, `forms::*` and `pane::viewport::*`.

## The three ways a page goes missing, and only one is in the ruling

The ruling's test is **page existence**, and that is right. What it
under-specifies is *whose* page and *in which pass*:

1. **The item is target-gated** (`WebStartupError`, `run_web`). The
   ruling's case, stated well. Decided by the item the comment sits on.
2. **The link's target is feature-gated out of a pass that runs at
   `-D warnings`.** The item has a page in both passes; the TARGET
   does not exist in one of them. Not the item, and not mentioned.
3. **Neither pass renders the item at all** — a doc comment inside a
   `#[cfg(test)]` module. Seven such spans were in the closing pass's
   population (`frame.rs`'s `mod tests` and `pane/viewport.rs`'s).
   Bracketing them is silently inert: no pass reads them, so the link
   is never checked and never reds. This is the ruling's own *"a
   bracket resolving at neither target spells a checked claim nothing
   anywhere checks"* — reached by a third route it does not name.

Cases 2 and 3 are about the **target** and about **which passes run**;
case 1 is about the item. A reader applying the ruling literally gets
case 1 right and cases 2 and 3 wrong, which is what happened.

## Why this is a ruling question and not a lane fix

The closing pass took the safe disposition — **name, do not link** — at
all 20 sites, which is the ruling's own remedy for a page that does not
exist, and the gate is green. So nothing is broken. What is wrong is
the **test**, and it is ratified text: `docs/DESIGN.md`'s convention
puts design decisions through Ev rather than through a lane's diff,
and this one was ratified this morning.

The proposed revision is small and does not change any disposition
already taken — it states the test over the passes the gate actually
runs rather than over one of them:

> Ask whether **every rustdoc pass that runs at `-D warnings`** renders
> a page for the item the doc comment sits on **and can resolve the
> link's target**. For this crate that is two passes, `--all-features`
> and default features, and a doc comment inside `#[cfg(test)]` is in
> neither.

The instrument stays what it is; only its quantifier changes. Worth
Ev's eye because *"bracket freely"* is the half people will quote.

---
id: rustdoc-posture-test-names-one-axis-of-three
kind: issue
title: the posture ruling says "the host pass" and doc-gate runs two of them, so a link can pass the ruling's test and red the gate
status: open
opened: 2026-09-10
---

Found by `doc-comments-name-symbols-that-do-not-exist` while applying
the ruling it was told to apply, on the day that ruling merged. **This
row is a question for Ev**, because the text it proposes to change is
ratified.

## The ruling, and the word that does the damage

`crates/viewer/README.md`'s **Rustdoc posture: the host pass is the
gate** decides which code spans in this crate may become intra-doc
links. Its instrument:

> Ask whether the host pass renders a page for the item the doc comment
> sits on — the same `cargo doc` without `--target`, then look for the
> page under `doc/viewer/`; it is there or it is not

**There is no such thing as "the host pass" in this crate.**
`scripts/doc-gate.sh` documents `viewer` twice at `-D warnings`:

- the workspace pass at `--all-features`, and
- a second pass at **DEFAULT features**, added by `--skip-viewer-toolkit`
  (`scripts/doc-gate.sh:897`), whose own error message names itself —
  *"rustdoc rejected the viewer pass at DEFAULT features — its
  renderer-free modules are gated on every run"*.

`app`, `drafts`, `forms`, `gpu`, `pane` and `widgets` are all
`#[cfg(feature = "app")]` (`crates/viewer/src/lib.rs:82-93`), so at
default features they do not exist. A link **from** a renderer-free
module **into** any of them resolves at `--all-features` and nowhere
else: it passes the ruling's test as written and reds the gate.

## What it cost, measured

The closing pass bracketed 64 spans, checked every one against
`--all-features` — the configuration the ruling's own instrument names
— and got **zero errors**. The gate then reded on **13**, in
`props.rs`, `tree.rs`, `vocab.rs`, `pickindex.rs` and `frame.rs`,
pointing at `app::*`, `forms::*` and `pane::viewport::*`.

**The crate already knew.** Three module headers say it in prose:
`theme.rs:9-12` (*"both modules sit behind the `app` feature, so an
intra-doc link to either breaks rustdoc exactly in the headless pass
this header celebrates — issue #1330"*), `vocab.rs:51-52` (*"because
`forms` is behind the `app` feature and a link to it does not resolve
in a default-feature build"*) and `forms.rs:18-20`. The practice is
established in the code; the ruling that now governs the practice does
not state it, and one of those 13 reds was a bracket placed **two
lines above the note explaining why it must not be one**
(`vocab.rs:50`).

## Why it is worth Ev's eye rather than a lane's edit

**The defect fires on someone else's branch, not on the branch that
writes it.** `scripts/ci-filter.py --files` over the closing PR's own
diff gives `RUN_VIEWER_TOOLKIT=true`, so `ci.yml:1834` runs
`--pr --scope` **without** `--skip-viewer-toolkit` — viewer at
`--all-features` only. The 13 reds that whole finding turns on would
**never have appeared on that PR's CI.** They surface later, on a
branch that reaches `viewer` through the dependency closure without
seeding it, where the author has no reason to look at viewer's doc
comments at all. A rule whose violations land on a stranger is worth
more care than one that reds its own author.

## The proposed revision, and its quantifier

The instrument stays exactly what it is. What changes is what it is
quantified over — and the quantifier must be **existential**, not
universal:

> Ask whether **some** rustdoc pass that runs at `-D warnings` both
> renders a page for the item the doc comment sits on **and** resolves
> the link's target. For this crate that is two passes, `--all-features`
> and default features. It does → bracket. It does not → name the item
> instead.

**Existential, because a link checked by one pass is a checked claim.**
That is the whole value of bracketing: rustdoc read the name and
resolved it, somewhere, at `-D warnings`. Demanding that *every* pass
check it would forbid linking anything behind a feature at all — under
a universal reading the closing PR's own 25 links inside `app`,
`forms`, `pane` and `widgets` all become illegal, because the
default-features pass renders no page for any of those modules. Those
25 are correct and the gate is green over them. A universal quantifier
would also make `app.rs`'s many existing internal links illegal, which
no one intends.

So the shape is: **the pair (pass, item, target) must exist together in
at least one pass.** The failure the ruling is actually about — a
bracket that resolves in NO pass — is exactly the negation of that, and
the existential form states it without collateral.

This revision changes **no disposition already taken** by the closing
PR: all 44 of its links satisfy the existential form, and all 20 of its
named spans fail it.

## Not part of this row

The closing pass also named 7 spans inside `#[cfg(test)]` modules.
**That is not a gap in the ruling** — `cargo doc` does not set
`cfg(test)`, so such an item has no page under `doc/viewer/` and the
ruling's literal answer is *it does not*, which is the remedy that was
taken. The pass reached the right disposition by the wrong reading: it
asked the question of the **module** rather than of the **item**, which
is why all 64 first answered *it does*. Recorded so a reader of that
PR does not inherit the mistake, and deliberately not filed as a defect
in the ruling, because the ruling is right about it.

One residue of that, left as a taste question rather than a claim: ten
bracketed links remain in `cfg(test)` doc comments (`frame.rs:2018`,
`:2019`, `:2148`, `:2228`, `:2232`, `:2347`, `gpu.rs:1466`, `:1467`,
`:1567`, `:1570`) and are inert by the same argument, so `frame.rs`
now spells one rule two ways inside a single comment at `:2232-2233`.
Whether `cfg(test)` prose should link at all is a question for whoever
answers this row.

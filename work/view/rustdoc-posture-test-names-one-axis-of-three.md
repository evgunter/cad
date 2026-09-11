---
id: rustdoc-posture-test-names-one-axis-of-three
kind: issue
title: the posture ruling says "the host pass" and doc-gate runs two of them, so a link can pass the ruling's test and red the gate
status: open
opened: 2026-09-10
needs_ev: true
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

## Changing the TOOL is the alternative, and the two are one decision

**Asked by Ev on 2026-09-11: why revise the rule rather than change how
rustdoc runs for this crate?** The row did not cost that, and costing it
changes the answer.

**`--all-features` renders a strict superset of what default features
renders.** `viewer`'s `app` feature is purely additive — every entry is
a `dep:` (`crates/viewer/Cargo.toml`) — and there is **no**
`cfg(not(feature = "app"))` anywhere under `crates/viewer/src` (zero
hits). So no item exists at default features and not at
`--all-features`, and no target resolves there and not here.

It follows that the default-features pass's `broken_intra_doc_links` can
fire on exactly one thing the `--all-features` pass cannot: **a link
whose item is renderer-free (rendered by both passes) and whose target
is `app`-gated (resolved by one).** There is no third category. A
renderer-free → renderer-free link breaks in both; an `app`-gated item's
links are not rendered by the default pass at all.

**So the two options are the same decision wearing different clothes.**

- **Change the tool.** Allow that one lint on that one pass — which is
  what `RUSTDOC_LINTS_INERT` (`scripts/doc-gate.sh:559`, used once at
  `:974`) already does for pass 3's feature axis. By the superset
  argument it costs *nothing else*: that lint has no other work on that
  pass. Then there is one link-checking pass, **the ruling's original
  wording is true exactly as ratified**, and no revision is needed.
- **Keep the tool.** Then the rule must say what the tool enforces.

Both answer one substantive question: **may a doc comment in the
renderer-free half link into the toolkit half?**

## What the tool change actually costs (Ev, 2026-09-11)

*"Costs nothing else"* above was true of lint coverage today and false
of the consequences. Three, and the first is the real one.

**1. The pass stops having a reason to exist.** The default-features
viewer pass is `-p viewer` alone (`scripts/doc-gate.sh:900-902`), it
renders a strict SUBSET of the items `--all-features` renders (zero
`cfg(not(feature = "app"))`), and the doc TEXT of a rendered item is
identical in both passes (zero `#[doc = …]`, zero `cfg_attr(…, doc)`).
Every other rustdoc lint is a property of the doc text, so it fires the
same way in both. **`broken_intra_doc_links` on this class is the pass's
only unique detection.** Make it inert and the pass can never red on
anything `--all-features` would not — it is dead weight, and the honest
follow-on is to delete it rather than quiet it. Deleting it is the
larger decision: it is the only thing standing between the
renderer-free half's docs and promises about the toolkit half.

**2. Three module notes become false.** `theme.rs:9-12`,
`vocab.rs:51-52` and `forms.rs:18-20` each say the link breaks the
headless pass. After the change that is no longer true, and a false
comment in code is worse than no comment — so they retire in the same
change, taking #1330's recorded reason with them.

**3. The superset argument has an expiry date.** It is a property of
today's tree, not a guarantee. Add one `cfg(not(feature = "app"))` arm
and the default pass becomes the only renderer of it — with the lint
inert, its links go unchecked silently. Keeping the pass then wants a
guard that no such arm exists; deleting the pass leaves such an arm with
no doc pass at all. Either way the change is not self-maintaining.

A fourth, minor: someone running `cargo doc` on `viewer` without `app`
— the reader that half of the crate exists for — gets a rendered page
with a link that goes nowhere. No error, because they are not at
`-D warnings`; just a dead link.

**This is why I recommend the rule change.** It costs one clause and
leaves the tool, the notes and #1330's reason alone.

## The recommendation, corrected

**It is the second, and the quantifier is CONDITIONAL — not existential,
and not universal.**

> Ask, of **every** rustdoc pass at `-D warnings` that renders a page
> for the item the doc comment sits on, whether that pass resolves the
> link's target. All of them do → bracket. Any one does not → name the
> item instead.

- An `app`-gated item's internal links: only `--all-features` renders
  the item, and it resolves the target. Vacuous at default features.
  **Permitted** — the closing PR's 25 stay legal.
- Renderer-free → `app`-gated: both passes render the item, the default
  one cannot resolve the target. **Forbidden** — the 13 stay named.
- A `#[cfg(test)]` item: no pass renders it, so a bracket there is inert
  rather than illegal.

**The existential form this row first proposed is wrong, and wrong in
the direction that matters.** Under it the all-features pass both
renders the item and resolves the target for all 13 of the
renderer-free → `app`-gated spans, so the rule *permits* precisely what
the gate *reds on* — rule and tool would disagree, which is worse than
the ambiguity being fixed. The claim below that all 20 named spans fail
it is false for 13 of them.

## Why keep the tool rather than quiet it

**The crate answered this three times, in prose, before any ruling
existed.** `crates/viewer/src/theme.rs:9-12`:

> (Named, not linked: both modules sit behind the `app` feature, so an
> intra-doc link to either breaks rustdoc exactly **in the headless pass
> this header celebrates** — issue #1330.)

`vocab.rs:51-52` and `forms.rs:18-20` say the same for their own
neighbours. The headless pass is not incidental to the renderer-free
half — it is the configuration that half exists for (*"the palette
compiles, and is asserted on, in ordinary headless CI with no toolkit
graph present"*), and #1330 already settled that a link into the gated
half breaks it. Someone building against `viewer` without the toolkit
reads those pages, and a link there points at an item their build does
not have.

Quieting the lint would overturn established practice to make a
sentence shorter. Codifying it costs one clause.

## What either answer does to the dispositions already taken

**The conditional form changes none of them.** All 44 of the closing
PR's links satisfy it and all 20 of its named spans fail it — which is
what makes it a codification rather than a change.

**Quieting the lint would change 13.** Those spans become linkable, and
`theme.rs`, `vocab.rs` and `forms.rs` would each want their prose note
retired in the same pass, since the reason those notes give would no
longer hold.


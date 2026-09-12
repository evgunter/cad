---
id: rustdoc-posture-test-names-one-axis-of-three
kind: issue
title: the posture ruling says "the host pass" and doc-gate runs two of them, so a link can pass the ruling's test and red the gate
status: closed
opened: 2026-09-10
closed: 2026-09-11
pr: 2332
branch: view/link-thirteen
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

## Ev, 2026-09-11: take the links AND delete the pass — and the spelling settles the last objection

Ev's reading: better linking *and* a redundant pass deleted is a win,
not a trade. **Measured, it is** — and the rendering objection above
belongs to a spelling, not to the plan.

Both forms of the same link, rendered rather than argued:

| spelling | `--all-features`, `-D warnings` | default features, lint inert |
|---|---|---|
| ``[`crate::pane::viewport`]`` | resolves | prose reads `[crate::pane::viewport]` — **literal brackets leak** |
| ``[`pane::viewport`](crate::pane::viewport)`` | resolves to `../pane/viewport/index.html` | prose reads `pane::viewport` — **identical to today**, dud href only |

So the plan is: **make the lint inert on the default viewer pass,
delete that pass as dominated, and spell these thirteen in the
reference form.** All three of the costs recorded above dissolve except
one — the notes at `theme.rs:9-12`, `vocab.rs:51-52` and
`forms.rs:18-20` retire, which is a gain rather than a cost, since what
they document stops being true.

**The one real residue.** Deleting the pass means a future
`cfg(not(feature = "app"))` arm gets no doc pass at all, silently —
today there are zero, so the pass is dominated, but that is a property
of this tree and not a guarantee. Either keep the pass with the lint
inert (costs CI, keeps every non-link lint over such an arm) or delete
it and guard that no such arm can arrive unnoticed. The choice is
small; making it silently is the part that is not.

**On spelling two ways.** `[`crate::X`]` is 1022 sites repo-wide and
the closing PR's lane declined the reference form on consistency
grounds. The reason to split is now measured rather than aesthetic, and
it is narrow: the reference form is for a link crossing into a
feature-gated half some pass does not render. That is a rule, not a
preference, and it belongs at the site.

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


## Closed (2026-09-11)

**Ev ruled: link the thirteen, and make the link lint inert on the
skip-mode viewer pass.** Done here. What follows records the ruling as
carried out, including the two places it differs from the plan this
file drafted above — both differences are corrections to this file, not
departures from the ruling.

### What was done

- **The thirteen are links**, in the house spelling ``[`crate::X`]``.
  Re-derived at `6891829ee` by the rule this file's companion states —
  every unbracketed backtick span in a `///` or `//!` line under
  `crates/viewer/src` whose whole content is `<mod>::<path>` with
  `<mod>` an `app`-gated module — and the list had not moved:
  `frame.rs:6`, `:85`, `:216`, `:385`, `:554`, `:1766`, `:1802`;
  `pickindex.rs:12`, `:13`; `props.rs:40`; `tree.rs:278`;
  `vocab.rs:50` ×2. The three inside `#[cfg(test)]` were left alone.
- **`scripts/doc-gate.sh`'s skip-mode viewer pass runs
  `RUSTDOC_LINTS_INERT`**, with Ev's ruling cited at the site and the
  coverage it drops stated there rather than left to be found. CIW owns
  that file; the change is announced on their slate as
  `view-made-the-skip-mode-viewer-doc-pass-lint-inert`.
- **Two module notes retired**, not three. `theme.rs:9-12` and
  `vocab.rs:51-52` said a link into the gated half breaks the headless
  pass; that is no longer true and they are gone, with `theme.rs`'s two
  bare module spans (`app`, `gpu`) linked in the same edit. **This file
  was wrong about `forms.rs:18-20`**: that note's reason is
  `pub(crate)` items on a public module page, not the headless pass —
  the companion row read it correctly and this one did not. It is still
  true and it stays.
- **`crates/viewer/README.md`'s posture section** now says there are two
  host passes and which one is the link gate; the heading lost the word
  this row was filed about. #1330's recorded reason lives there now,
  in the DEFAULT-features bullet.

### The two corrections to the plan above

**The pass is NOT dominated and was NOT deleted.** *"What the tool
change actually costs"* argues the default-features viewer pass renders
a strict subset of `--all-features` and so becomes dead weight. The two
viewer passes are the `if` and the `else` of one branch
(`scripts/doc-gate.sh:890-907` at `6891829ee`) and **never run on the
same invocation**: under `--skip-viewer-toolkit` the all-features
invocation does not name `viewer` at all, so on a skip-mode run the
default-features pass is the only rustdoc that reads this crate. It
still carries every lint that is not about a link target. Deleting it
would have removed the crate's only doc gate on exactly the runs it was
built for.

**The spelling is the house one, not the reference form.** The table
above recommends ``[`X`](crate::X)`` to keep default-features prose
reading as it does today. Ev, on the residue: *"totally fine for
`cfg(not(feature))` stuff to work badly — we already assume that
several places."* So the leaked brackets are accepted, the 1022-site
house spelling holds, and the crate does not grow a second link form
whose rule lives only in a closed tracker row.

### The evidence, with the commands

Red before the gate change, with the links in place; green after. Both
at `--pr --scope '-p viewer' --skip-viewer-toolkit`.

- **Before**: exit 1, `rustdoc rejected the viewer pass at DEFAULT
  features`, 15 distinct sites — the thirteen plus `theme.rs:7` and
  `:8`, the two spans linked when that note retired.
- **After**: exit 0.
- `--pr --scope '-p viewer'` (the non-skip path CI actually takes on
  any viewer diff): exit 0, all thirteen resolve.
- `--selftest`: exit 0, with both directions on the link lint pinned —
  a bare URL in the fixture's `viewer` member still fires in skip mode,
  a planted broken link there deliberately does not, and the same
  planted link fires in non-skip mode as the control.

**This PR's own CI cannot exercise the change it makes**, which is why
the local runs are recorded here: `scripts/ci-filter.py --files` over a
diff touching `crates/viewer` sets `RUN_VIEWER_TOOLKIT=true`, so
`ci.yml:1834` takes the non-skip path and the skip-mode pass never runs.

### The re-sweep before landing found a fourteenth, and `main` red

A sweep is accurate as of its merge base, not its merge. Re-run after
merging `origin/main` (#2320, `view/cancel-doors`), the population is
**fourteen**, not thirteen: `session/op.rs:797` names
`` `pane::create` `` in a production `///` comment, and that file
arrived with the merge. Linked with the rest.

**And the merge brought a live instance of this row's whole thesis.**
`session/op.rs:773` carries `` [`crate::widgets::drag_gesture_ops`] ``
— a renderer-free module linking into an `app`-gated one, as a LINK —
so **`origin/main` is red on the skip-mode viewer pass today**:

```
$ scripts/doc-gate.sh --pr --scope '-p viewer' --skip-viewer-toolkit
error: unresolved link to `crate::widgets::drag_gesture_ops`
ERROR: doc-gate: rustdoc rejected the viewer pass at DEFAULT features
exit 1
```

Taken at `origin/main` in a throwaway worktree with its own target dir.
It never appeared on #2320's CI for exactly the reason this row was
filed: that PR's diff touched `crates/viewer`, so
`RUN_VIEWER_TOOLKIT=true` and `ci.yml:1834` took the non-skip path. The
red is waiting for the next branch that reaches `viewer` through the
dependency closure without seeding the toolkit — a stranger's branch,
which is the argument *"why it is worth Ev's eye rather than a lane's
edit"* made in the abstract and is now a fact about `main`. **This PR
clears it**, as a side effect of the ruling rather than as a fix aimed
at it.

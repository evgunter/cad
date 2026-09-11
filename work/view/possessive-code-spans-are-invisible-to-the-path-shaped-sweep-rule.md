---
id: possessive-code-spans-are-invisible-to-the-path-shaped-sweep-rule
kind: issue
title: a possessive two-span spelling names an app-gated item and no sweep in this crate has ever seen one — five sites, one naming a symbol that does not exist
status: open
opened: 2026-09-11
refs: [comment-symbol-names-outside-rustdocs-reach-have-no-gate, named-not-linked-is-a-silent-disposition-at-eleven-of-thirteen-sites, 2332]
---


Found by `view/link-thirteen` while re-deriving the thirteen spans Ev
ruled should become links (`rustdoc-posture-test-names-one-axis-of-three`,
closed 2026-09-11). The thirteen came back exactly as filed; a *wider*
sweep, run to check that the narrow one was not a proxy, did not.

## The blind spot

Every sweep this crate has run over code spans in doc comments —
`doc-comments-name-symbols-that-do-not-exist` (closed) and
`named-not-linked-is-a-silent-disposition-at-eleven-of-thirteen-sites`
(closed) — states the same population rule:

> every unbracketed backtick span whose **whole content** is
> `<mod>::<path>`

That rule sees `` `app::FieldWriting` `` and cannot see
`` `app`'s `unit_picker` ``, which names the same kind of thing in two
spans and a possessive. Both are a doc comment pointing at an item
behind the `app` feature; only one of them is ever counted.

## The population, and the rule that produces it

Rule: every `///` or `//!` line under `crates/viewer/src`, outside the
`app`-gated modules, carrying a backtick span whose content is one of
this crate's `app`-gated module names (`app`, `drafts`, `forms`, `gpu`,
`pane`, `widgets`, or one of those with a `.rs` suffix) **immediately
followed by a possessive and a second backtick span**. Read 2026-09-11
at `6891829ee`: **five sites.**

| Site | Span pair | Target |
|---|---|---|
| `blend.rs:169` | `` `app` ``'s `` `unit_picker` `` | **nothing — see below** |
| `pickcache.rs:256` | `` `app` ``'s `` `fit_delta_on_scene` `` | `app.rs:349`, a field |
| `prefs.rs:387-388` | `` `app` ``'s `` `ViewerApp::remember_theme` `` | `app.rs:1038` |
| `scene.rs:887` | `` `app` ``'s `` `fit_delta_on_scene` `` | `app.rs:349`, a field |
| `pickindex.rs:1582` | `` `gpu.rs` ``'s `` `EDGE_CLIP_Z_SHRINK` `` | `gpu.rs:384` |

**What the rule cannot match**, stated because a sweep whose blind spot
is unstated is not a negative result: a span naming an `app`-gated
module in running prose with no second span after it (*"`app` maps a
[`Theme`] onto the chrome"*, `theme.rs:69-70`, `evalseam.rs:4`,
`sketch.rs:264`, `:278`, and the ~35 copies of the boilerplate *"names
no driver type and no `app`-only crate"* module-kind line). Those name
the module as a concept and have no item to point at, so linking them
is a prose question and not this one.

## Why it matters now and not before

Before 2026-09-11 the answer for all five was forced: a link from the
renderer-free half into the gated half reded `scripts/doc-gate.sh`'s
skip-mode viewer pass, so a bare span was the only legal spelling and
its silence cost nothing. Ev's ruling removed that constraint — the
thirteen path-shaped spans became `` [`crate::X`] `` links, and these
five did not, because no sweep has ever been able to see them. So the
crate now spells one relationship two ways with nothing saying which.

## The one that is a defect rather than a disposition

**`blend.rs:169` names `unit_picker`, and there is no `unit_picker`
anywhere in this workspace** (`grep -rn unit_picker --include=*.rs
crates/ demos/ tools/` — zero hits; the word *picker* appears only in
prose, `app.rs:380`, `:1041`, `:1410`, `:1421`, `forms.rs:269`, `:275`,
`:293-294`, `:299`). That is exactly the class
`doc-comments-name-symbols-that-do-not-exist` closed on 2026-09-10 over
64 spans, surviving inside the same crate because that sweep's rule
could not match this spelling. A reader is told where to look and the
place is not there.

`fit_delta_on_scene` is a struct FIELD and not a function, which the
two sentences citing it read as ambiguous either way; noted rather than
filed, since both sentences are true of a field.

## The shape of a fix

Deciding the four live ones is cheap — either link them
(``[`crate::app::ViewerApp::remember_theme`]``, and the two field
citations need a spelling for a private field) or leave them and say
once, somewhere, that a possessive span is prose. The `unit_picker`
site needs its real referent found or the clause rewritten, and that is
the half that should not wait on the other.

## It is a blind spot nobody has listed

`comment-symbol-names-outside-rustdocs-reach-have-no-gate` (open)
carries the enumerated blind spots of the same sweep — 1 through 6 —
and **this spelling is not one of them**. Its blind spot 3 is a span
whose TEXT is split across two `///` lines (measured at zero
own-module names, and that measurement stands); this is a span pair
split across a possessive, on one line, and the per-line scan sees both
halves and matches neither. A gate built to that item's spec — resolve
every `<own-mod>::<ident>` appearing in any comment — would still miss
`unit_picker`, because `unit_picker` is never written with a module
qualifier at all. Whoever builds that gate should decide this case
deliberately rather than inherit it.

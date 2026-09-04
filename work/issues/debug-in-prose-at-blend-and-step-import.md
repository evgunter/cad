---
id: debug-in-prose-at-blend-and-step-import
kind: issue
title: two Display impls render a payload through Debug where a Display exists: sweep BlendSite and step-import's {source:?} on TransformError/EulerOpError
status: open
opened: 2026-09-04
---


Found by FIX's `tier-3-prime-findings-render-through-debug` lane
(PR 1779) with a resolver sweep — every `impl fmt::Display` in
`crates/`, resolving each `{ident:?}` to its declared field type and
asking whether that type is brace-shaped. 370 sites, 32 brace-shaped,
3 fixed by that unit. These two are the remainder that belong to other
programs; reported rather than filed by the lane per
`docs/prompts/implementer-discipline.md` §6, and placed here by the FIX
orchestrator.

## The two

**`crates/sweep/src/blend/mod.rs:949, 955`** — `BlendSite` rendered
through `Debug` inside a `Display`. FILLET's ground.

**`crates/step-import/src/error.rs:455, 461`** — `{source:?}` on
`TransformError` and `EulerOpError`, **both of which already have a
`Display`**. EXCH's ground. This is the sharper of the two: the payload
can name itself and the consumer composes a debug rendering anyway,
which is the exact inversion of the standing rule that the layer which
raised a failure names it.

## Why these are worth taking rather than tolerating

The class this comes from was closed on 2026-09-04 as
`error-types-with-no-display-class`, and its durable finding was that
the census which produced its list was **keyed on the bare type name**
and so missed every type spelled `impl core::fmt::Display for`. That
item's list was stale in both directions. The resolver sweep above is
the method that does not have that failure mode, and these two are what
it turned up outside FIX's fence.

Neither is known to panic today. The tier-3′ case that motivated the
sweep did — `crates/pncad-py/src/errors.rs`'s `reads_as_prose` rejects
the field-brace fingerprint `" { "` and `py::typed_err` asserts it on
every raise, live under release — so **whether either of these reaches
a Python raise decides whether it is cosmetic or a live panic.** That
is the first thing to check, not an afterthought: the `editor-core`
sibling (`SlotId`, `ExprPath`) turned out to be the *same live panic*
rather than a cosmetic one, and is already filed at
`work/docm/debug-in-prose-residue-after-finding-sink.md`.

## Stated blind spots of the sweep that produced this

Carried over so a taker knows what the list does not cover: positional
`{:?}` (51 sites, unresolved); same-named fields collapsing across
variants; comment text inside matched bodies; cross-crate name
collisions; macro and trait-object `Debug`; the `interval` lane; and
**consumers that match a rendered message by substring fragment** —
that last one is not hypothetical, it caught the sweep's own author
(`crates/topo/tests/review_mate9_r1_probes.rs:124` matches
`w.contains("x: 0.45,")`, which contains neither the old spelling nor
the new, so no grep in either direction could see it; hosted CI caught
it).

## Home

`work/issues/` — the two sites are FILLET's and EXCH's respectively.
Re-home by header edit, or split into two items if the programs prefer
to take them separately.

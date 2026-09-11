---
id: debug-in-prose-at-blend-and-step-import
kind: issue
title: sweep's BlendSite renders through Debug inside a Display — a live panic at py::typed_err on any fillet or chamfer escalation at a link or a joint
status: closed
opened: 2026-09-04
refs: [step-import-source-debug-in-prose-panics-the-binding]
branch: fix/blend-site-display
pr: 2367
closed: 2026-09-11
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

## UPGRADED 2026-09-04: both are LIVE PANICS, executed

PR 1779's style review rendered them rather than reasoning about them.
Both reach `typed_err`'s live `debug_assert!(reads_as_prose(…))`:

**`BlendSite`** — all three variants rendered: `Chain` is prose;
**`Link { edge: EdgeKey(..) }` and `Joint { vertex: VertexKey(..) }`
carry `" { "`.** The path is `NodeErrorKind::Blend` →
`eval/mod.rs:1056` (`"the {verb} op refused: {error}"`) →
`py/value.rs::node_failure` → `typed_err(py, .., error.to_string(), ..)`.
**A fillet or chamfer escalation at a link or a joint panics the
binding exactly as `validate_pseudomanifold` did.** `sure` on the
rendering and the raise path; `likely` on reachability — it needs an
indeterminate predicate, which is what escalation is *for*.

**Why nothing caught it, which is its own finding.**
`blend/mod.rs:1195`'s `seeds()` roster looks exhaustive over
`BlendError` and samples `Escalated` with **`BlendSite::Chain` — the
sole brace-free variant.** An exhaustive-looking roster that picks its
own sample excludes the failing mode by construction. Any other
hand-written `seeds()`-style roster in the tree picks its samples the
same way.

**`step-import`** — `EulerOpError::StaleKey { key }` is a struct
variant, so `{source:?}` yields `StaleKey { key: .. }`.
`py/value.rs:1341` raises `err.to_string()` through `typed_err` under a
comment stating *every* arm of `StepImportError` is reachable there.
`sure`.

So the "cosmetic or live" question below is **answered: live**, and
this is the same panic as the one PR 1779 closed, in two more doors.

## The durable fix is not three point fixes

The reviewer's framing, adopted: what is missing is a **mechanical
guard** — a row that renders every `Display`-reachable refusal at every
struct-shaped payload variant and asserts `reads_as_prose` — rather
than another round of point repairs. `crates/pncad-py/src/errors.rs:376`
already writes the general warning ("*A future STRUCT variant of that
kernel enum would trip this assertion and panic where that arm means to
refuse gracefully*"), and there are now **three** instances against one
fix. That guard is cut as its own FIX unit,
`prose-gate-has-no-mechanical-guard`.

The two point fixes below remain FILLET's and EXCH's, and they are
worth taking before the guard lands rather than after — a live panic on
a public door does not wait on a test.

---

Neither was *known* to panic when first filed. The tier-3′ case that motivated the
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

`work/fillet/` — `crates/sweep/` is FILLET's territory and code-quality
Track T, which FILLET claims whole.

**SPLIT 2026-09-04**, taking this section's own offer ("split into two
items if the programs prefer to take them separately"): the
`step-import` half is now
`work/exch/step-import-source-debug-in-prose-panics-the-binding.md` on
EXCH's board, because a live panic on a public door should not wait on
another program's slate. This file keeps its id and stays the carrier
of the **sweep's method and its stated blind spots**, which are
evidence for both halves and which the EXCH item cites rather than
restates. Everything above about `step-import` is retained as that
evidence and is **not this program's to fix**.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/fillet/` in the tracker-wide re-home
sweep of 2026-09-04 (Ev's direction, in-chat). Title narrowed to the
blend half at the same time; the id is unchanged.

## Re-verified on head, and the Home section above names a program that no longer exists (FIX orchestrator, 2026-09-11)

**The panic is still live**, a week after the upgrade note, and the
citations above have drifted. On this head:

- `BlendError::Escalated { site: BlendSite, source }` is declared at
  `crates/sweep/src/blend/mod.rs:1050`.
- It renders at **`:1385` and `:1391`** (not `:949, 955`), inside
  `impl fmt::Display for BlendError` (`:1242`), as
  `"escalated at {site:?}"`.
- `BlendSite` (`:325`) is brace-shaped in two of its three arms —
  `Link { edge: EdgeKey }` and `Joint { vertex: VertexKey }`. Only
  `Chain` is fieldless and safe.
- `py::typed_err` (`crates/pncad-py/src/py/mod.rs:624`) asserts
  `reads_as_prose` on the message. Its own doc (`:610`) states the
  thing that makes this a shipping defect rather than a test-only one:
  *"It is a `debug_assert`, which in this workspace is live in every
  profile — the root manifest keeps them on under release too — so it
  runs over every door the Python suite exercises and stays on in a
  built wheel."*

So a fillet or chamfer escalation at a link or a joint panics the
binding in a shipped wheel. The title has said so since 2026-09-04.

**Why it has waited: `## Home` above sends it to `work/fillet/`, and
there is no such directory.** FILLET is gone and `crates/sweep/src/blend/*`
is **BLEND's** glob (`work/blend/program.md`). The 2026-09-04 split
took the EXCH half out on the explicit argument that *"a live panic on
a public door should not wait on another program's slate"* — and then
left this half pointing at a slate that does not exist, where it
waited anyway. The re-home sweep moved the file to `work/fix/` and did
not update the prose, so the directory (which is the claim, per
`work/README.md`) and the body have disagreed ever since.

**Disposition: it stays on FIX and is dispatched from here.** Not
re-homed to BLEND — that would restart the wait this item has already
served once. This is precisely the shape FIX's charter describes: one
PR, the fix written, the fence named in the PR body and the owning
program told. The `## Home` and `## SPLIT` sections above are retained
as the record of how it got here; this section is the operative one.

**Live overlap, flagged rather than discovered later:** BLEND's open
PR **#2215** (blend-6 ring clearance, open since 2026-09-08) edits
`crates/sweep/src/blend/mod.rs`, `build.rs` and `surgery.rs`. Whichever
lands second owes the merge-forward.

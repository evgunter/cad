---
id: emit-topo-destroys-the-edge-key-in-a-split-lineage-cycle
kind: issue
title: names/emit_topo.rs:127 discards SplitLineageCycle's EdgeKey, keeping the kind honest and destroying the locator
status: closed
opened: 2026-09-11
refs: [2378]
pr: 2474
closed: 2026-09-13
---


## Finding

Found by WIRE's `names-refusal-carries-cause` lane (PR 2378) in its
sweep — inside `names/` but outside the five files that unit touched,
so it is this program's and not a handover. Accurate at `af8bbca`.

`crates/editor-core/src/names/emit_topo.rs:127` `map_err`s away a
`SplitLineageCycle { edge }`, discarding the `EdgeKey`, and raises
`NamingError::Emission`.

The lane calls it a **weaker instance** of the class PR 2378 fixed, and
is right about why: unlike `clearance.rs:1979`, the **kind stays
honest** — an emission fault really is what happened. What dies is the
**locator**. A cycle in split lineage is exactly the failure where the
one thing a reader needs is which edge, and it is the one thing the
refusal no longer says.

## Fence

`crates/editor-core/src/names/emit_topo.rs` is in **no open program's
`paths`**, as are `names/geompred.rs`, `emit.rs`, `discriminate.rs` and
`names/README.md` — WIRE's `paths` name only `flush.rs`, `select.rs` and
`table.rs`. PR 2378 already edits three of those unowned files and draws
the fence in its PR body; this row travels the same way. Worth deciding,
when a lane next takes it, whether WIRE should simply widen its `paths`
to `crates/editor-core/src/names/*` rather than draw the fence one file
at a time — that is a `program.md` edit with no design content in it.


## Closed 2026-09-13 (PR 2474)

`NamingError` gained a `SplitLineage(SplitLineageCycle)` sibling with an
`impl From`, so the chase site is `Ok(body.split_root(..)?)` and the
`map_err` closure — the defect's habitat — is gone. The rendered refusal
keeps `Emission`'s framing and adds the locator, and both arms now read
that framing from **one const** rather than hand-writing it twice, which
is what the review caught the first cut doing eight lines apart.

**The sibling was worse than the subject, and it was swept here.**
`chase_b` is a second hand-written split-lineage walk with its own
budget, and on exhaustion it **returned a silently wrong root with no
refusal at all** — on the lane used for exactly the grafted operands
that can produce a cycle. It now raises the same locator. `chase()`
walks FACE fragment rows, so the `EdgeKey` refusal does not fit and a
`FaceKey` one costs a second public Python tag word; that is a
vocabulary decision rather than a mechanical sweep, so it is filed at
`work/wire/face-fragment-chase-returns-a-silently-wrong-root-on-exhaustion.md`
and **the result is labelled a half-fix**.

**What no row catches, stated rather than implied**: reverting either
chase to a silent fallthrough stays green, because neither raise is
reachable from this crate. Both are pinned by the type system — `?` with
no `map_err` to put an underscore in — not by a red row. The
*"unguardable, and here is why"* sentence is at the claim sites, with the
pointer to the live route (`work/bool/graft-copies-provenance-keys-verbatim.md`),
because Q6 asks for it at the claim and not only in a PR body.

**The unit's own unreachability claim was too broad and is narrowed.**
It said no door reachable from this crate constructs a cycling body;
`topo/src/props.rs` and the graft row say cycling lineages are real and
have fired on assembly products. What survives is narrower and is now
written as untested rather than proven: no case constructs a cycle
*this chase* can reach.

Two of the four announced territory crossings turned out to be
**convenient rather than forced** — `emit.rs` already held a
`mod display_tests` inside WIRE's own fence — so the rows moved there,
the new integration suite was deleted, and `git diff main -- tests/all.rs`
is empty. The `pncad-py` pair is genuinely forced.

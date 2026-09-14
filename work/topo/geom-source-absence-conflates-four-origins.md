---
id: geom-source-absence-conflates-four-origins
kind: unit
title: Option<GeomSource>::None covers imported, hand-built, kernel-derived AND a failed re-stamp, so absence cannot be read as any of them
status: closed
opened: 2026-09-12
refs: [2404]
branch: topo/geom-source-typed-absence
pr: 2576
closed: 2026-09-14
---


## Finding

Surfaced while answering Ev's question *"how do we know when there's no
provenance?"* on PR 2404, and filed here by the WIRE orchestrator
because `crates/topo/src/source.rs` is TOPO's. The answer was: **we do
not.**

`Body::surface_source(k)` (`crates/topo/src/body.rs:526`) returns
`Option<&GeomSource>`, and `clear_geom_sources` (`:658`) produces
**exactly the same `None`**. A bare absence conflates four situations:

1. **imported geometry** — never stamped. `import_step` is a free
   function at the kernel seat (`crates/step-import/src/lib.rs:638`);
   there is no `Node::Import`, so `stamp_minted`
   (`crates/editor-core/src/eval/wire.rs:472`) never runs on it.
2. **a hand-built body** — never stamped.
3. **a kernel-derived description** — never stamped.
4. **a description `transform_rigid` cleared and the recipe layer failed
   to re-stamp** — which is a **defect**, and is indistinguishable from
   the three above.

`transform_rigid` (`crates/topo/src/transform.rs:524-530`) clears every
source deliberately and correctly — it rewrites description bits, so the
same-source⇒same-bits claim would become false — and relies on the
recipe layer to re-stamp with `GeomSource::placed` immediately after.
That re-stamp is load-bearing and its failure is silent.

## Why it matters now

`docs/AXIS-DECLARATION-DESIGN.md` is ratified (Ev, 2026-09-12) with
**absence refuses** as one of its three rulings, and this conflation is
the argument that carried it: the one case you would silently tolerate
by verifying numerically instead of refusing is case 4, the bug.
Refusing makes a lost re-stamp loud. That is the right disposition given
the tree as it is — but it is a disposition about a signal that cannot
distinguish a bug from three legitimate states.

## What a taker owes

**Make origin positive.** `None` cannot be made informative by reading
it harder; the repair is to stop inferring origin from a missing entry
and record it — recipe, imported, hand-built, kernel-derived — so
absence becomes unrepresentable and case 4 separates from 1–3.

Two things a taker should weigh rather than assume:

- Whether the mark belongs on the description (beside the source maps)
  or on the body. The maps are per-key and so is the question.
- Whether a positive mark is worth its own channel or should be a
  variant of the source itself — an origin that carries no expression is
  still an origin, and a `GeomSource` whose `expr` says "not from a
  recipe" may be cheaper than a parallel map.

**What this row does NOT ask for** is a guard on the re-stamp alone.
That would catch case 4 and leave 1–3 conflated, which is a half-fix:
the design that depends on this needs to tell an imported body from a
hand-built one, not only from a broken one.

## This row is STEP 1 of a ratified sequence (WIRE orchestrator, 2026-09-12)

Added after cutting `work/wire/axis-shaped-identity-channel.md`, whose
cut section carries the whole sequence, its owners and its ordering.
Recorded here because that row is **parked on this one** and a taker
reading only this file would otherwise take it as a standalone cleanup.

What that means for whoever picks this up:

- `docs/AXIS-DECLARATION-DESIGN.md` is **ratified** (Ev, 2026-09-12) and
  names positive origin marking as *"the first step toward per-component
  provenance"* — this row and the axis channel are the same repair at
  two granularities.
- **`work/exch/step-import-discards-the-entity-ids-that-are-its-identity-channel.md`
  is step 2 and is downstream of the vocabulary you choose here.** It
  fills one of the four origins with real content, from ids
  `import_step` already holds. Choosing the origin representation
  without looking at what the STEP importer can supply risks a mark the
  adoption step cannot write.
- This row's "what a taker owes" already refuses the half-fix (a guard
  on the re-stamp alone, which separates case 4 and leaves 1–3
  conflated). The sequence is why that matters: the channel downstream
  needs to tell an imported body from a hand-built one, not only from a
  broken one.

Nothing here claims TOPO's ground or reorders TOPO's slate — it is a
cross-reference so the sequence survives WIRE's closure.

## Brief (TOPO, 2026-09-14) — block TOPO-B4 slot 0, dual at review

**The answer to give.** A reader of a surface's (or curve's, or
point's) provenance can tell the four cases apart: never stamped
because imported, never stamped because hand-built, never stamped
because kernel-derived, and cleared-by-`transform_rigid`-and-not-re-stamped
— the last of which is a defect no reader can see today. The
identity channel (`crates/topo/src/source.rs`, `Body::surface_source`
and its siblings, `clear_geom_sources`) says absence in a type, not
in `Option::None`: phase 1 decides whether that is an `Origin`-shaped
enum beside `GeomSource` (`Imported`, `Constructed`, `Derived`,
`Cleared { by }`) stamped by the producers that know, or a narrower
change — a distinct `Cleared` marker written by `clear_geom_sources`
so that "cleared and not re-stamped" is at least separable from
"never stamped" — and says why, from what the readers in the recipe
layer (`crates/editor-core/src/eval/wire.rs`'s `stamp_minted` and
whoever answers Ev's question on PR 2404) need. The N6 rule that
`revert` and `transform_rigid` flip or clear a source's tag stays;
the door that clears must leave a trace the re-stamp is expected to
overwrite.

**Rows.** Red-first: `transform_rigid` on a stamped body, no re-stamp
— on the merge base `surface_source` is `None`, indistinguishable from
a hand-built body's; at the head the reader sees the cleared state by
name. Control: a hand-built body reads never-stamped; a re-stamped
body reads its stamp; `revert ∘ revert` and the transform-then-re-stamp
path leave the channel byte-identical to the merge base's stamps.
The recipe layer's existing re-stamp rows stay green.

**Receipt.** Every producer and clearer of a `GeomSource`
(`stamp_minted`, `clear_geom_sources`, `revert`'s tag flip, the graft's
remap, `instance.rs`, `import_step`) with what it writes after the
unit; every reader of `Option<&GeomSource>` with what it does on each
absence kind. `crates/editor-core/src/eval/wire.rs` is WIRE's: the
stamping call is read and, if its signature moves, edited by
announced seam (announce on `work/wire/log.md` in the PR); the
reader that answers Ev's question is reported, not built here.

Branch `topo/geom-source-typed-absence`. PR title: "TOPO: provenance
absence says which absence it is". Do not close the item; the dual
runs at review.

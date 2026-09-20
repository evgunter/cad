---
id: two-emitter-refusals-a-legal-declared-union-reaches
kind: unit
title: Two boolean-emitter Emission refusals a legal declared union reaches: seam vertex parentage underdetermined, and unique_shared_edge over a fragmented merged face
status: closed
opened: 2026-09-07
refs: [2073, 2073]
branch: wire/emitter-refusals-a-legal-union-reaches
pr: 2688
closed: 2026-09-16
---

## What

Two `NamingError::Emission` refusals — the class reserved for a
mint-time fact inconsistent with the result body, a kernel bug by
definition — are reached from ordinary declared unions, measured by
both DOCM-8 reviews on head `6d433b6f`. Neither is in DOCM-8's diff.

1. **`"seam vertex parentage underdetermined from incident edges"`**
   (`crates/editor-core/src/names/emit_topo.rs`, the seam-vertex
   naming arm of `name_boolean_edges`'s vertex pass, ~line 1247).
   Reproducers: R1's split fixture (`a` = x∈(0,1), `c` = x∈(0.5,1.5),
   both y∈(0,1), z∈(0,1); `s` = x∈(0.2,0.4), y∈(0,1), z∈(0.5,1.5);
   `a`–`c` declared on all four flush families, `a`–`s` on the two
   y-walls) in the orders `[c, s, a]` and `[s, c, a]`; R1's
   three-neighbour star in 10 of 24 orders. The vertex where the
   seam of the stacked member meets the merged y-wall has neither one
   operand-descended edge on each side nor two seam lines, so the
   arm's case analysis has no answer and refuses.
2. **`unique_shared_edge`**'s refusal (`crates/editor-core/src/names/emit.rs`
   ~line 382: "shared-edge walk: two shared edges where one
   expected" / "no shared edge") when a merged cap is SPLIT by a
   later member and the cap–wall rim is derived combinatorially.
   Reproducer: R2's `r2_p4` (`a`, `b` flush along x with caps
   declared; a slab `g` = x∈(0.7,0.8), y∈(-1,2), z∈(0.5,3.5) rising
   through the merged top cap; then `c`), which refuses at step 2 of
   `[a, b, g]` before any fourth member is reached. This is also why
   the fragment-of-a-merged-face constituent shape is unreachable
   today (`member-space-look-through-stops-at-splits-containment-and-fragmented-merges`).

## Why it matters

Both are legal documents — nothing in the recipe is malformed — and
an `Emission` refusal tells the author the crate has a bug, not what
to change. Each needs either a naming rule for the shape (a seam
vertex with mixed parentage; a rim shared along two edges after a
split) or a typed refusal that names the construction.

## Where it stands

Open on DOCM's slate for placement (the emitter is `names/`, the
shapes are the union's); not touched by DOCM-8's fix pass.


## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/wire/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the file it names is WIRE's (`names/emit*.rs`, `eval/wire.rs`, `product.rs` are in WIRE's paths). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

(At DOCM's exit sweep, `refs` names the PRs `DOCM-8` stood for: `DOCM-8` = #2073 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Read against the tree (2026-09-15) — both sites live, and one half is dispatchable without a ruling

Read by the WIRE orchestrator before dispatch. Both refusals are where
the row says, modulo line drift:

- `crates/editor-core/src/names/emit_topo.rs` — `"seam vertex parentage
  underdetermined from incident edges"` (now ~1264, filed at ~1247).
- `crates/editor-core/src/names/emit.rs` — the shared-edge walk's
  `"two shared edges where one expected"` and `"no shared edge"` (now
  ~537 and ~542, filed at ~382). Note the walk carries four `bug(...)`
  refusals, not two: `"unmated half-edge"` and `"dangling mate"` /
  `"dangling loop"` sit beside them and were not measured by DOCM-8's
  reviews. Whether a legal document reaches those two is unmeasured, and
  a taker measures it rather than assuming the row's pair is the whole
  set.

**The row offers two answers and only one of them is Ev's.** *"A naming
rule for the shape"* is a design question — it decides what a document
means — and it is the same question as
`member-space-look-through-stops-at-splits-containment-and-fragmented-merges`,
now a ruling. **The other answer needs no ruling at all**: `Emission` is
defined as a mint-time fact inconsistent with the result body, *a kernel
bug by definition*, and these are reached from documents nothing is
wrong with. That classification is therefore simply false today,
whatever the eventual naming rule turns out to be, and a typed refusal
naming the construction is strictly better under either outcome. It also
stops the misclassification being load-bearing: a reader who meets
`Emission` today is told to file a kernel bug.

**Class: M**, and it is takeable now. The unit re-classifies — it does
not invent a naming rule — and its hard half is the measurement: which
of the emitter's `bug(...)` refusals a legal declared document can
actually reach. The row's own reproducers (R1's split fixture in the
orders `[c, s, a]` and `[s, c, a]`; the three-neighbour star in 10 of 24
orders; R2's `r2_p4` at step 2 of `[a, b, g]`) are the starting fixtures
and were executed once, on head `6d433b6f`; a taker re-takes them rather
than citing them.

Ordering note: this is also what makes the fragmented-merge constituent
shape unreachable, so it runs **before** any look-through work the
ruling authorises, not after.

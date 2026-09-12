---
id: two-emitter-refusals-a-legal-declared-union-reaches
kind: issue
title: Two boolean-emitter Emission refusals a legal declared union reaches: seam vertex parentage underdetermined, and unique_shared_edge over a fragmented merged face
status: open
opened: 2026-09-07
refs: [DOCM-8, 2073]
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


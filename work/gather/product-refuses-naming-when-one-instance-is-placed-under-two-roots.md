---
id: product-refuses-naming-when-one-instance-is-placed-under-two-roots
kind: unit
title: product refuses Naming when one instance's names appear under two transform roots, so a document the solve accepts cannot gather
status: closed
opened: 2026-09-05
refs: [does-n3-retire-loudly-generalise-to-the-folds-other-compositions]
priority: P0
cost: H
branch: gather/two-roots-refusal
pr: 3142
closed: 2026-09-25
---


Found by MSOLVE-1's correctness review (PR 1929, NOTE-3); filed by the
MSOLVE orchestrator, owner not obvious (the product's naming rule is
the gather's, the shape is an assembly's).

Document: one instance `top` fed into two transforms `T1`, `T2`, each
mated to its own base. Since MSOLVE-1 both mates are `Determining`, one
cluster, and both seats hold in the solve exactly. `product` then
refuses `ProductError::Naming { node: T1, name: top/… }` because the
instance's names — identical under both roots, N1's pass-through
rule — collide in the product's table. The solve accepts a document the
gather cannot represent. Whether the gather should qualify a
pass-through root's names by the root (the union emitter's
`FromMember` precedent) or refuse earlier and in the recipe's
vocabulary (an instance consumed by two placing roots) is the
question; today the refusal is late and names a collision the author
never authored.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/docm/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). The gather's naming rule is the document layer's (`product.rs` beside `assembly.rs`), and DOCM's `check-registry-gathers-product-twice` and LIB's `[ev]` PR 2020 are already on the same question; MSOLVE's mate consequence is announced.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/wire/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the file it names is WIRE's (`names/emit*.rs`, `eval/wire.rs`, `product.rs` are in WIRE's paths). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Read against the tree (2026-09-15) — live, re-kinded `ruling`

Read by the WIRE orchestrator before dispatch. The refusal site is live
in `crates/editor-core/src/product.rs` — the tie-row narrowing's
`tie_rows.finish(&mut names).map_err(|e| ProductError::Naming { node:
e.name.node, name: e.name })`, after the last source — and the code's own
comment confirms the row's reading of why the refusal is late and
mis-addressed: *"A refusal here names the node that MINTED the colliding
name rather than a root: the collision is between rows that arrived from
different sources, so no one root is its author."*

**Re-kinded from `issue` to `ruling`.** The row poses two answers and
they are not a lane's to choose between: qualifying a pass-through root's
names by the root (the `FromMember` precedent) changes what every
multi-root document's names ARE, and refusing earlier in the recipe's
vocabulary adds a refusal arm to the recipe layer. Either is a decision
about what a document means. What is NOT in doubt, and is worth stating
because it is the reason the row matters: **the solve accepts a document
the gather cannot represent**, so the two layers disagree today whichever
way the ruling goes.

`product.rs` is WIRE's, so the implementing unit lands here once the
ruling does; the shape it lands in is what is being asked.

## RULED (Ev, PR 2677, 2026-09-15) — re-kinded `ruling` → `issue`

Answered by `does-n3-retire-loudly-generalise-to-the-folds-other-compositions`:
a composition that breaks *one name denotes one entity* **refuses**, and
**offers** only where a **unique best** offer exists.

**This row's two options are decided, and the answer is the one already
half-built.** The gather should **refuse** — which `ProductError::Naming`
already does — so the refusal's KIND was never the problem. Qualifying a
pass-through root's names by the root is the other option this row
offered, and it is not available under the rule: it is not an offer, it
is a different naming scheme, and there is no *unique best* root because
both placements are equally the author's.

**What remains is a unit, not a decision.** The row's own complaint
survives intact: the refusal is **late** (after the last source, at the
tie-row narrowing) and **in the wrong vocabulary** — it names a collision
between rows that arrived from different sources, which is not a thing
the author wrote. `product.rs`'s own comment concedes the addressing
problem: *"A refusal here names the node that MINTED the colliding name
rather than a root: the collision is between rows that arrived from
different sources, so no one root is its author."*

So: refuse **earlier**, and say what the author actually did — one
instance placed under two roots — rather than reporting a name
collision. The recipe layer knows the shape before the gather runs.

**The standing fact that makes this worth doing** is unchanged and is
this row's real severity: **the solve accepts a document the gather
cannot represent.** Two layers disagree today, and the ruling did not
change that; it only settled which layer's answer is right.

`crates/editor-core/src/product.rs` is WIRE's, so this lands here.

## Implemented (PR 3142)

The gather refuses the shape from the recipe, before any root is read,
as `ProductError::PlacedUnderTwoRoots { placed, select, first, second }`
(`product.rs`, `placed_under_two_roots`). Both late `Naming` arms stay
reachable. The per-root carry is reached through a split's intact
pass-through (`wire_product_gather_tie`) and through one instance
index spelled two ways. Both the carry and the tie flush are reached,
FALSELY, by two `Part` roots over the halves of a split that separates
a tie: `work/gather/product-refuses-split-halves-as-roots-when-a-tie-narrows-to-unique.md`,
pinned as measurements in `gather_placed_under_two_roots`. The solve's
half is MSOLVE's:
`work/msolve/the-solve-accepts-a-body-placed-under-two-roots.md`.

## Closed (2026-09-25)

Merged as PR 3142. Tier: single FULL review. The adjudicated fix pass
landed on the same PR. Residue filed as its own rows:
`product-refuses-split-halves-as-roots-when-a-tie-narrows-to-unique`
(P0), `three-walks-over-the-name-carrying-edges` (P1), and MSOLVE's
`the-solve-accepts-a-body-placed-under-two-roots` (P1).

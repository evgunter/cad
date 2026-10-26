---
id: a-split-through-a-vertex-whose-above-runs-nest-crosses-its-section-face
kind: issue
title: A split through a vertex whose above runs nest under one builds a section face whose corners there overlap
status: closed
opened: 2026-10-08
priority: P1
cost: M
branch: cleave/apex-section-corners
closed: 2026-10-11
pr: 4565
---


Found by TANG's sweep for `nested-pierce-runs-have-no-ring-order`
(2026-10-08), the split lane's sibling of the pierce ring.

## What

`topo::split` of `topo::test_support::meeting::apex_pyramid` on the
plane through its apex at `MEET`, normal +z, builds both sides at their
closed-form volumes, but each side's section face has two corners at the
apex that overlap (`meeting::corners_disjoint` fails), in every one of
`meeting::poses()`:

| base | runs above, about +z | above | below | corners |
|---|---|---|---|---|
| a single crossing | 1 | 0.015 | 0.030 | disjoint |
| a zigzag, two arches up | 2, apart | 0.010 | 0.025 | disjoint |
| a crown, three arches up | 3, apart | 0.015 | 0.040 | disjoint |
| `meeting::comb` | 3, two nested under one | 0.040 | 0.015 | **overlap** (both sides) |
| `meeting::arch_cone` | 3, one inside another inside a third | 0.050 | 0.030 | **overlap** (both sides) |

The volumes are right. Tier 3 does see the crossed loop: check 9's
corner arm returns `ValidationError::PinchCornerCrossed` from
`validate_geometric` on every crossed side (20 of 20, PR 4300's review).
The split ships the body with no refusal only because `split` does not
run tier 3 on its sides (`splitting/mod.rs`, `split`'s doc: a pinch
side's touching pieces carry contacts it declares nowhere). A cheap
backstop until the order is read: run the corner check alone on split
sides, and refuse a side that fails it.

## The likely cause

`splitting/insert.rs` `insert_null_edges` mints one null edge per above
run at the vertex, and the section face is closed through them in an
order that assumes the runs' germ intervals lie one after another about
the plane's normal. The above runs are chords of the vertex's link that
do not cross but can nest. The boolean's pierce ring had the same
assumption: `boolean/vtxfac.rs` now sorts the runs' germs clockwise
(`germ_order`) and hangs its struts as the tree that sort gives
(`crate::null::ring_tree`), a star where one region borders every run,
a path or deeper tree otherwise. The split lane wants the same reading,
or a typed refusal, rather than a
crossed section face.

## Rows owed

The table above as a row, each pose, asserting `corners_disjoint` on
both sides with the closed-form volumes (the pyramid's volume is a
third of half a unit times the base polygon's area, split by `z = 0`).

The boolean's reading is now the pure `crate::null::ring_tree(order,
root)` (germs in clockwise order to the tree of struts, star or not),
which the split lane can call. Both designers who weighed the pierce
ring judged this split half the more urgent: it ships an unsound body
where the boolean refused.

## Built (branch cleave/apex-section-corners)

**Measured first** (origin/main 98a3817a1d, ε = 1e-9, 1e-6, 1e-12, every
pose): the comb and the arch cone split at their closed-form volumes with
both sides' section faces crossed (`corners_disjoint` fails; tier 3 reads
`PinchCornerCrossed`); the single arch, zigzag and crown build sound.

**Cause.** `insert_null_edges` minted one null edge per Above run at the
ON vertex, every one hung at the vertex: a star whose edges pair each
run's two germs. A null edge carries a section loop across the vertex
from the face at one of its germs to the face at the other, so the loops
close through the runs' germ pairs. The section's corners pair germs as
the material lies between them round the plane's normal; the two agree
only where the runs lie apart. Nested runs (and nested Below runs, the
D7 pinches) close each loop across other corners.

**Fix** (`splitting/insert.rs`): one null edge per corner of the section.
The germs are ordered about the normal (`section_chords`, four new named
readings, two germs along one direction placed by the alternation of
the material's starts and ends), each start paired with the next germ,
and the corners hung as the tree they make
(`crate::null::ring_tree`, the pierce ring's reading, rooted at the
piece holding the first Below entry). Each copy takes its corner's arc
of the orbit and stands for that piece. Apart runs give the star as
before. Two refusals: `SectionGermsEscalated` (a reading in band) and
`SectionGermsUnordered`.

**Rows.** `split_through_an_apex`: six bases (single arch, zigzag,
crown, comb, arch cone, branching cone) × five poses × both normals,
closed-form volumes, tier 3, `corners_disjoint`, and the vertices each
side keeps at the apex; and a W whose valley germs lie ~3ε apart
refusing `SectionGermsEscalated` at every pose. Run at ε = 1e-9, 1e-6,
1e-12.

**Moved.** The Λ-tip reduction rows (`m3_pr2_reduce`, `review_m3_pr2`)
re-baseline: one null edge at the tip and one hung at its Above copy,
no dangling strut, where the star minted two at the tip (one dangling)
and the join then refused `DegenerateSection`. The L-prism reflex-corner
unit row now pins the path its corners make. `review_m3_pr6`'s
both-sided pinch, pinned as refusing, now splits with tier 3 on both
sides; the D7 mirrored rerun is never entered over topo, sweep, stl,
verbs and editor-core (8,315 tests) and is retired, with
`SplitNaming::mirrored` and DESIGN.md's both-sided frontier entry.

**Validator.** Tier 3 catches the crossed loop (check 9,
`PinchCornerCrossed`); tier 2 cannot, a corner's order being an angle.
`split` does not run tier 3 on its sides by design (its pinch pieces
carry contacts it declares nowhere). No validator gap.

### Fix pass (PR 4565 review)

The first build read every germ's direction, which regressed two classes
base split soundly: crowns whose valley edges lie in the plane (every
germ one of a pair along an in-plane edge, refused `Unordered`), and an
in-plane edge whose far end is decided On within ε of the plane (its two
germs tilted apart by about `δ·cot(slope)` into the band, refused
`Escalated`). Now the topology decides what it can: a germ beside an
entry the plane holds is that entry's direction, and the two germs
either side of one such entry are ordered end first with no reading
(rule (b) sends the entry across only where its wedge is reflex, which
holds material on both sides of it in the plane, so no corner lies
between them). Only germs of different sectors are read, and two a
reading puts on one line refuse as a coincidence.

Measured with the reviewer's probe (31 families × 5 poses × ±n, ε =
1e-9, 1e-6, 1e-12, both trees on the `K·ε` corner check): no family,
pose or ε that base splits soundly fails on head.

Rows added: an in-plane edge at the apex exact and near (δ ∈ {0, ±0.3ε,
±0.6ε, ±0.9ε}); crowns whose valleys lie in the plane, one of them a
V-groove floor bent at the vertex, against a closed form; and the
coincidence refusal (a W whose germs lie 0.3ε apart). `corners_disjoint`
groups corners within `K·ε` rather than on a micron grid.

Refusals name their cause: `SectionGermsEscalated { reading }` (`Line`
or `Order`) and `SectionGermsUnordered { fault }` (`NoLine`,
`Coincident`, `Alternation`, `Crossing`). `Coincident` has a witness.
`Alternation` and `Crossing` are argued to need faces within ε of one
another and were not reached in about 63k fuzz cells. A corner's arc
always starts and ends on one side (a corner pairs a start with an
end), so that check is an `unreachable!` with its values.

The two germs beside one in-plane entry take the order the alternation
fixes from any lone germ; only where every germ is in such a pair are
they taken end first, an order argued for rule (b)'s reflex row and not
for its safety-default row.

The "no regression" measurement covers the review's 31-family probe.
The review's random fuzz (about 63k cells) found head ships no crossed
corner where main shipped about 1,500, and about 0.5% of cells per ε
now refuse `ChartResidual` in the band at an On vertex off the plane
where main's mirrored rerun had built them under the flipped normal.
The class already had 124 cells on main. Rows: four random bases with
corners in the band split sound, and one pins the residual refusal.

Filed: `work/honeone/germ-orders-about-a-normal-have-two-homes.md` (the
split's germ order beside the boolean's `germ_order`, `strut_order` and
`direction_sense`).
`work/topo/a-split-recharts-an-edge-at-a-vertex-read-on-off-the-plane-into-the-band.md`
(the residual class, its witness and orientation dependence);
`work/cleave/a-crown-touching-the-plane-along-one-edge-panics-at-the-section-spur-check.md`
(a D9 panic on exact input, on main and on this branch alike).

## Closed (PR 4565, 2026-10-11)

The split mints one null edge per corner of its section, hung by `null::ring_tree`. The germs are
ordered by the topology wherever it decides (an On entry's direction; the pair beside an in-plane
entry), and by the four `split_germ_*` readings only between germs of different sectors. Over
about 63k fuzz cells the split ships no crossed corner, against base's about 1,500. The D7 mirror
rerun is retired.

About 0.5% of cells now refuse an existing `pcurve_map_residual` certification class that base
escaped only through the rerun. That class is TOPO's `a-split-recharts-an-edge-at-a-vertex-read-on-off-the-plane-into-the-band`.
The crown spur panic is `a-crown-touching-the-plane-along-one-edge-panics-at-the-section-spur-check`
(P1).

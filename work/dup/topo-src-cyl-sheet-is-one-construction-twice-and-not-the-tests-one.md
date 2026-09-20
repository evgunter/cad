---
id: topo-src-cyl-sheet-is-one-construction-twice-and-not-the-tests-one
kind: issue
title: census.rs and chart_region.rs build the same cylinder sheet twice, and the tests/ door is that construction with a pub(crate) scar
status: closed
opened: 2026-09-19
closed: 2026-09-20
branch: dup/src-cyl-sheet
refs: [try-wall-sheet-stands-down-on-any-panic, geom-brep-inline-canonical-frame-surfaces]
pr: 2925
---


## Finding

Two in-`src` test modules grow the same cylinder-wall sheet:
`crates/topo/src/census.rs`'s `cyl_sheet` (with `cyl_sheet_b` beside
it) and `crates/topo/src/chart_region.rs`'s `cyl_sheet`, the latter
with its rim factored into `rim_spec`. **Measured byte-identical** at
equal arguments (`u ∈ [0.2, 1.4] × z ∈ [0, 1]`, canonical unit
cylinder): a sorted dump of every vertex, edge, half-edge, loop, face,
shell, solid, point, curve, surface, surface source and pcurve row
diffed empty, and `census.rs`'s extra `set_face_sense(face, true)` at
the end moves nothing. So they are one construction written twice, and
`chart_region.rs`'s `rim_spec` is the factoring the other lacks.

## Why it was not folded with the `tests/` half

`cyl_wall_sheet` (`crates/topo/src/test_support_fixtures.rs`, PR for
`the-cylindrical-patch-rim-builder-is-written-nine-times`) is **the
same construction**, reached through a door a `tests/` binary cannot
open. Read line by line the two are one thing: same Euler skeleton,
same rim closure with the same `ccw` / reversed-axis / `radial(u1)`
branch, same `Intersection { s1, s2, witness }` at the same midpoint.
One difference, and it is a visibility scar rather than a design:

| | `src` pair | the shared door |
| --- | --- | --- |
| rim plane's surface | `Body::add_surface` (`pub(crate)`) | scaffold `mvfs` + `set_face_surface` |
| arena counts | 1 solid, 2 faces, 4 vertices | 3 solids, 4 faces, 6 vertices |

The scaffold exists because `add_surface` is `pub(crate)` and a
`tests/` binary cannot call it. Nothing else differs.

**The scar is inert to every row that uses the shared door**, measured
2026-09-19: switching the door's rim planes to `add_surface` reds only
the door's own arena row and leaves all 617 `topo` integration rows
green at both lanes. So no suite on the `tests/` side is defending the
scaffolds. What stops the fold being free is the other side —
`census.rs`'s rows are about solids and faces, and removing the scar in
the other direction moves what they count. That is a full-tier unit
under this program's review posture, not a declaration-site move, which
is why it was dispositioned out of the folding unit rather than swept
into it.

## What a unit here owes

1. Decide which of the two doors both sides use. The scaffolds are
   already measured inert on the `tests/` side (above), so the open
   half is what `census.rs`'s solid-and-face rows mean to assert.
2. Re-take the count first — `census.rs` and `chart_region.rs` are
   live files and this was measured 2026-09-19.

## Why this row is not on `curved`'s slate

`scripts/work.py territory` reads `crates/topo/src/census.rs` as
`curved`'s. The finding is a duplication between two fixture
builders, which is this program's charter and not `curved`'s; it is
filed here so the class stays with the other members, and `curved`
owns the file whenever it wants the row.

## Closed

Folded. Re-measured 2026-09-20 at merge base `cd9fdfd6b`, by dumping
and diffing rather than reading: each spelling reproduced verbatim in a
throwaway in-crate probe, every vertex, edge, half-edge, loop, face,
shell, solid, point (by bit pattern), curve and surface row formatted
with its key, sorted and diffed.

**The pair is one construction.** `census.rs`'s `cyl_sheet` against
`chart_region.rs`'s, at the arguments their own call sites use: 66 dump
lines, **65 identical**. The one that differs is the second sheet's
face SENSE, which `census.rs` sets to `false` and `chart_region.rs`
leaves at the `mef` default — the `sense` parameter, not a second
construction. At equal `sense` the diff is empty, which is the parent
row's measurement reproduced.

**The parent's "one difference, and it is a visibility scar" is wrong,
and this is the correction it asked for.** There are TWO differences
between the `src` pair and the `tests/` door, and only one of them is
the scar:

| | measured | verdict |
| --- | --- | --- |
| rim plane minted by `add_surface` (`src`) vs a scaffold `mvfs` + `set_face_surface` (the door) | planting `add_surface` at the door reds **1** row — the door's own arena row — and **0** of the 566 default-lane integration rows | inert, a fossil |
| the cylinder key on the SEED face's surface slot (the door) vs a bare arena key (`src`) | planting the seed-face form at the door's callers reds **1** integration row (`split_edge_pcurve_rows::a_split_on_a_curved_chart_leaves_every_half_edge_a_row`); planting it in `census.rs` reds **6** census rows | load-bearing, in BOTH directions |

So what blocks a single spelling is not visibility at all. The scar's
stated reason — *"a caller outside this crate cannot reach
`add_surface`"* — was never true of the door, which is in `src` and
could always call it; the scaffolds are what the `tests/` closures had
to do before the parent moved them into `src`, and the doc kept
asserting a constraint the move had dissolved. **That paragraph is
deleted and the scaffolds with it.**

### Where the shared body went

`crates/topo/src/test_support_fixtures.rs`, beside `CylFrame`: one
`cyl_wall_sheet_keyed`, with the measured axis as its parameter
(`CylKey::OnSeed` | `Bare` | `Shared(key)`). Three spellings collapse
onto it — the public `cyl_wall_sheet` (`OnSeed`, pcurves minted), the
in-crate `unit_cyl_sheet` (canonical frame, bare key, sense set, no
mint) and `census.rs`'s `cyl_sheet_b` — and `crates/topo/src` now holds
**one** cylinder-sheet construction where it held four.

What it cost: the door's bodies lose their two scaffold solids, so its
arena row re-baselines from `(3, 4, 6)` to `(1, 2, 4)` and its
lone-vertex-face count from 2 to 0. Nothing else moved — 566 default,
617 interval, 571 probe, all green, identical to the merge base.

The fold is bit-identical up to one thing, stated because it is real:
the descending rim's carrier axis reads `Vec3 { -0.0, -0.0, -1.0 }`
where the deleted copies wrote the literal `Vec3::new(0.0, 0.0, -1.0)`.
The shared body computes `-frame.axis`, which is what the `tests/`
family has always produced. Two curve rows per body; numerically equal,
and no row reads the sign of a zero.

### The proof

Planting the descending rim's axis reversal away in the shared body
reds **15 lib rows** (11 `census`, 2 `chart_region`, the door's arena
row, and its key-arms row) and **22 of 566** integration rows
(`mate5_cyl_eps_rung` 8, `r1_mate5_probe` 7, `r2_probes` 4,
`split_edge_pcurve_rows` 3). Every folded site is live.

Two rows are NOT in that list and are the finding that came out of it:
`r1_mate5_probe::probe1_…` and `r2_probes::r2_tilted_disjoint_…`
swallow the planted break through `try_wall_sheet`'s `catch_unwind` and
report ok. Row: `work/tint/try-wall-sheet-stands-down-on-any-panic.md`.

Residue also filed:
`work/dup/the-canonical-unit-cylinder-literal-has-no-reachable-home.md`.

## Fix pass, 2026-09-20 — the guard reach the first cut removed

**The first cut moved the Euler sequence out of the surface the tier-1
mutation-door guard can read, and then edited that guard's prose to
describe the code that had left.** `crate::source_walk`'s `public_fns`
takes `pub fn` and rejects `pub(crate) fn` (its preceding token is
`)`), so hoisting `mvfs`/`mev`/`mev_line`/`mef`/`set_face_surface`/
`add_surface` from `pub fn cyl_wall_sheet` into
`pub(crate) fn cyl_wall_sheet_keyed` took the whole sequence out of
`mutation_doors`'s population. Nothing reddened only because
`cyl_wall_sheet` is on `review_m1_pr5_internal::ALLOWED` and an
allowlisted door's body is never text-checked — so the surgery-posture
read, too, was left reading a three-line delegation.

Restored rather than disclosed: `cyl_wall_sheet_keyed` is `pub`, is
re-exported through `topo::test_support` beside the door it carries,
and is named by **both** tables — `ALLOWED` for tier 1 and
`pcurves::staleness_posture::DECLARED` as `Neither` — which is what
`the_two_door_tables_cover_the_same_surface` requires of every door.
Its `ALLOWED` entry carries the `add_surface` argument that had been
written into `cyl_wall_sheet`'s; `cyl_wall_sheet`'s now describes its
own three lines. The section comment above that group claimed a union
of two halves over all its entries and is now false of one of them, so
it names the exception instead of asserting over it.

**`source_walk::DOORS_MEASURED` was one behind before this unit**: the
constant read 52 and the walk found 53 at the merge base, inside the
floor's two of slack. It reads 54 now, re-measured with the door this
unit added.

Still outside the walk, and stated rather than fixed: `unit_cyl_sheet`
is `pub(crate)`, so the `set_face_sense` it calls is read through the
walk's own **delegation** blind spot, which `mutation_doors` already
documents. Nothing moved there — it composes doors, it does not contain
a sequence that used to be visible.

### The generalisation, corrected

The first version of this said the gate keys on visibility. **It does
not — it keys on the `cfg` mount**, and `witness-not-ambient` is the
proof in the other direction: a `pub(crate) fn` inside
`#[cfg(test)] mod tests` is still skipped, while
`cyl_wall_sheet_keyed` is *lower* visibility than the `pub fn` it was
cut from and faces the gate anyway. The rule is:

> A construction hoisted out of a `#[cfg(test)]` mount into a shared
> one enters every gate that skips `cfg(test)`, and which gates it now
> faces is not a function of its visibility.

The tier-1 door walk is the one case where visibility does decide, and
it decides the opposite way — which is why a fold has to check both
directions rather than one.

### Residue re-routed

`the-canonical-unit-cylinder-literal-has-no-reachable-home` was
**deleted before it reached the board**: the class is already
`work/tint/geom-brep-inline-canonical-frame-surfaces`, open since
2026-09-03. Its evidence — the in-`src` `#[cfg(test)]` half that row's
`crates/geom-brep/tests/` scope never covered, the 204/17 denominators,
and the second candidate home `CylFrame` is — went into that row
instead. Filing a duplicate on the program whose charter is duplicates
is item 14 one level up, and it was caught by a reader and not by the
lane.

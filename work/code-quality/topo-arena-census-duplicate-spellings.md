---
id: topo-arena-census-duplicate-spellings
kind: issue
title: S52 residue — the duplicated-test-vocabulary class survives inside topo (two arena-census spellings, drifted Census copies) and in stl
status: open
opened: 2026-08-19
github: 672
refs: [668, 679, S52]
---

## From GitHub issue 672

Opened 2026-08-19; 1 comment.

Filed by #668 (S52 execution) so the residue carries an ID rather than the word "unscheduled". Q6: disclosure is not a schedule — this records what is still true, it does not commit a lane.

#668 collapsed exactly **one** copy: `crates/topo/tests/m3_pr5_boolean_ops.rs`'s `Census`, which was field-for-field identical to `ArenaCounts`. The *class* the finding names is not gone.

## 1. Two spellings of the arena census, in one crate — and one is a strict superset of the other

| | fields | where | reach |
|---|---|---|---|
| `ArenaCounts` | **7** — solids, shells, faces, loops, half_edges, edges, vertices | `crates/topo/src/test_support_impl.rs` | in-crate + `tests/` through `test-support` |
| `ArenaSnapshot` | **10** — those same seven, in the same order, **plus** points, curves, surfaces | `crates/topo/src/fixtures.rs:104` | `#[cfg(test)]`, in-crate only, `Body<f64>` only |

An earlier revision of this issue described `ArenaSnapshot` as the seven minus `half_edges` plus three. That was wrong — `fixtures.rs` declares `half_edges`, and the arithmetic did not work out either (7 − 1 + 3 = 9, not 10). The truth is the stronger case: **`ArenaSnapshot` is a strict superset of `ArenaCounts`** — topology arenas, then the three geometry arenas. Two names for one concept where one extends the other is a much better candidate for collapsing to a single type than two partially-overlapping structs would be.

Neither type's docs say why the other exists. #668 is the PR that consolidated the census and it left the crate with two of them — worth naming plainly.

## 2. Four `Census` copies still in `crates/topo/tests/`, with drifted field sets

- `m3_pr3_split.rs:69` — 4 fields (shells, faces, edges, vertices)
- `m3_pr4_boolean.rs:23` — 3 fields (vertices, edges, faces)
- `review_m3_pr1.rs:36` — 7 fields, but not `ArenaCounts`' 7: no half_edges, plus `rings`; carries a `CensusDelta` sibling
- `graft_disjoint.rs:131` — 8 fields, incl. points, surfaces, and `shell_refs`

Each is a genuine subset/superset with its own bound (`T: Real`, `T: Decide + Bounds`, `f64`), so none is a mechanical replacement — which is why #668 did not touch them and why "a third copy, field-for-field" was accurate about the copy it collapsed and misleading about the class.

## 3. Same shape elsewhere

- `crates/stl/tests/m3_pr6_exports.rs:34` and `review_m3_pr6_e2e.rs:32` — byte-identical `brick(x, y, z)` fixtures.
- `crates/mesh/tests/`, `crates/step-export/tests/`, `crates/editor-core/tests/` repeat local body builders; the last two already share through tests-side `common` modules.

## 4. And one scalar-generic blocker

`crates/sweep/tests/m6_surgery_interval.rs:66`'s `cube()` is the **same** fixture as `sweep::test_support::cube` one scalar up — same four-corner loop (`RawLoop::polygon` is `new` with every bulge zero), same `SketchPlane::xy()`, same `Tolerance::get()`, same `Extrusion::Distance` — differing only in `Interval` vs `f64`. Collapsing it needs the fixture to be made generic over the scalar, i.e. `cube<T: Real>(l: T) -> Body<T>` — a real generalization rather than a deduplication. (An earlier revision lost that signature to markdown, leaving the sentence contentless.)

## What would settle it

Whether `ArenaCounts` and `ArenaSnapshot` should be one type — the superset relation says they plausibly should — and whether the tests-side `Census` copies should be projections of it, is a design question, not a mechanical cleanup. Not proposing an answer here.

## Comments

**2026-08-19** — comment:

#679 closes items 1–3 of this issue. Leaving it **open** — item 4 (and part of item 3) is untouched.

**Landed:**

1. **Two spellings of the arena census → one.** `ArenaSnapshot` (`crates/topo/src/fixtures.rs`) no longer restates the seven; it *holds* an `ArenaCounts` beside the three geometry-arena lengths, and `arena_snapshot` delegates to `Body::arena_counts()`. The crate now has exactly one producer of the topology census — the same one the D1 debug postcondition already cross-checks against each operator's declared `ArenaDelta`. The superset relation this issue identified is what made the composition the right answer rather than a merge.

2. **The four `tests/` copies were not one class**, which is the substantive finding. Only `m3_pr4_boolean.rs`'s 3-field `Census` was the arena vocabulary; it is gone, replaced by `topo::test_support::arena_counts`. The other three carry quantities no arena census has, and now carry names that say so:
   - `m3_pr3_split.rs` → `SideCensus` (a 4-arena projection; its six expectations are hand-derived for exactly those four)
   - `review_m3_pr1.rs` → `EulerCensus`/`EulerCensusDelta` (`rings` is the `r` of `v − e + f − r`, summed over faces — not an arena length)
   - `graft_disjoint.rs` → `GraftCensus` (`shell_refs` is summed `solid.shells.len()` — likewise)

   Four identically-named types for four different quantities was itself the drift.

3. **`stl`'s two byte-identical `brick` fixtures** now share `crates/stl/tests/common/mod.rs`.

**One assertion changed meaning, deliberately:** `m3_pr4_boolean.rs`'s "operands functionally untouched" check ran over a three-component sample; it now runs over all seven topology arenas. A widening, not a rename — flagged rather than passed off as mechanical.

**Still open here:**

- **Item 4** — `crates/sweep/tests/m6_surgery_interval.rs`'s `cube`, which needs `cube<T: Real>(l: T) -> Body<T>`. A generalization over the scalar, not a deduplication; nothing in #679 touches it.
- **The tail of item 3** — the local body builders in `crates/mesh/tests/`, `crates/step-export/tests/` and `crates/editor-core/tests/`.

The S52 row in `docs/SMELL-SCAN-2026-08.md` is updated under #654's convention to record exactly this split.

Also worth recording for whoever takes item 4: #641's review point applies to any further census consolidation here — a transposition across two correct field names in a producer like `arena_snapshot` compiles and is invisible to the entire suite, because the type is only ever compared with itself. #679's conversions were therefore hand-audited site by site rather than trusted to the batteries; see its description for the per-site result.


---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

Code quality: this is the residue of the scan's `S52` duplicated-test-vocabulary finding, which the migration census carries as `work/code-quality/S52.md` ("#672 residue") — a duplicated-spellings structural finding, this program's kind.

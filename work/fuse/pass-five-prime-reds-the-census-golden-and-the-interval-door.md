---
id: pass-five-prime-reds-the-census-golden-and-the-interval-door
kind: issue
title: Pass 5′ (#5227) turned two editor-core rows red on main: the perf12 census golden gains cut_cylinder's two section-arc overlaps (correct, re-bless), and boss_union's census escalates six one-curve readings at Interval only (regression)
status: closed
opened: 2026-10-16
closed: 2026-10-17
priority: P1
cost: M
---

Two rows in `editor-core`'s `all` binary are red on main (`9ae7e2e959`) and were red from the merge of #5227 (`37edb540a9`, `fuse/curve-pair-reader-c3`, pass 5′). Bisected over first-parent merges: both pass at its first parent `f4ebb63a56` (and at `75cf96347d`, `a8c602f047`, `d848fd268a`) and fail at `37edb540a9`. #5227's local verification ran `geom`, `geom-brep`, `topo`, `sweep` and `step-import`, not `editor-core`.

## 1. `perf12_census_goldens::the_at_rest_census_verdicts_are_goldened_bit_exact`: correct, re-bless

`crates/editor-core/tests/perf12_census_goldens.rs:187` fails at line 199 of the golden, at **every** ε row (1e-6, 1e-9 and 1e-12 alike), with the same diff in each of `crates/editor-core/tests/golden/perf12_census_{1e-6,1e-9,1e-12}.txt`:

```
 ## cut_cylinder
 solids=2 faces=8 edges=12 vertices=8
 assemble: Err(AtRest)
-census: 12 findings
+census: 14 findings
 UnrecordedContact { contact: VertexVertex { a: VertexKey(3v1), b: VertexKey(7v1) }, witness: "(0.5, 0.0, 0.3453318751951884)", ... }
 UnrecordedContact { contact: VertexVertex { a: VertexKey(4v1), b: VertexKey(8v1) }, witness: "(-0.5, 0.0, 0.6546681248048116)", ... }
+UnrecordedContact { contact: EdgeEdgeOverlap { a: EdgeKey(5v1), b: EdgeKey(11v1) }, witness: "(0.0, -0.5, 0.5)", margin: Some(MarginDiag(Value(0.0, None, ZeroWord(0)))) }
+UnrecordedContact { contact: EdgeEdgeOverlap { a: EdgeKey(6v1), b: EdgeKey(12v1) }, witness: "(0.0, 0.5, 0.5)", margin: Some(MarginDiag(Value(0.0, None, ZeroWord(0)))) }
```

**The new findings are right.** `cut_cylinder` (`crates/editor-core/tests/corpus/cut_cylinder.rs`) is an r = 0.5, H = 1 cylinder split by the plane through `(0, 0, 0.5)` with normal tilted 0.3 rad in xz, and its two halves are placed at rest against each other. Each half's section ellipse is cut into two arcs at the seam vertices `(±0.5, 0, z)`, which the golden already reports as unrecorded vertex-vertex contacts. The two halves' arcs are the same ellipse, so each pair overlaps along its whole length; the witnesses `(0, ±0.5, 0.5)` are the ellipse's minor-axis ends, on both arcs, at margin 0. Before pass 5′ no tier read a curved edge pair, so these overlaps went unreported; pass 5′ is the pass built to see exactly this. **Fix:** re-bless the three goldens with `PERF12_BLESS_CENSUS=1`, one run per ε row.

## 2. `m10_di_dual_corpus::assemble_census_verdicts_match_f64_at_interval`: regression

`crates/editor-core/tests/m10_di_dual_corpus.rs:228`: on `boss_union`, `assemble` passes at `f64` and refuses at `Interval` with 6 findings, each "no declaration: whether two parts of the body touch is undecided ... this is a kernel defect". The body is one solid (10 faces, 21 edges). The `f64` census is empty; at `Interval` (read with a temporary probe, not committed) it is six of:

```
CensusEscalated { cause: Indeterminate { margin: MarginDiag(Invalid), terminal_sliver: false, band: Band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("curve_pair_pose_one_curve") } }
```

**This is a regression.** The census door now disagrees across certifying scalars on a corpus document. The `Interval` lane fails closed with an `Invalid` margin where `f64` decides "one curve", so the fault is in evaluating the margin, not in the band. A likely mechanism, which has not been executed: `one_curve_gap` → `frame_gap` (`crates/geom-brep/src/curve_pair/pose.rs:956-979`) ends in `sqrt(mid + sqrt(off² + (d1·d2)²))`, with `mid = (d1·d1 + d2·d2)/2`. For two coincident conics `d1` and `d2` enclose zero, and a self-dot read as products of an interval straddling zero has a negative lower end. The outer `sqrt` then reads a straddling enclosure, so it is Invalid at exactly the coincidence the predicate exists to decide. `frame_gap` predates #5227; pass 5′ is the first to route a census pair through it (boss_union's coincident rim arcs). **Fix:** make the one-curve margin certify zero at Interval, for example by reading each self-dot through `norm_squared` or a squaring that never reads below zero, or by clamping `mid` at zero. Add a geom-brep row of two coincident circles and two coincident ellipses at `Interval` that asserts `Pose::Coincident`.

## §1 landed in INTENT stage 3 B (PR 5232, 2026-10-17)

The perf12 re-bless this item prescribes was ported into PR 5232: the
three goldens (`perf12_census_{1e-6,1e-9,1e-12}.txt`) gain cut_cylinder's
two `EdgeEdgeOverlap` findings, and nothing else moved from that cause.
Both rows fail identically at that PR's merge base (`4760a784b2`),
built and run in place. §2 (boss_union's Interval census) is untouched,
and stays this item's.

## §2's causes, and their fixes

The row had two causes beyond the census golden, both in
`crates/geom-brep/src/curve_pair/pose.rs`, both an `Interval`
enclosure wrapping what `f64` reads to its rounding.

- **`boss_union`: the one-curve margin's outer `sqrt`.** `frame_gap`
  read its Gram eigenvalue as a straddling enclosure for two
  coincident conics, so the margin was `Invalid`
  (`curve_pair_pose_one_curve`). The eigenvalue is non-negative by
  construction and is clamped at zero (`4c9ef9805f`). Pinned by
  `curve_pair::one_curve_rows::a_conic_read_twice_is_one_curve_at_interval`.
- **`die_pips`, `die_tool`, `die_composed`, `die_composed_tour`: two
  angle cuts.** A pip's rim on the face `z = 1` against its section
  arc in the plane `y = 0.5`: the rim's height above that plane has
  cosine part `−r` and sine part an enclosure of zero, so
  `atan2(sp, cp)` straddled its branch cut and read `θ_hi` as the turn
  `[−π, π]`. The root gap's chord between `C(θ_hi ± ψ)` then read
  `[0, 0.228]` and escalated (`curve_pair_pose_root_gap`), where
  `f64` reads the two ends' joint. `angle` reads the angle a turn on,
  `atan2(−y, −x) + π`, where `x` reads negative; every reader takes it
  modulo a turn. It serves the line × conic and transverse harmonics,
  the coplanar circles' line of centres and the window's foot seed.
  With it, `die_composed_tour` still refused on a pip touching the
  section arc's plane at its lowest point (`Unsettled::Meter`): the
  polish's reach read the angle from `θ_hi` through
  `reduce_periodic_centred`, whose cut lies exactly at the touch, half
  a turn on, so the reach was the whole half turn and the basin meter
  refused. `separation` reads that angle from `θ_hi + π` past a
  quarter turn. Pinned by
  `curve_pair::meet_rows::a_height_harmonic_on_the_negative_axis_is_read_clear_of_the_cut`
  (red on the principal branch, with the census's own
  `root_gap` enclosure `[0, 0.228…]`) and
  `curve_pair::meet_rows::a_touch_at_a_harmonics_lowest_point_is_read_clear_of_the_cut`
  (red without `separation`, `Unsettled::Meter`), each a pair taken
  from the corpus in its `Interval` enclosures.

## Closed

Both rows are green on `fuse/interval-one-curve-margin`:
`m10_di_dual_corpus::assemble_census_verdicts_match_f64_at_interval`
passes, with `f64` and `Interval` agreeing on every corpus document,
and `perf12_census_goldens` stays green with no golden moved. Run
locally: `nextest -p geom -p geom-brep -p topo -p sweep -p editor-core`,
11173 passed and 0 failed.

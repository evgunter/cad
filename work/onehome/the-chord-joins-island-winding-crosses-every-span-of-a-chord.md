---
id: the-chord-joins-island-winding-crosses-every-span-of-a-chord
kind: issue
title: The chord join's island winding takes about 1.2 s of a 4.4 s unequal-radius cylinder boolean in release, most of it the curve-pair walk's whole-carrier foot on 630-pole chords
status: closed
opened: 2026-11-10
priority: P1
cost: M
closed: 2026-11-21
refs: [a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses, the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes]
---

## What

Filed by SPIRICARM's cylinder-pair cost lane (branch
`spiricarm/cylinder-pair-cost`, 2026-11-10), measuring the P1 row
`a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses`.
Re-measured by ONEHOME's chord-join-cost lane (2026-11-15).

On the unequal-radius pose (unit cylinder against a `r = 0.6` cylinder
across it, axes meeting; `the_fenced_poses_keep_their_own_outcomes` in
`sweep/tests/verbs_germarms2.rs`; `cargo run -p sweep --release
--example cylinder_pair_cost_probe unequal`), an operation takes
4.2–4.8 s in release (4.6–5.7 s before the acceleration bound was
kept, same machine, same run). The chord join reads an island's winding
twice per operation (`topo::boolean::join` `IslandClosing::Carried` →
`chord_join::path_island_winding_closed_by`). The skew pose reads none.

**Where it goes** (timing probes, not committed). Nearly all of the
winding is 40 calls of `geom_brep::curve_pair::meet` from `ring_path.rs`
`spline_crossings`: a path's ruling or circle-arc piece against the
island's spline arc, a chord of 630 or 631 poles, which the certified
walk reads (`curve_pair/walk.rs` `meet`):

| Where | Time |
|---|---|
| the walk's stop: `Curve3::foot` on the spline, 1110 stops, each sampling all 630 knot spans before polishing | 0.85 s |
| the walk's ends and witnesses: the same whole-carrier foot | 0.28 s |
| the rest of the walk and evaluation | 0.05 s |

`NurbsCurve3::accel_upper_bound` (about 33 ms on the chord, 0.66 s per
operation computed afresh at every meeting) is kept on the curve beside
the speed bounds, so it is read once per chord.

**What remains is the stop's foot.** Holding the stop's feet to their
own pieces brings the winding to about 0.35 s, but it moves tier-3′
verdicts on three sweep rows. That is
`work/onehome/the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes.md`
(P1, design), and the 0.85 s waits on it. The measured candidate is on
the unmerged branch `onehome/foot-within-probe`.

The row's first reading (2.6–3.2 s of a 15–17 s operation, `perf` in
`ring_path::path_parity` → `spline_crossings` under
`geom_core::ladder::climb_agreeing`) predates the work that brought the
operation to 5 s.

## Why it matters

About a quarter of each unequal-radius cylinder boolean, on ordinary
geometry (the P1 row's measured cost).

## Re-banded P1 (2026-11-11)
By SPIRICARM's orchestrator, under the blocker rule (`work/README.md`, Priority): `spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses` (P1, about 10 s per ordinary cylinder boolean in release) is parked on this row and ONEHOME's/PCERT's sibling, which are the remainder of its cost after PCERT's chord hulls (re-measured there).

## Parked (2026-11-16) on the foot design row

Parked on `work/onehome/the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes.md`
(P1, design). Every cut left here changes which foot the search returns, the
stop's 0.85 s and the ends' and witnesses' 0.28 s alike.

**A pruned span search is not bit-identical.** The obvious cost cut would skip
a knot span whose lower bound on its distance exceeds the best sample so far.
That bound can be made sound, but the cut does not keep the foot.
`Curve3::foot` on a spline (`crates/geom/src/curves/on_carrier.rs:450-482`)
samples every knot span. It then polishes from every start `spline_starts`
picks: the domain's ends, each local minimum of the sampled gaps, and the few
least gaps. It keeps the smallest polished gap. The polish is Gauss–Newton held
to the domain, not to the start's span. So:

- a local minimum inside a skipped span is a polish start, and its polish can
  leave the span and descend below the current foot's gap, where it wins;
- dropping a span's samples changes which of its neighbours read as local
  minima, so the start set itself changes.

A span's distance bound says nothing about where the polish from its minima
ends. Confining each polish to its start's piece would make a pruned search
exact, but that changes which foot the search returns, which is the design
row's question. That row's answer decides this row's whole remainder.

## Closed (2026-11-21): the stop's foot is gone, and most of the winding with it

Re-measured by ONEHOME on main `6abbae4ae5`. PR 7048 (`ecab2a0ea9`,
2026-11-16) changed the walk's stop to read its Gauss–Newton pair as a
pair, with no foot (`crates/geom-brep/src/curve_pair/walk.rs`,
`read_polished`). The 0.85 s that waited on the foot design row is gone
with it, and the ends' and witnesses' feet are now about 3 ms.

**The operation**, `cargo run -p sweep --release --example
cylinder_pair_cost_probe unequal`, three runs, on the same container as
the timing split below:

| run | union | intersect | subtract |
|---|---|---|---|
| 1 | 2.79 s | 2.68 s | 2.76 s |
| 2 | 2.70 s | 2.30 s | 2.63 s |
| 3 | 2.68 s | 2.69 s | 1.96 s |

That is 2.0–2.8 s, against 4.2–4.8 s here before. It is below
3.4–4.0 s, what holding the stop's feet would have reached.

**The winding's split**, per operation, from timing probes on the
unmerged branch `onehome/stop-foot-probe`, three more runs of each
operation (those runs read 2.3–3.3 s an operation):

| Where | Time |
|---|---|
| `chord_join::path_island_winding_closed_by`, in all | 0.20–0.35 s |
| of it, `curve_pair::meet` from `ring_path.rs` `spline_crossings` (all walk) | 0.13–0.24 s |
| of that, the ends' feet | 0.001 s |
| the witnesses' feet | 0.002–0.003 s |
| the bisection | 0.06–0.12 s, nearly all the stops' polish and pair read |
| the rest of the walk (its floor, its reads' setup, `finish`) | 0.06–0.12 s |

The winding was 1.2 s of 4.4 s; it is now about a tenth of each
operation.

The walk's stop still settles or refuses by where its pair polish
lands. That is a question of refusal, not soundness:
`work/onehome/the-stops-settling-rests-on-where-the-pair-polish-lands.md`
(P2).


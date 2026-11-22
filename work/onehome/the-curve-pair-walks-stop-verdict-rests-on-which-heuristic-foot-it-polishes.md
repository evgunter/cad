---
id: the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes
kind: issue
title: The curve-pair walk's stop settles its contact window or not by which heuristic foot it polishes, so a tier-3′ verdict rests on the foot search's starts
status: closed
opened: 2026-11-14
priority: P1
cost: M
design: true
closed: 2026-11-21
---


Found by ONEHOME's chord-join-cost lane (2026-11-14), building the cost fix for `work/onehome/the-chord-joins-island-winding-crosses-every-span-of-a-chord.md`. The measured candidate is on the unmerged branch `onehome/foot-within-probe`.

**What the stop does.** The certified walk (`crates/geom-brep/src/curve_pair/walk.rs`, `meet`) reads two pieces that do not clear as a stop. It polishes the pair toward a meeting by Gauss–Newton, then takes each carrier's foot of the other's point (`Curve3::foot`, `crates/geom/src/curves/on_carrier.rs`), and reads that candidate under the window the two pieces make (`window`, `pose`'s module docs, "The gap's lower bound"). No foot certifies anything (`geom::Foot::seed`): the window and the point's reading on both spans do.

**What the probe found.** On the branch, the stop's feet are held to their own pieces, widened to the polished parameter (`Curve3::foot_within`), with the whole-carrier foot as fallback wherever the held search stops at an end of its interval. That is the same search restricted to the stretch the window covers, and every start the whole search has inside it the held one has too. Three sweep rows that pass on main then read `t3p=false`:

- `crates/sweep/tests/verbs_germarms2.rs` `the_fenced_poses_keep_their_own_outcomes`;
- `crates/sweep/tests/verbs_germarms2.rs` `a_rod_through_a_drum_wall_builds_at_its_sliced_volume`;
- `crates/sweep/tests/a_ringed_wall_divided_by_a_carried_chord.rs` `a_wall_holding_one_balls_ring_is_divided_by_the_others_chord`.

Each body's `validate_pseudomanifold` refuses `CensusUnsupported { cause: Meeting(MeetUnread(Unsettled(Meter { row: "curve_pair_meet_window_contact" }))) }` on two edges. The volume, the closed check and the certificate are unchanged. So whether the stop's contact window settles depends on which polished foot the stop reads, not only on the pieces and the meeting: the whole-carrier foot and the held one are both heuristic stationary points of the gap (`work/onehome/a-spline-foot-misses-its-true-minimum-on-a-fifth-of-random-draws.md` measures how often each misses the true minimum), and the verdict moves between them.

**Why it matters.** A tier-3′ verdict that rests on a heuristic's choice of starts is what `[verdicts]` forbids: the same body reads valid or unsupported by the foot search's arithmetic. It is also what keeps the cost fix off: on the unequal-radius cylinder pair (`cargo run -p sweep --release --example cylinder_pair_cost_probe unequal`), the stop's whole-carrier feet are 0.85 s of the island winding's 1.9 s per operation, and the probe's held feet bring the winding to 0.35 s (each operation 4.9–5.5 s → 3.4–4.0 s).

**Open (a design question).** What the stop's window should read so that its verdict does not rest on the foot: a contact read off the pieces alone, a foot the window's premise certifies (the nearest point within the pieces, not a stationary point), or the window's premise widened to cover any stationary point in them. Measure first which of the two edges' meetings each foot reaches.

## Closed (2026-11-21): overtaken by PR 7048

Measured by ONEHOME on main `6abbae4ae5`, with the probe on the unmerged
branch `onehome/stop-foot-probe`. PR 7048 (`ecab2a0ea9`, 2026-11-16) took
the foot out of the stop: it now reads its Gauss–Newton pair, held to
the spans, as a pair (`walk.rs`, `read_polished`; `Found::read_pair`),
and `(s, t)` reach the window straight from the polish. That is the
first of the three options above, a contact read off the pieces alone.

**The three rows on main.** At every stop of each, the probe read the
stop three ways, each through the same `at_end`, `window` and
`read_pair`:
- the polished pair, main's;
- the whole-carrier feet from it, this item's old stop;
- the feet held to the pieces, its candidate.

Each was set against an oracle of the pieces' least distance: a
`201 × 201` grid refined by Gauss–Newton held to the pieces.

| row | stops | outcomes agree | faults | met stops | gap above the least, worst: polished / whole / held | met points apart |
|---|---|---|---|---|---|---|
| `a_wall_holding_one_balls_ring_is_divided_by_the_others_chord` | 387 | 387 | 0 | 147 | 4.2e-14 / 4.2e-14 / 4.2e-14 m | ≤ 1.2e-15 m |
| `a_rod_through_a_drum_wall_builds_at_its_sliced_volume` | 240 | 240 | 0 | 8 | 1.3e-16 / 6.0e-17 / 6.0e-17 m | ≤ 2.4e-16 m |
| `the_fenced_poses_keep_their_own_outcomes` | 116 | 116 | 0 | 10 | 1.4e-16 / 1.4e-16 / 1.4e-16 m | ≤ 4.4e-16 m |

All three rows pass on main. Every stop's three readings agree on the
meetings read, their points and their refusals (none). No reading lies
more than `10⁻⁹` above the pieces' least distance, so none is a
non-minimal stationary pair, and `curve_pair_meet_window_contact` never
fires. The stops are a circle, a line or a spline against a spline.

**The window is sound from any pair.** Its bound (`pose`'s module docs,
"The gap's lower bound") is read about whatever pair `(p, q)` the polish
returns:
- the part of the gap a move could still take up is the slack `σ_w`,
  with no stationarity assumed;
- the walk's span (P1) is each piece's own run from the polished
  parameter (the docs' "The walk's window is its pieces"; `window`'s
  `run`);
- the composition reads a gap in the band at the pair as a meeting
  "with no premise on the polish".

Where the polish lands decides only whether the basin, the move and the
floor settle, not what the answer is. That is
`work/onehome/the-stops-settling-rests-on-where-the-pair-polish-lands.md`
(P2), guarded by `walk::tests::every_stop_polishes_to_its_pieces_least_distance`.


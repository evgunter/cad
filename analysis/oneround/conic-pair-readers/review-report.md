# Review: PR 7543, `oneround/conic-pair-readers`, frozen at `6510faceecd09835a86719d5d0e85a1cf49e6181`

Reviewer lane for ONEROUND. Row: `work/oneround/an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre.md`.
Everything the review ran is under `analysis/oneround/conic-pair-readers/review/` on `oneround/conic-pair-readers-review`
(probes, their 60-digit oracles, mutant patches, outputs under `review/out/`).

## Verdict: NOT-MERGEABLE-AS-IS

The unit does what it set out to do beside the anchor, and I could not break that part: both ladders reproduce
(the unit's own tables bit for bit), every near-anchor wrong answer of the suspects' harness is gone, and no apart
twin reads a meeting anywhere I looked. It is held back by its own acceptance. The row asks for "no pose certified
wrongly" on the ladder; the head certifies 19 wrongly at `f64`, **ten of which the base did not certify wrongly**
(five it answered acceptably, five it refused), and the PR's account of them ("the evaluator, not a reader") is false
for at least one and likely two, which this unit's own readers decide. The row filed for them schedules a fix that
cannot close those. Either the nineteen refuse, or the row's owner re-cuts the acceptance; the PR body cannot do that for itself.
Everything else below is fixable in place.

## What was executed, and what was only read

EXECUTED (my own runs, own target directories; hosted CI not consulted):

- Both ladders, base and head, ε 1e-9 and 1e-12, and both oracles (`mpmath` 1.4.1). Base is the commit I built from
  `git merge-tree --write-tree bd07a867e7 33281203395d` (see NOTE N1).
- Six probes of my own with 60-digit oracles: `Height::roots` at the anchor touch; the level, the clearance,
  `height_of` and `on_carrier` on and beside the stored curve (2,002 points per ε); the ladder re-seamed a quarter
  turn (2,016 readings per ε, base and head); `height_of` and the decision log at every `f64` line pose of the ladder;
  the `Interval` window pose on both placements and the in-plane pairs at `Interval`, base and head; the ignored
  `Interval` touch measurement.
- Thirteen mutants, each alone, at both ε, against 491 tests (`door_rows`, `arc_held_rows`, every `curve_pair::` and
  `curve_face::` row of the ci profile, `conic`, `on_carrier`); the survivors again against the slow-set oracles of
  `curve_pair`.
- The differential: `cargo nextest run --profile ci -p geom -p geom-brep -p topo -p sweep -p step-import`, head and
  base, ε 1e-9 and 1e-12.
- `scripts/run-gates.sh` (exit 0), `cargo fmt --check` (clean) and `scripts/work.py lint` (0 problems) on a clean
  checkout of the frozen head.
- The three reds the PR body disowns, on the base and on the head.

ONLY READ: `conic.rs` end to end (the largest file the unit rewrites; Q8), the whole diff of `pose.rs`,
`pose/window.rs`, `on_surface.rs`, `curve_face/pose.rs`, `on_carrier.rs`, `door_rows.rs`, `arc_held_rows.rs`; the rows
filed. NOT DONE: `cargo clippy` (the gate that arms the centre-read lint ran, clippy itself did not), the slow set
beyond `curve_pair`'s oracles, the demos, the two re-baselined topo counts beyond reading them, `Interval` on the
suspects' harness (it is `f64` only).

Counts below are each judged against the 60-digit oracle named beside them, or against the base run, as stated.

## Findings

### MAJOR

**M1. The row's acceptance is not met, and ten of the nineteen wrong certificates are new.** DEMONSTRATED, `sure`.
The unit's ladder at `f64`, judged by the unit's `oracle.py`: 7 wrong at 1e-9 and 12 at 1e-12 (reproduced; the body
says so). Joined pose by pose to the base (`review/out/ladder-base-vs-head.txt`), which the body does not do:

| the 19, by what the base read | 1e-9 | 1e-12 |
|---|---|---|
| wrong on the base too | 3 | 6 |
| refused on the base, certified wrongly now | 2 | 3 |
| answered acceptably on the base (truth in the gap, read apart), now a meeting placed 1.1 to 3.5 Kε from the true closest point | 2 | 3 |

One of the five refused-then-wrong is a new **"`Ok`, no meeting"** for curves that meet: ε 1e-12, fat `a` = 1e5,
θ = 2, the circle at 5°, true gap 2.2e-13 (the base refused it). Another is **below the regression floor**: ε 1e-12,
thin `a` = 1e4, θ = 2, the line at 5°, a meeting placed 1.2e-11 m (1.2 Kε) from the true one where the base
refused; the P1 row's table lists the pose and the body does not say it is under the size at which main is never
wrong. Seven of the nineteen are of that kind, the one
direction main is wrong in. The acceptance reads "no pose certified wrongly … a differential against the base: every
verdict that moves, judged against the oracle"; the body's table gives totals (60 → 7, 145 → 12) and says the
remainder is "not a reader of where the conic is". On the question the brief puts: a wrong certificate left at a
public door is not acceptable under that sentence, and it is this unit's to refuse. The row itself says how ("where a
reading stays on [a datum rounded past the tolerance], its verdict is metered … so it refuses and cannot certify"),
and the readers that decide these are the ones this unit rewrote (M2). All nineteen poses stand 1.2e4 m or more from
the stored point, an order past the session box (1e3 m), which is where the admission rule stops covering a
coordinate's own rounding; I name that as the fact, not as a design.

**M2. "The evaluator, not a reader" is false for at least one of the nineteen, and likely two: this unit's own
readers decide them, on their own rounding and on a frame they take as square.** DEMONSTRATED (measured readings at
60 digits and the kernel's decision log, `review/probe_far.rs`, `review/out/far-1e-9.txt`, `far-1e-12.txt`); the
confidence is on each.

- ε 1e-12, thin `a` = 1e5, θ = 2, the line at 5°: `Ok`, no meeting, true gap 4.2e-13. **The last decision in the
  kernel's log is `curve_pair_pose_parallel: Positive`**: the router's height, which this unit rewrote as
  `k.height_of(o + u·mid)`. Nothing in that reading is an evaluated point of the ellipse: it is the line's numbers,
  the stored point and the stored axis. At 60 digits the line stands 7.6e-13 m over the plane through the stored
  point normal to the stored axis, along its whole span, and the stated curve 1.18e-12 m over it where they meet.
  `height_of` of the line's `f64` point there reads −1.09e-11, past the escalation: a point formed 1.4e5 m from the
  stored point moves by its coordinates' rounding (their spacing there is 2.9e-11 m) and the dot rounds at
  `u·|p − x|`, neither metered. `sure`.
- ε 1e-9, fat `a` = 1e9, θ = 0.5, the line at 90° lifted 1e-8: `Ok`, no meeting, true gap 3.8e-10. At 60 digits the
  line stands 7.89e-9 m over that plane and the stated curve 8.27e-9 m over it: **the curve the stored numbers state
  does not lie in the plane normal to its stored axis.** The stored seam is out of square with the stored axis by
  6.8e-17, so the curve leaves that plane by `a·|u·axis|·(1 − cos θ)`, 8.3e-9 m at θ = 0.5 and 1.35e-7 m at the
  antipode on this section (measured for all twelve ladder sections). `height_of` is `(p − x)·axis` and
  `Conic::along` projects the direction across that plane first, so every height this unit reads takes the curve to
  lie in it; `height_of` of the line's `f64` point reads 1.49e-8. The log ends on `curve_pair_pose_level_arc:
  Positive` and `curve_carrier_piece_clear_by: Positive`; I did not trace which inputs that last walk read, so
  `likely` for the decider. The P1 row's own table gives "—" for the evaluator's error at this pose.

Away from the anchor that is the row's own class ("a reader that decides from a datum rounded at a scale larger than
the tolerance, with no charge for it, certifies what it cannot see"), with the point's offset from the stored point as
the datum. Measured over the ladder's `f64` line poses: `height_of` is off its own exact value by more than the zero
level in 48 of 72 far poses at `a` ≥ 1e8 (ε 1e-9; up to 2.7e-8 m), and by at most 7e-14 m within 1 km of the
anchor. `on_carrier` shows the same reader at a second door (`review/probe_level.rs`, seam at the anchor, far
points): a definite `Off` inside the band's gap 4 times at each ε and once for a point within ε of the stored curve
(1e-12), the row's finding B persisting away from the anchor. **Consequence for the row filed:** P1's fix is "a meter
of the evaluator's running bound (`Conic::running_point`, or `Height::running_at`)". Both take the frame as exact and
neither meters `height_of` or `level_at` of a foreign point, so that fix leaves the first pose above certified, and
the second if it is what I read it as. The square-frame assumption is disclosed in the dispatch but in no row and no doc comment I could find (`conic.rs`
"A unit frame, by its type" says the opposite: "its axes unit and square by the mint that made it"); it is
unscheduled (Q6).

### MINOR

**m1. `Height::roots` answers the wrong extreme at an exact touch at the anchor.** DEMONSTRATED (measured wrong value,
`review/probe_roots.rs`, `review/out/probe-roots.txt`), `sure`. Arc-held, anchor at the seam's vertex, the height
along `−u_ref` from the anchor: it is `a(1 − cos θ) ≥ 0`, touching zero at θ = 0 only. `roots(0)` returns `[π, π]`,
where the height is `2a` (10 m on the 5 × 3 ellipse, 2e6 m at `a` = 1e6), at `f64` and at `Interval`
(`[3.1415926535897927, 3.141592653589794]`, an enclosure that excludes the root). The centre-held twin returns `2π`
and `0`. Cause: `conic.rs:710`, `h.select_le_zero(at_hi, at_hi + π)` breaks the tie `h = 0`, `Q = 0` toward the crest,
and the anchor is the trough when `P < 0`. With `+u_ref` it is right. No reader of this unit reaches it today (each
decides the extremes' signs first, and the plane door returns the right root, `Roots([0.0])`, through its graze arm),
so it is a door-level wrong value and not a wrong verdict; but it is an axis-aligned exact pose, the doc promises "its
extreme nearer it" only "where the height does not reach `level`", and no row pins it (my mutant that swaps the two
extremes outright dies on one assertion, the far-past-the-crest one; the tie itself is untested).

**m2. A re-seamed arc-held ellipse is certified wrongly BESIDE its anchor, and neither ladder holds one.**
DEMONSTRATED (`review/ladder_seamed.rs`, the unit's ladder with every section passed through
`Ellipse::seamed_near(π/2)`, judged by the unit's oracle unchanged), `sure` of the readings, `likely` of the cause.
Head, `f64`: 11 wrong at 1e-9 and 37 at 1e-12, of which **7 and 28 are within 1 km of the anchor** (the line door on the
fat sections, `a` = 1e9 at 1e-9, `a` ≥ 1e6 at 1e-12), seven of them "`Ok`, no meeting" for a line through the arc (1e-12, `a` = 1e8
and 1e9). The base reads 35 and 86 wrongly on the same poses, so nothing regressed, and no apart twin reads a
meeting. The cause is what `conic.rs` already says of a quarter-turned seam ("a point near the anchor holds to the
semi-axis's rounding, not its own"): the same level probe shows `level_at`/`clearance` reading the TRUE distance
there (5e-8 m at `a` = 3e8) while `on_carrier` answers `On` on the evaluator's point. It matters because the claim
"near the anchor reads at the arc's scale" holds only with the seam at the anchor, production re-seams sections
(`crates/topo/src/chord_join.rs:2053`, `crates/geom-brep/src/carrier_resolution.rs:202`), the P1 row is titled and
argued "away from its anchor", and `door_rows` tests the quarter-turned seam with a tolerance that admits the loss
(`2·ε_mach·A·|θ₀|`).

**m3. The regression floor is crossed, by refusals.** DEMONSTRATED (`review/out/ladder-right-to-refused-f64.txt`),
`sure`. `f64` poses right on the base and refused on the head: 64 at 1e-9, 122 at 1e-12, of which 18 and 14 are below
the floor (1e8, 1e5). Most are the ladder's 10ε twin, which is planted at exactly `band.escalate()` (NOTE N3), where
a refusal is as right as an answer. Four are not: meeting poses the base met and the head escalates on
`curve_pair_walk_pair_gap` (margins 1.8e-12 to 2.1e-9): ε 1e-9 the line, fat `a` = 1e7, θ = 2, 5°; ε 1e-12 the
circle, `a` = 1e4, θ = 2, three poses. All four are far from the anchor. The suspects' harness loses nothing below
the floor (its only right-to-refused are family F at/above it, 14 and 45, as the body says, and one G at 1e-12).
The body's totals (refused 234 → 230, 94 → 195) hide the 64 and 122.

**m4. The P3 row and the body attribute the `Interval` loss to the wrong reader.** DEMONSTRATED (counts from the
judged ladders), `sure`. Right on the base and refused on the head at `Interval`: 134 (1e-9) and 289 (1e-12). By the
refusing row: `curve_pair_pose_basin` on the circle door 103 and 245, `curve_pair_walk_pair_gap` on the circle door
27 and 42, `curve_pair_pose_level_arc` **2 and 2**. The P3 row says of `level_arc`'s 146 and 306 readings "most of
them poses the base read `apart` rightly": 2 of 146 and 2 of 306 were. The loss is the circle door's, and the same
probe that confirms C6 shows it is not the placement class: with `holds_only` reading the parts it used to pass
over (`likely` the cause; I did not bisect it), an ellipse held at its CENTRE with semi-axis 100 m, θ = 0.5, at ε 1e-12 goes from `Ok`, one meeting, to
`curve_pair_pose_basin`, and an arc-held one of 10 m at θ = 2 likewise (`review/out/window-base-1e-12.txt` against
`window-head-1e-12.txt`). So a centre-held verdict IS newly refused, at `Interval`, well below the floor, and nothing
schedules it: the P3 row names the line door and family F only.

**m5. "Every one is named in the tables" is not true of the unit's ladder, and the list of names is short.**
DEMONSTRATED, `sure`. `ladder/head-*.txt` print `refused:Escalated` and `refused:Unsettled`, never a row (the
suspects' tables do carry the reason). From my judged runs the head's `f64` refusals are `curve_pair_pose_basin`
(68, 78), `curve_pair_walk_pair_gap` (126, 79), `curve_pair_pose_closest` (31, 17), `curve_pair_pose_offset` (5, 19)
and `curve_pair_meet_window` (0, 2); `pose_closest` and `meet_window` are in no list in the body, and "escalations in
the band's gap (truth in the gap …)" does not describe them: 40 of the escalations at 1e-9 have their truth outside
the gap (34 apart, 6 meeting), and 95 `f64` refusals at each ε do (55 apart + 40 meeting; 48 + 47). The row asks for
"every refusal that remains named and metered".

**m6. `one_curve_gap` and `level_on` still form a derived centre; the gate's count fell because the read was
respelled.** BY-INSPECTION, plus the author's surviving mutant re-planted (S4, survives), `sure`.
`pose.rs:2044`, `k.point_at(0).mid(k.point_at(π))`, is `anchor − a·u` to a rounding for an arc-held ellipse: the same
derived centre, reached through the evaluator at the antipode instead of through `frame()`, and charged as before
(`rounding_charge`). It is exactly the shape `conic-centre-reads.sh` lists under "what it does not hold" ("a centre
restated without these doors"). The charge is one-sided (it can only withhold `Coincident`), so I found no wrong
answer; but the README's clause is "re-posed, never charged", the body's table files it under "fixed", and
`pose.rs` 8 → 1 counts it as gone. `level_on`'s `centre` (`pose.rs:2440`) is the same construction feeding a bound.
This is the trap the style brief names: a lane closing an unguarded reader of the centre wrote two readers of the
centre the guard cannot see.

**m7. `Height::mean()` is the derived centre's height under another name, and its arc arm has no row.** BY-INSPECTION
for the first half, DEMONSTRATED for the second (mutant R3: the arc arm returning the anchor's height; 491 passed at
both ε), `sure`. `self.at − (p·c₀ + q·s₀)` is `(centre − o)·n` exactly; three calls with the basis vectors hand out
the derived centre, and `unit_in_plane`'s `(c₀ + du, s₀ + dv)` hands out its unit coordinates. The dispatch fixed "no
door that hands out … a centre". Numerically both call sites are safe (in the parallel gate `p`, `q` are within the
band; at a graze `|mean| ≈` the swing, which was decided definite), and the plane door does answer an arc-held graze
rightly (probe). But no row holds a plane grazing an arc-held ellipse, so the arm that was written "to keep
centre-held bits" is unpinned for the placement the unit exists for.

**m8. The merge of main into the branch dropped a recorded seed from a work row.** DEMONSTRATED (`grep`), `sure`.
`work/germ/the-square-wall-fuzzer-finds-a-strip-crossing-at-a-tubes-top-read-tangent.md` on `bd07a867e7` (PR 7510)
carries "Another seed, red on main … `0x28ab5920d3c632ea`"; `33281203395d` adds a different entry at the same place;
the frozen head keeps main's and has lost 7510's. When 7510 and then this merge, the deletion wins silently.

**m9. Rows that cannot go red where it matters.** DEMONSTRATED (mutants; table under C9), `sure`.
`Over`'s level (the other plane levelled by its own point's height) can be dropped; `Level::jet_at`'s third
derivative, rewritten by this unit, can carry the wrong sign; `Window::height_fourth` can be zero; none of the 491
rows moves, and none of the 17 slow-set oracle rows of `curve_pair` either (`window::oracle`, `meet_oracle`,
`one_curve_rows`, `touch_spread`, run with `--ignore-default-filter` at both ε). In `arc_held_rows` the coplanar rows take the meeting's place from the answer and check it with
the kernel's own `clearance` on the ellipse only (`arc_held_rows.rs:332-341`, `367-376`), the tolerance on a
meeting's place is ten escalations (`reach = 10·band.escalate()`, `e * 10.0`), every row is the fat section at 0.3 m
with the seam at the anchor, and only the first two rows read `Interval` (and pass on any refusal).

### NOTE

**N1. Two premises of the dispatch.** (a) `git merge-tree --write-tree bd07a867e7 33281203395d` is not clean here:
it conflicts in the `work/germ` row of m8 and writes tree `ea0d163bab`, not `e066c47d25`. Against it the unit's own
change is 34 files, 16 under `crates/` at +1560 −330, as stated; the conflict is in a `.md` and the base builds.
(b) "CI has not concluded" stands; nothing here rests on it.

**N2. The body's verification is of an earlier tree.** It says the full differential ran on `83d198efa`; after it come
`cf4a0884a`, which changes code (the anchor's root angle, the two point means of m6), a merge of main (`0fe820035`)
and one more. The frozen head runs 10,605 tests under the profile; 10,592 is the base's count. My runs are of the frozen head: base 10,592 passed at 1e-9 and 10,591 with one failure at 1e-12; head 10,605 passed at 1e-9 and 10,604 with
the same one failure at 1e-12 (sweep's `a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`). The 13
more are the unit's rows; its ten `arc_held_rows`, copied into the base, are 10 of 10 red there at both ε.

**N3. Two things about the unit's ladder.** The "near … 1 km of arc" pose on a thin section is `θ = 1000/b`, which
on `a` = 1e8 and 1e9 puts the point 1.44e5 m from the stored point (`|q − S|`, measured): it is a far pose, and the
body's "far from the anchor … or 1 km of arc on a thin section" is that. And the 10ε twin is planted at exactly Kε
(K = 10), so its true gap is the escalation to a relative 1e-7: whether it is "apart" or "between" is rounding, and a
third of the ladder cannot tell an apart certificate from a refusal. The 1000ε twin does that work.

**N4. The oracle (C4e).** `oracle.py` states the curve from the stored point, phase, semi-axes, axis and seam, with
`v = axis × u` formed at 60 digits; nothing calls kernel code; `F(x)` reads each printed `f64` exactly. Independent,
`sure`. Three limits: the truth is one local closest pair from a seed; a meeting is judged by its nearest point only,
so an extra spurious point passes; and by taking `u` as stated it makes the stated curve non-planar at `u·a`, which
is the honest reading and is what M2 turns on.

**N5.** `Height` derives `Debug` with its private `held` field, so `{:?}` prints the placement and phase. Opaque by
visibility, not by type.

## The claims

**C1 (the doors).** (a) In ℝ each reading is the one stated, on both placements, *given a square unit frame*: I
checked `height_of`, the two levels and their gradients, `Height::at/slope/extremes/mean/roots` by hand (`A² − k² =
(p·s₀ − q·c₀)²`; the `tan(φ/2)` quadratic and its stable pair), and against 60 digits beside the anchor (level within
9e-14 m, `height_of` within 9e-14 m, within 1 km of the anchor at every size to 1e9; `review/out/level-head-*.txt`).
Not true without the frame square (M2). (b) The `c₀² + s₀² − 1` term is exactly zero for every stored phase; roots
beside the anchor read to their own precision (probe: ±1.4142135623730951e-9 at `a` = 1e6 where the centre-held
twin reads `[0, 0]`); the touch at the anchor is m1; near the antipode `h − 2P` cancels at `u·a`, which is the far
class. (c) No reader outside `conic.rs` matches on the placement (`Held::` appears only there and in `door_rows`);
what leaks is m7 and N5. (d) m7: a centre under another name, safe where it is used. (e) Not one home: beside
`Height` stand `Window::height_jet` (the same function's Taylor jet formed again from `height_of` and the
derivatives, its fourth bound from a `swing` field the caller computes), `Range` with a second root reader
(`RootsFrom::Crossings`), and `Level`. `Height` replaced `pose::height` and `Slant`'s tuples; it did not become the
polynomial the others read.

**C2 (the readers).** Read every row of the table against the tree. No reader in `curve_pair/`, `plane_conic` or
`clearance` takes `frame()`, `center()` or a hand-made `(centre, axis, u_ref)` for an ellipse; `span.rs`, `walk.rs`,
`polish.rs` and `curve_pair.rs` read no conic datum at all. Still on a derived centre: `one_curve_gap` and
`level_on` (m6), `mean()` in the plane door (m7). Different quantity: one, the router's reach, which was an outer
bound (centre ± the larger semi-axis) and is now the exact projection, so the `curve_pair_pose_gap` margin lost a
slack it used to have for free; sound in ℝ, and the vertex row covers it at one pose. `Over` is the same height
(`h − level`), `−lo·hi` is `A² − c₀²`, `min(hi, −lo)` is `R − |d₀|` (bit for bit for a centre-held conic), the new
jet is the old one term for term. `one_curve_gap`: the same derived centre, charged; not re-posed (m6).

**C3 (the level).** Holds beside the anchor, and that is all it claims: on the stored curve and ±0.5, 5, 30 ε off it,
fat and thin, `a` 1e2 to 1e9, both ε, within 1 km of the anchor `F` is right to 9e-14 m (over `|∇F|`), and
`on_carrier` gives no definite wrong answer and no definite answer in the gap there (seam at the anchor; the 16 `On`
readings past Kε at 1e-12 are the thin sections' θ = 1000/b point, hundreds of kilometres from the anchor, N3).
Away from the anchor `F` loses `u·a` (4.6e-7 m at 1e9) and decisions do read it definite and wrong (M2's `on_carrier`
counts). The author's "still divides by the squared semi-axes" is accurate; the aspect row is not met and is not
claimed.

**C4 (the ladders).** Reproduced exactly: suspects 164 + 9 → 0 and 553 + 22 → 0, right/refused 1978/566 and
1959/585; unit ladder 60 + 75 → 7 + 0 and 145 + 112 → 12 + 0; the four committed `ladder/*.txt` tables equal mine
byte for byte. (a) Wrong on the head and not in the tables: m2. (b) No apart twin reads a meeting: 0 on the unit's
ladder, 0 on the suspects' harness, 0 on the re-seamed ladder, both ε. `sure`. (c), (d): m3, m4, m5. (e): N4.

**C5 (the nineteen).** M1 and M2. One is decided by this unit's router on a reading no evaluated point enters, a
second likely so; the wrong certificates are not acceptable under the acceptance as written.

**C6 (`holds_only`).** Both halves hold. On the base, at `Interval`, a circle through the ellipse's point reads
`Ok`, no meeting, for the arc-held ellipse AND for the same locus held at its centre (ε 1e-9: `a` = 1e6, θ = 2;
ε 1e-12: `a` = 1e4, θ = 0.5, and 1e6); the head refuses both. At `f64` `reaches` differs from `positive` only on a
NaN width. The row `an_interval_window_reads_a_part_it_has_not_decided_empty` goes red on the author's mutant (not
re-planted by me) and uses the arc-held placement only. Its cost is m4.

**C7 (the `Interval` touch).** The measurement reproduces (±2.0e-12 rad about the anchor against ±5.3e-8 about the
centre at 0.3 m; ±1.9e-9 at 3e5 m), and it is an ignored test that prints: nothing goes red if it stops being true
(Q6). The enclosure along a conic's own normal is as disclosed, and not only at size: `[−2.7e280, 7.4e-14]` and
`[−7.6e-14, 2.7e280]` at `a` = 100 m. I found no reader that decides on it unsoundly: `Over`'s parallel screen
errors on it and the other conic's narrow reading decides, the slant it feeds is an upper bound, `Slant::read`'s
floor takes it to zero, the plane door's belly cannot decide and its parallel offset reads `mean()`. In-plane pairs
at `Interval` answer rightly or refuse at every size tried (`review/out/window-head-*.txt`), and better than the
base. `door_rows`' `holds` does pin a width (`≤ 4·slack`); none of its rows reads the normal direction.

**C8 (the differential).** Run on the frozen head and on the base: base 10,592 passed at 1e-9 and 10,591 with one failure at 1e-12; head 10,605 passed at 1e-9 and 10,604 with
the same one failure at 1e-12 (sweep's `a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`). The 13
more are the unit's rows; its ten `arc_held_rows`, copied into the base, are 10 of 10 red there at both ε. No row differs between base and head but the unit's own and the five it re-baselined or moved, each of which
passes on its own tree. The five moved rows: `a_graze_side…` green unchanged (the body's account of
`mean()` is consistent with the code: bit for bit `d₀` at the centre). The plane-crossing fixture: K = 10, so the old
δ = 1e-9 put the crossing at exactly the escalation and the row depended on a last bit; 5e-10 is mid-gap. The move is
right, and it is the fixture that was wrong. `MAP_RESIDUAL`: a refusal at one rung became a refusal at another for a
pose in the gap; acceptable, but the array is now empty and its branch, doc and the pcurve pin are dead text (style).
The two counts (24 → 27, 33 → 25) I read and did not re-derive: they are equality pins on how many band-edge queries
disagree across chart frames, so any change in a root's last bit moves them, and "57 → 52 in all" is a consolation,
not an argument; `unsure`. Which formula is kept: `curve_pair`'s (crest ± `2·asin √r` from the extreme that crossed
the less); the plane door's `acos(−d₀/R)` is gone. I found no centre-held verdict that became wrong; one became a
refusal at `Interval` (m4), `likely` by `holds_only` and not by the roots.

**C9 (the mutants).** Planted by me, each alone, both ε, 491 tests (baseline 491 passed):

| mutant | result |
|---|---|
| S1 the foot seed from the centre | survives (as claimed) |
| S2 the level jet's `P″` as `c − p` | survives (as claimed) |
| S3 `unit_in_plane` from the centre | survives (as claimed) |
| S4 `one_curve_gap` from the derived centres | survives (as claimed) |
| K1 `height_of` from the derived centre | killed, 7 rows at 1e-9, 8 at 1e-12 |
| R1 `Over`'s level dropped | **survives** |
| R2 `Slant::read`'s whole-turn arm dropped | **survives** |
| R3 `Height::mean` on the arc returns the anchor's height | **survives** |
| R4 `Level::jet_at`'s third derivative with the wrong sign | **survives** |
| R5 `Level::jet_at`'s second derivative with its squares term halved | killed, 5 rows |
| R7 `anchored_roots` reads the farther extreme where it does not cross | killed, 1 row (`door_rows`) |
| R8 the arc-held trough halved | killed, 1 row (`door_rows`) |
| R9 `Window::height_fourth` zero | **survives** |

R1, R3, R4, R9, S1 and S2 were run again against the 17 slow-set oracle rows of `curve_pair` (the dense-distance
window oracle among them), at both ε: all survive. The four claimed survivors: each argument is true as far as it goes. S1 is a seed. S3's one reader in
`curve_pair` is a magnitude; but the body names one other caller and there are two, and one of them DECIDES on it:
`topo/src/splitting/section_loops.rs:447` (`conics_clear`, `1 − |c′| − reach` against the band) and
`sweep/src/blend/open/end_face.rs:390`. Both changed behaviour with the override, neither is in the 491 tests, and I
ran no topo or sweep row against S3 (`unsure` whether any would see it). Note also what the override cannot buy:
`c₀ + du` beside `c₀ = 1` keeps `du` only to `u`, so the unit coordinates of a point beside the anchor still round at
`u·a` metres, as from the centre. S4 is m6: "by design"
means the reading was not re-posed. S2: the argument is "inside a Taylor bound"; R4 shows the jet's next term is not
pinned at all in the ci set, so I would not lean on the bound. Of mine: R2 is dead code (after `row::SWING` reads the
swing definite, the residual cannot lie within ±e all round), an equivalent mutant; R1, R3, R4, R9 are m9. R7 and R8,
the two in the unsound direction at the new door, die on one `door_rows` assertion each and on no reader's row.

**C10 (the gate and the sweep).** Counts as stated (`pose.rs` 1, `window.rs` 0, `on_surface.rs` 2); the gate passes.
The left sites are circles by dispatch, not by type: `coplanar_circles` takes `&dyn Conic` and is reached only from
`Arm::CircleCircle`. My sweep (`rg` over `curve_pair/`, `curve_face/pose.rs`, `geom/src/curves/` for `center()`,
`frame()`, `origin()`, `unit_in_plane`, `in_plane`, `mean()`, `plane_offset`, `semi_axis_vectors`, `point_at(π)`,
`data()`, and for conic kinds matched by hand) finds what the gate cannot count: the two antipodal means (m6),
`mean()` and `unit_in_plane` (m7), and two trait defaults that still read `frame().origin()` (`Conic::height_of`,
`Conic::unit_in_plane`, `conic.rs:307-309`, `334-336`), correct for the circle that inherits them and a silent centre
read for any third implementor. What my pattern cannot match: a position restated through a `Frame` handed on, and
anything in `curve_face/` beyond the plane door (`conic_quadric.rs`, `circle_torus.rs`, `implicit.rs` stay on the
gate; out of this unit's scope).

**C11 (the rows).** P1: true of the tree in its table, wrong in its cause for one or two of nineteen and in its fix (M2),
silent on the re-seamed case (m2); P1 as banded is defensible (the readers' architecture guarding a latent wrong
answer, as the parent). P3: its first paragraph is not true of the tree (m4); P3 for refusals is the right band.
The `cpair` seed: true (`CAD_FUZZ_SEED=0x11467e023845fce7` at 1e-12 fails
`each_window_reading_holds_on_a_dense_distance` on draw 48 with the same two `Recedes` readings on the base and on
the head). Disclosed and unscheduled: the square-frame assumption (M2); the unpinned width (C7);
"the slow set and the demos were not run" (the slow rows in files this diff edits are selected per PR, so the hosted
run does take them; the demos nothing schedules); the `Interval` loss on the circle door (m4).

**C12 (the body).** True of the frozen head except: the test count and the commit the differential ran on (N2);
"every one is named in the tables" (m5); the account of the `Interval` loss (m4); "every remaining wrong reading …
not a reader of where the conic is" (M2); and it does not say that ten of the nineteen are new (M1). The disowned
reds, each run on the base and on the head: `recourse_roster` fails in geom-brep and in topo on both, with the same
message line for line; `certified_torus_answers_hold_against_the_true_distance` at seed `0x53500b4c590de496`, ε 1e-12,
fails on both alike; sweep's `a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta` fails at 1e-12 on
both and passes at 1e-9 on both. All three are the base's.

## Style

Questions exercised: **Q1** (prose sweep and constants sweep over the added lines; found the copies below),
**Q2**, **Q3** (by mutants), **Q4** (who cited the old premises: the module docs), **Q5**, **Q6**, **Q7**, **Q8**
(`conic.rs` read end to end, 2,320 lines; `pose.rs`'s 700-line header read for staleness, its body by diff only).

- **The fix mints the defect it closes.** `crates/geom-brep/src/curve_pair/pose.rs:2044`, `2440`: two centres formed
  outside the conic from two evaluated points, in the unit whose point is that nothing outside `conic.rs` forms one,
  and invisible to the gate that counts the unit's progress. `sure`. Class: any "mean of antipodal points", `P + P″`,
  or `anchor − a·u` by hand; I would look in `curve_face/conic_quadric.rs` and `implicit.rs` when their turn comes.
- **Two homes for one root rule.** `crates/geom/src/curves/conic.rs:662-673` (`centred_roots`, "the extreme that has
  crossed the less", `2·asin √r`) and `crates/geom-brep/src/curve_pair/pose.rs` `Range::roots`' `Crossings` arm with
  `crate::roots::arc_past` (`roots.rs:328`). The same formula in two crates, the lower one unable to call the upper;
  neither says so. `sure`.
- **`parallel_as` beside `constant_as`** (`pose.rs:2182` and `2160`): the same three-arm match, one over
  `(value, spread)`, one over `(lo, hi)` with the row hard-wired. `sure`.
- **A test fixture written out eight times.** `swing: { let (p, q) = ka.along(kb.axis()); (p² + q²).sqrt() }` in seven
  `window.rs` rows and `window/oracle.rs:181`, where production writes `h.swing()`. A `Window` is now built from a
  number its caller must compute the way `Height` does, by convention. `sure`.
- **The module docs describe the reading the unit removed.** `pose.rs:16` ("the conic's centre ± its larger
  semi-axis"), `:209` (`cos φ₁ = (e − c₀)/A`), `:225` (`cos φ₀ = −c₀/A`); `window.rs:216-218` ("the twelve of `c₁`
  and `s₁`", fields that are gone). The doc rotted, the code is right (Q4, first sub-case). `sure`.
- **A count with its derivation deleted.** `pose.rs:2452`, `up::<32, _>`, was `24` with its roundings enumerated ("a
  dot product's five, `ρ`'s four more, and each side's six"); the new doc says "raised past its roundings". Nobody
  can check 32. `sure`.
- **An invariant by `unreachable!`.** `pose.rs:2534-2538`, `Range::band_arc`: `let RootsFrom::Along { .. } = self.roots else {
  unreachable!(…) }`. `Range` became a struct with an enum inside so that one of its two methods is only valid for one
  variant. `likely` there is a cleaner split.
- **Two orders for one pair.** `Height::roots` returns `[falling, rising]`; `Slant::roots` (`pose.rs:1785`) turns it into `(rising,
  falling)` three lines from where `Slant::read` destructures `[near_f, near_r]`. `sure`, taste.
- **Dead code left by a re-baseline.** `crates/sweep/tests/section_end_poses.rs:356`, `MAP_RESIDUAL: [..; 0] = []`
  with its `contains` branch, its pin and a doc paragraph about a pose that is no longer in it; and `Slant::read`'s
  `all_round` (`pose.rs:1827`, mutant R2). `sure`.
- **Two rules for "the plane of an edge".** `on_surface.rs:766-781`: a circle's at its stored centre, an ellipse's at
  its span's middle point, chosen because the other choice moved two counts (the body says so). The comment
  reconciles two spellings of one rule (Q2). `likely` worth one rule; `unsure` which.
- **Justification by bit-identity.** `mean()` exists, by the body's account, so that a centre-held row keeps its
  bits. That is D9's fence, so I only note that the door it added is the one the direction excluded (m7).
- **A claim resting on a measurement with no guard** (Q6): the `Interval` touch (C7); the body's "no pose found where
  it decides" for S2; the nineteen's attribution.
- **`arc_held_rows`' tolerances** (`e * 10.0`, `10·escalate`): a meeting 9 Kε from where it was built passes. `sure`
  it is loose; `unsure` whether a tighter one holds at 1e9.
- **`OnSurface::plane` and `Curve3::clearance` keep an `#[expect]` for their circle arm**, so a second read added to
  either function's ellipse arm would be covered by the expectation (the gate says as much of itself). `likely`.
- Class note for the fix pass: m6 and the trait defaults are one class (a centre the gate cannot see); sweeping only
  `one_curve_gap` would be a half-fix.

## Ratings

- **Idiom and structure: 3.** `Height` is a clean opaque value and the readers got shorter; but it carries the
  placement enum and matches on it in six methods, and the unit leaves three other harmonics and two respelled centres
  beside it.
- **Test quality: 2.** The rows pin the near-anchor contract and die on the centre mutant (K1: 7 rows), but five of my
  eight mutants survive (one of them equivalent), the unsound ones at the new door die on a single unit assertion each, nothing holds a
  re-seamed, thin, grazing or far pose, and the placing tolerance is ten escalations.
- **Doc and comment honesty: 3.** The doors' docs say what they do and the body discloses a great deal; but the body's
  central attribution is wrong for at least one of nineteen, a filed row misstates what refuses, the module header still
  describes the removed reading, and "named in the tables" is not so.

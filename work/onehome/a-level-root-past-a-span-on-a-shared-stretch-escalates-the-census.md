---
id: a-level-root-past-a-span-on-a-shared-stretch-escalates-the-census
kind: issue
title: A Level root past a span, on one stretch with a found end meeting, escalates the census where main read it apart
status: open
opened: 2026-11-20
priority: P2
cost: M
---


Filed by ONEHOME from O1's PR (the slant reading, `Slant` in
`crates/geom-brep/src/curve_pair/pose.rs`). It is that PR's, not
main's: on main the two rows below are sound.

**The answer.** `sweep`'s `review_arm_vs_reach_probes::review_curved`
(`r = 0.05`, `c = 0.05`, `θ = 1e-4`, both `lb`: `A ∪ B`, `B ∪ A`,
`B ∖ A`) and `review_curved_plate` (`r = 0.05`, `lb = 0.04`,
`θ = 1e-4`: `A ∪ B`, `B ∪ A`) return the same body as main, but its
tier 3′ census escalates: `CensusEscalated` on
`curve_pair_span_past_reach_straddle`, margin `[−2.7·10⁻⁵, 10⁻⁵]`. The
rows pin it as known (`crates/sweep/tests/review_arm_vs_reach_probes.rs`,
the `report` filters at `review_curved` and `review_curved_plate`).

**The pose.** The boss's edge (the line, span `[0, lb]`) and the disc's
arc share the vertex at the origin, where the edge crosses the circle
at `1e-4` rad. The ends are read first (`crates/geom-brep/src/curve_pair.rs:424`),
so the vertex is a known meeting, and `owed`
(`crates/geom-brep/src/curve_pair/pose.rs:2285`) hands the Level route
only the mirror root, at `x ≈ −10⁻⁵`. That root is placed past the line
span (and past the arc, where the arc is split there). The two roots lie
on one stretch: the sagitta between them is `2.5·10⁻¹⁰`, inside
`e = 10⁻⁸`, so `R ≤ e` all round. The stretch's run is `~10⁻⁴` m, so
the mirror root's window (`place`, `pose.rs:1454`) holds the vertex
meeting itself. Its ball is not clear of either span, and `Span::past_by`
straddles. Main read the root posed, as apart; the slant reading reads
it as a pair (`WindowSpans::MayPass`, `curve_pair.rs:474`),
whose window does not lie on the spans.

**The open argument.** Drop a candidate placed past a span when a
proved inequality (`R ≤ e` all round, or `R ≥ −e`) puts it on one
stretch with another root, offered or known. Half of the argument
closes:
- Any on-span pair in the zero run that holds the dropped root is
  reached from it along that run (the conic's point and its foot on the
  line both move continuously). The path leaves the spans' rectangle at
  a pair within the zero with one parameter at a span end and the other
  inside its span. `end_candidates` reads that end against the other
  span first, so a meeting there is known, or the end read escalates.

What does not close is the rest of the root's arc of `|R| ≤ zero`.
There the zero set is the preimage of the disc of radius `zero` under
`P(θ) = (R, H)`, the conic projected along the line. Where `R` is
monotone on the arc, `H` as a function of `R` is convex (a piece of
the projected ellipse). Such a graph can cross the disc, dip below it
and re-enter it, which is two components. Only one of them holds the
root, and the other can lie wholly on the spans. Kept, the past root's
window reads that component and escalates. Dropped, it is held only
where the other candidate's reading holds it, and a meeting candidate's
window is not read. In the plane (`H ≡ 0`) every component holds a
root, and the argument closes. So a fix needs either a bound on `H`'s
turn over the arc that the Level route's tilt premise gives (`H`'s
amplitude is the conic's size times the tilt, which the router read
level), or the drop restricted to a decided `H ≡ 0` within the band.

**The same question stands under "One candidate stands for its
stretch"** (the module docs, "A slanted pair in one plane"). A
stretch whose one candidate reads a meeting is read as one meeting,
and a second component of the kind above would be a second meeting
with no candidate. This is to be checked, with a row if it can be posed
at the band's scale. The bound on `H`'s turn would close both.

**Where the rootless second component is ruled out.** `pose`'s
module docs prove it ("No second part on a side arc"): on a side arc,
under (S), definite crossings, and (T), the tilt's second branch, no
second part exists for `k = e/z ≥ 2.3247…`. At `k` below `2.33` a slanted
pair refuses (`curve_pair_pose_slant_tight`).

**What the proof does not cover, the remaining proof of this item:**
1. **The decisions taken exactly.** (S) reads "decided past the band"
   as `> e`, and (T) reads "not definite" as `≤ e`. Real margins carry
   rounding charges and levers (`Margin::levered`). The bound is
   continuous in `e`, so that slack moves `k` a little, by an amount not
   quantified.
2. **The first tilt branch**, where `tilt·(len + 2e)` reads zero. `len`
   is the projected overlap, possibly much shorter than `a`, so
   `a·|den|` is not bounded by `e` and the chain's
   `ab·|den| ≤ A·e` fails.
3. **The tight band, `k` below `2.325`.** Not proved, and refused.
   Searches (400,000 draws in the plane per band, 30,000 in space per
   branch per band) found two parts only at `A ≈ 1`–`4e`, always less
   than `e` apart, which ring as one.
4. **One-arc stretches** (a touch whose extreme is in the zero band, or
   `hi ≤ e` through `owed` with known meetings) lie outside (S), and the
   monotone graph fails there. The full review's M1 was in this class.
5. **Exact arithmetic only.** The floating-point floor and gap readings
   are separate.

A lean below the zero that turns over the arc, read as flat, is not
argued either.

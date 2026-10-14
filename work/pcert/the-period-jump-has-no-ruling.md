---
id: the-period-jump-has-no-ruling
kind: issue
title: what a periodic reduction or a strict branch owes at its jump has no ruling
status: closed
opened: 2026-10-13
priority: P3
cost: M
refs: [periodic-branch-floor-remains-at-the-raw-picks, periodic-branch-floor-remains-at-three-sites]
closed: 2026-10-14
---


Found by PCERT's `periodic-branch-floor-remains-at-the-raw-picks`, by
measurement. Two kinds of site make a `floor` decision whose jump has a
convention but no ruling. Each could be decided as structure, as
`geom_brep::whole_periods` decides a nearest branch, but only once the
jump's meaning is ruled. This item asks the question; it does not answer
it.

Measured with a scratch probe over the topo, sweep, step-import, mesh,
editor-core, geom-brep and profile suites at default ε (2026-10-13).

## 1. The sphere-pole strict branch

`topo::chord_join::run_azimuth_images`, at a run vertex decided on a
sphere pole, takes the unique branch in an open period, chosen by the
face's orientation: `q.floor() + 1` advancing, `−(−q).floor() − 1`
returning, with `q = (prev − raw)/τ`. Its jump is at integer `q`: the
entry meridian leaving along the half-meridian the last one arrived on
(a slit).

- **Reached:**
  - f64: 88k calls, **80k exactly on the jump** (e.g.
    `pi_seam_and_kiss_through_the_boolean`, `snowman`), landing a whole
    period on by the floor's convention;
  - `Dual`, `Sym<f64>`;
  - `Interval`: straddling the slit. The joint after it escalates
    (`split_window_branch`, PR 4875's ruling (A)).

What a structural form needs:
- (i) **a meter that is not the joint's lever.** At a pole the lever is
  band-sized, so a lever-metered mark always reads Zero. Candidates: the
  sphere's radius (the meridians' turning in metres at the equator), or
  the angle itself against a ratified angular band.
- (ii) **a ruling on the jump.** When the two meridians coincide (a slit),
  is the strict-next branch the one a whole period on (today's f64
  convention), a refusal, or an escalation as at `Interval`?
- (iii) **a literal.** Given (i) and (ii), `whole_period_count(gap ± τ/2)`
  gives the strict-next branch, with its marks at whole periods (the
  jump).

## 2. `reduce_periodic`, the `[0, τ)` reduction

`x − p·⌊x/p⌋` (`geom_core::Real::reduce_periodic`), and the same
open-coded. It is a reduction, not a branch pick: its result is an
extent or a window position. Its jump is at `x ≡ 0`, where a coincident
end reads as 0 or as a whole turn.

Sites:
- `topo::chord_join`'s arc extent (`(th2 − th1).reduce_periodic(τ)`);
- `topo::splitting::classify`'s `t₀` retry anchor (and its test twin);
- `topo::shell`'s split (period π) and half reads;
- `topo::boolean::boxes`' crest read;
- `profile::sugar`'s `ccw` and extent;
- `topo::boolean::rim_wedge`'s open-coded `x − τ·⌊x/τ⌋` (×4);
- `editor-core::names::emit_topo`'s `Track` offset and its `fwd`
  closure, both open-coded `x − p·⌊x/p⌋`.

Reached:
- f64 at every site, many exactly on the jump (`rp_chord_join_arc`,
  `rp_boxes`, `rp_rim_wedge`, `rp_shell_split`, `sugar_signed`'s twin);
- `Sym<f64>` at `boxes` and the arc extent;
- `Interval` **wide**, a whole turn, at `boxes` (8 tests), whose rows
  rely on the straddle to answer.

## 3. A signed angle's half turn: which branch

`profile::sugar::signed_swept` folds `(to − from)·turn` into `[−π, π)`,
and its readers read the SIGN as the branch: the fillet reach and fit
gates, and the path corner gates. At `d = ±π` both fold to `−π`, so a
point exactly half a turn ahead reads behind. Reached at f64 147 times in
the suites (scratch probe). Today's soundness is a configuration
argument at its doc: a tangent point diametrically opposite the corner,
which the reach and fit gates classify first. The question is the same
one: what does the half turn owe, ahead, behind, or a refusal?

## The question

What does a coincident end owe at a periodic jump?
- (a) Zero: a coincident end is no extent.
- (b) A whole turn: a coincident end closes the circle. This is today's
  convention at the strict pole branch, and at `reduce_periodic` where a
  caller has already decided the span is a full turn.
- (c) Neither: the jump refuses or escalates, and the caller states which
  it means, from structure it already holds (a closed carrier, a decided
  full span).

Under any answer, the `Interval` straddle at `boxes` must keep answering
or become a typed refusal that its rows re-pin, as PR 4875's slit did.

## The answer

Ruled by Ev (PR 4975, 2026-10-14): as recommended, with no half-turn cap on an arc side ("i don't want 'an arc side sweeps at most half a turn to its corner'"; then "sounds good!"). Group 1 is PCERT's to implement; group 2 is doc sentences; group 3 is PATHS' (`an-arc-side-sweeping-half-a-turn-or-more-to-its-corner-is-refused-as-behind`).

No one convention: the three groups are three different quantities,
and one principle covers them. **A periodic reduction or branch pick
owes nothing at its jump. A whole turn is never read off numbers; it
comes from structure the caller holds (a closed carrier, a self-loop,
an edge the face lies on both sides of). Where structure says the two
ends are distinct, a coincidence is a typed refusal, and one in band
escalates. Where the reader is invariant under the branch, nothing is
owed.** The filing's (a), (b) and (c) are each right in one case: (a)
for a bare numeric coincidence, (b) where structure says so, (c) where
a coincidence contradicts the structure.

The counts above conflate two jumps. The pole site's "80k exactly on
the jump" is the nearest-branch half-period tie (`q = k + ½`, the
two-band ball's meridians half a turn apart), the middle of the strict
pick's open period. The strict pick's integer jump is reached 4–8 times
in the suites, all in `pole_slit_window`, all with the walk entering
and leaving along the same edge. `chord_join`'s arc extent reached its
jump 0 times in 52k calls.

1. **The sphere-pole strict branch.** Same edge in and out: a whole turn,
   as a literal (`k = ±1` by orientation) at every scalar. Distinct
   edges: `whole_period_count` with marks at whole periods, metered by
   D4 ¶1's parallelism lever, the angle between the two edges' tangents
   at the pole levered by the length over which it displaces them (the
   shorter edge's furthest distance from the axis, capped at the
   sphere's radius, and for a curved edge at its relative curvature
   radius). A Zero mark (two distinct edges leaving along one direction,
   an overlap or a cusp) is a typed refusal; in band escalates. The
   `Interval` straddle at the slit goes, and `pole_slit_window`'s
   escalation rows re-pin to answers.
2. **The `[0, τ)` reductions.** No code change. Every site's reader is
   invariant under the branch, or the caller states the jump as a type
   (`ArcEnd::WholeTurn`, `arc.closed`). `reduce_periodic`'s doc says
   coincidence reads zero and a whole turn comes only from structure;
   `rim_wedge` and `emit_topo` gain the sentence their readers lack. The
   `Interval` whole-turn box at `boxes` is the honest enclosure of a
   continuous function and stays.
3. **`signed_swept`'s half turn.** Not a branch question: folding into
   `[−π, π)` gives a circle a "behind" it does not have, and today an
   arc side sweeping half a turn or more to its corner is refused
   `BehindIncomingRay` (filed on PATHS:
   `an-arc-side-sweeping-half-a-turn-or-more-to-its-corner-is-refused-as-behind`).
   The path gates read the forward sweep in `[0, τ)` and refuse only a
   zero sweep; the fillet setback folds about the leg's middle,
   `m + centred(d − m)` with `m = extent/2`, its branch decided as a
   `whole_period_count` literal at the leg's radius, so the jump sits
   off the leg and a semicircular leg whose tangent lands on its far end
   fits exactly.

No ratified text changes; the pole meter follows D4 ¶1 as written.

## Closed

Groups 1 and 2 by `pcert/period-jump-pole-branch`. Group 3 is PATHS'
row, `an-arc-side-sweeping-half-a-turn-or-more-to-its-corner-is-refused-as-behind`.

- **Group 1.** `topo::chord_join::pole_branch`: the same edge in and
  out is the previous branch `± 1`; distinct edges take
  `whole_period_count` of `gap ± τ/2`, metered at `pole_lever` (the
  shorter reach of the two edges from the axis, capped at each curved
  edge's `1/κ_g`). A Zero mark refuses `SplitJoinError::PoleEdgesCoincide`,
  and a mark in band escalates (`split_sphere_pole_branch`).
  `pole_slit_window`'s interval walk now answers on every slit, each
  window end a point-width enclosure of the `f64` one.
- **Group 2.** Doc sentences on `Real::reduce_periodic`,
  `rim_wedge::locus_param` and `emit_topo`'s `Track::offset`.
- **Census before the change** (ci profile, default ε, topo, sweep,
  step-import, mesh, editor-core and geom-brep, about 39.6k pole calls).
  Every distinct-edge call kept its k: no mark, nothing in band. The
  four same-edge calls were all in `pole_slit_window`. At `f64` their k
  was unchanged, and at `Interval` the `[−1, 0]` and `[0, 1]` straddles
  became the `f64` literals.

Filed from the review of PR 5080:
- `the-pole-lever-overstates-a-lens-about-four-times` (PCERT, design);
- `inline-tau-reductions-do-not-enclose-tau-at-interval` (PCERT, with the
  territory noted);
- `mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers` (TESS).

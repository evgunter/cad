---
id: the-period-jump-has-no-ruling
kind: issue
title: what a periodic reduction or a strict branch owes at its jump has no ruling
status: open
opened: 2026-10-13
priority: P3
cost: M
design: true
refs: [periodic-branch-floor-remains-at-the-raw-picks, periodic-branch-floor-remains-at-three-sites]
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
- `topo::boolean::rim_wedge`'s open-coded `x − τ·⌊x/τ⌋` (×4).

Reached:
- f64 at every site, many exactly on the jump (`rp_chord_join_arc`,
  `rp_boxes`, `rp_rim_wedge`, `rp_shell_split`, `sugar_signed`'s twin);
- `Sym<f64>` at `boxes` and the arc extent;
- `Interval` **wide**, a whole turn, at `boxes` (8 tests), whose rows
  rely on the straddle to answer.

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

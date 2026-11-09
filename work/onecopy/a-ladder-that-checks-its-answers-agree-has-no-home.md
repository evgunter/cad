---
id: a-ladder-that-checks-its-answers-agree-has-no-home
kind: issue
title: Two searches read every rung and refuse where two answers disagree, and ladder has no climb for that shape
status: closed
opened: 2026-11-05
priority: P2
cost: M
closed: 2026-11-09
branch: onecopy/refusal-standing
pr: 6606
---


Two door searches read EVERY rung, and refuse where two decided readings
disagree. `geom_brep::ladder::climb` stops at the first answer, so it
cannot hold them, and each spells its own copy of keep, rank and agree:

- **`topo::boolean::join::seam_germ_side`** (`crates/topo/src/boolean/join.rs`, ≈5617 on main at closing).
  It reads every chord, and refuses `ClassificationInvariant` where two
  chords read the loop's regions on opposite sides. A region normal
  refused in band (`Err(_) => continue`, gone with the port, registered in
  `scripts/gates/escalation-discards.counts`) and an in-band germ
  (`germ_side`'s `Ok(Sign::Zero) | Err(_) => None`) are dropped, not
  kept. "No chord decides" is `Ok(None)`, which leaves the loop to the
  witness ladder's evidence.
- **`topo::boolean::sectors::tangent_lump`'s two bounds**
  (`crates/topo/src/boolean/sectors.rs:1333`, the `bounds` loop). It
  reads both bounds, and refuses where they curve to opposite sides. It
  keeps the first escalation, which refuses only where neither bound
  shows a side.

The shape is the ladder's `Answering::Any` with its premise checked on
every decided rung: "every rung that answers gives one answer", read
rather than assumed. That is the debug check
`work/onecopy/door-searches-left-on-the-hand-rolled-ladder.md` asks for
on the `Any` ports, whose premises are prose nothing checks
(`whole_body_side`, `stands::ladder`, the chord-join side ladders,
`one_region::read_cycle`, `fate::rehome`, `slit::holding`,
`side_reader::cast_aims`, `chart_singularity::Sides::holds`). So one
home, e.g. `ladder::climb_agreeing`, could serve both: in production at
these two sites, and as a debug check on the `Any` ladders.

Open design:
- what a disagreement returns (an abort the caller maps);
- whether a kept reading ahead of an answer is superseded where every
  answer agrees (`Any`'s record) or stays live;
- what the debug form costs per ladder (a reading per ladder, or every
  rung).

Measured at `onecopy/ladder-climb-3` with probes over the topo and
geom-brep `ci` set and the sweep and step-import `ci` set: no test drops
a region normal in `seam_germ_side`, and `tangent_lump`'s in-band bound
is reached only by its own unit test
(`tangent_lump_is_three_outcome_honest_on_the_arm`, twice).

## Decided (2026-11-06, orchestrator ruling A)

**One home: `geom_brep::ladder`.**
- `climb_agreeing(rungs, read, agree)` reads every rung. An abort stops
  it. Where two decided answers fail `agree`, it returns
  `Err(Disagree { first, other })` and keeps every run live. Where all
  agree, it answers the first, and the kept runs are recorded as
  superseded (`Any`'s record). Where nothing answers, it refuses with
  the evidence, as `climb_evidence` does.
- `climb_evidence` and `climb_agreeing` share one recording (`Held`),
  so a pass ahead of every kept run is spliced at once.

**A disagreement is an abort that the caller maps.** Two readings that
contradict are the ladder's own abort wording. Both ports map it to
`BooleanError::ClassificationInvariant`, as their hand-rolled copies
did.

**Ports.**
- `join::seam_germ_side` climbs agreeing over the chords. A region
  normal refused in band is now kept (`rung_or_abort` by
  `NormalAtError::refusal`), not dropped. The registered
  `Err(_) => continue` is gone, and `join.rs` leaves
  `escalation-discards.counts`.
- `sectors::tangent_lump`'s two bounds climb agreeing. An in-band bound
  is kept, and refuses only where neither bound shows a side, as before.

**The debug check on the `Any` ladders is not in this row's PR.** It
was built in commit `a8e175f9`, which is in `onecopy/refusal-standing`'s history (`climb_any`: `climb(Any)` in production, and in a debug build
every rung after the answer read off the record and asserted to agree),
and wired at five sites whose premise held. It cost about 25% on topo
`ci` with every `Any` site wired. It found two premises false, filed P1
as `side-at-infinity-aimed-rays-disagree` and
`whole-body-side-reads-both-sides-at-a-pinch`. The design fork those
raised converged on a different form: `Answering::Any` kept for theorems
of the rule, an `Alike` kind for "every rung reads one fact of one
input", and the every-rung audit as a probe-gated test, not a debug
build's. That goes to Ev as an `[ev]` PR, and the wiring waits on its
answer, so it is the fork's to land, not this row's.

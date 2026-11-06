---
id: a-ladder-that-checks-its-answers-agree-has-no-home
kind: issue
title: Two searches read every rung and refuse where two answers disagree, and ladder has no climb for that shape
status: open
opened: 2026-11-05
priority: P2
cost: M
---


Two door searches read EVERY rung, and refuse where two decided readings
disagree. `geom_brep::ladder::climb` stops at the first answer, so it
cannot hold them, and each spells its own copy of keep, rank and agree:

- **`topo::boolean::join::seam_germ_side`** (`crates/topo/src/boolean/join.rs:5605`).
  It reads every chord, and refuses `ClassificationInvariant` where two
  chords read the loop's regions on opposite sides. A region normal
  refused in band (`Err(_) => continue`, line 5647, registered in
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
- `climb_any(rungs, read, agree)` is `climb(Answering::Any)` in
  production. Under `debug_assertions`, where it answered and did not
  abort, it reads the remaining rungs detached (unrecorded) and panics
  where an answer fails `agree`. So the `Any` premise is read, not
  assumed. The debug cost on topo `ci` is about 25% with every `Any`
  site wired, and within noise on the five that are wired.
- `agree_ok` compares two `Result` answers, `Ok` and equal.
- `climb_evidence`, `climb_agreeing` and `climb_any` share one recording
  (`Held`), so a pass ahead of every kept run is spliced at once.

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

**`climb_any` wired** at `one_region::read_cycle`, `fate::rehome`,
`slit::holding`, `chord_join::holds_outer` and `chord_join::ring_side`.

**Not wired:**
- `chart_singularity::Sides::holds`: it answers `()`, so there is
  nothing to compare.
- `stands::ladder`: it climbs with `climb_evidence`.
- `side_reader::cast_aims` and `splitting::finish::whole_body_side`:
  their premise is false on fixtures. They stay on plain `climb`, with
  the check off rather than loosened. They are filed P1 as
  `side-at-infinity-aimed-rays-disagree` and
  `whole-body-side-reads-both-sides-at-a-pinch`.

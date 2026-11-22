---
id: a-least-band-reading-off-the-log-whose-escalation-refuses-has-no-door
kind: issue
title: A least-band reading taken off the log whose escalation is a refusal has no door: prove logs, choose ties
status: open
opened: 2026-11-22
---


Found by BOXFINISH's expression-question unit (the profile's direction
mint, `crates/profile/src/seg.rs` `axis_or`).

**The two doors.** `scripts/gates/least-band-on-a-door.sh` holds that a
margin read on its sign alone goes through one of two doors in
`crates/geom-core/src/k_stats.rs`: `prove` (a reading rests on it; it
is LOGGED, and an escalation is returned) or `choose` (a choice between
sound candidates; it is taken off the log under a ledger row, and an
escalation is folded into `Pick::Tie`).

**The reading neither fits.** The mint stores the axis `(0, ±1)` where
one component of a chord is proved zero as an expression, and signs it
by one reading of the other component (`segment_direction_axis`, audit
row F43). That reading is:

- **not a choice between sound candidates**: `(0, +1)` is wrong where
  the component is negative, so `choose`'s tie (an escalation read as
  "pick nothing") would store a direction nothing decided. ERROR-DESIGN
  E12's ratified text has the escalation propagate as the verb's
  refusal;
- **not loggable**: only a symbolic scalar takes it (a numeric scalar
  carries no expression, so the reading does not exist there by type),
  and the `f64` witness of a drive stores the quotient. A logged row
  would be one the witness's verdict vector cannot have, and every leaf
  with an axis hop would compare unequal to its witness
  (`k_stats::unlogged`'s own test for what belongs off the log).

So it is a proof's inequality read off the log: returned escalation,
no verdict row.

**What the unit does.** It keeps the form audit row F43 already had: a
reader handed the band (`seg::axis_sign(other, band)`), run inside
`k_stats::unlogged("F43", ..)` at `Band::least()`. The gate lists that
shape under what it does not refuse (a least band handed to a reader as
its band), and names it among its blind spots (a least band that
reaches the door through a parameter). `k_stats::check_unlogged(..,
Band::least(), ..)` is the same reading spelled at a door, and the gate
refuses it.

**The question.** Whether `k_stats` wants a third least-band door for
this kind (an unlogged proof: `unlogged(ledger_row, || classify(name,
margin, Band::least()))` with the escalation returned), with
`flagged_census` checking its row as it checks `choose`'s, so the gate
reads a name here instead of passing a parameter. One site today.

**Closed when** the reading goes through a door the gate reads by
name, or the gate's header says why the compound form is this kind's
home.

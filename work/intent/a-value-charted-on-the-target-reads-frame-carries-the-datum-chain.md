---
id: a-value-charted-on-the-target-reads-frame-carries-the-datum-chain
kind: issue
title: A value charted on the target read's moved frame would carry every datum chain verbatim and retire ValueOnAMovedWorld
status: open
opened: 2026-10-18
priority: P2
cost: M
parent: a-placement-pins-by-written-values-without-a-mate
---


D charts a value from the target BODY's own frame swung onto the mated direction (row 156, C-1; ruled (A) for D). For a world target that frame is the world's identity (`eval/wire.rs` `wire_place`'s `Target::World`, `mate/value.rs` `Chart::of`), not the world read's moved frame. So spec §5's migration splits a datum chain. The out-of-plane part (tilt, standoff) goes onto the world read. The in-plane slides and the spin go onto the values, as expressions over the chain's own variables. `tests/intent_s3_d_placements.rs::a_datums_in_plane_offset_rides_on_the_values` pins it: a world read slid (3, 4) with zero values lands at the origin.

**The lever.** Chart a value on the target read's moved frame (option (B) of the D lane's report). Then:

- every datum chain rides the read verbatim, with the constant `0` on every freedom, which is §5's first spelling;
- a tilted datum whose tilt reads a variable no longer needs an `atan2` expression to separate;
- inline at a moved landing composes the landing's motion into the read alone, and `InlineError::ValueOnAMovedWorld` retires (see `an-inline-at-a-moved-landing-reading-a-variable-refuses`).

**The cost.** To be consistent it charts EVERY mate from the target read's moved frame, since §5's relative placements carry moves on a body read too. That changes what D10 `[placement.values]` decides (Ev-approved text, PR 5249), so it takes a designer round and an `[ev]` PR. It also recomputes every C-era world-mate value. Nothing in the corpus needs it today.

Raised by the orchestrator's (A)/(B) ruling on PR D. `needs_ev` once someone picks it up.

- 2026-10-18: superseded by the placement pivot (fork-log row 162, PR 5312). `stand` becomes an unmated placement whose values are the datum chain or six zeros, and `Anchors`, the in-plane/out-of-plane split and the moved-landing refusal retire with it. This closes with that unit, whichever PR carries it.

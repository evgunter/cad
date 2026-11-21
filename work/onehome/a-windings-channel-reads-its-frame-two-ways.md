---
id: a-windings-channel-reads-its-frame-two-ways
kind: issue
title: winds.rs: a channel's certified clear reads the frame's polar factor while its f64 azimuth reads the stored entries
status: open
opened: 2026-11-21
priority: P2
cost: E
---


## What

`crates/geom-brep/src/pair_section/winds.rs:236` `Channel::clear` decides a sign on the frame's polar factor (`ssi::residual::certified_frame`, PR 7131). The same channel's `f64` azimuth (`winds.rs:218` `Channel::azimuth`) reads the stored entries, which are that rotation only to rounding. So one channel reads two frames. Decide whether the azimuth should read the polar factor's rounding, or whether `clear` should be stated against the azimuth's frame with the gap charged, and make the two one reading.

Filed from the second delta review of PR 7131.

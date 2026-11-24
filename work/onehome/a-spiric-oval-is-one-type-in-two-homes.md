---
id: a-spiric-oval-is-one-type-in-two-homes
kind: issue
title: A spiric oval's data is one type in two homes: ring_path::Oval and splitting::spiric_arc::Oval both hold the spiric's frame, major, minor and offset
status: open
opened: 2026-11-24
priority: P2
cost: E
---


Found by the review of PR 7663 (`onehome/mints-topo-rest`, S4). That PR
made `ring_path::Oval` (`crates/topo/src/ring_path.rs:757`) hold the
spiric carrier's frame, as `splitting::spiric_arc::Oval`
(`crates/topo/src/splitting/spiric_arc.rs:26`) already did. Each now
carries the same four data of a `geom::Curve3::Spiric`: its frame, major
and minor radii and offset. `ring_path`'s adds the arc's span, its plane's
frame and the loop its own oval lies on; `spiric_arc`'s is wrapped by a
`SpiricArc` with its span.

One home: the shared four as one type (say a `SpiricOval`, built only from
a `Curve3::Spiric` and handing it back), which both arcs hold beside their
own span and side data. Not fixed in 7663, which is about frames.

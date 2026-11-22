---
id: a-loop-edges-view-is-read-on-the-value-channel-so-an-interval-span-has-none
kind: issue
title: Curve3::clamped_net reads a span's ends on the value channel, so a seated or joined loop edge at Interval has no view
status: open
opened: 2026-11-17
priority: P3
cost: E
---


Found at PR 7052's review. Latent: the march lane is `f64` only, so no
loop edge is built at `Interval` today.

`geom::Curve3::clamped_net` (`crates/geom/src/curves.rs`, `fn
clamped_net`, line 1801 at the fix pass) hands
`PeriodicNurbs3::clamped_view` the span's ends as
`SpanLocate::value_channel`, which is NaN for an enclosure wider than a
point. A seated span `(at, at + T)` or a joined one whose ends were
computed at `Interval` is such an enclosure, so the view refuses
`PeriodicViewError::Span` and the edge door refuses
`CertifyError::PeriodicView`. A lifted `f64` body's point spans read
fine.

A fix decides what a view over an enclosed end is (the hull of the
views over its members, or a refusal that names the width).

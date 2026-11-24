---
id: a-trim-side-is-read-clear-of-the-fitted-image-not-of-the-boundary-it-stands-for
kind: issue
title: A trim side may be read clear of the fitted image by the band without charging the image's certified envelope
status: open
opened: 2026-11-24
priority: P2
cost: E
---


Found answering Ev's question on `[ev]` PR 7641, 2026-11-24, by one designer from module docs; **the function bodies were not read and nothing was run.**

`crates/topo/src/spline_trim.rs`'s module doc (lines 26–33) says a side is read only when the query is clear of the trim by more than the band, and charges rounding. `Trim::place` (`crates/topo/src/spline_region.rs:163`, pieces from `spline_trim.rs:1290` `image_pieces`) measures that clearance from the stored fitted image. The image is certified within its envelope of the boundary it stands for, so `In` may guarantee only (band − envelope) of clearance from the true boundary, which can be near zero. A second designer's reading that the trim cannot flip between `In` and `Out` rests on the certificate reading zero at the same band the trim's "clear" uses, and did not compare the two thresholds.

Asks: read `Trim::place` and the certificate's zone; if the envelope is not charged, charge it (the mass-property lane does: `crates/topo/src/props/quad_lane.rs` about 1783, `crates/geom-brep/src/props/quad/trimmed.rs:1651`), with a row where a query sits between (band − envelope) and band of the image. If it is charged, close this with the line that does it.

Beside it, already filed: `a-general-images-climb-reads-only-midpoints-between-feet` (a certificate refusal that is an accident of which fit was drawn, not of the model).

---
id: PROPS-1
kind: unit
title: linalg lost-correlation: mirror_across_plane and reject_from mention their operand once
status: review
opened: 2026-09-05
branch: props/1-linalg-lost-correlation
parent: certified-lane-non-real-contract-audit
pr: 1918
---


The lost-correlation members of the DL6 audit
(`certified-lane-non-real-contract-audit`), CERT-3 batch members 1, 3
and 4, taken together so the tree re-baselines once:

- **`frame::mirror_across_plane`** (member 1) — the translation is
  `n̂·(2·(n̂·q))` rather than `q − L·q`, so the anchor is mentioned once.
  At `Interval` the retired spelling charged `2·width(point)` to every
  component, including the ones where the plane's normal vanishes and
  the true translation is exactly zero; the shipped spelling attains the
  true width of the image of the anchor's enclosure.
- **`Vec3::reject_from`** (member 3) — `(onto × self) × onto / |onto|²`,
  so `self` is mentioned once. `onto` is still mentioned three times;
  the gain is on `self`'s width only. The doc's two rounding claims are
  re-derived and measured for the new spelling.
- **`Point2::lerp` / `Point3::lerp`** (member 4) — decided and LEFT. The
  one-difference form stays; each doc now carries a paragraph stating
  its `Interval` cost (`2·width(self)` at `t = 1`, exact at `t = 0`) so
  the trade is on the record at the site.

Evidence: `crates/geom-core/tests/props1_evidence.rs` — corpora as
literals, four `#[ignore]`d instruments, nine gating pins (single-mention
bound, vanishing-component exactness, parallel narrowing, containment of
the true value over sampled boxes in both spellings, and the re-derived
reconstruction claim).

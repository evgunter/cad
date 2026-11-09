---
id: the-line-router-charges-its-margins-with-world-origin-noise
kind: issue
title: The line router's two arms charge their screens and the in-plane Range with world-origin noise
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6642
---

Filed by `onecopy/transverse-range`, which deleted `conic_conic`'s mint of the same form (`transverse-range-charges-the-global-height-with-world-origin-noise`).

## What

`geom-brep` `curve_pair::pose` still mints two charges whose terms grow with the carriers' distance from the world origin, not with the magnitudes the readings sum:

- **Two lines** (`line_line`): `noise = rounding_charge(|o₁| + |o₂| + ends + 2·lever)`. It feeds `route`'s parallel margin (`offset − rate·half − noise`).
- **A line and a conic** (`line_conic`): `noise = rounding_charge(|o| + |c| + lever + speed)`, plus `screened`, which adds the line's ends. It feeds:
  - the router's screens;
  - the polish's `e = escalate + noise`;
  - the in-plane `Range::first_harmonic(.., noise)`, through `extreme_sign`'s spread and `band_arc`'s `e`.

`[simple.conditioned]` says an ill-conditioned form is re-posed, not charged. Each reading here is a local difference: an offset between two points, or a point's height above a plane. Posed about a local origin (a span's middle), each rounds as its few terms do. The world-origin terms are what reach the band's zero far from the origin: about 7.5e-13 at coordinates of about 180.

## Owed

- Re-pose the two arms' readings locally, then delete both mints.
- Pin a far-from-origin row, as `order_rows::the_needle_far_from_the_origin_reads_as_at_it` does for two conics: a line beside a conic and two lines, each translated by 1e3, read as at the origin in both orders.
- Lower `pose.rs`'s row in `scripts/gates/bare-rounding-sites.tsv` again.

## Resolved (PR 6642)

Both mints are deleted. `route`'s parallel margin, `line_conic`'s polish `e` and its in-plane `Range` read uncharged. `OnOneLine` keeps the charge for the spans' ends as evaluated, `geom_core::one_line`'s own home. With every `Range` uncharged, `Range.noise` is gone. Pinned by `order_rows::a_line_beside_a_circle_or_a_line_far_from_the_origin_reads_as_at_it`, which is red with the mints: at 1e3 a line tangent to a circle escalated, and two parallel lines 2e-11 apart refused. `pose.rs`'s gate count drops from 7 to 5.

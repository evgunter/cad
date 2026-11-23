---
id: a-counted-sections-winding-is-a-claim-not-a-reading
kind: issue
title: A counted section's winding is a claim the march only has to exceed, and an arm that does not read essentiality claims none
status: open
opened: 2026-10-29
priority: P2
cost: M
refs: [a-general-torus-pose-lists-no-components]
---


Filed by SECT's `sect-components` lane (PR 6011), naming a deviation
from `a-general-torus-pose-lists-no-components`'s `## Decided` ("winds
agree") and where it is scheduled: here.

## Measured

- **The cross-check is one-way.** `PairSection::matched`
  (`crates/geom-brep/src/pair_section.rs`) accepts a branch for a listed
  component where every winding the arm claims holds on the branch
  (claimed ⇒ marched), not where the two agree. Equality would desync
  the lily: `torus_arms::parallel_torus_arm` lists `[false, false]`
  without reading essentiality ("Essentiality is not read"), and its
  loops can wind.
- **A listed `false` is no claim**, and hides essential loops: in the
  dual review of PR 6011, 28 of 126 counted loops measured were
  essential on a carrier the arm listed `false`. The certificate reads
  the classification's `winds` for a counted section
  (`section_cert::of_pair_section`), so each such loop loses W2 and is
  cleared only by its witness (or refuses R-undec). Nothing reads it
  wrong; it costs W2 only.
- **Four vocabularies for one winding**: `geom_brep::Winding {
  about_axis, about_core }` (turns, `pair_section::winds`); `winds:
  [bool; 2]` on `SectionComponent` and `CarriedComponent`; the
  certificate's `Component { essential_f, essential_g }`; and
  `pcurve_cache`'s per-channel winding of a chart loop. And
  `topo/src/injective.rs`'s `lift`/`at` repeat the point lift that
  `winds.rs` now takes from `Real::from_f64`.

## What a fix has to supply

Either every counted arm reads essentiality (so `false` means null and
the check can be equality), or the classification's `winds` becomes a
three-valued claim (essential, null, unread) with the certificate and
the cross-check reading it so; and one winding type the four readers
share.

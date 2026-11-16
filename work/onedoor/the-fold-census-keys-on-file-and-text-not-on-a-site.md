---
id: the-fold-census-keys-on-file-and-text-not-on-a-site
kind: issue
title: face_normal's fold census keys on (file, negated text), so a same-name swap between sanctioned and unsanctioned sites passes
status: open
opened: 2026-11-14
priority: P3
---


## Finding

Found by the review of TESSLANE's sense-door unit (`tesslane/sense-door`),
2026-11-14, and demonstrated with a mutant.

`crates/topo/src/face_normal.rs`'s `no_source_file_folds_the_bit_by_hand`
compares a sorted list of `(file, negated text)` hits with its `sanctioned`
list. A sanctioned entry therefore covers any fold of that text anywhere in
that file, not the site it was sanctioned for:

- **The mutant.** Delete the test-module `area` fold in
  `crates/mesh/src/trimmed.rs` (sanctioned as a chart-frame scalar), and add
  a production 3-D normal fold also named `area` inside
  `TrimmedFace::outward`. The census stays green.
- `crates/topo/src/injective.rs` carries three identical `"1.0"` entries
  (`present_plane`, `present_plane_corner`, `present_plane_vertex`). Only a
  comment nothing checks tells them apart, so a fourth site can replace any
  one of them unseen.

Class: a guard that cannot see a same-name swap. No program's `paths` cover
`face_normal.rs`. This row sits beside the closed
`face-normal-guard-gained-a-pcurves-hit-at-6900-behind-mains-red`.

## What closing it takes

Key each sanctioned entry on something a swap changes. Options: the
enclosing function's name (the hit's nearest `fn` in the code view), or the
condition text beside the negated text, or both. A production/test split is
part of it, since a test-module entry must not cover production code.
Re-derive the list in the new key, and pin the mutant above as a scanner
row (`the_hand_fold_scanner_sees_both_shapes_and_nothing_else` is the model).

## 2026-11-14, evidence from TESSLANE's winding-rule unit

The census reads one shape, a value negated under the bit. A COMPARISON with
the bit is invisible to it, and the rule that unit gave a home
(`geom_brep::PolygonOrientation`: a face's chart polygon is counter-clockwise
exactly when its sense is set) is spelled that way as readily as by a negation.
So nothing goes red when a new site spells the rule by hand. Witnesses, found
by that unit's review with a grep for `== sense` / `!= sense`:

- `crates/geom-brep/src/chart_singularity.rs`, `holds`: `left != sense`
  (routed through the home by that unit);
- `crates/topo/src/injective.rs`: `left_is_above(…) == piece.boundary.sense`,
  twice (`work/embed/injective-spells-the-chart-winding-rule-by-hand.md`);
- `crates/step-import/src/normalize.rs`: `face.sense != ccw` and
  `rim[0].0 != sense`
  (`work/importgate/step-import-spells-the-polygon-orientation-rule-by-hand.md`);
- `demos/tour/src/uvdump.rs`: `(area > 0.0) == sense`, in a root the census
  does not walk;
- test code: `topo/src/boolean/sphere_region.rs` (`forward == sense`),
  `geom-brep/tests/chart_singularity_sweep.rs` (four).

The mesh sites that unit routed included two of this shape
(`(shoelace2(..) > 0.0) != face.sense`, `rising == sense`), neither of which the
census had ever listed.

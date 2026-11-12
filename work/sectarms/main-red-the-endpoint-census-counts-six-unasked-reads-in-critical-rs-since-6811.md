---
id: main-red-the-endpoint-census-counts-six-unasked-reads-in-critical-rs-since-6811
kind: issue
title: Main red (slow set): the certification endpoint census finds critical.rs at 37 reads, 6 new and unasked, since PR 6811; node_frame's width gates NODE_BOX unasked
status: closed
opened: 2026-11-11
priority: P1
cost: E
closed: 2026-11-11
branch: sectarms/critical-census-disposition
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`) on merging main `da005e4b`.

**What fails.** `geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`: the tree holds `crates/geom-brep/src/pair_section/spline_implicit/sweep/critical.rs` at 37 reads, 26 asking; the table says 31 / 26. The six came with PR 6811 (`sectarms/nurbs-implicit-arm-stage3b`, merge `ee63af5d`).

**The six reads, and what each needs** (a correctness decision per read, not a count bump):

- `grown`'s 4 (`half`, from `b`'s two brackets' ends). These look safe: a seed for the doubling, and every box it grows to is proved again by `krawczyk`, which asks `is_certified()`. A refused `b` only changes where the doubling starts.
- `node_frame`'s 2 (`width`, the widest coordinate of `S(b)` from `jet.point`'s ends). This one is a decision. `listed` places the node only where `width ≤ band.zero()` and otherwise refuses `Pinched { NODE_BOX }`, and `node_frame` never asks whether `jet.point` certifies. `Jets` holds a rational net, so `S(b)` divides by the weight sum. Either:
  - argue at the site that the enclosure always certifies (point knots, controls and weights from a finite f64 net with positive weights, a certified `b` from `range`); or
  - make `width` NaN unless all three coordinates are certified, which already reads `Pinched`, so the change can only refuse more.

PCERT's lane left this row alone (it is sectarms' ground and a soundness call). The census stays red on main, and on every PR that edits the census file, until it is dispositioned.

**Proposed fix** (PCERT's lane; sectarms owes the decision): in `node_frame`, read `width` as NaN unless `jet.point.x`, `.y` and `.z` each answer `is_certified()`. `listed` already refuses a NaN width as `Pinched { NODE_BOX }`, so the change can only refuse more, never place a node it would not place now. With it, `node_frame` asks by name and the census counts its 2 reads as asking. `grown`'s 4 then take the seed disposition above, and the row reads 37 / 28.

PCERT's PR 6831 lands with this row red: the red is inherited from main, where main's own tree `c43415ae` fails the census on `critical.rs` at 37 / 26 against the table's 31 / 26.

## Closed

Fixed on `sectarms/critical-census-disposition` by the proposed fix. `node_frame` reads `width` only where `jet.point.x`, `.y` and `.z` each answer `is_certified()`, and NaN otherwise, which `listed` refuses as `Pinched { NODE_BOX }` (`node_rows.rs`'s `a_node_not_placed_within_the_band_refuses` now pins a NaN width). `grown`'s 4 carry the seed disposition, stated at the site. The census row reads 37 / 28. The same run found a second miss on main, `ssi/implicit_chart.rs` at 2 reads, 0 asking, unlisted (`pass_key`'s rendering of the lift into the pass key, from `179e0cc798`). Its row is now listed, dispositioned as an identity rather than a bound, and the census passes.

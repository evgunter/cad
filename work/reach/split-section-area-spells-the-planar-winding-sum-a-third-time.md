---
id: split-section-area-spells-the-planar-winding-sum-a-third-time
kind: issue
title: the planar-region signed-area sum (Newell / shoelace + conic correction, 2A/P) has several spellings outside crate::loop_winding — split_section_area, chart_region, and geom-brep's loop_vector_area among them
status: open
opened: 2026-10-02
priority: P3
cost: M
refs: [unclaimed-half-edge-read-as-a-minus-half-in-reach]
---

Found by JOIN's `ring-run-winding-is-a-second-spelling-of-the-loop-winding-sum`
sweep (PR #3771, branch `join/ring-run-winding`), and widened by its
review. That PR gave the loop winding and the boolean join's open-run
winding one home in `crates/topo/src/loop_winding.rs`
(`Body::planar_run_winding_decided`, `Body::planar_loop_winding_decided`,
both over one private `winding_of_halves`). It unified nothing else;
this row is the class.

## The spellings of the region-area functional

**Same functional (signed `2A` of a carrier-bounded planar region,
some metered by `P`) — candidates for one home:**

- `crates/topo/src/splitting/join.rs`, `certify_section_area`
  (`split_section_area`). 3-D chord Newell about the first vertex plus
  a per-conic correction spelled from the carrier's centre,
  `[(c − O)×(B − A)]·n̂ + s_a·s_b·(axis·n̂)·Δt − [(A − O)×(B − O)]·n̂`,
  where `loop_winding::conic_segment_term` spells
  `axis · s_a·s_b·(Δ − sin Δ)`. Same area algebraically; not the same
  statement. The ellipse lever is `s_a·|Δt|` with `s_a` the STORED
  (signed) major, where the home reads the larger semi-axis magnitude.
  A spiric or NURBS edge is skipped (`continue`), i.e. wound by its
  chord — unreachable today (`splitting/classify.rs` refuses both
  kinds). `forward = edge.he_plus == he` is already listed in
  `unclaimed-half-edge-read-as-a-minus-half-in-reach`. A magnitude
  predicate: the repair reads the signed sum from the home and takes
  `abs`.
- `crates/geom-brep/src/props/loop_area.rs`, `loop_vector_area` — the
  vector-area boundary integral `(1/2)∮(p − ref)×dp`, an exact home of
  its own: conics spelled centre-based (`w×chord + axis·a·b·Δt`), and
  a NON-rational NURBS arm integrated exactly per knot span by fixed
  Gauss–Legendre (rational NURBS and spiric refuse). This is the
  strongest candidate for THE home, and it contradicts the old
  `loop_winding` doc "no closed form exists" for a spline (corrected in
  #3771: non-rational has one, here).
- `crates/topo/src/chart_region.rs`, `loop_measures` and the
  `chart_region_orientation` / `chart_region_area` /
  `chart_region_cyl_band_area` decisions — the 2-D `perp_dot` form of
  the same `2A/P` over a chart polygon. Its own comment argues the
  accumulator could be shared one way (embedding `Point2` as
  `(x, y, 0)`, `n̂ = ẑ`, reduces `a×b·n̂` to `perp_dot` bit for bit) but
  the polygon is already sampled, so no carrier set or claim is read.

**Normal / volume-only copies — the Newell cross-sum, not the region
area; listed so the next sweep does not re-find them:**

- `crates/geom-brep/src/newell.rs`, `newell_plane` — the plane's normal
  from a vertex ring; certifies planarity, decides no winding. Not this
  class.
- `crates/mesh/src/planar.rs` — the anchor-translated Newell normal of
  a planar face's walk (orients the walk CCW about it). Normal only.
- `crates/mesh/src/walk.rs`, `loop_area` — f64 vector area of a sampled
  cycle about its bbox centre; mesh-side, chord-only by construction
  (sampled points). Not carrier-bounded.
- `crates/mesh/src/curved.rs` — a UV shoelace (`area2`) read for a
  chart DIRECTION flip, not a winding verdict (its own comment says so).
- `crates/step-export/src/volume.rs` — `area2` cross-sum per face for a
  divergence-theorem volume; Line-only (any other carrier refuses
  `CurvedShellClassification`).
- `crates/step-import/src/recognize.rs` — Newell over a control net's
  boundary ring to recognize a plane; normal only.

## What the taker owes

Decide whether `loop_vector_area` (exact, spline-aware) or
`loop_winding`'s sum is the one home of the signed region area, and
move `split_section_area` and `loop_winding` (and, if the 2-D/3-D
argument in `chart_region` is settled, `loop_measures`) onto it. That
is a choice of home across crates (`geom-brep` vs `topo`), so it is
worth an orchestrator's ruling before code.

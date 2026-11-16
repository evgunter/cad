---
id: exact-zero-shortcuts-that-can-drop-a-dual-derivative
kind: issue
title: Thirteen is_exact_zero shortcuts read the value channel alone, and five of them can drop a Dual derivative
status: open
opened: 2026-11-15
priority: P2
cost: M
---

Raised by PR 6988's delta review (the decided least-turn mint, fork
270), as a style finding.

**What is there.** PR 6988 found that the least turn's identity shortcut
read `geom_core::is_exact_zero`, which reads the value channel alone. At
`Dual<f64>` (the ∂m/∂p door) a pair one direction in value but not in
derivative took the shortcut and dropped the turn's derivative. It added
`Real::is_exact_zero_in_every_channel` (`crates/geom-core/src/real.rs:420`;
`Dual` asks both channels, `crates/geom-core/src/dual.rs:574`) and moved
the mint, the mate chart and the sphere re-cut onto it. These shortcuts
still read `is_exact_zero`, and each takes a cheaper spelling where its
input reads zero:

- `crates/geom/src/curves.rs:955` (`Ray::start_point`), `:971`
  (`Ray::from_start`) and `:983` (`Ray::past_start`): the line's origin,
  `reach` and `at` in place of the start added or subtracted.
- `crates/profile/src/lib.rs:310` (`lower_to`, test-support only) and
  `:330` (`bulge_leg`): a zero bulge lowers to a line, with no arc.
- `crates/geom/src/cover.rs:1126` (`Tube::new`): a zero major radius is
  a sphere's hub, the centre, rather than the core circle.
- `crates/geom-brep/src/pcurve_cache.rs:8655` (`arc_rim_normal_weight`):
  equal end weights read `w₁/w₀` in place of `w₁/√(w₀·w₂)`;
  `:8676` (`one_value`) and `:9184` (the arc rim's end weights against
  its chart column's): equality checks.
- `crates/geom-brep/src/description.rs:466` and `:471`, and
  `crates/topo/src/boolean/spline_known.rs:118` and `:120`: which
  domain end a fixed parameter sits on.

**Which can see a `Dual` derivative they drop**, as far as reading each
site shows (no site was run at `Dual` for this row):

- **Can.** The ray's three (a start that is a function of a parameter
  passing through zero loses its derivative from the point it names),
  `bulge_leg` (a bulge parameter passing through zero lowers to a line,
  and the arc's first-order bulge is lost), and `Tube::new` (a major
  radius passing through zero). `lower_to` is the same rule behind a
  test-only cfg.
- **Cannot drop a derivative.** The three weight sites: their weights
  are held (`pcurve_cache.rs:8671`: "a held weight is a point at
  every scalar"), so the derivative is zero in every channel already. The
  four domain-end sites return a label (`ChartEnd`, `ChartSide`) and
  build no value, so nothing is dropped. They can only name an end
  whose derivative moves off it, which is a question about the label,
  not a lost derivative.

**What would fix it.** Each site in the first group reads
`is_exact_zero_in_every_channel` (`Real`'s method), so a value-zero
input with a live derivative takes the general spelling. Each needs a
`Dual` pin, as PR 6988's three do (the value-channel mutation turns
them red).


---
id: section-carriers-read-off-acos-near-a-degenerate-pose-carry-unread-rounding
kind: issue
title: a section arm's carrier read off acos or atan2 near a degenerate pose carries rounding its margin does not read
status: open
opened: 2026-10-11
priority: P3
cost: M
refs: [pn-apex-line-reach-is-read-at-the-apex-not-over-the-consumed-span]
---

## What

`plane_cone_section` builds its served generators at azimuth `φ ± γ`,
`γ = acos(−c·cos α/(s·sin α))` and `φ = atan2(n·v, n·u)`. Near tangency
(`σ = √(D·M)/sin α` small), `acos`'s slope `s/σ` amplifies the rounding
in its argument, and `φ` carries its components' rounding over `s`.
Over a span reaching `R` from the apex the lines then move
`sin α·R·Δ` off the section. The span fuzz in
`crates/geom-brep/tests/pn_apex_pair_over_a_span.rs` found three such
pairs served `1.1–3.4·ε` off at ε `1e-9` and `1e-12`. They are pinned
there as `a_pair_near_tangency_reads_its_azimuths_rounding`,
`a_plane_square_to_a_flat_cones_axis_reads_its_traces_rounding` and
`a_needles_plane_near_tangency_reads_its_normals_rounding`.
`pair_bound` (`crates/geom-brep/src/intersect.rs`) now reads that
rounding beside the pair's bound, at the conditioning it derives.

## The class, not swept

Any section arm that builds a carrier off an `acos`, `asin` or `atan2`
of a quantity that goes degenerate at a pose its margins admit carries
rounding no margin reads.

- **First candidate: `ApexTangentLine`, in the same arm.** At tangency
  `γ`'s argument is `±1` by construction, where `acos`'s slope is
  unbounded. Its `|δ|/M` reach reads none of that. The split lane mints
  a chord on that ruling (`chord_join::section_case`'s `Tangent`).
- **Then:** every other carrier constructor in `intersect.rs` and
  `locus.rs` that reads an angle off a ratio of two computed lengths.

Grep for `acos(`, `asin(` and `atan2(` there, and, for each hit, find
the margin that admits a pose near the function's singular argument.
The fix shape is `pair_bound`'s: bound the rounding at its derived
conditioning, levered by the consumed reach, or construct the carrier
without the singular function.


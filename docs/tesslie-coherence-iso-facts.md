# Chart-coherence iso facts

Facts for the exact-iso question
(`work/tesslie/chart-coherence-reads-a-non-iso-circle-as-an-iso-side.md`):
for every coherence witness in the tree and every sphere pose that
produces a false finding, what each boundary carrier is, where it lies
relative to its face's surface, how far it strays from the iso curve the
examination reads it as, and what the examination reports on it.

## How each column is measured

- Every row is one boundary traversal of a chart-bearing face. Rows that
  agree in every column but the face are merged, and `traversals` counts
  them.
- **Distance from surface**: the maximum, over 65 evenly spaced
  parameters of the edge's own span, of the distance from the carrier
  point to the surface point the chart's inversion names for it
  (`Chart::uv_of`, then `Surface::eval`). The vertices' column measures
  the edge's two end vertices the same way.
- **Door reads it as**: `topo::chart_iso::classify_kind`'s answer.
  `none (run boundary)` is a conic or spline carrier.
- **Deviation from that iso curve**: the maximum over the same samples
  of `radial · separation(u, u_raw)` for a meridian, or
  `v_lever · |v − v_raw|` for a rim (wrapped where v wraps), against the
  reference the examination itself reads (`u_raw`/`v_raw`).
- **Deviation from the other family**: the same quantity for the other
  family, referenced to the carrier's mid-parameter point.
- **Door reports**: `topo::examine_chart_coherence`'s findings whose
  edge is this traversal's, in metres. A continuation finding is
  reported on the CONTINUING edge, measured against the edge that opens
  its side, so its row can be one whose own deviation is zero.
- Every body is built at ε = 1e-9 (the default row). The Euler-door
  witnesses certify at that ε. The imported fixtures carry the file's
  own adoption tolerance ε_in, listed per fixture.
- The instruments, all `#[ignore]`d printing rows that this table is
  copied from:
  - `cargo nextest run -p topo --test all tesslie_iso_facts --run-ignored only --no-capture`
  - `cargo nextest run -p mesh --test all tesslie_iso_facts --run-ignored only --no-capture`
  - `cargo nextest run -p step-import --test all tesslie_iso_facts --run-ignored only --no-capture`

## Witnesses in `topo/tests/mesh8_coherence.rs` (Euler doors, ε = 1e-9)

`oblique_cap(tilt, offset)` is a unit sphere (axis z) cut along one
circle. The circle's plane has its normal at `tilt` from z and sits
`offset` from the centre, and two arcs of it bound two sphere faces.

| body | carrier | max dist. of carrier from surface (m) | max dist. of its vertices from surface (m) | door reads it as | deviation from that iso curve (m) | deviation from the other family (m) | door reports on this edge (m) | traversals |
|---|---|---|---|---|---|---|---|---|
| pole_crossing_half_cap (issue 1571) | Circle | 5.589e-16 | 1.061e-16 | Meridian | 2.721e0 | rim 1.047e0 | closure 2.721e0 | 2 |
| pole_crossing_half_cap (issue 1571) | Circle | 1.943e-16 | 1.061e-16 | Rim | 1.110e-16 | meridian 1.360e0 | — | 2 |
| pole_crossing_half_cap, arc split (issue 723's shape) | Circle | 5.589e-16 | 2.220e-16 | Meridian | 1.571e0 | rim 7.854e-1 | closure 1.571e0 | 1 |
| pole_crossing_half_cap, arc split (issue 723's shape) | Circle | 4.965e-16 | 2.220e-16 | Meridian | 0.000e0 | rim 2.618e-1 | mer-cont 1.571e0 | 1 |
| pole_crossing_half_cap, arc split (issue 723's shape) | Circle | 1.943e-16 | 1.061e-16 | Rim | 1.110e-16 | meridian 1.360e0 | — | 2 |
| pole_crossing_half_cap, arc split (issue 723's shape) | Circle | 5.589e-16 | 2.220e-16 | Meridian | 1.571e0 | rim 7.854e-1 | mer-cont 1.571e0, closure 1.571e0 | 1 |
| pole_crossing_half_cap, arc split (issue 723's shape) | Circle | 4.965e-16 | 2.220e-16 | Meridian | 0.000e0 | rim 2.618e-1 | — | 1 |
| chord_wobble(1024 ε) | Line | 1.049e-11 | 4.235e-22 | Meridian | 1.024e-6 | rim 0.000e0 | closure 1.024e-6, closure 1.024e-6 | 2 |
| chord_wobble(1024 ε) | Circle | 4.235e-22 | 4.235e-22 | Rim | 0.000e0 | meridian 1.024e-6 | — | 2 |
| two_circle_rim_cap(c for 1024 ε) | Circle | 1.629e-3 | 1.061e-16 | Rim | 9.432e-4 | meridian 1.360e0 | — | 2 |
| two_circle_rim_cap(c for 1024 ε) | Circle | 1.943e-16 | 1.061e-16 | Rim | 1.110e-16 | meridian 1.360e0 | rim-cont 1.024e-6 | 2 |
| split_meridian_wobble(1024 ε) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 5.000e-1 | — | 2 |
| split_meridian_wobble(1024 ε) | Circle | 6.939e-18 | 6.939e-18 | Rim | 0.000e0 | meridian 2.250e-2 | — | 4 |
| split_meridian_wobble(1024 ε) | Line | 6.939e-18 | 6.939e-18 | Meridian | 0.000e0 | rim 2.500e-1 | — | 1 |
| split_meridian_wobble(1024 ε) | Line | 1.049e-11 | 6.939e-18 | Meridian | 1.024e-6 | rim 2.500e-1 | mer-cont 1.024e-6, closure 1.024e-6, closure 1.024e-6 | 1 |
| split_meridian_wobble(1024 ε) | Line | 1.049e-11 | 6.939e-18 | Meridian | 1.024e-6 | rim 2.500e-1 | closure 1.024e-6, closure 1.024e-6 | 1 |
| split_meridian_wobble(1024 ε) | Line | 6.939e-18 | 6.939e-18 | Meridian | 0.000e0 | rim 2.500e-1 | mer-cont 1.024e-6 | 1 |
| tilted_lune(R 1, θ 1, poles 0.5 ε off) | Circle | 4.710e-16 | 3.189e-16 | Meridian | 2.154e-16 | rim 1.571e0 | — | 2 |
| tilted_lune(R 1, θ 1, poles 0.5 ε off) | Circle | 5.034e-16 | 3.189e-16 | Meridian | 3.778e-17 | rim 1.571e0 | — | 2 |
| oblique_cap(tilt 0°, offset 0.3) | Circle | 5.118e-16 | 5.841e-17 | Rim | 5.551e-17 | meridian 1.498e0 | — | 2 |
| oblique_cap(tilt 0°, offset 0.3) | Circle | 5.551e-16 | 5.841e-17 | Rim | 5.551e-17 | meridian 1.498e0 | — | 2 |
| oblique_cap(tilt 0°, offset 0) | Circle | 5.118e-16 | 6.123e-17 | Rim | 0.000e0 | meridian 1.571e0 | — | 2 |
| oblique_cap(tilt 0°, offset 0) | Circle | 5.551e-16 | 6.123e-17 | Rim | 0.000e0 | meridian 1.571e0 | — | 2 |
| oblique_cap(tilt 30°, offset 0.3) | Circle | 4.611e-16 | 2.559e-16 | Rim | 4.848e-1 | meridian 1.366e0 | — | 2 |
| oblique_cap(tilt 30°, offset 0.3) | Circle | 4.879e-16 | 2.559e-16 | Rim | 5.624e-1 | meridian 1.667e0 | — | 2 |
| oblique_cap(tilt 30°, offset 0) | Circle | 5.553e-16 | 7.764e-17 | Rim | 5.236e-1 | meridian 1.571e0 | — | 2 |
| oblique_cap(tilt 30°, offset 0) | Circle | 4.710e-16 | 7.764e-17 | Rim | 5.236e-1 | meridian 1.571e0 | — | 2 |
| oblique_cap(tilt 59°, offset 0.3) | Circle | 4.041e-16 | 2.989e-16 | Rim | 8.856e-1 | meridian 1.292e0 | — | 2 |
| oblique_cap(tilt 59°, offset 0.3) | Circle | 5.034e-16 | 2.989e-16 | Rim | 1.174e0 | meridian 1.812e0 | — | 2 |
| oblique_cap(tilt 59°, offset 0) | Circle | 5.004e-16 | 1.206e-16 | Rim | 1.030e0 | meridian 1.571e0 | — | 2 |
| oblique_cap(tilt 59°, offset 0) | Circle | 5.940e-16 | 1.206e-16 | Rim | 1.030e0 | meridian 1.571e0 | — | 2 |
| oblique_cap(tilt 61°, offset 0.3) | Circle | 4.965e-16 | 2.483e-16 | Meridian | 1.289e0 | rim 9.059e-1 | closure 1.289e0, closure 1.289e0 | 2 |
| oblique_cap(tilt 61°, offset 0.3) | Circle | 4.309e-16 | 2.483e-16 | Meridian | 1.820e0 | rim 1.223e0 | mer-cont 3.108e0, closure 1.820e0, closure 1.820e0 | 2 |
| oblique_cap(tilt 61°, offset 0) | Circle | 5.321e-16 | 1.243e-16 | Meridian | 1.571e0 | rim 1.065e0 | closure 1.571e0, closure 1.571e0 | 2 |
| oblique_cap(tilt 61°, offset 0) | Circle | 5.389e-16 | 1.243e-16 | Meridian | 1.571e0 | rim 1.065e0 | mer-cont 3.142e0, closure 1.571e0, closure 1.571e0 | 2 |
| oblique_cap(tilt 75°, offset 0.3) | Circle | 5.468e-16 | 2.001e-16 | Meridian | 1.272e0 | rim 1.082e0 | closure 1.272e0, closure 1.272e0 | 2 |
| oblique_cap(tilt 75°, offset 0.3) | Circle | 5.850e-16 | 2.001e-16 | Meridian | 1.280e0 | rim 1.450e0 | closure 1.272e0, closure 1.272e0 | 2 |
| oblique_cap(tilt 75°, offset 0) | Circle | 3.747e-16 | 1.520e-16 | Meridian | 1.571e0 | rim 1.309e0 | closure 1.571e0, closure 1.571e0 | 2 |
| oblique_cap(tilt 75°, offset 0) | Circle | 5.866e-16 | 1.520e-16 | Meridian | 1.571e0 | rim 1.309e0 | mer-cont 3.142e0, closure 1.571e0, closure 1.571e0 | 2 |
| oblique_cap(tilt 90°, offset 0.3) | Circle | 3.511e-16 | 1.110e-16 | Meridian | 1.266e0 | rim 1.266e0 | closure 1.266e0, closure 1.266e0 | 2 |
| oblique_cap(tilt 90°, offset 0.3) | Circle | 4.611e-16 | 1.110e-16 | Meridian | 1.266e0 | rim 1.266e0 | closure 1.266e0, closure 1.266e0 | 2 |
| oblique_cap(tilt 90°, offset 0) | Circle | 2.151e-16 | 1.837e-16 | Meridian | 2.820e0 | rim 1.571e0 | closure 3.218e-1, closure 2.820e0 | 1 |
| oblique_cap(tilt 90°, offset 0) | Circle | 5.472e-16 | 1.837e-16 | Meridian | 2.356e0 | rim 1.571e0 | mer-cont 2.678e0, closure 7.854e-1, closure 2.356e0 | 1 |
| oblique_cap(tilt 90°, offset 0) | Circle | 2.151e-16 | 1.837e-16 | Meridian | 2.820e0 | rim 1.571e0 | closure 2.820e0, closure 3.218e-1 | 1 |
| oblique_cap(tilt 90°, offset 0) | Circle | 5.472e-16 | 1.837e-16 | Meridian | 2.356e0 | rim 1.571e0 | mer-cont 2.678e0, closure 2.356e0, closure 7.854e-1 | 1 |

## Witnesses in `mesh/tests` (ε = 1e-9)

- `tilted_meridian_sliver` (mesh8r1) is built with its tilt set for the
  named closure gap.
- The tilted halves are an extruded unit cylinder split by a plane
  tilted 0.3 rad.
- The two pins are in `mesh8r2_probes`.
- `wobbled_sliver` is the nist_ftc_09 shape (a cylinder sliver whose
  meridian side is a line with its endpoints stated non-co-azimuthally,
  radius 0.05 m). At the traced 21.4 pm it **does not build**: the
  chord's span would be below what certifies. The file itself,
  `nist_ftc_09_asme1_rd.stp`, refuses at import at every ε and is in
  `wild.rs`'s refusal corpus.
- keyway, oblique_lens and slit are the r2r corpus.

| body | carrier | max dist. of carrier from surface (m) | max dist. of its vertices from surface (m) | door reads it as | deviation from that iso curve (m) | deviation from the other family (m) | door reports on this edge (m) | traversals |
|---|---|---|---|---|---|---|---|---|
| tilted_meridian_sliver(4 ε) (mesh8r1) | Circle | 1.180e-1 | 0.000e0 | Meridian | 4.022e-9 | rim 5.000e-1 | closure 4.022e-9, closure 4.022e-9 | 2 |
| tilted_meridian_sliver(4 ε) (mesh8r1) | Circle | 1.110e-16 | 0.000e0 | Rim | 0.000e0 | meridian 4.000e-1 | — | 2 |
| tilted_meridian_sliver(4 ε) (mesh8r1) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 5.000e-1 | — | 2 |
| tilted_meridian_sliver(4 ε) (mesh8r1) | Circle | 1.241e-16 | 0.000e0 | Rim | 0.000e0 | meridian 4.000e-1 | — | 2 |
| tilted_meridian_sliver(1024 ε) (mesh8r1) | Circle | 1.180e-1 | 0.000e0 | Meridian | 1.030e-6 | rim 5.000e-1 | closure 1.030e-6, closure 1.030e-6 | 2 |
| tilted_meridian_sliver(1024 ε) (mesh8r1) | Circle | 1.110e-16 | 0.000e0 | Rim | 0.000e0 | meridian 4.000e-1 | — | 2 |
| tilted_meridian_sliver(1024 ε) (mesh8r1) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 5.000e-1 | — | 2 |
| tilted_meridian_sliver(1024 ε) (mesh8r1) | Circle | 1.241e-16 | 0.000e0 | Rim | 0.000e0 | meridian 4.000e-1 | — | 2 |
| tilted half above (mesh8r2) | Ellipse | 4.475e-16 | 1.225e-16 | none (run boundary) | — | meridian 1.571e0, rim 3.093e-1 | — | 1 |
| tilted half above (mesh8r2) | Line | 1.225e-16 | 1.225e-16 | Meridian | 0.000e0 | rim 7.797e-1 | — | 2 |
| tilted half above (mesh8r2) | Circle | 4.475e-16 | 1.225e-16 | Rim | 0.000e0 | meridian 1.571e0 | — | 1 |
| tilted half above (mesh8r2) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 4.703e-1 | — | 2 |
| tilted half above (mesh8r2) | Ellipse | 5.979e-16 | 1.225e-16 | none (run boundary) | — | meridian 1.571e0, rim 3.093e-1 | — | 1 |
| tilted half above (mesh8r2) | Circle | 5.551e-16 | 1.225e-16 | Rim | 0.000e0 | meridian 1.571e0 | — | 1 |
| tilted half below (mesh8r2) | Ellipse | 6.956e-16 | 1.225e-16 | none (run boundary) | — | meridian 1.571e0, rim 3.093e-1 | — | 1 |
| tilted half below (mesh8r2) | Line | 1.225e-16 | 1.225e-16 | Meridian | 0.000e0 | rim 4.703e-1 | — | 2 |
| tilted half below (mesh8r2) | Circle | 5.551e-16 | 1.225e-16 | Rim | 0.000e0 | meridian 1.571e0 | — | 1 |
| tilted half below (mesh8r2) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 7.797e-1 | — | 2 |
| tilted half below (mesh8r2) | Ellipse | 3.511e-16 | 1.225e-16 | none (run boundary) | — | meridian 1.571e0, rim 3.093e-1 | — | 1 |
| tilted half below (mesh8r2) | Circle | 4.475e-16 | 1.225e-16 | Rim | 0.000e0 | meridian 1.571e0 | — | 1 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Line | 5.242e-13 | 1.225e-16 | Meridian | 1.024e-6 | rim 0.000e0 | mer-cont 1.024e-6, closure 1.024e-6, closure 1.024e-6 | 1 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Circle | 2.220e-16 | 0.000e0 | Rim | 0.000e0 | meridian 1.571e0 | — | 1 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 4.703e-1 | — | 2 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Ellipse | 4.475e-16 | 1.225e-16 | none (run boundary) | — | meridian 1.571e0, rim 3.093e-1 | — | 1 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Line | 1.225e-16 | 1.225e-16 | Meridian | 0.000e0 | rim 7.797e-1 | — | 2 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Line | 5.242e-13 | 1.225e-16 | Meridian | 1.024e-6 | rim 0.000e0 | closure 1.024e-6, closure 1.024e-6 | 1 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Circle | 0.000e0 | 1.225e-16 | Rim | 0.000e0 | meridian 1.024e-6 | — | 1 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Ellipse | 5.979e-16 | 1.225e-16 | none (run boundary) | — | meridian 1.571e0, rim 3.093e-1 | — | 1 |
| ellipse_trimmed_with_a_rim_chord(1024 ε) (pin) | Circle | 5.551e-16 | 1.225e-16 | Rim | 0.000e0 | meridian 1.571e0 | — | 1 |
| ellipse_trimmed_with_a_conic_chord (pin) | Line | 2.097e-12 | 1.225e-16 | Meridian | 2.048e-6 | rim 1.298e-12 | closure 2.048e-6, closure 2.048e-6 | 2 |
| ellipse_trimmed_with_a_conic_chord (pin) | Line | 1.225e-16 | 1.225e-16 | Meridian | 0.000e0 | rim 7.797e-1 | mer-cont 2.048e-6 | 1 |
| ellipse_trimmed_with_a_conic_chord (pin) | Circle | 4.475e-16 | 1.225e-16 | Rim | 0.000e0 | meridian 1.571e0 | — | 1 |
| ellipse_trimmed_with_a_conic_chord (pin) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 4.703e-1 | — | 2 |
| ellipse_trimmed_with_a_conic_chord (pin) | Ellipse | 4.003e-16 | 6.123e-17 | none (run boundary) | — | meridian 1.571e0, rim 3.093e-1 | — | 1 |
| ellipse_trimmed_with_a_conic_chord (pin) | Ellipse | 3.599e-16 | 1.225e-16 | none (run boundary) | — | meridian 2.048e-6, rim 1.946e-12 | — | 1 |
| ellipse_trimmed_with_a_conic_chord (pin) | Ellipse | 5.979e-16 | 1.225e-16 | none (run boundary) | — | meridian 1.571e0, rim 3.093e-1 | — | 1 |
| ellipse_trimmed_with_a_conic_chord (pin) | Circle | 5.551e-16 | 1.225e-16 | Rim | 0.000e0 | meridian 1.571e0 | — | 1 |
| ellipse_trimmed_with_a_conic_chord (pin) | Line | 1.225e-16 | 1.225e-16 | Meridian | 0.000e0 | rim 7.797e-1 | — | 1 |
| keyway (r2r) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 5.000e-1 | — | 4 |
| keyway (r2r) | Circle | 1.241e-16 | 0.000e0 | Rim | 0.000e0 | meridian 7.500e-1 | — | 2 |
| keyway (r2r) | Circle | 0.000e0 | 0.000e0 | Rim | 0.000e0 | meridian 2.500e-1 | — | 2 |
| keyway (r2r) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 2.000e-1 | — | 4 |
| keyway (r2r) | Circle | 1.110e-16 | 0.000e0 | Rim | 0.000e0 | meridian 2.500e-1 | — | 2 |
| keyway (r2r) | Circle | 1.241e-16 | 0.000e0 | Rim | 0.000e0 | meridian 2.500e-1 | — | 2 |
| oblique_lens (r2r) | Circle | 4.079e-16 | 2.040e-16 | Rim | 6.507e-1 | meridian 1.463e0 | — | 2 |
| oblique_lens (r2r) | Circle | 4.495e-16 | 2.040e-16 | Rim | 6.507e-1 | meridian 1.463e0 | — | 2 |
| slit (r2r) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 5.000e-1 | — | 4 |
| slit (r2r) | Circle | 1.241e-16 | 0.000e0 | Rim | 0.000e0 | meridian 3.750e-1 | — | 2 |
| slit (r2r) | Line | 0.000e0 | 0.000e0 | Meridian | 0.000e0 | rim 2.500e-1 | — | 2 |
| slit (r2r) | Circle | 0.000e0 | 0.000e0 | Rim | 0.000e0 | meridian 3.750e-1 | — | 2 |
| slit (r2r) | Circle | 1.241e-16 | 0.000e0 | Rim | 0.000e0 | meridian 7.500e-1 | — | 2 |
| wobbled_sliver(1024 ε) (nist_ftc_09 class) | Line | 1.049e-11 | 4.235e-22 | Meridian | 1.024e-6 | rim 0.000e0 | closure 1.024e-6, closure 1.024e-6 | 2 |
| wobbled_sliver(1024 ε) (nist_ftc_09 class) | Circle | 4.235e-22 | 4.235e-22 | Rim | 0.000e0 | meridian 1.024e-6 | — | 2 |

## Import corpora (own, FreeCAD, wild; ε = 1e-9)

One line per fixture. A circle counts as "read > ε off its iso curve"
when the deviation from the curve the examination reads it as exceeds
ε. No fixture holds such a circle, so there are no detail rows.

| fixture | ε_in (adoption) | traversals on chart faces | circles | circles read > ε off their iso curve | max such deviation (m) | max carrier dist. from surface (m) | findings |
| own/cube | 1e-9 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| own/die | 1e-9 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| own/kiss_assembly | 1e-9 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| own/cut_cylinder | 1e-9 | 8 | 2 | 0 | 0.000e0 | 5.467e-16 | 0 |
| own/boss_union | 1e-9 | 12 | 6 | 0 | 0.000e0 | 4.441e-16 | 0 |
| own/notched | 1e-9 | 8 | 4 | 0 | 0.000e0 | 3.331e-16 | 0 |
| own/washer | 1e-9 | 8 | 4 | 0 | 0.000e0 | 3.331e-16 | 0 |
| own/ball | 1e-9 | 4 | 4 | 0 | 0.000e0 | 3.216e-16 | 0 |
| own/cone | 1e-9 | 6 | 2 | 0 | 0.000e0 | 5.797e-16 | 0 |
| own/donut | 1e-9 | 8 | 8 | 0 | 0.000e0 | 3.331e-16 | 0 |
| own/lily_lantern | 1e-9 | 16 | 12 | 0 | 0.000e0 | 2.618e-16 | 0 |
| own/filleted_die | 1e-9 | 72 | 48 | 0 | 0.000e0 | 1.119e-16 | 0 |
| own/die_pips | 1e-9 | 126 | 126 | 0 | 0.000e0 | 1.163e-16 | 0 |
| own/composed_die | 1e-9 | 303 | 279 | 0 | 0.000e0 | 1.570e-16 | 0 |
| own/loft_prism | 1e-9 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| own/nonuniform_loft | 1e-9 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| own/swept_elbow | 1e-9 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| own/voided | 1e-9 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| freecad/box | 1e-10 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| freecad/cylinder | 1e-10 | 4 | 2 | 0 | 0.000e0 | 1.212e-19 | 0 |
| freecad/cone_trunc | 1e-10 | 4 | 2 | 0 | 0.000e0 | 6.345e-19 | 0 |
| freecad/cone_apex | 1e-10 | 6 | 2 | 0 | 0.000e0 | 6.793e-19 | 0 |
| freecad/sphere | 1e-10 | 4 | 4 | 0 | 0.000e0 | 3.216e-19 | 0 |
| freecad/torus | 1e-10 | 4 | 4 | 0 | 0.000e0 | 5.421e-19 | 0 |
| freecad/box_hole | 1e-10 | 4 | 2 | 0 | 0.000e0 | 2.424e-19 | 0 |
| freecad/fuse_boxes | 1e-10 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| freecad/box_fillet_edge | 1e-10 | 4 | 2 | 0 | 0.000e0 | 3.833e-20 | 0 |
| freecad/box_fillet_corner | 1e-10 | 15 | 9 | 0 | 0.000e0 | 1.355e-19 | 0 |
| freecad/compound_two | 1e-10 | 4 | 4 | 0 | 0.000e0 | 1.198e-19 | 0 |
| freecad/box_importexport | 1e-10 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| freecad/twobody_importexport | 1e-10 | 4 | 4 | 0 | 0.000e0 | 1.198e-19 | 0 |
| wild/adafruit/328_2500mAh_battery.step | 1e-5 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| wild/adafruit/1982_MPR121.step | 1e-5 | 16 | 8 | 0 | 0.000e0 | 1.735e-18 | 0 |
| wild/adafruit/805_slide_switch.step | 1e-5 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| wild/adafruit/931_OLED_128x32_I2C.step | 1e-5 | 32 | 16 | 0 | 0.000e0 | 3.476e-18 | 0 |
| wild/adafruit/64_Halfsize_Breadboard.step | 1e-5 | 0 | 0 | 0 | 0.000e0 | 0.000e0 | 0 |
| wild/stepcode/sg1-c5-214.stp | 5e-6 | 48 | 24 | 0 | 0.000e0 | 4.076e-17 | 0 |
| wild/nist/nist_ftc_11_asme1_rb.stp | 1e-5 | 20 | 16 | 0 | 0.000e0 | 1.963e-17 | 0 |
| wild/occ-oss/cq_red_cube_blue_cylinder.step | 1e-6 | 4 | 2 | 0 | 0.000e0 | 8.674e-19 | 0 |

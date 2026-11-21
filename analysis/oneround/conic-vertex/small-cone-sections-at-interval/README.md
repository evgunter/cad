# Cone sections with semi-axes at most 1 km, at `Interval`: stored at the vertex against stored at the centre

Measured 2026-11-21 on `oneround/anchored-conic` at `8d5978300` (step A)
with a temporary switch in `plane_cone_section`'s mint. Facts only.

- **(i) stored at the vertex**: as step A does,
  `Curve3::ellipse_anchored(vertex, n, toward, (major, minor))`.
- **(ii) stored at the centre**:
  `Curve3::ellipse(vertex − toward·major, n, major, minor, toward)`, a
  centre-form conic, the centre computed at the mint from the same vertex
  and semi-axis.

## The suite's fixtures

88 rows reach the mint (65 per-PR rows at the default ε, 23 slow rows
that name a cone). `fixtures.txt` lists each with how often it mints, the
range of its major semi-axis, its least `K`, and its widths at
`Interval`.

- 78 of the 88 mint only sections with a major semi-axis at most 1e3 m.
  The other ten reach 3e3 to 5.7e6 m (the first ten lines of
  `fixtures.txt`).
- Two of the 88 mint at `Interval`, both `a_ring_on_a_cone_face` rows
  (the slow set): `…at_the_certified_scalar` and `…through_point_maps`,
  major semi-axes 0.925 to 1.13 m, `K` at least 0.16. No other row here
  mints a cone section at an enclosure scalar.

Run of all 88 at ε = 1e-9, debug build:

| storage | result |
|---|---|
| (i) vertex | 88 of 88 |
| (ii) centre | 87 of 88: `carrier_resolution::tests::a_far_centred_section_stands_at_the_scale_its_form_is_stored_at`, whose sections reach 4.6e6 m, fails |

So at the default ε no row whose sections are at most 1e3 m changes
verdict between the two storages.

`a_ring_on_a_cone_face`, volume enclosure width in m³ (bar 1e-10):

| row, op | centre-stored base | (ii) stored at `vertex − u·a`, measured before the phase | (i) step A |
|---|---|---|---|
| certified scalar, cone ∪ box | 4.86e-11 | 4.80e-11 | 3.22e-11 |
| certified scalar, box ∪ cone | 7.78e-11 | 7.75e-11 | 5.39e-11 |
| certified scalar, cone ∖ box | 4.83e-12 | 4.76e-12 | 2.68e-12 |
| point maps, cone ∪ box | 2.02e-11 | 2.05e-11 | 1.15e-11 |
| point maps, box ∪ cone | 3.00e-11 | 3.08e-11 | 1.64e-11 |
| point maps, cone ∖ box | 2.09e-12 | 2.11e-12 | 9.31e-13 |

The (ii) column is from this lane's run of the same switch on the head
before the phase (the `box ∪ cone` figure is the one on the unit's closed
row `an-anchored-cone-section-encloses-wider-at-interval`); a centre-form
conic does not read the phase. It was not re-run on `8d5978300`. On these
box-sized sections, `K ≥ 0.16`, the widths at the mint are: semi-axis
5e-14 to 1.1e-13, vertex 1e-14 to 2.7e-14, `vertex − u·a` 5e-14 to
1.2e-13, the base's centre formula 5e-14 to 1.3e-13.

## Toward the parabola, at sizes under 1 km

A probe, not a suite row: the cone of `carrier_resolution`'s far-section
row (half-angle 0.4, apex (0.2, 0.1, 2.0)), cut by a plane through the
axis point `apex gap` below the apex and tilted `off` short of the
parabolic tilt, every input lifted to a point `Interval`, so every width
is the mint's own arithmetic. `near-parabolic-table.txt` is the table;
the two `.txt` beside it are the raw lines.

Read at the mint, the same under both storages:

- the vertex is 8.4e-15 to 8.9e-16 wide at every `K`;
- the semi-axis is about `5·u·a/K` wide: 4.7e-14 at `a = 0.17, K = 8.5e-3`;
  4.5e-10 at `a = 16.6, K = 8.4e-5`; 4.3e-8 at `a = 166, K = 8.4e-6`;
  4.5e-6 at `a = 166, K = 8.4e-8`.

The worst implicit residual of a point of the carrier on the cone or the
plane it was cut from, an enclosure's larger end, against a zero band of
1e-9:

| a (m) | K | stored at | t = 0 | 1e-3 | 0.05 | 0.5 | 3 |
|---|---|---|---|---|---|---|---|
| 1.66 | 8.4e-6 | vertex | 6.5e-16 | 1.1e-15 | 3.7e-13 | 3.3e-11 | 5.3e-10 |
| 1.66 | 8.4e-6 | centre | 5.3e-10 | 4.7e-10 | 5.7e-10 | 5.0e-10 | 5.3e-10 |
| 16.6 | 8.4e-7 | vertex | 6.2e-16 | 2.1e-14 | 3.5e-11 | 3.3e-9 | 5.3e-8 |
| 16.6 | 8.4e-7 | centre | 5.4e-8 | 3.7e-8 | 5.5e-8 | 5.1e-8 | 5.3e-8 |
| 166 | 8.4e-6 | vertex | 6.7e-16 | 5.0e-14 | 3.7e-11 | 3.3e-9 | 5.3e-8 |
| 166 | 8.4e-6 | centre | 5.3e-8 | 4.7e-8 | 5.7e-8 | 5.0e-8 | 5.3e-8 |
| 166 | 8.4e-8 | vertex | 6.3e-16 | 1.8e-12 | 3.5e-9 | 3.4e-7 | 5.4e-6 |
| 166 | 8.4e-8 | centre | 6.5e-6 | 6.0e-6 | 5.5e-6 | 5.1e-6 | 5.4e-6 |
| 167 | 8.4e-4 | vertex | 7.6e-15 | 1.2e-14 | 4.2e-13 | 4.0e-11 | 5.8e-10 |
| 167 | 8.4e-4 | centre | 5.7e-10 | 5.7e-10 | 4.0e-10 | 5.8e-10 | 5.7e-10 |
| 1.91 | 9.3e-2 | vertex | 8.4e-15 | 8.4e-15 | 9.8e-15 | 3.0e-14 | 2.5e-13 |
| 1.91 | 9.3e-2 | centre | 1.8e-13 | 1.8e-13 | 1.8e-13 | 1.4e-13 | 2.0e-13 |

Stored at the centre, a point's residual is the semi-axis's width at
every parameter, the vertex included. Stored at the vertex, it is the
vertex's own width at the vertex and grows with the chord from it,
reaching the centre form's at the far side (`t = 3`). Against the band:

- `a = 166, K = 8.4e-6`: at the vertex and to `t = 0.05`, (i) reads
  under 1e-9 (5e-14 to 4e-11) and (ii) reads 5e-8, past the zero band
  and past the escalation band of 1e-8.
- `a = 16.6, K = 8.4e-7`: the same, (ii) at 4e-8 to 5.5e-8.
- `a = 1.66, K = 8.4e-6`: (ii) reads 5e-10, under 1e-9 and past 1e-12.
- `K ≥ 1e-3` at these sizes: both under 1e-9; (ii) is one to four
  decades wider near the vertex.

Not measured: whether a body's build through the join keeps or refuses
in each case (the probe reads the carrier's residual on its two
surfaces, which is what certification's surface-residual check meters,
and not the join).

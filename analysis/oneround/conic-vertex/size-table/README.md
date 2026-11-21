# Where the centre form starts to lose, by size: the near-gap families

Measured 2026-11-21 on `oneround/anchored-conic` at `7f8faa67c` (step A)
with a temporary probe beside
`in_band_sweep::the_near_gap_families_certify_by_their_stored_form`
(`crates/topo/src/boolean/conic_quadric/in_band_sweep.rs`). Facts only.

**Correction (2026-11-21).** The "reader-chord" cells below were judged
against the curve through the rounded vertex `fl(centre + u_ref·ρ)` (the
oracle run on the anchored conic built there), not against the stored
centre's curve. Judged against the stored centre's curve the column is
0 wrong up to ρ = 1500 and 86 wrong of 200 at ρ = 1e4 (ε = 1e-9):
`../exact-vertex-chord/README.md`. The centre and anchored columns stand.

## The poses

The row's own: a conic of semi-axis ρ grazing the wall of radius 1 at
`(1, 0, 0)`, ten depths 0.5 to 2 µm into it, four tilts, each crossing
twice; 40 poses per family. Families: a circle, and ellipses of semi-axes
`ρ` and `ρ·ratio` at ratio 1.25, 0.999, 1.001, 0.8, seamed at the vertex
that grazes. The door is `conic_quadric_roots(c, −1, 1, wall, band)`.

Three forms of each pose:

- **centre**: stored with its centre (`tangent_circle`,
  `Ellipse::new(Frame(at − toward·ρ), …)`).
- **anchored**: stored anchored at the graze point (`Circle::anchored`,
  `Ellipse::anchored`).
- **reader-chord**: the centre-stored conic's own frame, with the point
  summed by chords from `fl(centre + u_ref·ρ)`: built as an anchored conic
  whose anchor is that sum, which is the arithmetic of a reader that reads
  a centre-stored conic's zero point and sums from it.

Every run is at K = 10, band `(ε, 10ε)`. `tolerance::admissible(ε, 10)` is
true at ε = 1e-9 and at ε = 1e-12 (`ε_min(10) ≈ 2e-13`). The session box
is 1e3 m: ρ = 1.5e3 and 1e4 put the conic past it.

The centre of each conic lies ρ − 0.07 … ρ − 0.7 m from the origin
(9.33 at ρ = 10; 99.3; 499.3; 899.3; 999.3; 1499.3; 9999.3): inside the
box up to ρ = 900, at its edge at 1000, past it beyond. A large conic
whose centre is well inside the box cannot be posed in this family: the
conic passes through `(1, 0, 0)`, so its centre is ρ from there.

Each cell is `right / wrong / refused` of 40. **Right**: certified, the
count of roots the exact oracle's (`conic_oracle::exact::truth`), and
each root within ε of arc of the oracle's. **Wrong**: a definite answer
that fails that. **Refused**: a typed refusal; over all runs these were
`Unsettled::Meter { row: "bool_conic_quadric_sub_root_slack" }` and
`BooleanError::Escalated { decision: ArcCylinderRoo… }`.

"Slack" is the largest margin the door handed any `…root_slack` decision
over the family's 40 poses, in units of `u·ρ` (`u = 2⁻⁵³`). It is the
meter's whole reading, the pose's conditioning included, not its
rounding term alone.

## ε = 1e-9, K = 10 (admitted)

| ρ | form | circle | 1.25 | 0.999 | 1.001 | 0.8 | slack, u·ρ (circle … 0.8) |
|---|---|---|---|---|---|---|---|
| 10 | centre | 40/0/0 | 32/0/8 | 40/0/0 | 40/0/0 | 39/0/1 | 1.6e5, 2.4e4, 3.0e5, 1.4e5, 3.2e5 |
| 10 | anchored | 40/0/0 | 32/0/8 | 40/0/0 | 40/0/0 | 39/0/1 | 1.9e4, 3.1e3, 3.7e4, 1.7e4, 3.4e4 |
| 10 | reader-chord | 40/0/0 | 32/0/8 | 40/0/0 | 40/0/0 | 39/0/1 | as anchored |
| 100 | centre | 40/0/0 | 32/0/8 | 40/0/0 | 40/0/0 | 40/0/0 | 1.4e4 … 1.8e4 |
| 100 | anchored | 40/0/0 | 32/0/8 | 40/0/0 | 40/0/0 | 40/0/0 | 197 … 254 |
| 100 | reader-chord | 40/0/0 | 32/0/8 | 40/0/0 | 40/0/0 | 40/0/0 | as anchored |
| 500 | centre | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 1.32e4 … 1.66e4 |
| 500 | anchored | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 37.8 … 47.5 |
| 500 | reader-chord | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | as anchored |
| 900 | centre | 33/0/7 | 33/0/7 | 33/0/7 | 33/0/7 | 21/0/19 | 1.31e4 … 1.65e4 |
| 900 | anchored | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 20.9 … 26.2 |
| 900 | reader-chord | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | as anchored |
| 1000 | centre | 28/0/12 | 28/0/12 | 28/0/12 | 28/0/12 | 15/0/25 | 1.31e4 … 1.64e4 |
| 1000 | anchored | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 18.8 … 23.5 |
| 1000 | reader-chord | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | as anchored |
| 1500 | centre | 7/0/33 | 7/0/33 | 7/0/33 | 7/0/33 | 1/0/39 | 1.31e4 … 1.64e4 |
| 1500 | anchored | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 12.5 … 15.7 |
| 1500 | reader-chord | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | as anchored |
| 1e4 | centre | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 | 1.30e4 … 1.63e4 |
| 1e4 | anchored | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 1.87 … 2.34 |
| 1e4 | reader-chord | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | 40/0/0 | as anchored |

No wrong definite verdict in any cell. The centre form's slack is about
1.3e4 `u·ρ` from ρ = 100 up, so it grows in metres with ρ: 1.5e-10 m at
ρ = 100, 7.3e-10 at 500, 1.3e-9 at 900, 1.45e-9 at 1000, 2.2e-9 at 1500,
1.45e-8 at 1e4, against a zero band of 1e-9. The anchored form's is
about 2e-12 m at every ρ from 100 up (2.2e-12, 2.1e-12, 2.1e-12, 2.1e-12,
2.1e-12, 2.1e-12). The reader-chord form reads the anchored form's
counts and slack at every ρ, to the digits printed.

The eight 1.25-ellipse refusals at ρ = 10 and 100 and the one 0.8-ellipse
refusal at ρ = 10 are the same in all three forms.

## ε = 1e-12, K = 10 (admitted), the same depths (0.5 to 2 µm)

| ρ | form | circle | 1.25 | 0.999 | 1.001 | 0.8 |
|---|---|---|---|---|---|---|
| 10 | centre | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 | 2/0/38 |
| 10 | anchored | 3/0/37 | 7/0/33 | 3/0/37 | 3/0/37 | 2/0/38 |
| 100 | centre | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 |
| 100 | anchored | 11/0/29 | 11/0/29 | 11/0/29 | 11/0/29 | 2/0/38 |
| 500, 900, 1000, 1500 | centre | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 |
| 500, 900, 1000, 1500 | anchored | 13/0/27 | 13/0/27 | 13/0/27 | 13/0/27 | 5/0/35 |
| 1e4 | centre | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 | 0/0/40 |
| 1e4 | anchored | 13 certified (3/10/27 by the check) | 13 (5/8/27) | 13 (3/10/27) | 13 (3/10/27) | 5 (1/4/35) |

The reader-chord form reads the anchored form's cells at every ρ up to
1500. The slacks in `u·ρ` are the ε = 1e-9 table's (the poses are the
same).

**The "wrong" entries at ρ = 1e4 are the check's, not shown to be the
door's.** In each, the door's two roots are `±t`, the oracle's are `t`
and `2π − t` as `f64`, and the pair that fails is the door's `−t` against
the oracle's `2π − t`: e.g. certified `−2.958336963574683e-7`, oracle
`6.28318501134589`. An `f64` near 2π is spaced 8.9e-16 apart, which is
8.9e-12 m of arc at ρ = 1e4, above ε = 1e-12; the other root of each
pair agrees to 7e-14 m. The reader-chord form at ρ = 1e4 shows the same
kind (7/6/27, 2/11/27, 0/13/27, 4/9/27, 1/4/35). ρ = 1e4 is ten times
past the session box.

## ε = 1e-12, K = 10, depths scaled with ε (0.5 to 2 nm)

Every form refuses every pose at every ρ on the sub-root slack (0/0/40),
but the 0.8 ellipse at ρ = 10, which certifies 10 in all three forms.
The slack is 4e5 to 8e5 `u·ρ` in the centre form at every ρ, and in the
anchored form 7e4 (ρ = 10), 6e3 (100), 1.2e3 (500), 660 (900), 594
(1000), 395 (1500), 59 (1e4).

## Not measured

- A (ε, K) outside the admission rule: none was run.
- The meter's rounding term apart from the pose's conditioning.
- Any family but the row's own.

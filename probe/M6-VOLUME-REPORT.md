# M6: check 7's volume reading over a box, term by term

Probe branch `boxfinish/probe-volume-reading` (never merged), on main `2e08069a`. Hooks: `topo::props::probe_volume_reading` (the role walk as check 7 makes it: `role_walk`, certified lane, `PLUS_V_SIGN`), `probe_plane_shell`, `BOXFINISH_VOL_TRACE=1` traces in `rederive_polygons`, `rederive_faces`, `quad_lane::polygon_face_about` / `planar_face_about`, `Round::certify`, and phase clocks in `validate`'s `plus_v_by_sign`. Tests: `crates/sweep/tests/probe_volume.rs` (print only, assert nothing).

    export CARGO_TARGET_DIR=/home/user/boxfinish-m6-target CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 \
      CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only
    BOXFINISH_VOL_TRACE=1 cargo test -q -p sweep --test probe_volume <item1_slab_terms|item3_|item4_|item5_> -- --nocapture

All box numbers are at `Sym<Interval>`; volumes are flux/3. The slab is the unit square extruded by depth `d`.

## 1. The slab, term by term (`item1_slab_terms`)

Lane (measured): every face is a plane bounded by lines, so `rederive` takes `shell_polygons` -> `rederive_polygons`. `exact_polygon_volume` is `None` (a vertex is no point interval), so the reading is the fallback sum of `polygon_face_about` over the six faces, about the shell's reference `corner_of` = `(0, 1, d)`, a TOP vertex, itself boxed. No pad, no quadrature, `limit=None`.

Depth `[0.75, 1.25]`, reading `[-0.969853, 2.969853]` (width 3.940; truth `[0.75, 1.25]`, width 0.5):

| face | true term (f64 members 0.75 / 1 / 1.25) | plane flux as read | tilt slack as read | term as read | width |
|---|---|---|---|---|---|
| top cap (n +z) | 0 / 0 / 0 | [-0.3333, 0.3333] | ±0.5774 | [-0.9107, 0.9107] | 1.821 |
| bottom cap | 0.25 / 0.3333 / 0.4167 | [0.25, 0.4167] | 0 | [0.25, 0.4167] | 0.167 |
| wall y=0 | 0.25 / 0.3333 / 0.4167 | [0.25, 0.4167] | ±0.2366 | [0.0134, 0.6533] | 0.640 |
| wall x=1 | 0.25 / 0.3333 / 0.4167 | [0.25, 0.4167] | ±0.2366 | [0.0134, 0.6533] | 0.640 |
| wall y=1 | 0 / 0 / 0 | 0 | ±0.1680 | ±0.1680 | 0.336 |
| wall x=0 | 0 / 0 / 0 | 0 | ±0.1680 | ±0.1680 | 0.336 |

Operands (measured, trace): every stored normal is exact; bottom ring exact; every top vertex has `z = [0.75, 1.25]`.
- Top cap: `anchor.lever = (0, 0, d)`, `lever = anchor.from(centre) = (0.5, -0.5, [-1, 1])` where the true z is 0: `(lever.z - centre.z) + offset.z` is `(d - d) + mean(d - d)`, two self-differences of the one boxed depth. Its fan `va = Σ (p - l) × (q - l)/2` reads `([-1, 1], [-1, 1], 1)` (true `(0, 0, 1)`): `p.z - l.z` is `d - d` again. So `across = ([-1, 1], [-1, 1], 0)` and `tilt_slack = ±|lever|·|across| = ±1.2247 × 1.4142`.
- Each wall: `va·n = [0.75, 1.25]` (the real width), but `across = va - n (va·n)/(n·n) = d - d = [-0.5, 0.5]` (true 0): the wall's own area subtracted from itself.
- Width budget, 3.940 = 0.500 real (bottom + two walls) + 0.667 top cap's lever + 1.155 top cap's tilt slack + 1.618 four walls' tilt slack.
- `[0.95, 1.05]`: reading `[0.679437, 1.320563]` (0.6411 = 0.100 real + 0.133 + 0.139 + 0.269). `[0.99, 1.01]`: `[0.937329, 1.062671]`. Point box: `[1, 1]` exactly.
- The walk's own sums at the walk's scalar are wider still (`[-1.583, 3.583]` at ± 0.25) but do not decide: the lane's re-derivation does.

Establishes where every unit of width enters on the slab. Does not cover plain `Interval`: the extrude door refuses a boxed depth there (off-fence 1); by `QuadLane.lift = Interval::from_certified` the reading would be the same bits (inferred).

## 2. Dependency or real (`item1_slab_terms`, the `R` lines)

All of the excess is dependency (measured): the one boxed depth entering two operands that cancel in the true value, at three sites: (a) anchor minus centre on the top cap, (b) `p - l` inside the top cap's fan, feeding `tilt_slack` and the area, (c) `va - n(va·n)/(n·n)` on every wall. No per-face pad and no quadrature term exists on this route (measured: `limit=None`, zero width at a point box).

The symbolic channel is not read (measured + read in code): every top vertex's z renders as the same atom `1·?#a5dc1bb2`; the fan sum formed by hand at `Sym<Interval>` has the form `(6·?#a5dc1bb2) / (6)` (the volume IS the depth, in the DAG) while its value channel reads `[0.6667, 1.3333]`; the reading lifts each coordinate through `Interval::from_certified`, which reads `Bounds`, and `Bounds for Sym` is the value channel verbatim. The role walk takes 0 session decisions (measured, item 5). `Sym::decide_sign` proves only `Zero` from a form (read in code), so the form could not hand a sign as the tier stands (inferred).

Hand re-posings, width of the enclosure (each is centred on the truth; truth width in the last row):

| re-posing | ±0.25 | ±0.05 | ±0.01 | ±0.05 placed at 1000 |
|---|---|---|---|---|
| R0 the code's arithmetic by hand (equals the reading to the bit) | 3.940 | 0.641 | 0.1253 | 0.641 |
| R1 same, tilt slack dropped | 1.167 | 0.233 | 0.0467 | 0.233 |
| R3 about an exact point of the body, slack kept | 4.000 | 0.683 | 0.1321 | 0.683 |
| R3' about an exact point, slack dropped | 0.833 | 0.167 | 0.0333 | 0.167 |
| R2 about the world origin, slack kept | 4.000 | 0.683 | 0.1321 | 922 |
| R4 fan `Σ det(r0, ri, ri+1)/6` about the world origin | 0.667 | 0.133 | 0.0267 | 2.67e5 |
| R5 fan about an exact point of the body | 0.667 | 0.133 | 0.0267 | 0.133 |
| R5' fan about the code's boxed centre | 1.833 | 0.367 | 0.0733 | 0.367 |
| R6 caps paired `(top - bottom)·n × A`, walls about an exact point, no slack | 0.500 | 0.100 | 0.0200 | 0.100 |
| truth (f64 members) | 0.500 | 0.100 | 0.0200 | 0.100 |

Establishes: the tilt slack is 70% of the width, the boxed centre 0.33 of the rest, and pairing the caps recovers the truth exactly on the slab. Does not establish that any re-posing is sound or cheap in general (R1/R3'/R6 drop the slack that covers a loop standing off its plane).

## 3. Scaling (`item3_scaling`, `item3_threshold`, `item3_far_reposings`)

| half-width | box width | reading | reading width | ratio to box |
|---|---|---|---|---|
| 1e-6 | 2e-6 | [0.999993758, 1.000006242] | 1.2485e-5 | 6.2425 |
| 1e-4 | 2e-4 | [0.999375731, 1.000624269] | 1.2485e-3 | 6.2427 |
| 1e-2 | 2e-2 | [0.937328864, 1.062671136] | 1.2534e-1 | 6.2671 |
| 0.05 | 0.1 | [0.679436859, 1.320563141] | 6.4113e-1 | 6.4113 |
| 0.25 | 0.5 | [-0.969853371, 2.969853371] | 3.9397 | 7.8794 |

Linear in the box, 6.24 times the box (truth: 1 times), with a small quadratic term (the slack is a product of two boxed lengths). Placed at 10 and at 1000 (every axis) the five readings are the same to all nine digits: no dependence on the distance from the origin, because the centre is a vertex of the body (measured; R2/R4 show what the world origin would cost). The slab's sign is decided at half-width 0.14 (`[0.0259, 1.9741]`) and not at 0.15 (`[-0.0549, 2.0549]`); its rays read `Out` through 0.215 and are all set aside at 0.22.

## 4. Other bodies

- **Seven faces, turning walls** (my fixture, not `m10_3`'s: a pentagon `(0,0) (2,0) (x,1) (1,2) (0,1)`, depth 1, `x = 2.5 ± w`; truth `x + 1`). ±0.05: `[2.597, 4.464]`, width 1.867 against a true 0.1; ±0.01: `[3.3226, 3.6798]`, 0.357 against 0.02 (18 times the box). Polygon route. The two turning walls carry 1.66 of the 1.867: their stored normals are boxed (`([0.876, 0.912], [-0.502, -0.394], 0)`), so `plane_flux = (lever·n)(va·n)/(n·n)` holds the box in three operands (0.301 and 0.343 wide) and their slack is ±0.211 and ±0.295; the caps' slack adds 0.176 (true cap width 0.033). Here the plain fan R4 reads 0.167 and pairing the caps does not help (0.851): the width is in the walls. ±0.3 does not validate as a profile (off-fence 5).
- **Loft** (square `[-1,1]²` at z=0 and z=h, four NURBS walls; truth `4h`). The rays aim at nothing (`aimed: 0`, measured). ±0.001: `[3.9373, 4.0627]`; ±0.01: `[3.3701, 4.6299]` (true `[3.96, 4.04]`); ±0.05: `[0.7833, 7.2167]`, still `Outer`; ±0.1: `[-2.613, 10.613]`, refused as `VolumeSignUnresolved { limit: Quadrature }`. Linear, 63 times the box (truth 4 times). Route `rederive_faces`, `limit=Some(Quadrature)`: caps through `planar_face_about` (top cap ±0.1067 where the truth is 0, the slab's sites (a) and (b)); each wall is `quadrature: walk's enclosure (untight)` at round 0. At ±0.01 the walls carry 1.020 of 1.260: `[-0.0807, 0.0807]` twice (truth 0) and `[1.159, 1.508]` twice (truth `[1.32, 1.3467]`). At f64 members the same walls are 5e-15 wide, so it is the box, not the quadrature's error (measured); that it is the control net carried by the boxed centre (`quad_lane`'s `about`) is inferred from the code, not split out. One round only: nothing refines.
- **Two-arc / circle extrusion over a boxed depth: not measured**, the door refuses each spelling I built (off-fence 2). At a point box its lanes are `planar_face_about` for the caps (the slab's arithmetic) and the cylinder closed form moved by the centre for the walls; reading `[3.14159265358977, 3.14159265358981]`.

## 5. Cost (`item5_cost`, `item5_loft_cost`; opt-level 1, three repeats, traces on)

| slab depth | finish | check 7: round zero | role walk (the reading) | side reader | finish decisions sym / num | frozen | outcome |
|---|---|---|---|---|---|---|---|
| point | 30-35 ms | 0.3 ms | 0.23 ms | 14-18 ms | 348 / 475 | 182 | finished |
| ±0.025 | 27-28 ms | 0.25 ms | 0.21 ms | 12.6-13.6 ms | 348 / 475 | 206 | finished |
| ±0.2 | 41-42 ms | 0.3 ms | 0.24 ms | 25.7-27.2 ms | 348 / 505 | 382 | finished (rays) |
| ±0.25 | 87-88 ms | 0.3 ms | 0.25 ms | 72-73 ms | 348 / 637 | 1631 | `OrientationUndecided` |

The reading alone, called directly: 0.5-0.7 ms, 0 symbolic and 0 numeric session decisions, all six faces closed-form (`REDERIVED dt 0.25 ms`). It runs on every finish, before the rays (`plus_v_by_sign` walks first). The cost of a finish that "falls to the volume" is the side reader setting rays aside: 36 numeric decisions and 13 ms where the first ray reads, 198 and 72 ms where all six are set aside (measured). Loft finish: 52-62 ms, of which round zero (the walls' quadrature) 11 ms, the reading 0.18 ms, the side reader 0.01 ms (11 ms where the sign is unresolved and the bracket is read again). No profiler was used; phase clocks only.

## Where the width enters

On every body measured, the reading is formed in plain `Interval` from the value channel of the stored coordinates, about a centre that is a vertex of the body and is itself boxed, and the boxed coordinate is then subtracted from itself where the true value is zero or constant: in the anchor-to-centre lever of the face holding the centre, in the vertex-minus-anchor differences of that face's fan, and in `tilt_slack`'s `va - n(va·n)/(n·n)` on every face whose area depends on the box. The slack then multiplies two such widths and is added symmetrically per face. None of it is a real width, a pad or a quadrature error on the slab; on a turning wall the stored normal's own width adds a real factor that the same formula counts three times; on a loft the walls' quadrature enclosures carry four fifths of it.

## Corrections to the row's premises

1. "Reads only at a box width of about 0.03 or less" is stale: with the frames exact the slab's reading decides through a box width of 0.28 (± 0.14). `{-1.617, 4.129}` at ± 0.05 is `[0.679, 1.321]` today.
2. The slab's volume reading gives out (± 0.15) before its rays do (± 0.22), so on the slab the volume never rescues a body the rays lost; between 0.215 and 0.22 nothing changes for it.
3. "A finish that falls to it costs 254 to 787 ms" is not the reading's cost: the reading is 0.5 ms and no decisions, on every finish. The cost is the rays' set-asides.
4. "Where the width enters is not traced": it is three self-differences of the boxed coordinate and the tilt slack built on them; not distance from the origin (identical at 0, 10, 1000).
5. A loft's wide reading refuses as `VolumeSignUnresolved { Quadrature }`, not `OrientationUndecided`; it decides through ± 0.05 of a unit height.

## Off-fence (listed, not filed)

1. Plain `Interval`: `extrude` refuses any boxed depth on the slab, `Certification { EndpointEnd, "carrier_endpoint_end", Enclosure { 0, box width } }`; `loft_body::<Interval>` with a boxed top height refuses the same way (`Euler(Certification ..)`).
2. `Sym<Interval>` arc-walled extrude over a boxed depth refuses at the door: `bulge_loop` arcs with `MappedSource` / `"carrier_matches_mapped_source"` `{0, box width}`; `Arc2` arcs with f64 π sweeps with `EndpointEnd` `{2.4e-16, box width}`. Possibly my fixtures' inexact π; not chased.
3. `loft_body::<Sym<Interval>>` of the square with a POINT top height refuses `Pcurve(Certify { IsoUnsupported: "a LINE seam on a RATIONAL chart column ..." })`, while a boxed height builds and plain `Interval` at a point builds.
4. A loft of an arc-sided square with only its height boxed refuses `Skin(Structure(Built(CarrierWeightNotAPoint { index: 1 })))` at both box scalars.
5. The pentagon with a corner at 2.5 ± 0.3 refuses profile validation (`LineSpan`, margin from -0.9996) though every member is a simple convex-cornered polygon; at plain `Interval` it refuses at every width (`ChordSide`).

## Not measured

Plain `Interval` over any box; an arc or circle extrusion over a box; `m10_3_driver_interval`'s own seven-face body (not built; a pentagon prism stands in); which operand inside the NURBS lane widens a loft wall; a profiler's view of the cost.

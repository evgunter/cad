# M15 — why tier 3′'s census is undecided on a brick whose moving cap is tilted

Tree: `boxfinish/probe-side-reading` with `origin/main` merged (a86f5378bc) plus
probe prints. Everything is **measured** unless marked *inferred*. Raw traces:
`probe/m15/`.

## What I ran

`topo` lib test `m15_probe` (ignored, prints; in `side_reader_tilted_cap_rows.rs`),
under `BOXFINISH_M15=1|2` (+ `_NAMES`, `_EXPLAIN=3`): the census by itself
(`census::census_and_certify`, no records) on `tilted` at (0.3, 0.2) for
`Sym<Interval>` at ± 0.05, ± 0.25 and width 0, plain `Interval` at width 0, five
`f64` members, the square brick (`tilted` at (0, 0) and `lined_brick`) at ± 0.05,
and (−0.45, 0.6) at ± 0.25. Prints: in `Sym::decide_sign`, every decision the
numeric channel did not sign, with its enclosure, whether the tier was asked, its
answer, and the plain and early forms (`report::render_node`); in
`census::pair_edge_face` the pair's stored points, origin, normal and residuals; in
`contain::one_vertex_at` the two points.

## The answer in three lines

1. **Dependency on the value channel, not a zero crossing.** The height is ONE
   symbol in every stored value; the tier's form of each margin is box-free (or
   zero-free over the box). The `Interval` value beside it reads the height twice.
2. **The tier is asked every time and answers nothing, because it only proves the
   zero form** (`sym::ladder`: `plain.is_zero()`, then the rungs' `is_zero()`), and
   these margins are not zero: they are a rounding-size constant, or a definite
   non-zero function of the height. There is no rung that reads a non-zero form's
   sign or size.
3. On the square brick the same residual IS the zero form (`top − top`), proven at
   the `Plain` rung; the tilt makes it `6.6e-19`.

## 1a. `pm_census_ef_residual` (`census::pair_edge_face`, pass 4)

Pair: edge 5, the strut over (0, 0), against face 1, the cap (not adjacent: the
strut bounds two walls). Margin `r1 = normal · (p1 − origin)`
(`geom::plane_offset`), `p1` the strut's top end.

| stored value | enclosure | carries the height through |
|---|---|---|
| `p1.z` | [0.95, 1.05] = `top` | the cap's vertex (`prism_ops`' point, `top + (0.3x + 0.2y)`) |
| `origin.z` | [1.15, 1.25] = `top + 0.2` | the cap's plane, whose origin is the cap's corner vertex over (0, 1) |
| `origin.y`, `p1.x`, `p1.y`, `origin.x` | 1, 0, 0, 0 | — |
| `normal` | (−0.28221626, −0.18814417, 0.94072087), width 3e-16 | — (an `f64`-normalized literal; \|n\|² = 1.0000000000000002) |

Term by term: `nx·0 = 0`; `ny·(0 − 1) = +0.18814417`;
`nz·(p1.z − origin.z) = 0.94072087·([0.95, 1.05] − [1.15, 1.25] = [−0.3, −0.1]) = [−0.2822, −0.0941]`;
sum **[−0.09407, +0.09407]** = ± `nz·2w`. Both heights are the cap's vertices; no
wall line or edge curve enters.

- Members (five `f64` tops 0.95 … 1.05): `r1` = +5.29e-17, −5.16e-17, +5.29e-17,
  +5.29e-17, +5.29e-17; census 0 findings at each.
- As one symbol (the tier's plain form, measured):
  `(106787790023391·2^-107) / sqrt(324518553658426775224777685791785·2^-108)`
  = **6.58e-19**, no parameter in it (numerator 1 term, degree 0). So the height
  already cancels in the expression; what is left is the plane not holding its own
  vertex exactly (normalized normal, `f64` corner heights).
- Three of the cap's four struts read this (enclosures ± 0.09407 each, constants
  6.58e-19 or 1.32e-18); the fourth, over the plane's own origin, is `p − p` and
  is proven at `Plain`.

## 1b. `bool_contact_vertex` (`contain::one_vertex_at`, from `cycles_pre_pass`)

Downstream of 1a, and asked only because 1a is undecided: `read_undecided` takes
the branch where the strut pierces the cap, `census::pierce_span` builds
`t = r0/(r0 − r1) ± reach` from `r0 = [−0.988, −0.894]` and `r1 = ± 0.094`, and the
pierce point `p = (0, 0, [0.7848, 1.2971])` is located in the cap, which compares
it with each cap vertex. Against the strut's own end `q = (0, 0, [0.95, 1.05])`:
`dx = dy = 0` (proven), `dz = [−0.265, +0.347]`, distance **[0, 0.347]**.

- The height enters four times: `r0`, `r1` (each through `p.z` and `origin.z` as
  in 1a), the strut's end in `p0 + t·(p1 − p0)`, and `q.z`.
- Members and width 0: the question is **never asked** (0 calls): the residual
  reads Zero there and the pierce branch is not taken. So there is no member value
  to sample; *inferred from 1a's constant:* with one symbol `t = 1 − 7e-19` and
  the distance is the span's own `reach`, about 1e-16, inside the band.
- The tier's form: `sqrt(0 + 0 + dz²)` with `dz²` **FROZEN**; `dz` itself is
  numerator 8 terms degree 5 over denominator 3 terms degree 3, with `abs(…)` and
  `sqrt(…)` atoms (the rounding charges of `pierce_span`'s `reach`). Squaring it
  passes the session's degree budget (8), so the square is an opaque atom.

## 2. The symbolic tier, one line per kind (tilt (0.3, 0.2), ± 0.05)

| kind | asked | answer | why no proof |
|---|---|---|---|
| `pm_census_ef_residual` (3) and `pm_census_ef_residual_side` (3, same node) | yes | none | a reduced expression that is a non-zero constant (6.6e-19); nothing to prove zero; no budget, no missing identity |
| `bool_contact_vertex` (3) | yes | none | `dz²` frozen on the degree budget (`SymCounts.frozen` = 6 for the body), and `dz` carries charge terms that are not zero in any case |

Session counts for the body: symbolic 293, numeric 133, retried 0, frozen 6,
registrations refused 0. No retry rung is reached on these.

## 3. The square brick (census passes)

Same pair, same predicate: `p1.z = top` (node …8060) and `origin.z = top`, **the
same node**; `normal = (0, 0, 1)` exact. The value channel still reads
`[0.95, 1.05] − [0.95, 1.05]` = **± 0.1** (the dependency is there too), but the
form is `0` and the tier proves it at `Plain` (attempt 0). `bool_contact_vertex`
is then not asked at all (0 calls). The whole difference is `origin.z = top + 0.2`
times `nz = 0.9407…` against `ny·(−1)`: `0.94072087·0.2` and `0.18814417` agree
to 6.6e-19 and not exactly.

## 4. Width 0 on the tilted brick

Neither predicate escalates. `Sym<Interval>` at width 0: 0 findings; the three
residuals read numeric **Zero** in band (enclosures about ± 4e-16 to ± 8e-16), the
tier still answers none on them, and `bool_contact_vertex` is not asked. Plain
`Interval` at width 0: the body builds, 0 findings, `r1 = [−2.2e-16, 3.6e-16]`.
So this is not among the nominal-point escalations.

## 5. The three wider kinds, tilt (−0.45, 0.6) at ± 0.25 (top ∈ [0.75, 1.25])

| kind | value enclosure | the tier's form (one symbol) | reading |
|---|---|---|---|
| `curve_carrier_gap` (`geom`'s `on_carrier`) | [0, 2.66], [0, 8.10] | `0.912·|top|`, `0.857·|top|` → [0.68, 1.14], [0.64, 1.07] | **the same dependency**: zero-free over the box |
| `pm_census_ee_parallel` (`census::parallel_reading`) | [0, 0.95] | the constant 0.4104, no parameter | **the same dependency** (the two directions are read off vertex differences, each carrying the height twice) |
| `pm_census_ef_pierce_lever` (`census::pierce_span`) | [0, 1.04] | `0.8·|top − 0.45|` less charges of 1e-15 → [0.24, 0.64] | **the same dependency**; its real zero is at top = 0.45, outside this box |

In all three the tier is asked and answers none for the reason of line 2 above:
the form is not zero. (Form values computed by hand from the printed forms.)

## Rows this is plainly the same cause as

- `an-undecided-at-rest-decision-climbs-the-ladder-on-a-margin-that-is-not-zero`
  (review, PR 7712): the same tier behaviour, a margin that is not the zero form
  gets no answer; here the forms are small and cheap, so it costs no time, only
  the verdict.
- `a-point-built-on-a-frame-of-normalized-directions-does-not-fold-onto-its-wall`:
  the plane's twin of it — a vertex built on a plane whose normal is an
  `f64`-normalized direction is on it to 6.6e-19 and not as the zero form.
- `the-census-reads-an-edges-direction-off-its-two-ends`: `pm_census_ee_parallel`'s
  directions.

## Not measured

- The constant forms' enclosures at the lane (only their point values, by hand).
- `bool_contact_vertex` at a member: it is not asked there.
- The wider kinds on tilt (0.3, 0.2): they first appear at ± 0.45 on that tilt;
  item 5 used the steeper tilt, where they appear at ± 0.25.
- (−0.45, 0.6) at ± 0.05 was not re-traced; M14 shows the same two kinds there.
- No fix, no timing.

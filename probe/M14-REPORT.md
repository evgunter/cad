# M14 — the side reader and tier 3′ over a box

Tree: `origin/main` 9e0bc5a1df plus probe rows, branch `boxfinish/probe-side-reading`
(new; nothing else pushed). CI build flags, own target dir. Everything below is
**measured** unless marked *inferred*. Raw tables: `probe/m14/`.

## What I ran

- `topo` lib test `m14_probe` (ignored, prints; appended to
  `side_reader_tilted_cap_rows.rs`): bodies (ii) `brick_at` (no edge lines stated)
  and `lined_brick`, (iii) `tilted` at (0.3, 0.2) and (−0.45, 0.6); top `1 ± w`;
  per body `each_aim_report`, `aimed_rays_report`, and `census_and_certify`.
  `brick_at` / `lined_brick` made generic over the scalar (no other fixture change).
- `sweep` example `m14_slab_probe`: body (i), `extrude` of the unit square by
  `Extrusion::Distance { depth: 1 ± w }`, `into_body()`; `topo::test_support::each_aim`,
  `aimed_rays`, `topo::census_traces` (`CensusStrategy::Realized`).
- Switches: `BOXFINISH_M14_FIX=1` (in `side_reader::aimed_ray`, aim and `out` replaced
  by the midpoints of their enclosures as point values at the lane scalar, immediately
  before `ray_crossings`; `selection_reach` and the start parameter untouched);
  `BOXFINISH_M14_B2=1` (in `solid_contain`'s planar-face arm, `crosses_by_turn` handed
  the ray's unit direction and the ray's point `q` in place of the face normal and
  the crossing point); `BOXFINISH_M14_CENSUS=1` (topo row only).
- **Isolation of tier 3′**: the census entry is called directly on the body with
  empty `ContactRecords` and no region lane; tiers 1–3 (check 7 included) are not run,
  so nothing gates it. The census findings are identical with and without the fixed
  ray (diff of the two runs: 0 lines).

## Correction to the premise first: plain `Interval` has no body

At plain `Interval`, at every width, none of the three bodies is built, so neither
question is reached (**not measured, for that reason**):

| body | where | refusal |
|---|---|---|
| (i) door slab | `extrude` | `Op { DescriptionUncertified { Minted, Escalated { EndpointEnd, sample 8, carrier_endpoint_end } } }`, enclosure `[0, 2w]` (0.1, 0.2, 0.5, 0.9) |
| (ii), (iii) fixtures | `prism_ops`' strut `mev` | same check and predicate, enclosure `[0, 0.211]`, `[0, 0.444]`, `[0, 1.33]`, `[0, 3.27]` |

All rows below are `Sym<Interval>`.

## A. Tier 3′'s census by itself (`Sym<Interval>`)

Every finding is `CensusEscalated` (undecided); none is a refusal of the body.

| body | 0.05 | 0.1 | 0.25 | 0.45 |
|---|---|---|---|---|
| (i) door slab | passes | passes | passes | passes |
| (ii) `lined_brick` | passes | passes | passes | passes |
| (ii) `brick_at` | passes | passes | passes | undecided: 4 `curve_carrier_gap` [0, 6.1], 4 `pm_census_ee_line_gap` [0, 3.16] |
| (iii) tilt (0.3, 0.2) | undecided: 3 `pm_census_ef_residual` ±0.094, 3 `bool_contact_vertex` [0, 0.33–0.35] | same two, ±0.188, [0, 0.73–0.83] | same two, ±0.47, [0, 2.8–5.5] | 3 `ef_residual` ±0.85, 4 `curve_carrier_gap` [0, 7.8–11.3], 4 `pm_census_ee_parallel`, 2 `pm_census_ef_pierce_lever`, 1 `bool_contact_vertex` |
| (iii) tilt (−0.45, 0.6) | undecided: 3 `pm_census_ef_residual` ±0.08, 3 `bool_contact_vertex` [0, 0.34–0.40] | same two, ±0.16, [0, 0.79–1.24] | 3 `ef_residual` ±0.4, 2 `bool_contact_vertex`, 2 `curve_carrier_gap`, 1 `ee_parallel`, 1 `ef_pierce_lever` | 3 `ef_residual` ±0.72, 4 `curve_carrier_gap` [0, 8.4–22.4], 4 `ee_parallel`, 3 `ef_pierce_lever` |

So the embedding question passes over the axis-aligned boxes at every width where the
edges carry stated lines, and stays undecided on the tilted cap from the narrowest
width, first on `pm_census_ef_residual` and `bool_contact_vertex`.

## B. Reach: largest of the four widths at which each aim reads

"–" = reads at none; in brackets, what stops it at the next width (predicate, the
reading's enclosure; `sai_` = `side_at_infinity_`). "reader" = `aimed_rays_report`
per width (0.05 / 0.1 / 0.25 / 0.45).

**(i) door slab = (ii) `lined_brick`** (the two tables are identical row for row):

| aim | today | fixed ray |
|---|---|---|
| top cap | 0.25 (0.45: `sai_order` [−9e-15, 2.39]) | 0.45 |
| bottom cap | 0.45 | 0.45 |
| four walls | 0.05 (0.1: `sai_edge` [0, 0.641]) | 0.1 (0.25: `sai_winding` [−5.11, 0.03]) |
| reader | Out / Out / Out / Out | Out / Out / Out / Out |

**(ii) `brick_at`** (edges without stated lines):

| aim | today | fixed ray |
|---|---|---|
| top cap | 0.1 (0.25: `sai_advance` [−18.5, 3.51]) | 0.1 (0.25: `sai_advance` [−15.8, 3.46]) |
| bottom cap | 0.1 (0.25: Graze) | 0.1 (0.25: Graze) |
| four walls | – (0.05: `sai_edge` [0, 0.682]) | 0.05 (0.1: `sai_advance` [−10, 11.3] / [−15.6, 7.45]) |
| reader | Out / Out / Unread / Unread | Out / Out / Unread / Unread |

**(iii) tilt (0.3, 0.2):**

| aim | today | fixed ray | fixed + B2 |
|---|---|---|---|
| top | – (0.05: `sai_winding`) | 0.1 (0.25: `sai_winding` [−2.41, 16.1]) | 0.25 (0.45: `sai_winding` [−13.6, 20.4]) |
| bottom | 0.05 (0.1: `sai_winding` [−6.66, 9.01]) | 0.05 (same) | 0.1 (0.25: Graze) |
| wall −y | – (`sai_edge` [0, 0.921]) | 0.1 (0.25: `sai_advance` [−106, 63.8]) | 0.1 (same) |
| wall +x | – (`sai_edge` [0, 0.856]) | – (0.05: Graze) | – (0.05: Graze) |
| wall +y | – (`sai_edge` [0, 0.847]) | – (0.05: Graze) | – (0.05: Graze) |
| wall −x | – (`sai_edge` [0, 0.828]) | 0.1 (0.25: `sai_advance` [−125, 78.7]) | 0.1 (same) |
| reader | Out / Unread / Unread / Unread | Out / Out / Unread / Unread | Out / Out / Out / Unread |

**(iii) tilt (−0.45, 0.6):**

| aim | today | fixed ray | fixed + B2 |
|---|---|---|---|
| top | – (0.05: `sai_edge` [0, 0.392]) | 0.1 (0.25: `sai_winding`) | 0.1 (0.25: `sai_winding` [−11.1, 13.9]) |
| bottom | 0.05 (0.1: `sai_winding` [−6.67, 9.67]) | 0.05 (same) | 0.1 (0.25: Graze) |
| wall −y | – (`sai_winding`) | 0.1 (0.25: `sai_advance` [−80.6, 47.2]) | 0.1 (same) |
| wall +x | – (`sai_edge` [0, 0.367]) | 0.1 (0.25: `sai_advance` [−151, 123]) | 0.05 (0.1: `sai_winding` [−7.16, 7.75]) |
| wall +y | – (`sai_edge` [0, 1.09]) | 0.05 (0.1: `sai_winding`) | 0.05 (same) |
| wall −x | – (`sai_edge` [0, 0.917]) | – (0.05: `sai_edge` [0, 0.566]) | – (same) |
| reader | Out / Unread / Unread / Unread | Out / Out / Unread / Unread | Out / Out / Unread / Unread |

**B2 on the axis-aligned bodies** (not asked, came with the run): `lined_brick`'s
bottom cap drops from 0.45 to 0.25 (0.45: `sai_winding` [−5.57, 7.61]); its other
aims and the reader are unchanged. So B2 is not uniformly a gain.

**Soundness, as the rows check it:** every `f64` member at five tops across each
box (`1 + f·w`, f ∈ {−1, −½, 0, ½, 1}; 80 fixture members, 20 door members) reads
`Side(Out)`; no boxed reading in any of the runs (today, fixed, fixed + B2, B2
alone) is `Side(In)`, per aim or per reader.

## In one paragraph

Over `Sym<Interval>`: fixing the ray moves the reader's own answer on the tilted
brick from w = 0.05 to w = 0.1 (to 0.25 on the gentler tilt with B2), and leaves the
axis-aligned bodies' answer where it was (already read at all four widths where
lines are stated; still unread at 0.25 and 0.45 on `brick_at`). Per aim, the fixed
ray trades the `sai_edge` stop for `sai_advance` / `sai_winding` one or two widths
later; no aim of the tilted brick reads at 0.45 in any variant. Tier 3′ by itself
does not pass the tilted brick at any of the four widths. *Inferred, not measured:*
the fixed-ray wall stops at 0.25 are the ray leaving through a different face for
different members (cap for a low top, far wall for a high one), which no fixed ray
through that aim avoids.

## Not measured

- Anything at plain `Interval` (no body, above).
- B2 without the fixed ray was run only for the soundness count, not tabulated.
- Why the two `+x` / `+y` wall aims of tilt (0.3, 0.2) graze under the fixed ray.
- No timing; no tour; no pins touched.

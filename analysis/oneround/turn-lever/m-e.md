# M-e: where the lever is read, against an exact truth

## The truth

On a plane-plane line edge whose two faces are both bounded by straight edges
only, each face is a polygon in its plane. The distance from the edge's line is
convex, so its maximum over the faces is at a vertex. So the farthest face
vertex's distance from the edge's LINE is exact. That is the truth here.
- **Population.** Every such edge the doors read (`WedgeReach::reach` with both
  faces planes, the carrier a line, every boundary edge a line). Reads with a
  curved boundary edge were skipped: 1.12 M at 1e-9.
- **Counts.**

  | ε | reads |
  |---|---|
  | 1e-9 | 8.78 M |
  | 1e-12 | 3.11 M |
  | 1e-6 | 3.97 M |

## The three sites, each with both bounds

- **middle** (today): the door's two bounds from the span's middle.
- **station:** from each of the 7 interior certification stations
  (`sample_param(t0, t1, 1..=7)`). From below: the farthest point (faces'
  vertices and the edge's ends). From above: the farthest of the covers, and
  the points.
- **line:** from the edge's line. From below: the same points' farthest
  distance, which is the truth itself on this population. From above: the
  covers' far distance from the line (`Cover::distance_range(Site::Line)`), and
  the points.

**The verdict.** For each lever, the verdict is the `dihedral_wedge` decision
`Margin::levered(sin θ, lever)`, with `sin θ` from the two plane normals. It is
detached from the log and compared with the verdict at the truth. On planes
the folded arm is the reach itself.

## Counts

| | middle | station | line |
|---|---|---|---|
| reads, 1e-9 | 8,782,805 | 61,479,635 | 8,782,805 |
| bound from below above the truth | 8,737,043 (99.5%) | 61,380,111 (99.8%) | 0 |
| bound from above below the truth | 0 | 0 | 0 |
| verdict ≠ truth's, both bounds alike: `+` where the truth reads `0` | 27 | 189 | 0 |
| `E` where the truth reads `0` | 7 | 49 | 0 |
| `+` where the truth reads `E` | 16 | 112 | 4 (above only) |

At 1e-12 and 1e-6 the counts are the same: 27 / 7 for the middle and
189 / 49 for the station, with no `+E` cases.

At the middle and the stations, **both** bounds give the same wrong verdict.
The bound from below is not a bound from below: a face's run ALONG the edge
counts as lever there. The line site's bound from below is the truth, so its
verdicts never differ from it. Its 4 `+E` cases are its bound from above
reading `+` where the truth is in band. That is the safe direction: acting only
where the two agree, they refuse.

**Bodies** of the differing cases. In every listed case except the probe, every
vertex of both faces lies exactly on the edge's line (truth 0) and up to 0.5 m
from the span's middle along it. These are degenerate faces, test fixtures. In
the probe the faces stand 1.49e-9 off the line and 1 m along it.
- `tier3_tests::description_references_keep_a_surface_alive`
- `tier3_tests::dangling_description_is_a_tier1_error`
- `euler::tests::the_gate_carries_and_refuses_a_line_under_an_intersection_description`
- `a_vertex_through_a_declared_seat::a_seat_divided_along_a_ring_or_met_with_a_seat_refuses_at_the_carry`
- `review_blend_k_rk_probes::print_rk_probe` (below 1, above 1, truth 1.49e-9:
  `+` at the middle, `E` at the truth)
- `review_m2_pr3::survives_sliver_dihedral_isolated_at_rest` and
  `tier3_tests::sliver_dihedral_at_rest_is_rejected` (`E` where the truth reads `0`)

## The turn, and the open row's own table

- **The joint turn** was not measured at the three sites. Its doors
  (`stored_reach`, `edge_reach`) take no pivot. There is also no truth outside
  hand-built fixtures, and none was built.
- **The open row's table**
  (`work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`) is a
  line-into-arc joint, a plane wall against a cylinder wall. It is outside this
  truth's population, so it was not checked here.

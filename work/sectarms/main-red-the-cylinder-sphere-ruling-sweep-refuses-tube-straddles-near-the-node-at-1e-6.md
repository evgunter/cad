---
id: main-red-the-cylinder-sphere-ruling-sweep-refuses-tube-straddles-near-the-node-at-1e-6
kind: issue
title: Main red at eps 1e-6: since PR 6518, seven cylinder x sphere and spline section rows refuse TubeStraddles on a zero margin near the node
status: closed
opened: 2026-11-11
closed: 2026-11-11
priority: P1
cost: M
branch: sectarms/ruling-sweep-straddle-1e-6
refs: [cylinder-sphere-frame-rows-take-twenty-minutes-a-boolean-at-1e-12, an-r3-tube-no-rung-makes-a-graph-is-never-refined]
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`), in nightly 42965378060 (main `b2916558d9`), at ε 1e-6 only. Every row below was green at ε 1e-6 in the nightly before (42761779270, `50cfcf499d`).

**Bisected, first-parent, at ε 1e-6** (`cargo nextest run --profile ci --ignore-default-filter`):

- green at `2e4c44de`, `10e131e7` (PR 6528) and `08f0298b` (PR 6717), which is `fc00197c^1`;
- red at `fc00197c`, the merge of PR 6518 (`sectarms/cylinder-sphere-ruling-sweep`, which rewrote `pair_section/ruling.rs` and touched `ssi.rs`);
- still red on main `019bed88`.

| row | at `fc00197c` and on main |
|---|---|
| `geom-brep::all pair_section::one_loop_keeps_its_order_near_the_node` | `d 0.2, R 0.699: Refused(March(TubeStraddles { verdict: Zero(Classified { margin: 0.0, band: 1e-6 }), boxes: 435 }))` |
| `geom-brep::all pair_section::two_loops_keep_their_sites_and_order_near_the_node` | `d 0.2, R 0.701: … TubeStraddles … boxes: 208` |
| `geom-brep::all pair_section::near_the_node_no_site_reads_on_the_other_sheets_loop` | `2328 sites read, 4 refusals` |
| `sweep::all cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall` | `one loop, ball wider than the wall, A ∪ B: refused GermSectionUnbuilt { Cylinder × Sphere, why: Carry(March(TubeStraddles { Zero, margin 0.0 })) }` |
| `sweep::all cylinder_sphere_frame::every_segment_joins_neighbouring_sites_along_its_loop` | the same refusal, from the matcher |
| `topo ring_path::spline_fuzz::a_path_across_a_carried_loop_reads_the_membership_parity` | `Refused(March(TubeStraddles { … boxes: 435 }))` |

In the nightly itself, `pair_section::wiring::close_rings_carry_counted` and `pair_section::a_seated_loop_is_the_loop_point_for_point` were red with the same refusal; both pass on main `019bed88` at ε 1e-6.

The tube's straddle verdict is a `Zero` on an exact `0.0` margin at 1e-6's band, so the march stops where it carried before. Decide whether the new verdict is the march being honest at that band (pin each row's 1e-6 stop as typed, with the reason) or the ruling sweep handing the tube a pose it should not reach. Precedent: `work/sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot.md` (closed), the same refusal after PR 6011.

**The 1e-12 job.** The same nightly's 1e-12 job was cancelled at its 120-minute limit. Its last minutes show `cylinder_sphere_frame`'s `a_ball_across_a_cylinder_wall_{in_two_loops, in_two_loops_turned, turned_chart, wider_than_the_wall, centre_inside}` each past 960 s, beside `germ_torus_components`' two germ-mint rows, `a_ringed_wall_divided_by_a_carried_chord` and `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`. The nightly before ran the whole 1e-12 job in 106 minutes and none of the cylinder × sphere rows reached its tail. A probe at ε 1e-12 (debug build, 4 cores; each run capped at 400 s):

| row | `08f0298b` (`fc00197c^1`) | `fc00197c` (PR 6518) |
|---|---|---|
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_in_two_loops` | ends in 99 s, red | still running at 400 s |
| `germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint` | still running at 400 s | still running at 400 s |

So PR 6518 turns the cylinder × sphere row from a quick end into a run of many minutes at 1e-12. The nightly before passed it at 1e-12. Its red at `08f0298b` comes from a merge between `50cfcf499d` and `08f0298b`, and is not bisected. The torus germ row's cost is not this PR's: it is as slow at the parent.

## Closed

2026-11-11, by `sectarms/ruling-sweep-straddle-1e-6`. The ruling (b):
the sweep handed the march a seed it should not, so the sweep is fixed
and no row is pinned.

**Mechanism.** PR 6518 stopped seeding the cylinder × sphere march from
the cell subdivision (`exhaust::seed_r3`). It seeds from the ruling
sweep's witnesses instead (`ssi::march_cylinder_sphere`). A closed trace
starts and ends at its seed, and `finish_r3` fits it as an open spline,
so the seed is where the fit's two free ends meet. `cut_loop` witnessed a
loop at the median of its inside points in loop order. For one loop, that
order runs up one sheet from the rise to the fall and back down the other,
so the median sits beside the far branch point. For the whole turn it sits
at `φ ≈ π`, the far ruling. Both are where a loop near the node turns
sharpest. On `one_loop_keeps_its_order_near_the_node` (`d 0.2, R 0.699`) the
seed was `(−0.478, 0.145, 0.115)`, on the far side of the loop. At ε 1e-6
the samples there are 1.7e-2 to 1.9e-2 apart. At every rung down to the
floor (1.07e-5), the chain's boxes 1 and 2 of 435, the second and third
spans from the seam, read graph margin `0`. They are 4.3e-2 by 3.3e-2
across. Box 0 reads 8e-3 and box 3 reads 6e-3. Hence the exact `0.0`:
`zero_free_lower_bound` of a straddling enclosure. At the default ε and
at 1e-12 the steps are 5.6× and 31× shorter, and the end spans stay
graphs.

**Fix.** A loop the ball holds whole is witnessed at its middle: halfway
along its arc by the angle it spans (`arc_components`), or, over the whole
turn, at the first cut, `φ = 0`, where the cylinder × sphere frame points at
the centre and `Δ` peaks. Pinned by
`pair_section::ruling::tests::a_loop_is_witnessed_at_the_near_ruling`,
which fails on the median (`one loop: witnessed at φ 0.29`).

**Verified** (local, debug):

- At ε 1e-6, all six rows listed above pass, and so do the two nightly-only
  rows (`close_rings_carry_counted` and `a_seated_loop_is_the_loop_point_for_point`).
  So does every `cylinder_sphere_frame` row.
- In `geom-brep`, every row matching `pair_section|cylinder|ruling|sphere` passes
  (362, slow set included) at the default ε and at 1e-6.
- At ε 1e-12, `one_loop…` and `two_loops…` pass (368 s and 340 s). Of the
  same filter, 264 rows finished in a 590 s cap, and none failed.
- `close_rings_carry_counted` stays pinned at 1e-6 and still stops there on
  its own text (`boxes: 206`). That row is two rings 3e-4 apart, which no
  seed choice separates (`an-r3-tube-no-rung-makes-a-graph-is-never-refined`,
  where this mechanism is added as a second instance).

**The 1e-12 job is not this PR's to fix, and it is re-homed:**
`cylinder-sphere-frame-rows-take-twenty-minutes-a-boolean-at-1e-12`. The
witness move leaves the 1e-12 cost where it was: `A ∪ B` took 1187 s here
against main's 1220 s, with the same 6368-sample section on both. The
"99 s, red" at `08f0298b` is that tree's runs stopping early on
`ResultInvalid { VolumeUncomputable }`, not a fast pass. The baseline is
the nightly at `50cfcf499d`, where the row took 567 s.


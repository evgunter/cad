# Verification of PR #4123, frozen head 3c17e711

**Verdict: NOT VERIFIED.** There are two blocking points.

1. **MAJOR: the Zone arm reads a sphere face's box tighter than the face, by more than the callers' 12 ε pad.**
2. **The PR merged onto current main turns a `geom-core` row red.**

Every claim in the "Last fix pass" section otherwise holds. All four mutants are red as claimed. Every review finding is fixed, filed or explained. My own random-pose split oracle found 0 wrong answers at all three ε.

Scope:
- **Branch:** `reach/split-gate-sphere-azimuth` did not move past `3c17e711504cf6721d9b92d95825447d366c42d8`.
- **Review read:** `analysis/reach-review/4123`, `review.md` and `probes/`.
- **Lane claims read:** the PR body, through GitHub MCP `get`. I read no PR comments or reviews.
- **Build:** my own target directory, `CARGO_INCREMENTAL=0`.

## Blocking 1: the Zone arm's `√(1 − a²)` (MAJOR, by the brief's bar)

**Where.** `crates/topo/src/boolean/boxes.rs` `sphere_reach`, in the `Zone` arm of `at`. For a face that wraps alone, or whose seam direction is undecided, the reach takes the axis's share of the normal plane as `(T::one() - ax.powi(2)).max(T::zero()).sqrt()`. Here `ax` is the axis's component along the box row, after rounding.

**Why it is unsound.**
- When a row is nearly parallel to the axis, the true share is `p = √(a_y² + a_z²)` (the other two components), and it is small.
- `1 − ax²` loses it to cancellation. The error is about `r·u/(2p)` (`u` the f64 unit roundoff), and it is first order wherever the face's extreme along that row sits on a rim.
- At `p ≲ 1e-8` the computed share is exactly 0, so the box drops `r·p·cos v` entirely.
- Neither the 16-ulp `SPHERE_REACH_ULPS` slack, which is charged in ulps of `|c_k| + r`, nor the callers' `12 ε` pad covers that.
- The `Rect` arm does not have the problem: it reads `p` and `q` from the seam frame's own components.

**Measured.** I used a cap face (one rim, and a meridian strut to the pole; the Zone arm reads it). Its axis is tilted `t` off the world `z` row, and I compared against the exact support (`c + r·sin(v₀ + …)`, closed form per rim) along `z`. These are deficits of `face_box` **with the 12 ε pad its readers pass**:

| ε | r | tilt t | box tighter than the face by (beyond the 12 ε pad) |
|---|---|---|---|
| 1e-9 | 1e3 | 1e-7 | 2.0e-8 to 2.7e-8 m |
| 1e-9 | 1e3 | 1e-8 | **8.0e-6 to 9.8e-6 m** (about 800× the pad) |
| 1e-9 | 1e3 | 1e-9 | 7.9e-7 to 9.7e-7 m |
| 1e-12 | 1 | 1e-8 | 8.0e-9 to 9.8e-9 m |
| 1e-12 | 1e3 | 1e-4 | 9.4e-11 to 1.2e-10 m (a 0.006° tilt) |
| 1e-12 | 1e3 | 1e-3 | 2.5e-12 to 3.0e-12 m |
| 1e-6 | any | any | covered by the pad |

`census::face_reach` and `sphere_reach` read the same windows; they share the arm. At pad 0, random poses (below) show 27 to 44 ulps of `|c| + r` against the claimed 16-ulp outward charge.

**Reach.**
- **Pre-existing, for the gate:** main's `classify::zone_extent` has the identical `√(1 − a²)` spelling.
- **New, through this PR:**
  - `face_box` and the census lane read the whole ball for every sphere face on main; they read this zone now.
  - So do the readers of those boxes: `Separation`, the boolean candidate sweep and `pair_verdict`/`sphere_faces_apart`, and the census proximity backstop.
- **NOTE 3's fix pass claims** "each end is charged 16 ulps … `atan2`/`cos`/`sqrt` leave a few ulps". That is overstated for this term.

**No wrong body produced.**
- I built a valid body: a full-turn ball truncated at `y = −0.3`, `r = 1e3`, posed by `transform_rigid` with tilts of 0, 1e-8, 1e-7 and 1e-3.
- I poked a 3 µm cube 0.5 µm through the sphere face, in the sliver the box misses, clear of the rim and of the cap disk.
- A∪B and A−B answer correctly at every tilt: `point_in_solid` reads the poke region In and Out respectively. The boolean finds the overlap through the cube's corner vertex.
- A∩B refuses identically at every tilt, tilt 0 included ("a sphere face's boundary does not bound a region of its sphere" at the result gate), so that refusal is not this.
- The split gate catches cuts here through the rim edge's own reach. The deficit's extreme always sits on a rim, which is Q3's point.

So the bar the brief sets ("a box tighter than its face … is a MAJOR") is broken at box level, beyond the pad, but I did not turn it into a wrong answer through a public door.

**Fix suggested.** Take the normal-plane share from the frame's own components:
- `√(a_j² + a_k²)` of the mapped axis; or
- the seam frame's `(p, q)` as `Rect` does, with `U` the whole turn whenever the seam is decided.

Add a row with an axis `1e-8` off a box row at `r = 1e3`.

**Repro** (the probe is not committed; this is its core). Build `strut_cap(h0 = −0.3, strut to the south pole)` on a sphere of radius 1e3 with axis `(sin t, 0, cos t)`. Then compare `face_box(body, face, 12 ε, band).max_z` against `r·(sin v₀·cos t + cos v₀·sin t)`.

## Blocking 2: the merge with current main turns `bounds_census` red

- **Main fixed it separately.** Commit 3c17e711 adds a `crates/sweep/src/blend/surgery.rs` `worse` roster line, classified `Payload`. Main's 6014eda8 has since added the same door, classified `Selection`.
- **The merge is textually clean**, so neither GitHub nor `git merge-tree` shows a conflict.
- **The merged tree fails.** It holds both lines, and `geom-core` `bounds_census::the_roster_names_each_door_once` panics: "the roster lists these doors more than once … [("crates/sweep/src/blend/surgery.rs", "worse")]".
- **What I confirmed:**
  - `every_sole_bracket_bound_door_is_in_the_roster` is red on 3a8aedf4.
  - It is green on main c28379f4 and on the PR head.
- **Fix:** merge main and drop one of the two lines. They disagree on the disposition, so pick one.

## Mutants (each applied by hand in a separate worktree, run, reverted)

| mutant | edit | row | result |
|---|---|---|---|
| fix-pass M1: a strut's level folds | `sphere_chart_trim` wrap branch returns every loop level | `topo boxes::sphere_rect_rows::a_caps_strut_does_not_bound_its_latitude_window` | **red**: `sphere_reach`'s top `0.30000000000000365`, as claimed |
| fix-pass M2: no pole level | the vertex-at-pole push dropped | same row | **red**: `floors = [-1.0000000000000002; 4]`, as claimed |
| PR M1: zone only | `trim.az` forced to `None` | `topo …a_sphere_faces_box_is_its_chart_rectangles_support` | **red** |
|  |  | `sweep reach_split_gate_azimuth::every_cut_clear_of_a_partial_turn_sphere_face_splits` | **red** (2.0 rad cap, s = 1e-3, `CurvedBooleanUnsupported`) |
|  |  | `sweep …every_cut_into…` | green (not claimed) |
| PR M2: window low end +0.05 rad | `Rect` `az.lo + 0.05` | `topo …a_sphere_faces_box…` | **red** |
|  |  | `sweep …every_cut_into_a_partial_turn_sphere_face_refuses_at_the_gate` | **red** (empty half vs 3.9e-8, as claimed) |
|  |  | `sweep …every_cut_clear…` | green (not claimed) |

No claimed kill survived. The PR's "M1 and M2 still red" after the merge is ambiguous between the two pairs, so I ran all four.

**The reviewer's probes**, mounted as their headers say (`topo` lib module; `sweep::all` module):

| probe | ε 1e-9 | ε 1e-6 | ε 1e-12 |
|---|---|---|---|
| `probe_sphere_rect_sweep` | 844 built, 382 rect, 0 under | 646, 283 rect, 0 under | 722, 329 rect, 0 under |
| `probe_strut_cap` | now ball (`[-1, 1]`) on every lane, every sense | same | same |
| `probe_booleans_…` | 900 ok / 150 `Join`, 0 wrong | 600 / 100, 0 wrong | 610 / 195, 0 wrong |
| `probe_sphere_sphere_booleans` | 120 / 60, 0 wrong | 80 / 40, 0 wrong | 92 / 64, 0 wrong |
| `probe_splits_…` | 2878 clear split + 2 `SliverSector`, 2880 into refused at the gate, 0 wrong | 1918 + 2, 1920, 0 wrong | 2208, 2208, 7 stand-down, 0 wrong |

These reproduce the review's figures at 1e-9.

## ε results

- **nextest `-p topo -p sweep` at ε 1e-9:** 4623 run, 4621 passed.
  - The 2 reds (`sweep every_suite_file_is_aggregated` and `topo boxes::tests::every_door_that_reads_a_box_is_inventoried`) come from my mounted probe files.
  - Both pass on a clean tree of the head, so the head is **4623/4623**, as the lane claims.
- **nextest `-p geom-core` at ε 1e-9:** 866/866.
- **The PR's new and changed rows at ε 1e-6 and 1e-12:** all green.
  - Rows covered: `reach_split_gate_azimuth::every_*`, `rect_extent_rows::*`, `sphere_rect_rows::*`, `reach_split_gate_window::*`, `splitting::classify::*` and the `topo` `boxes::*` module; 54 rows, the slow clear-cut row included.
  - The only red at each ε is the same probe-mount artefact.
  - No red needed checking against main. Main's listed reds sit outside these rows, and I did not run the full suites at 1e-6 or 1e-12.
- **Hosted CI on 3c17e711:**
  - `demos (tour + wild)`, `lint` and the rest are green.
  - `test` and `gate ok` are red.
  - The job log's tail is doctests only, so I did not independently confirm the lane's reading that only main's 1e-6 reds are in it.

## Random poses against my own oracles (0 wrong is the bar)

**Box soundness** (`topo`): random frames (axis, centre, `r` 0.5 to 2 × s, s ∈ {1e-3, 1, 1e3}).
- Families:
  - random chart rectangles: azimuth width 1e-3 to 6.2 rad, latitude windows including either pole, both senses, so rectangle or complement;
  - strut caps: one rim, a strut up or down, short of or onto a pole, every sense pair.
- Lanes: `face_box` (pad 0, world), `census::face_reach` (world) and `face_reach_in`/`sphere_reach` along 5 aims × 3 rows each.
- Oracle: the exact support, either the critical point `c ± r·e` or a boundary circle's closed-form extreme, itself checked against a dense sample (432 checks per ε).

| ε | rect faces (reads) | wrong | strut faces (reads) | wrong (pad 0) | worst |
|---|---|---|---|---|---|
| 1e-9 | 1366 (49 176) | 0 | 3024 (108 864) | 8 | 44 ulps of `|c| + r` |
| 1e-6 | 1084 (39 024) | 0 | 3096 (111 456) | 12 | 44 ulps |
| 1e-12 | 1026 (36 936) | 0 | 2440 (87 840) | 4 | 28 ulps |

- **The strut misses are all Zone reads of caps strutted onto a pole:** the `√(1 − a²)` term of blocking 1. They are under the 12 ε pad in random poses and past it only near alignment (table above).
- **The rectangle arm is sound to 1.2 ulps.**
- **Refusal (ball) rate:** about 50% of rectangle faces (every complement), and about 90% of strut faces (struts short of a pole, and senses the side guard refuses).
- **Coverage gap:** my wrap-alone two-rim zone builder failed `mint_pcurves` (`LoopNotClosed`), so I have no topo-level zone-with-seam family. The full-turn revolve in the end-to-end poke covers that class only through the public doors.

**Splits** (`sweep`): random partial-turn sphere revolves (capped cylinder with a random cap, a ball truncated to a pole at a random height, two-rim zones; Θ 0.25 to 6 rad), at random scale and in a random rigid pose.
- **The cuts:** 10 per body:
  - random directions, at distances 10^-5.5 to 10^-0.5 off the face's least or greatest support;
  - plus crest cuts along the sphere normal at random interior points.
- **Support oracle:** exact.
- **Volume oracle:** a midpoint quadrature over `(y, φ)` of the exact `ρ`-integral of the half-space. Halves are held to `volume_pad + 2e-4·V`.

| ε | bodies (stood down) | cuts | clear split | clear refused at the gate | into refused at the gate | into split | wrong | worst volume error |
|---|---|---|---|---|---|---|---|---|
| 1e-9 | 130 (0) | 1300 | 615 | 0 | 612 | 0 | **0** | 4.2e-6·V |
| 1e-6 | 86 (44, under 10⁴ ε) | 860 | 408 | 2 | 382 | 0 | **0** | 3.2e-6·V |
| 1e-12 | 89 (41, not finished at ×1e3) | 890 | 485 | 0 | 405 | 0 | **0** | 2.8e-6·V |

At 1e-6, one clear cut more refuses `SliverSector`, the class filed in `split-bisector-side-in-band…`. One half's props refused at 1e-9 and one at 1e-6, under `Converged`.

## Claim checks (the "Last fix pass" section)

**Review findings:**

| finding | claim | checked |
|---|---|---|
| MINOR 1 | fixed: rims plus pole vertices only; prose re-scoped | **true**. The row exists and both its mutants red. The reviewer's `probe_strut_cap` now reads the ball on every lane. |
| MINOR 2 | `torn-body-refusal-families…` re-pointed to `sphere_window` | **true** (`:180`, `:191`) |
| MINOR 3 | `separation.rs:45` reworded | **true** |
| NOTE 1 | remainder added to `sphere-operand-box-is-the-whole-ball.md` | **true** (`:61`–`:76`) |
| NOTE 2 | second `SliverSector` witness added | **true** (`:49`–`:55`) |
| NOTE 3 | 16-ulp outward charge per end | **implemented; overstated.** It does not cover `√(1 − a²)` (blocking 1). |
| NOTE 4 | filed `work/boxes/sphere-window-re-reads-the-loop-on-every-reach.md` | **true** |
| Q3 | filed `work/reach/boundary-crossing-cuts-cannot-see-an-over-tight-face-box.md` | **true** |
| style | `SphereWindow { Rect, Zone, Ball }`; `FaceBoxRule::SphereWindow`; `bool_box_sphere_seam_unit` registered | **true** (`boolean/mod.rs` `decision_words`). `work.py lint` is clean. |

**Other claims:**

- **Merges:** both resolutions keep both sides.
  - `.config/nextest.toml`: both slow lists.
  - `quadrature-convergence…`: both witnesses.
- **Port to the linked walk** (main's f2c43f5f, 3d3cb33e, e22e81ef): **meaning kept.**
  - **The panics:** `torn_outer_loop` re-reads every hop through `loop_members_linked` (`proven` half-edge, `linked` edge), then `edge_curve_linked`, `linked_vertex_point` and `proven_half_edge_end`. That is the same named premise panics as main's `sphere_zone_reach`.
  - **The lone-vertex pre-check:** it is now `[BoundaryMember::Isolated]`. It reads the lone vertex's point as a link, so a torn point under an `Empty` loop now panics, where main returned the ball. That is consistent with D2 row 4.
  - **The rows:** the `a_torn_record_under_the_sphere_rect_panics` assertions are green.
- **"No caller reads a sphere face's box as the whole ball":** **true** by grep. The hits are:
  - `ops.rs:1130` (`FaceRow.bbox`) and `:3155`;
  - `mod.rs:4088`;
  - `reduce.rs:335`, `:339`, `:787`, `:1020`;
  - `separation.rs:165`, `:417`;
  - `census.rs:622`, `:2654`, `:4895`;
  - `classify.rs:160`;
  - `pieces.rs:346`;
  - `rest.rs:633` (spheres take `ExtentBall::of_carrier` first).

  Each is overlap or non-overlap, or containment. None assumes the ball, but every one of them assumes the box contains the face, which is what blocking 1 breaks.
- **Inaccurate:** the section header cites review head `08cf6427`; `review.md` reviewed `0c91849a30`. Prose only.
- **Not checked:** the hosted `test` red's composition (above).

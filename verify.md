# Verify PR #3980: frozen head bef41106a9

Branch `reach/rest-mate-intersect-diff`. The head has not moved past `bef41106a99ce3618280108558ebf55591dd3b41`.
The last fix pass is `d4a1e2a0a`, plus merges of main. It was read against the dual review `analysis/reach-dual/3980-r1` and `-r2`, frozen at 5a5c62365b.
Local runs used a private target dir with `CAD_TOLERANCE_EPS`. I changed no code on the PR branch.

**Verdict: VERIFIED.**

## Mutants

Each mutant is the smallest source edit that matches the description. Rows: sweep `all`, filtered to `rest_mate_every_op | full_turn_bore_mate | mate7a | reach_aligned_half_rods | reach_continuation` (54 rows), at ε 1e-9. Every run rebuilt topo.

| # | mutant | edit | result | rows that go red |
|---|---|---|---|---|
| 1 | the exemption answers nothing | `Exempt::Rest(_) => false` | **red** (6) | `full_turn_bore_mate::intersect_and_differences_answer_the_closed_form`, `mate7a_torus_rest::subtract_and_intersect_on_the_torus_rest_fixtures`, `rest_mate_every_op::{a_ball_filling…, a_pebble_buried…, arc_split_bore_intersect_and_differences…, full_turn_bore_intersect_and_differences…}` |
| 2 | the key reads the A face alone | `pairs.iter().any(\|p\| p.0 == fa)` | **red** (1) | `rest_mate_every_op::a_cavity_with_one_sphere_pair_undeclared_keeps_the_sphere_refusal` (hollow ∪ ball builds) |
| 3 | sphere arm `all` → `any` | `.all(rest_pair)` → `.any(rest_pair)` | **red** (1) | `a_cavity_with_one_sphere_pair_undeclared…` |
| 4 | sphere arm operand order swapped | A/B arms of `rest_pair` exchanged | **red** (1) | `a_ball_filling_a_spherical_cavity…` (`SpheresMeet`) |
| 5 | continuations let into the Rest list | drop `.filter(\|d\| d.class == REST)` in `rest_contacts` | **red** (5) | `mate7a_torus_rest::{a_fully_covered…, subtract_and_intersect…, the_admitted_torus_lane…}`, `reach_aligned_half_rods::a_declared_half_rod_stack_keeps…`, `reach_continuation::declared_rounded_continuations…` |

All five are killed. Head itself is green on the same 54 rows.

Note on mutant 2: only the sphere row kills it. On the cylinder section pass, no row tells the A-face key from the exact pair. My two-peg probe (below) refuses at the crossing layer before the exemption, under head and under mutant 2 alike. There is one `Exempt::answers`, so the claim holds as written.

## ε

| ε | suite | result |
|---|---|---|
| 1e-9 | topo + sweep + editor-core, default profile (slow set included) | 6912/6912 pass |
| 1e-6 | same | 6912/6912 pass |
| 1e-12 | same | 6912/6912 pass |

The PR's new and changed rows are inside these runs. Nothing is red, so no comparison against main was needed.

## Claims

1. **The exemption reads only verified Rest. Holds.**
   - `rest_contacts` (`boolean/mod.rs`, at the end of the reduction) is `decls.coincident_faces` filtered to `class == REST`, then to `declared.verified.one_carrier`.
   - It is therefore a subset of the declared and verified pairs. An undeclared pair cannot enter it by construction.
   - Shared-recipe pairs no longer enter: the scan and the section pass take `red.rest_contacts`, not `red.coincident`.
   - Probes, undeclared:
     - X = Big∖S against S itself: all four ops refuse `CurvedPierceUnsupported`.
     - X against S∩Big: all four refuse `UndeclaredCoincidence`.
   - So "they refuse earlier with `CurvedPierceUnsupported`" is true for one spelling. The other refuses earlier with `UndeclaredCoincidence`. Both refuse before the exemption. That is a naming imprecision, not a soundness gap.
   - I could not build an undeclared or shared-recipe pair that reaches the exemption.
2. **One Exempt type. Holds.**
   - `ops::Exempt { Nothing, Declared(&decls), Rest(&[(A,B)]) }` with one `answers`.
   - The crossings path, the section pass, the sphere arm, `section_report` and `section_cert_rows` all go through it.
3. **Sphere arm: every face on the carrier must be a verified Rest against `yf`. Holds.**
   - The two removed guards cannot admit anything new. The new condition (`all` over every face on the carrier, a set that includes `face`) implies the old one (non-empty, `all` over the box-filtered subset).
   - Head's skip set is therefore a subset of the old one. Removing them can only turn a skip into a question, never a question into a skip.
   - I found no fixture where they differ. With the cavity's two faces, every pair overlaps by box. Leaving a diagonal pair undeclared refuses at the crossing layer, as the row's doc says.
4. **The five mutants are killed. Holds** (table above).
5. **New rows. Hold.**
   - `a_cavity_with_one_sphere_pair_undeclared_keeps_the_sphere_refusal` asserts `SpheresMeet` for ∪, ∩ and both ∖, for both off-diagonal omissions.
   - `a_pebble_buried_in_the_collar_beside_the_mate_is_its_own_shell` checks ∩, both ∖ and ∪ in both orders against closed form at two poses. Mutant 1 makes it red, so the row needs the exemption to pass.
   - I did not re-run main for "main refused this". r1 NOTE-2 measured main refusing `FallbackExtentUnsupported`.
6. **Filed items. Hold.**
   - `work/reach/undersized-rest-shaft-union-refuses-volume-backstop-while-intersect-and-differences-build.md` and `work/reach/sphere-section-circle-inside-a-hole-refuses-at-the-plane-arms-edge-boxes.md` both exist, with measurements and causes.
   - `scripts/work.py lint` reports 0 problems.
7. **Slow set is well formed. Holds for this PR.**
   - The count comment is gone. The filter parses (`cargo nextest list --profile ci` runs clean), and its parentheses balance.
   - It has 193 `test(=…)` entries, all distinct.
   - The PR adds exactly the four `rest_mate_every_op` matrix rows, and all four exist.
   - Not this PR's: two entries match no test, `rigid_map_near_eps_plane_nurbs::every_rigid_map_moves_an_edge_certified_near_eps` and `::the_certificate_re_derives_within_rounding_under_the_map`. They are on origin/main identically; the file's tests were renamed (e.g. `…under_a_rotation`). Under the slow set's own rule they now run per-PR.
8. **6912/6912 at three ε. Holds** (table above).

## Review findings against the fix pass

- **Fixed:**
  - r1 MINOR-1 and r2 MINOR-1 (M2 `all`→`any`; M3 and M6 guards): `all` is pinned and the guards are removed.
  - r1 MINOR-2 and r2 M5 (A face alone): pinned, through the sphere row.
  - r1 NOTE-1 (rung 1): excluded by construction.
  - r1 NOTE-2 (the pebble): the row is added.
  - r1 NOTE-4 and r2 NOTE-3 (the census): the count is removed.
  - Stale comments: the no-crossings comment, the `SpheresMeet` doc and the module's Known limitations.
  - r2 Q1: two exemption spellings became one `Exempt`.
  - The local `decls` became `wall_decls_at`.
- **Addressed by doc:** `no_crossings_certificates` now says it runs as the path runs on *undeclared* operands. It still passes no Rest pairs, so it is still not a twin for a declared input. Its doc is now honest about that.
- **Left, minor:**
  - `ball` and `hollow` in `rest_mate_every_op.rs` still re-spell `curved_mergedoor.rs`'s builders.
  - The `Exempt::Rest` lookup is still a linear `contains` (r2 Q7, taste).

## Probes (not committed to the PR)

- Shared recipe: X = `peg_of(1.5,0,1,1) ∖ peg_of(0.5,0,0.5,2)` against S and against S∩Big. Undeclared, all four ops: refusals as in claim 1.
- Two pegs, one shell each: `peg_of(0.5,0,1.1,0.3) ∪ peg_of(0.5,0,1.6,0.3)` in `collar_of(0.5,1.5,0,1,1)`.
  - Only one peg's walls declared: ∩ and both ∖ refuse `CurvedPierceUnsupported`, under head and under mutant 2.
  - All walls declared: ∩ is `Empty`, and collar ∖ pegs = 6.283185307 (2π).

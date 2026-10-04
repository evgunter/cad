# Verify: PR #3977 (reach/check7-interval), last fix pass

Frozen head **94d9f8870d30**. The branch has not moved past it. Review: delta 2 (`analysis/reach-delta2/3977`, frozen 831dcd7dc). Fix brief: `briefs/fix-3977-last.md`.

Method:
- Worktrees: the head; a mutant copy of the head; and main at the head's merge base `f4a478cb3`, used for the cost runs.
- Each worktree had its own target directory.
- Mutants are switched by `DMUT` in one copy of the head. They are delta 2's `mutants.diff`, re-applied by hand where it conflicted, plus `noquadre` and `offgap`, written as below.
- Delta 2's `probe_d2_sweep.rs` and `probe_d2_cost.rs` were mounted in sweep `tests/all.rs`, unshipped.
- "io" counts inside-out passes in delta 2's families. "vr" counts valid upright bodies refused.

## Mutants (topo whole crate, sweep `far_thin_disc_sign` rows, and the d2 families)

| mutant (edit) | row | 1e-9 | 1e-12 | lane claim |
|---|---|---|---|---|
| `oldenc` (`rederive`: centre = world origin, so a straddle reads `Open`, i.e. exempt; planes through `closed_form_of`) | tier3 `a_far_thin_curved_body_is_read_by_its_exact_volume` | **red**: `far true, inside-out: check 7`, `left: Ok(())` | green (the far arm stands down) | holds |
| | tier3 `a_sign_is_read…`, `an_inside_out_slab…`, `check_10_reads…` | red | red | holds |
| | sweep `far_thin_discs_are_read_by_their_exact_volume` | red | red | holds |
| | contact9 `a_pierce…`, `a_vertex_pair…` | green | red | holds |
| | `validate::tests::an_unresolved_sign_refuses_where_an_in_band_one_passes` | **green** | **green** | **does not reproduce** (claimed red) |
| | io in disc + quadrature families | 80 + 0 | 16 + 0 | holds (80 / 16). All families: 204 / 90 |
| `exempt` (`plus_v_round`: the `Unresolved` arm sets `false`) | `validate::tests::an_unresolved_sign…` | red | red | holds. Nothing else is red; io 0 / 0 |
| `fanorigin` (the plane's fan anchored at its carrier origin) | tier3 `a_sign_is_read…`, `an_inside_out_slab…`, `check_10_reads…` | red | red, plus contact9 ×2 | holds. No curved row is red; io 0 |
| `fansecond` (fan anchored at the loop's second point) | `door_backstop_settled_residue` and everything else | green | green | holds |
| `inflane` (`at_infinity_side` with no certifying lane) | tier3 `a_sign_is_read…` | red | red, plus contact9 `a_vertex_pair…` | holds |
| `noquadre` (`rederive_about`: the tight re-run never fires) | sweep `far_tilted_cut_discs_are_read_by_their_exact_volume` | red (10 valid cut discs refused `VolumeSignUnresolved`) | red (2) | holds. topo is all green; io 0 |
| `offgap` (`mass_properties` volume × (1 + 1e-10)) | `door_backstop_settled_residue` | red: `13.50000000135 vs 13.5, the gap 4.56e-12` | red | holds. topo 116 / 117 red |

`offgap` perturbs the measurement read by the row's gap oracle. It does not perturb the boolean's geometry. It shows the oracle is live, but no mutant here builds a geometrically wrong body.

## ε results (head)

- **Battery** (topo, sweep, editor-core, pncad-py, non-ignored, probes excluded): **7144 / 7144 at 1e-9, 1e-6 and 1e-12.** The main run gave 7117. A `not test(d2_)` filter also caught 27 real rows, which were run separately: 25 in sweep and 2 others, all green at each ε.
- **The PR's new and changed rows** are green at all three ε:
  - `far_tilted_cut_discs…`;
  - `validate::tests::an_unresolved_sign…`;
  - `a_settled_in_band_coincidence…`;
  - tier3 `a_far_thin_curved_body…`;
  - `mass_props_are_thread_count_invariant` (digest);
  - `tcost_k3_certificate::*` and `sign_walk_plus_v::*`.
- **No red**, so there was nothing to check against origin/main.
- **`d2_quadrature_kind`:**
  - 1e-9: 48 / 48 upright pass, 48 / 48 inside-out refused `NegativeVolume`;
  - 1e-12: 36 / 36 and 36 / 36.
- **Disc, revolved and planar-fan families, 1–20 km:** io = 0 at 1e-9 and 1e-12. The only vr is 48 at 1e-9, all the sliver fixture's `LaminaWedge` (48 upright + 48 inverted = the lane's 96). The probe stands down at 1e-6.
- **Not run:** Python `unittest`, census and the k-probe sweep (the lane claims 923 / 923).

## Claim checks

| claim | result |
|---|---|
| R1: `cut_face_rounds` takes a centre; the cylinder uses `(origin − c)·A⃗` | **true**, by reading. The components are lifted and dotted in interval arithmetic. |
| R1: NURBS and trimmed lanes carry the net by −c | **true**: the `about` helper on both lifted nets. The pcurve and trim chords stay in uv. |
| R1: Approx takes the NURBS path | **true**: `Surface::spline_chart` maps `Approx(a)` to `a.fit()`. |
| R1: `certify_role` reads in two stages | **true**. `interval(false)` keeps the walk's enclosure less c·A⃗, and `interval(true)` is read only on `Unresolved`. A role decided by the untight read stands; that enclosure is sound. |
| R1: 48/48 + 48/48 at 1e-9, 36/36 + 36/36 at 1e-12; families at 1–20 km | **true** (above) |
| R1: one digest line moved (`sym_thin_strip`, 692 → 700); tcost one-read rows green | **true**. Against `f4a478cb3`, the only diff is that line at each ε. Its verdict stays `REFUSED`, and `num` goes 654 → 662. The tcost rows are green. |
| R2: unit test pins the arm; `exempt` turns it red | **true** |
| R3: the row asserts only what geometry decides; `fansecond` green; `offgap` red | **true** (see the note on `offgap` above). The `corner_of` doc is corrected, and the parked item carries the anchor numbers. |
| R4: faithful `oldenc` turns the tier3 row red at 1e-9 because the inside-out half-disc passes | **true** |
| Mutant table: `oldenc` also turns `validate::tests::an_unresolved_sign…` red | **false** under the faithful mutant. Minor over-claim in the PR body. |
| Battery 7144 / 7144 at three ε | **true** |
| Cost | Re-measured below. The PR body says "not re-timed on this head". |
| Territory: 24 paths; each moved digest explained; behaviour outside check 7 unchanged | **24 paths: true** (`work.py territory --base f4a478cb3`). **Digest: explained.** **Behaviour outside check 7 is not literally unchanged:** the last pass's `PastTarget::interval_volume` (the backstop's interval confirm, `ops.rs` `interval_measure`) passes `tight = true` unconditionally. So a backstop confirm on a body with quadrature faces now re-measures each one about c. It is narrower (the sound direction) and costs one quadrature per face per confirm. The PR body does not name this. No battery row moved. |
| Merge state | origin/main is 35 commits past the merge base. `git merge --no-commit origin/main` **does not conflict** (textual only, not built). |

## Cost: `validate_geometric`, release, median of 200, 3 runs, ms (head / main@f4a478cb3)

| body | head | main | ratio |
|---|---|---|---|
| brick | 0.19–0.22 | 0.14–0.21 | ~1.0–1.5× |
| block, 8-arc bore | 0.86–0.92 | 0.35–0.36 | ~2.5× |
| 4-arc disc at 5 km | 0.39–0.41 | 0.17–0.30 | ~1.4–2.4× |
| ball | 0.24–0.28 | 0.085–0.11 | ~2.5–3× |

These match delta 2's 1.3–2.5× (the ball reads a little higher). The box was otherwise idle (4 cores).

## Verdict: **VERIFIED**

The bar holds:
- no inside-out body passes in any family at 1e-9 or 1e-12;
- no valid in-domain body main accepts is refused beyond the sliver fixture main also refuses.

Every listed mutant does what the lane says, at the rows that matter.

Non-blocking corrections for the PR body:
1. `oldenc` does not turn the unresolved unit test red.
2. The backstop's interval confirm now re-runs quadrature faces about c (`tight = true`). This is a behaviour and cost change outside check 7, and the body does not name it.
3. Cost was re-timed here, as above.

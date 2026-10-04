# Verify: PR #3987 (reach/door-finished-body), last fix pass

Frozen head **149ed1091331b83e0e1f864c6542e2e1a52396e3**. The branch head has not moved past it. Its merge base is current main, c860806e8.

All runs were local, using nextest's `ci` profile unless a row says otherwise. ε is set through `CAD_TOLERANCE_EPS`, which is read at run time (`geom_core::tolerance::ENV_EPS`).

The delta reviewer's probes from `probes/delta-3987/` were mounted as its README says. One adaptation was needed: main renamed `set_face_surface_stranding_for_tests` to `set_face_surface_unvouched_for_tests` with the same signature and body, so the probe calls the new name. Before any mutant, all 17 probes plus the `inside_out_operand` rows were green at 1e-9.

## Mutants (ε 1e-9, applied by hand, then reverted)

| mutant | edit | row | result |
|---|---|---|---|
| MA: no dual orientation read | `reduce::gate_unverdicted_operand`: `inside_out_solids(..).first().filter(\|_\| false)` | `inside_out_operand::an_inside_out_operand_refuses_in_every_op_at_a_dual` | **red** |
| MA | same | `inside_out_operand::the_public_reduction_refuses_an_inside_out_operand_at_a_dual` | **red** |
| MA | same | delta probe `d_inside_out_part_of_a_two_solid_operand_at_a_dual` | **red** |
| MA | same | delta probe `d_public_boolean_reduce_at_a_dual_refuses_the_inside_out_wedge` | **red** |
| MA | same | B's `r2_inside_out_wedge_at_dual` | green. It prints and asserts nothing, so this is not a finding. |
| MC: public reduce takes `&Body` | `boolean_reduce` / `boolean_reduce_declared` params set to `&Body<T>`, `gate_unverdicted_operand` loop removed (it compiles) | `the_public_reduction_refuses_an_inside_out_operand_at_a_dual` | **red** |
| MC | same | doctest `boolean_reduce` (compile_fail) | **red** ("compiled successfully, but it's marked compile_fail") |
| MC | same | doctest `boolean_reduce_declared` (compile_fail) | **red** |
| MC | same | delta probe `d_public_boolean_reduce_…` | **red** |
| MD (added, for MINOR 1): the public doors skip the maximal-faces gate | the six `reduce::gate_maximal_faces(x_operand, ..)?` calls in `boolean/mod.rs` wrapped in `if false { }`; the test-support door is left intact | `m3_pr4_boolean::non_maximal_operand_refuses` | **red** |
| MD | same | `review_f7_pole_r1_probes` p1, p2, p3, p4 | **red** ×4 |
| MD | same | `verbs_f7_r2_probes` (topo) ×3 | **red** ×3 |
| MD | same | `neighbours_across_a_closed_edge::a_disc_on_its_hosts_plane_refuses_as_coplanar_neighbours` | **red** |
| MD | same | `offer_rows::every_offered_tolerance_passes_just_below_it` (holds the `neighbours_kinked_*` public cases) | **red** |

At baseline the four doctests are green: 2 compile_fail and 2 no_run/compile.

## ε results (whole workspace, `cargo nextest run --workspace --profile ci`, probes unmounted)

| ε | result |
|---|---|
| 1e-9 | **11651/11651** |
| 1e-6 | **11651/11651** |
| 1e-12 | **11651/11651** (`contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex` PASS) |

The changed crates' suites at 1e-9 are inside the whole-workspace row. `demos/tour` (outside the workspace) at 1e-9, default profile: **95/95**, including its slow `eps_regression` rows, which run the tour at 1e-6 and 1e-12.

There are no reds, so no comparison against `origin/main` was needed.

## Claim checks

| claim | check | verdict |
|---|---|---|
| **F1**: an inside-out operand at Dual64 refuses `InsideOutOperand` in every op and both orders; MA reddens the row; class probes pass | The row loops over ∖/∩/∪ × both orders and asserts the named operand. MA reddens it and both class probes. All delta probes pass at the head. | **true** |
| **MINOR 1**: public `NonMaximalFaces` / `CoplanarNeighbours` rows restored through `topo::union` and `boolean_reduce`, from the reviewer's three poses; the test-support door stays only for (c) | Mutant MD reddens 11 public rows covering both arms. `test_support::maximal_faces_gate` is called only at `neighbours_across_a_closed_edge.rs` (c). Three in-crate offer cases still call `reduce::gate_maximal_faces` directly: `neighbours_bent_at_the_band`, `neighbours_bent_across_a_circle` and `neighbours_bent_far_origin_at_the_gate`. Each has its stated reason in the PR body ("What stays at the gate"), and two of them predate this PR on main. The false-premise text is gone from the item, `offer_rows` and `neighbours_across_a_closed_edge.rs`. The item is retitled to the curved gate and `curved_face_arm`. | **true** (the one-line summary's "only (c)" is true of the test-support door; in-crate gate-level offer cases also remain, as disclosed) |
| **MINOR 2**: compile_fail doctests and the dual public-reduce row; MC reddens both doctests and the row | Executed: MC reddens both doctests and the row (and the delta probe). | **true**. Rustdoc does not check the `E0308` code on stable, so any compile error satisfies the doctest. A separate signature (or other) compile error would also pass it, but MC as specified is caught. |
| **NOTE 1**: contact9 row green at 1e-12 | PASS in the 1e-12 workspace run. | **true** |
| **NOTE 1**: `edit_refusal_recourse` lists `positive_volume_exact` | Present in the "tier 3, at the boolean door" row (`edit_refusal_recourse.rs` ~:283). The suite is green at all ε. | **true** |
| **NOTE 1**: the `dsc_checks` in-band void row reads the identical `ShellRoleUndecided` finding at the failed root | I diffed the test body against main. Only the extraction path changed: `ChecksError::Product → ProductError::RootInvalid → one SourceFinding` became `ChecksError::Root(Failed{root}) → node_error(root) → BooleanError::ResultInvalid{errors}`. Every finding assertion is unchanged and passes: exactly one `ShellRoleUndecided{Escalated}`, predicate `positive_volume`, the run's band, the margin ≈ −h/(2+4h) to 1e-6, and the rendered ending. The "one source / node == root" attribution assertion is replaced by the failed-root assertion. `ProductError::RootInvalid` is still exercised by `product_gate_attribution.rs` (two rows), `asm_r2b_assembly.rs`, `refusal_concision_at_rest.rs` and `refusal_concision_chains.rs`, all green. | **true** |
| **Main's new door callers** (`cylinder_sphere_frame`, `far_thin_disc_sign`, `pocket_ring_steep_ellipse`, `pocket_wall_crossing_a_side_face`) refuse nothing main built | `finished` panics on refusal, so a green row means its operands finished. `far_thin_disc_sign::tilted_cut` wraps its build in `catch_unwind` and stands down, so I compared every `SKIPPED` line and panic message of those modules (plus `conic_edge_curved_face`'s fixtures) on head and on main c860806e at all three ε. They are identical, modulo source line numbers, with 13/13 passing on both. The 1e-9 / 1e-12 tilted-cut stand-downs are the door's `VolumeUnmeasured` (QuadratureBudget) on both trees, with identical values. | **true** |
| **Battery**: 11651/11651, whole workspace, three ε | Reproduced exactly (table above). | **true** |
| **k-lint**: dev-probe row red on main by the same flag set (100 on both) | Re-run against **current main c860806e** (`delta3987_ksweep.sh`, 1e-6 + 1e-9): head and main both `GATE FAILED`, **104 margins** (rule 1: 82, rule 2: 3, rule 3: 20; 105 FLAG lines). The flag lists are **identical** once line numbers are stripped. Samples: head 5.75 M, main 3.17 M. | **"same flag set" true; "100" is stale.** It is 104 on both against the head's actual base. The PR body and the closed item still say 100, measured against 46ce5d4d4. This is a NOTE, not blocking. |

## Central bar

Nothing the door refused before now passes, at any scalar, as far as these runs reach:
- the dual orientation read (door and public reduction) is pinned by rows that go red under MA and MC;
- the public maximal-faces arms are pinned by public rows that go red under MD;
- the whole workspace is green at all three ε, with no assertion relaxed in the last pass's diffs that I read (`dsc_checks` keeps every finding assertion).

## Verdict

**VERIFIED**

No blocking points. NOTEs for the PR body:
1. The F8/k-lint count is 104 on both trees against the current base, not 100. The flag set is identical.
2. The MINOR 1 summary "test-support door only for (c)" should read alongside the three in-crate gate-level offer cases, which the body already discloses.

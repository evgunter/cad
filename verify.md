# Verify: PR #3978, frozen head 15bf113bab

Verifier lane for REACH, 2026-10-03. The branch head has not moved off `15bf113ba`. The last fix pass is `44b3a0ec9`, whose parent is the review's frozen `65e53562c`. I read both dual reviews (`analysis/reach-dual/3978-r1`, `-r2`). I changed no code on the PR branch and posted nothing.

**Verdict: VERIFIED.** No claim is false. Two points are thinner than the PR text suggests, and neither blocks (notes 1–2).

## Mutants (each applied by hand, run against `section_cert_rows` and `extent_scan_off_face_tangency`, then reverted)

| # | mutant (edit in `section_cert.rs`) | rows red | result |
|---|---|---|---|
| 1a | sphere×plane `at` offset 0.5·zero tangentially | `a_touch_is_the_centre_of_every_loop_its_margin_admits` | red |
| 1b | sphere×plane `at` offset 0.5·zero along the normal | same | red |
| 1c | sphere×plane `at` offset 5·zero tangentially | same | red |
| 2a | sphere pair `at` = `c1 + k·r1` (on carrier 1 only) | the centre row, `…snowman…trimmed_builds`, `…tilted_frame` | red |
| 2b | skew walls `at` = `foot + m·r1` (on wall 1 only) | the centre row, `the_same_tangencies_on_both_faces_refuse`, `…tilted_frame` | red |
| 2c | sphere×cylinder `at` = `cs − perp/e·ρ` (on the sphere only) | the centre row | red |
| 3 | event rule ignores events (`if !evented` → `if true`) | `a_touch_clears_only_out_of_a_face_on_a_silent_pair` | red |
| 4 | evented touch clears (`Err(Tangent)` → `Ok([TouchOut(F)])`) | same | red |
| 5 | placement inverted (`== Out` → `== In`) | the silent-pair row plus 6 sweep rows | red |

All five claimed mutants are red. Mutants 1a and 1b are caught by the exact-pose `< 1e-12` assertion, not by the centroid test: that test's 1 % tolerance (about 4e-7 at ε 1e-9) is far looser than the offset.

## ε runs

| run | 1e-9 | 1e-6 | 1e-12 |
|---|---|---|---|
| the PR's rows (46: `section_cert_rows` + `extent_scan_off_face_tangency` + `x4`) | 46/46 | 46/46 | 46/46 |
| `-p topo -p sweep`, full | 4248/4248 | — | — |
| new rows `…hole_holds_the_touch…` and `…tilted_frame` on `65e53562c` | red | red | red |

The hole and tilted rows are red on the previous head at all three ε and green on the current head, so that claim holds. Every `scripts/gates/*.sh` passes, including `bounds-allowlist` with ops.rs = 20.

## Claim checks

1. **Removing the touch-ball guard: holds.**
   - **"No event" is true where the scan reads it.** `sphere_faces_apart` hard-codes `evented = |_, _| false`. The scan runs only inside `if red.null_pairs.is_empty()` on the no-crossings path (`ops.rs` `through_the_join`), so the reduction found no crossing anywhere.
   - **Only four arms emit a touch:** sphere×plane, sphere×sphere, sphere×cylinder and skew cyl×cyl (`grep -c '\.touch(&'` = 4). Torus and cone tangencies stay R-tan.
   - **Probes trying to break S** (not committed; all six ops × both orders, at ε 1e-9, 1e-6 and 1e-12):
     - *Plate with a hole about the touch.* Hole radius `rh` ∈ {1e-2 … 3e-5}; ball R ∈ {1, 100, 1e4}; ball pushed `s·ε` with s ∈ {−0.5, 0, 0.5, 0.9}, so the crossing loop γ has radius up to 4e-3.
       - Every pose with `rh < γ` refuses in the crossing layer: `Escalated{Coincidence}` or `CurvedBooleanUnsupported`.
       - Every build has `rh > γ`, so the loop really misses the face. All 176 built bodies pass validate, validate_closed and validate_geometric, and their volumes match the closed form to 1e-9.
     - *Hole in a sphere face, a wall, or both faces.* I tried a bore (rod subtracted) and a dent (small ball subtracted at the touch), with rd from 1e-2 to 1e-5. Every construction refuses: `GermFrameUnsupported`, `GermFrameCylinderPinch`, `SpheresMeet`, `Escalated` or `Join`. So the lane's "holed sphere face refuses as an operand" holds, and it holds for cylinder walls too. The both-faces-holed (edge×edge) configuration cannot be built at all.
     - *Polar cap removed by revolution* (cap radius 1e-1 to 1e-3, outer and inner balls): every pose refuses `CurvedPierceUnsupported`.
   - **No counterexample.** I found no pose that builds while γ lies on both faces.
   - **Nested-zero arm.** It now `continue`s where it used to fall through. Nothing follows the `match` in the loop body, so the two are equivalent.
2. **New row and its five mutants: holds** (table above).
3. **Rigid-tilt and holed-plate rows: hold.** Both are red on `65e53562c` at all three ε and green now.
4. **SphereQuestion docs: hold.** `AgainstPlane` and `Apart` now say that an in-band margin refuses. The work item (lines 94–95) and the PR body say the same. No stale "passes where the section certificate certifies" text remains in `crates/`.
5. **Bounds allowlist: holds.** ops.rs is pinned at 20 and the gate passes.

Review findings closed by deletion: R1 MINOR 1, 3 and NOTE 2; R2 MINOR-1, NOTE-1 and NOTE-2 (spread, `zero_bound`, `boundary_clear_of` are gone). R1 MINOR 2 and R2 Q4 are closed by the docs. R2 MINOR-2 and R1 NOTE 1 are closed by the new rows. The style items are closed by `faces()`, `centred_box`, the per-arm `const` names and the updated `Touch::at` doc.

## Notes (not blocking)

1. **S is shown by execution, not by proof.** I did not audit the reduction's edge×face near-contact logic for every kind. The evidence is the probe set above: in every buildable pose, an edge within the band of the other carrier refused. The module doc states S as a premise, which is accurate.
2. **No body-level row reaches an evented touch.** Mutants 3 and 4 go red only on the unit row, which hands `certify` fabricated placements. By the lane's own argument, no buildable silent-pair pose separates the two rules, so this follows from the design rather than from a missing row.


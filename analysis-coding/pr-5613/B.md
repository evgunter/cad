# Review B — PR 5613 (D′2), frozen head ca29c81468

**Verdict: APPROVE-WITH-FIXES**, with one MAJOR that is Ev's to rule on, not the implementer's to fix. No glimpse. CI on this head green (run 39925572244, `gate ok`). Probes pushed to a review branch (`r?_dprime2_probe.rs`).

## MAJOR
**M1 (claim 2): the chart's antipode now sits under every flat downward face, and a nanoradian tilt teleports the copy.** `mate/value.rs` `Chart::of` → `mate.rs` `reference_about`.
- Setup: boss under plate `B`'s bottom face, `Slide(X)=10`, `B` turned by `Spin(Y)=tilt`.
- Measured (execution): tilt 0 or 1e-12 → origin (10,0,−1), x axis (1,0,0); 1e-9 → (−10.000000001, 0, −0.99999999), x axis (−1, 1.2e-16, 1e-9); 1e-6, 1e-3, 0.1 stay near −10.
- A 1e-9 change moves the copy 20 units and turns it half a turn. Nothing refuses: `mate_chart_reference_antipode` returns Zero at 1e-12 and flips just outside its band. Tilts about x continuous; tops and sides fine.
- New: main swung through the target body's own frame. D′2 moves the singularity to the space's −z.
- The PR does what the spec says; needs an [ev] ruling. If it merges first, file an issue with this probe as its row. Sure (measured).

## MINOR
**m1 (claim 1): the retired `frame_at` survives in the tour demo, undisclosed and unfiled.** `demos/tour/src/world.rs` `frame_off` + `moves` re-implement `frame_at` (evaluate, first line-or-conic edge and a vertex, atan2 Euler, `moved_frame`); on main `plane_on`/`axis_on` called `frame_at`. `test_support.rs` `frame_on_in` is a third copy (test-support only). The PR body and the filed viewer item name only the viewer's copy. These write poses, not bundles: the narrow claim holds, "no behaviour under another name" does not. Viewer deviation acceptable and scheduled; add tour and test_support copies to that issue. Sure (inspection).

**m2 (claim 4): `AddConstraint` silently drops a value that reads a named variable, and the round trip loses it.** Unmated copy with `Spin(X)=tilt` + five values; AddConstraint of a rest gives `[mate, Slide(X), Slide(Y), Spin(Z)]`; `tilt` drives nothing; the record lists only the minted mate id. RemoveConstraint writes 0 on `Spin(X)`; `tilt` doesn't come back. The spec puts this drop in the viewer's mate tool; the PR moved it into the kernel door for every API/Python caller, undisclosed. The rule is consistent; missing is disclosure and a record. Sure (execution, `…_rebundle_round_trip`).

**m3 (claim 6): the msolve14 seats that moved no longer touch what they are seated on.** `chain`: bolt 3 at (0,0,4); bolt 2's head spans x∈[4,5], y∈[1,2], so bolt 3 floats ~3 units off it; fixture comment false. `pinned-chain`: bolts measured from world origin, ~5 units from the slab's origin, likely off the slab. Spec allows the move, but the generator was not rewritten (`at(4,1,0)`). Sure for chain; likely for pinned-chain.

## NOTE
- n1 (premise correction): spec test 20 "Spin(Z), Spin(X) and three slides" is impossible; the test uses three spins, correct; fix the spec sentence. Sure.
- n2 (claim 3): bundle rule holds by inspection + unit rows (ZXZ admitted; ZXX, XXY, XYZX, short chain, slide beside a ball refused); interleaved slides consistent; `point_motion` turns about the mated point. Likely.
- n3 (claim 7 holds, execution): main-era die_tool, golden.cad, plate_param, gallery_ring refuse typed with recourse; hand-mutated `spin_about` → "unknown variant", value `mate` → "unknown field". die_tool/plate_param messages typed but hard to read. Sure.
- n4 (claim 6 partial): m10_p_fence, lib_g16, edit_placement_corpus_bits, name_words_corpus changed only in comments and pass; two named moves re-derived analytically; main not built; "ids only" re-pins not re-derived. Likely.
- n5: PR body "Chart::of takes the space frame" — it takes none. Sure.
- n6 (claim 5): flatten's typed refusals look complete by inspection; no probe. Unsure.
- Not run locally: tour, wild, viewer, pncad-py (green on CI).

## Style
Exercised Q1, Q2, Q3, Q5, Q6, Q8 (partial: `mate/pin.rs` whole, `refactor.rs` through `flatten`).
- S1 (Q1 + fix-mints-the-defect, sure): atan2 Euler decomposition 2 copies on main → 4 on this head (viewer `moves_of`, tour `moves`, `test_support::frame_on_in`, fixture); picked-frame rule 3 copies.
- S2 (Q3, sure): the bundle-constructor gate scans only `pin.rs`; the shape reappeared in three other files.
- S3 (Q1, likely): `pin::residual` and `solve::residual` are one meet written twice; `Placed::with_families` and `rebundle` are parallel completers, only rebundle applies `kept`.
- S4 (Q3, sure): test 24 turns B only about z; no downward or tilted-face row.
- S5 (Q2/Q5, sure): msolve14 fixture prose stale; re-pin note silent on the seat now off its head.
- S6 (Q6, sure): the value-drop moved into the kernel door is not in the deviations list.
- S7 (Q2, unsure): `value.rs` module doc never mentions the antipode consequence.

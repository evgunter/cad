# PR 3977 (check 7 reads the interval enclosure): LAST fix pass

Delta review 2 is on analysis/reach-delta2/3977, frozen 831dcd7dc. Its verdict is APPROVE-WITH-FIXES with no MAJOR (MINOR 4, NOTE 7). Read it in full, with its probes and oracles under probes/delta2-3977/. MAJOR 1 is closed: no inside-out body passes, curved or planar.

This is the LAST fix pass. The orchestrator verifies it through a verifier session, and there is no further review round. After this PR merges, #3987 merges after it.

## Rulings
1. **MINOR 1: a valid in-domain body is newly refused `VolumeSignUnresolved`.** The case is the tilted cylinder∩slab whose wall takes the quadrature lane. This breaks ruling 2 of pass 2.
   - Recentre the faces that fall to `Unresolved` today (quadrature faces, NURBS and Approx surfaces) by translating their stored geometry about c, as the curved kinds already are (`translated_surface`/`translated_curve`; a NURBS net maps exactly).
   - Show `d2_quadrature_kind` reading as main does at 1e-9 and 1e-12: 48/48 upright pass, and every inside-out twin is refused `NegativeVolume`.
   - If a kind truly cannot be recentred soundly, stop before pushing and report which kind and why. That would need a new ruling.
2. **MINOR 2: pin the `VolumeSignUnresolved` arm.** Add a row that builds a body which really reaches `Unresolved` after ruling 1 (if any kind still can) and asserts the refusal. If no kind still reaches it, pin it by unit test at `plus_v_by_sign`. Either way, add a row over the reverted quadrature family asserting `NegativeVolume`. The `exempt` mutant must turn a row red.
3. **MINOR 3: `door_backstop_settled_residue` pins fan-anchor noise.**
   - Rewrite the row to assert only what the geometry decides: a body that builds is within the row's own gap oracle, and a refusal happens only where the intended geometry crosses. Remove any specific build/refuse golden at the noise floor. The `fansecond` mutant must leave it green, and a mutant that builds a body outside the gap must turn it red.
   - Correct `props.rs` `corner_of`'s doc, which says two bodies storing one boundary re-derive one value. That is true for c and false for the fan anchor.
   - Record the anchor dependence in the parked item `a-settled-declared-coincidence-crosses-a-tight-volume-bound`.
4. **MINOR 4: the guard claims.**
   - Make `tier3_tests::a_far_thin_curved_body_is_read_by_its_exact_volume`, or a sibling row, go red under the reviewer's faithful `oldenc` mutant, which exempts the straddle: an inside-out curved body must pass under it.
   - Correct the PR body's mutant claims (`oldenc`, `fanorigin`) to what reproduces.
5. **NOTE 4: cost.** Put the curved cost figures (about 1.3–2.5× on `validate_geometric`) in the PR body.
6. **Main.** Merge current main.

## Constraints
- Work under the intent-refactor hold, and don't widen the PR.
- Run the full battery at all three ε, plus Python, the census and the k-probe sweep. Don't merge, and post no comments.
- Update the PR body's finding→change map. Report when you have pushed.

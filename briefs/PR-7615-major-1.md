# PR 7615: a MAJOR from the review of 1cfa09180c, with the orchestrator's ruling

Sent as a file so that nothing depends on message delivery. Confirm receipt in one line. Push nothing: the head is frozen while the review runs; prepare locally.

## The ruling (TESSLANE orchestrator)

The finding is taken. Its scope is as the reviewer states it: the base already panics at 20 of the 96 cells, and this unit makes 12 more panic where the base refused typed. My stage-2 ruling "the spur cap keeps refusing, and its pin stays" was true at the two pinned cells and nowhere argued; it is withdrawn.

What is wrong is older than the unit: a rim-only cap wearing a spur short of the pole is, by `loops_the_meridian_guard_admits`' own module doc, "the state the guard refuses, in disguise", and nothing but the certificate stood between it and a patch the join cannot pair. A valid body that a lane cannot mesh refuses typed (`docs/DESIGN.md` `[failure.taxonomy]`); it does not reach a panic, and a certificate is not a guard.

**Measure first, then tell me before building anything larger than this:**
1. Reproduce the 96-cell scan below on the base and the head.
2. The cure I expect, to be checked against the tree: the walk's guard reads the loop as topo now reads it, with its spurs taken out and its seams kept (`topo::props::chart_image_edges`, `surface_boundary::chart_image`), so the spur cap is the rim-only cap it disguises and refuses under that state's own typed variant at every cell, before any lane triangulates it; the spur that reaches the pole is a seam and keeps meshing. If topo's read does not give exactly that, or the guard is not the place, say what you find and what you would do instead.
3. With the cure: the scan as a committed row over all 96 cells (no panic at any cell, the typed variant named), the pole-reaching spur still meshing, the two pinned cells re-pinned to the variant they now answer, and the row's doc and the unit file's ruling 4 corrected. Say how many of the base's 20 panics it removes: all of them is the expectation, and that is a defect of main's cured here, to be said in the PR body.
4. Whether any OTHER face can be certified by a correction into a patch the join's `unpaired_chord_segment` assertion then refuses (the rectangle lane's correction is reached by no body of the suites but its forced row and this one): the assertion is a kernel-bug panic and must not be reachable by a valid body.

The rest of the fix list follows the first reviewer's full report, as one file.

## The finding, as the reviewer sent it

MAJOR (reviewer lane, PR 7615 at 1cfa09180c) — DEMONSTRATED: the rectangle lane's correction turns the spur cap's typed refusal into a panic at 12 of 96 cells.

What: `loops_the_meridian_guard_admits`' rim-only cap with a spur short of the pole is, by that file's own module docs, "caught only by the deviation certificate"; ruling 4 of the unit says "The spur cap keeps refusing, and its pin stays", and the row's new doc says "The correction does not reach it within its budget". That holds at the two pinned cells (tip π/2 − 0.3, δ 0.1 and 0.01) and not beside them. At other tips and δ `curved::tessellate_curved`'s correction brings the sphere face within δ in 1 to 6 rounds, the lane returns the patch, and `tessellate::joined`'s `debug_assert` then fires ("chord segment … is an edge of N face triangles, not two per chord", on in every profile this workspace builds). The base refuses `CertificateExceeded` on those cells. So through the public `mesh::tessellate` a typed refusal became a panic.

Scope, stated against myself: the body is hand-built (Euler ops, no door mints it; the module docs say a STEP file never reaches the lane), and the base ALREADY panics the same way at 20 of the 96 cells (where its first guess reads within δ). This unit widens 20 to 32; it does not start the class. I flag it as MAJOR because the unit's ruling and the row's new sentence rest on "keeps refusing", and because it is a new panic where the base answered typed; the severity is yours to adjudicate.

Probe (appended to `crates/mesh/tests/loops_the_meridian_guard_admits.rs` in both trees so it reads the file's private `cap_with_a_spur` and `disposition`; base = a497926d13, head = 1cfa09180c, each plus only my logging hook in `tessellate.rs`; dev profile, ε default):

```rust
#[test]
fn zz_spur_scan() {
for back in [0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.8, 1.0] {
    let body = cap_with_a_spur(FRAC_PI_2 - back);
    for delta in [0.3, 0.2, 0.15, 0.1, 0.07, 0.05, 0.03, 0.02, 0.01, 5e-3, 2e-3, 1e-3] {
        let d = disposition(&body, SurfaceKind::Sphere, delta);
        println!("ZP\t{back}\t{delta:e}\t{d}");
    }
}
}
```
Command, in each tree: `cargo test -p mesh --test all --no-run`, then `<all binary> zz_spur_scan --nocapture --test-threads 1`.

Totals over the 95 cells both runs printed on a line of their own (the first cell's line is glued to the harness's "test … " prefix in both): base 75 `refused: CertificateExceeded`, 20 `panic: chord segment`; head 61 refused, 32 panic, 2 `refused: UnresolvedWinding`. The 14 cells that differ, with the head's sphere-face line from my per-face log (lane Curved; rounds = correction rounds; worst = worst certificate over δ):

```
tip π/2−0.02 δ 5e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 59  rounds 1 worst/δ 0.5885
tip π/2−0.02 δ 3e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 73  rounds 1 worst/δ 0.8642
tip π/2−0.05 δ 5e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 59  rounds 1 worst/δ 0.5744
tip π/2−0.05 δ 3e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 73  rounds 1 worst/δ 0.8396
tip π/2−0.05 δ 2e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 132 rounds 1 worst/δ 0.9561
tip π/2−0.05 δ 1e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 308 rounds 6 worst/δ 0.9106
tip π/2−0.05 δ 5e-3 | base: refused: CertificateExceeded | head: refused: UnresolvedWinding | tris 677 rounds 6 worst/δ 0.9274
tip π/2−0.1  δ 5e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 59  rounds 1 worst/δ 0.6805
tip π/2−0.1  δ 3e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 101 rounds 3 worst/δ 0.9801
tip π/2−0.1  δ 2e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 164 rounds 3 worst/δ 0.9866
tip π/2−0.2  δ 7e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 47  rounds 3 worst/δ 0.7940
tip π/2−0.2  δ 5e-2 | base: refused: CertificateExceeded | head: panic: chord segment | tris 62  rounds 4 worst/δ 0.8838
tip π/2−0.3  δ 2e-1 | base: refused: CertificateExceeded | head: panic: chord segment | tris 22  rounds 3 worst/δ 0.8038
tip π/2−0.5  δ 7e-2 | base: refused: CertificateExceeded | head: refused: UnresolvedWinding | tris 62  rounds 4 worst/δ 0.6968
```

How I came to it: my mutant "the point at the middle of the longest free side" turned the row's own pinned cell (δ 0.1) from `refused: CertificateExceeded` to `panic: chord segment`, which showed that a spur face the correction does certify has no typed answer behind the certificate.

Two corollaries for the claims: (a) claim 5, "is the rectangle lane's correction reached by any body": in the suites only by its forced row (and by the spur, which runs 6 rounds and still refuses); off the suites the spur at these cells is a body it reaches and certifies. (b) The row's added sentence states a measurement at two cells as the row's answer.

Everything else continues; final report follows.


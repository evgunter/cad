# PROPS rotation-floor — the diagonal's width floor is documented, not respelled; the composition rider re-homed

**Binding at dispatch** (PROPS program; the item is
`work/props/rotation-about-diagonal-width-floor.md` — read it in full;
an **E rider**: single style review, outside the A/B experiment, no row).
Read `docs/prompts/implementer-discipline.md` in full. Branch
`props/rotation-floor`, cut from `main`; dispatched after
`props/vec3-doors` in the same worktree.

## The ruling (the item's "what a decision owes", answered)

**No respell.** The item's own measurement decides it: retiring
`1 − cos` alone recovers 0 % (the near-unit sum still rounds outward by
an ulp); respelling both `t` and `c` from the half angle recovers ~17 %
at a `RevolvedPoint` start sample and 0 % at its full-period sample; the
irreducible part is the backend's `cos` enclosure at exact angles. A
sixth of the residue at best is not worth moving `f64` bits under every
rotation in the kernel (goldens, k-lint baselines, content keys through
stored frames), and PROPS-1's re-baseline machinery is for respells
that close a defect, not for ones that narrow a floor by a sixth.

**What the unit delivers instead**, so nobody re-measures:

1. A present-tense paragraph at `Mat3::rotation_about`
   (`crates/geom-core/src/linalg/mat.rs:~85-110`, beside the existing
   evaluation-order paragraph): the diagonal entry `t·nᵢ² + c` carries a
   width floor at exact angles that is the SUM of two enclosures —
   `1 − cos`'s (`[0, 4.44e-16]` at `θ = 0`, where the true value is
   zero) and `cos`'s own — and what each respell recovers (the numbers
   above, with the instrument named:
   `crates/geom-core/tests/cert3_evidence.rs`'s `#[ignore]`d rows);
   that `identity_minus_rotation_about` uses the half-angle forms for a
   DIFFERENT reason (the vanishing factor must multiply, not
   difference); and that the floor is the backend's, not a spelling's.
   No history ("was measured on PR 1277"): the invariant and its
   decomposition only.
2. **Re-verify the two numbers before writing them.** Run the
   `cert3_evidence` rows at this head (both lanes where they apply) and
   quote what they print; if the ~17 % / 0 % pair has moved, write the
   measured pair and say so in the body.
3. **The rider re-homed.** `MappedCurve::restrict`'s per-split
   composition through `Affine3::Mul` re-applies the diagonal enclosure
   per split (+3.55e-15 per split on an exact-axis fixture, PR 1277's
   law row). Its fix is composition-side — compose in the PARAMETER
   (restrict the domain, keep one placement) rather than in the
   placement — and belongs to whoever owns `MappedCurve`. File it with
   `python3 scripts/work.py new mapped-curve-restrict-composes-placements-per-split --kind issue`
   (add `--program` only if the owner is obvious from the file's
   header; otherwise `work/issues/`), citing the law row by file:line
   at this head and the composition-side fix by name; the item under
   review then closes with a `## Closed` section that records the
   ruling (no respell; the doc; the rider's new home) so it is not
   re-asked.

## Fence

`crates/geom-core/src/linalg/mat.rs` (doc only — no arithmetic moves;
`cargo test -p geom-core` bit-identical by construction, and the k-lint
gate rows do not move) and the two tracker files. Nothing else.

## Posture

- Red-first: none (a doc unit); say so.
- ε posture: none. No `CI-Config:` trailer; the docs-only tier is
  expected — if the run is code-tier because `mat.rs` changed, that is
  fine and the body says which tier ran.
- Review: single style review, outside the experiment; the reviewer's
  one executable claim is the re-verified pair.
- Landing: item closed as above; the new issue filed; the spec deleted
  at merge with its `## Per-merge deletion` line in
  `docs/DOC-LEDGER.md`; no `Co-Authored-By`.

## Acceptance

The paragraph at `rotation_about` with re-verified numbers; the rider
filed at its composition-side home; the item closed with the ruling;
hosted CI green.

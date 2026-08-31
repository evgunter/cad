# MATE-6 stored symmetric review brief (substitute {R}, {PROBES}, {OTHER})

You are a blinded adversarial REVIEWER (label: {R}) for unit MATE-6
of the S-MATE program (repo evgunter/cad).

UNIT UNDER REVIEW: PR #1420 — "MATE-6" (issue 946: minting moves
into the product gather; the Q1 drift-closure against
ASSEMBLY-DESIGN A3's ratified "Declaration minting" sentence).
FROZEN review head `65fcc134da2fec61b78ed396aa58d673b0eaec14` on
branch `mate/6-mint-to-eval`; review THAT head. Hosted CI run
33441266280 gated it green (24 jobs; point default/default, drawn).

READ FIRST, by path: docs/prompts/reviewer-style-lane.md and
docs/MATE-6-SPEC.md (both on main); ASSEMBLY-DESIGN §A3
("Declaration minting") and §A5 — the ruling's ground; the PR #1420
body in full (its five deviations are audit targets).

CLAIMS TO FALSIFY (execution outweighs inspection):
1. Persistence premise: minted records (`ContactRecords`,
   `MintedDeclaration`) are evaluation-side only — no serde, no
   persist consumer, no schema implication. Verify independently.
2. Mint as pass 4 of `product_recorded`, AFTER the aggregate
   at-rest gate — the PR claims assemble's old refusal PRECEDENCE
   is exactly preserved. Execute a document failing both a mint
   precondition and the gate on both trees and diff which refusal
   surfaces.
3. Mint is now TOTAL over live mates (refusals become MintRefusal
   rows; the walk continues) where old mint returned on the first
   bad mate; `assemble` raises `unminted.first()`. Construct a
   document with MULTIPLE bad mates and diff the raised error
   against main — order and identity must match, or the deviation
   must be argued.
4. The seam: ZERO eval/ lines — `carry_contacts` already re-keys
   through the descendant map. Re-run the red-first ×3-stand row
   against main (24 UndeclaredContact, 0 carried) and confirm the
   quotes; the two-level nesting row's 8-solid/4-declaration claim.
5. Deviations 3+4: the outer-refuted CARRIED declaration lands
   `ContactContradicted` (counter-evidence) and its attribution is
   `Unattributed` (the mate name does not cross the seam) — loud
   either way. Attack: is there any path where a carried-but-wrong
   declaration is now SILENTLY accepted rather than loudly refused?
   That is the unit's one silent-wrongness shape.
6. The record-not-raise design point: the gather RECORDS mint
   refusals so `product()` (and the viewer) still renders while a
   mate is broken. Attack: does any consumer that previously
   refused on a bad mate now silently proceed? `checks.rs::
   declared_pairs` (the disclosed out-of-fence fix — a would-be
   double-mint) claims byte-identical output; execute it.
7. Bit-identity: no-mates documents gather bit-identically vs
   main; the MATE-1 probe rows untouched and green.
8. The re-baselined `asm_r2b_assembly::row2_a` (patches 0 → 1; the
   old "the gather itself mints nothing" premise is the exact
   claim the ruling reverses): is the re-bless right, and do any
   SIBLING rows still assert the old premise? Sweep the test tree
   for gather-mints-nothing assumptions.
9. The pncad `NOT_CARRIED` list widening (MintRefusal, 101→102):
   verify no Python-visible door/type/tag moved.
10. Sweep honesty: "exactly one mint call site and one
    declared-gate call site tree-wide" — verify the grep; spot-check
    two dispositions; are the stated blind spots real?

METHOD AND RULES:
- Own worktree, own default target/; sibling lanes share the
  machine; foreground, one at a time; never end a turn with
  background work active. Work only inside your worktree — the
  shared session scratchpad is OFF-LIMITS.
- Commit probes to branch {PROBES} and push. Do NOT push to
  mate/6-mint-to-eval, no PR comments, no PRs.
- ISOLATION: until your report is delivered, do not fetch, read,
  or check out the other review lane's branches or artifacts
  (anything named like {OTHER}), and do not read mate/ab-state or
  any MATE-AB-STATE file. Disclose any accidental glimpse.
- BLINDING: never speculate about the implementing model; no model
  names anywhere.
- Structural lane: duplication; rows that red only at a chosen
  fixture; invalidated premises; comment truth.

REPORT (final message, ≤150 lines): verdict, findings MAJ/MIN/NOTE
each with demonstration, silent-deviation count, rubric triple
idiom/tests/docs (1–5), claims-to-falsify outcomes one line each,
probe branch contents, isolation/blinding disclosure.

---
id: rounded-plate-bulges-are-asserted-by-nothing
kind: issue
title: The conic corpus's rounded plate can lose both its bulges and no row reds
status: open
opened: 2026-09-20
---


## Finding

- **Where**: `crates/sweep/tests/common/operands.rs`'s `rounded_plate`,
  read by `s16_box_soundness::conic_corpus` and
  `n3r1_prune::corpus`.
- **Importance**: low-medium — one fixture, but its defining feature
  is the thing no row measures
- **Confidence**: sure; measured by planted mutation on
  `dup/one-line-fixture-wrappers` at merge base `b29fe8bd1`
- **Raised by**: the `dup/one-line-fixture-wrappers` lane, 2026-09-20,
  proving its own fold live

The fixture's reason to exist is its bulges: its doc says the plate has
*"bulge arcs on two sides, so its extruded walls carry rims whose
`u_ref` the sweep mints rotated"*. **Setting both bulge parameters to
`0.0` — an all-line hexagonal plate, no arc, no rotated `u_ref` —
reds nothing.** Both consuming suites stay green:
`s16_box_soundness` 7/7, `n3r1_prune` 1/1 (the filtered run is
`cargo test -p sweep --test all -- n3r1_prune s16_box_soundness`,
8 passed 0 failed with the plant in).

The plate is not inert in every direction. **Doubling it in `x`** reds
`n3r1_prune::n3r1_prune_corpus_examines_98_pairs_and_loses_no_accepted_one`,
whose hard count moves — so the corpus counts the pairs the rounded
plate contributes, and that is all it does with them. In
`s16_box_soundness` **neither** plant reds anything: the size plant
leaves `conic_pruning_never_loses_an_accepted_pair` green, because
that row is one-sided (it asserts no ACCEPTED pair is lost, and a
fixture that is examined more is not a loss).

So the coverage hole is specific and worth stating as two claims:

- the plate's ROUNDNESS is asserted by nothing in either suite;
- the plate's presence in `s16_box_soundness` is asserted by nothing
  at all — no plant on it reds an `s16` row.

**Why it sits here and not on S-DUP's slate.** It is a coverage defect
— a row that cannot go red — which is S-TINT's charter and explicitly
not S-DUP's (`work/dup/plan.md`: *"a suite with no assertions is a
coverage defect, and a fixture built from scratch in six crates is a
vocabulary defect"*). The fold that found it is S-DUP's; the finding
is not. `scripts/work.py territory` puts both consuming files on
S-TCOST's and S-TINT's ground, and this is the S-TINT half.

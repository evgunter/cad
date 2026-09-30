---
id: mate6r1-shared-has-eleven-tests-and-no-assertions
kind: issue
title: Three mate/fix suites print unasserted probe answers; mate6r1_shared's eleven assert nothing at all and its in_part named the wrong node for its whole life
status: open
opened: 2026-09-15
priority: P3
cost: E
---


Found by SUITE's `editor-core-suites-carry-eleven-part-resolver-stubs`
migration. `crates/editor-core/tests/mate6r1_shared.rs` has eleven
`#[test]` functions and **zero `assert` of any kind**; every row ends
in a `println!("R1-PROBE …")`. `work/tint/plan.md`'s ratified ground
already says *"an assertion-free test never gates"*, and this suite is
the receipt: it ran green on every PR for its whole life while eight of
its eleven printed lines were reporting a refusal it never meant to
provoke.

Its private `in_part` named `RecipeNodeId(1)` — the PROFILE node of a
part document `fixture::on_frame` builds as frame(0), profile(1),
extrude(2) — where every sibling suite names the extrude as
`PART_BODY = RecipeNodeId(2)`. So its "good" member names were as
unresolvable as the `dangling` ones the suite mints on purpose, and the
`dangling` doc comment asserted the opposite in prose (*"the part's
node 1 has caps"*). The migration to `fixture::resolver::in_part`
changes eight of the fourteen printed probe lines, e.g.

- `r1_true_carried_declaration_at_both_doors`: `Other(assembly: … does
  not name a face of the product …)` -> `Ok(minted=0)`
- `r1_three_stands_exact_counts`: `patches=0` -> `patches=3`
- `r1_mint_refusal_precedes_the_census`: `Reference(mate=2, side=A)` ->
  `NoAtRestRecord(mate=2, class=Tangent)`

The migration fixes the NAME. It does not fix the suite: the eleven
rows still assert nothing, so the printed answers are unguarded and the
next drift is invisible the same way. What this row owes is a decision
per row — an assertion over the printed value, or retirement naming the
row that now owns the claim (S-TCOST's keep-out).

## The class, not the instance

Three suites ride the shared `PART_BODY` with nothing that can see it
move. Under a planted mutation (`PART_BODY` = `RecipeNodeId(3)`) on the
tree BEFORE the shared store's guard landed, each went **zero rows red**:

- `mate6r1_shared` — 0 of 11 (no assertions at all)
- `mate1_r1_probes` — 0 of 8
- `rev_fix_xsplit_unreachable` — 0 of 6 `in_part` call sites, imported
  at `:26`, no row red

`crates/editor-core/tests/mate6r2_probes.rs` is `mate6r1_shared`'s twin
and belongs to the same decision: eight tests, **four of which assert
nothing**, a header that says the differential property "is spent", and
one row red under the mutation. Same class, same directory, found by
the same method.

`PartStore::insert` now refuses a document whose body is not at
`PART_BODY`, so the mutation above goes red in 224 rows rather than
none — but that guard checks the CONSTANT against the builders, not
these suites' printed answers against anything. The rows below are
still unguarded.

`crates/editor-core/tests/mate1_r1_probes.rs` carried the same
`RecipeNodeId(1)` spelling and does assert, but only over
`solve_document`, where a member name is an opaque token; its one
resolver-driven observation,
`r1_which_branch_does_the_consistent_loop_row_take`, printed
`Err(… does not name a face of the product …)` on main and prints
`Ok(minted = 2)` after the migration — the question the row's own name
asks, answered for the first time, and also unasserted.

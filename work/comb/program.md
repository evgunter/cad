---
id: comb
kind: program
title: COMB — the roll-ups and the sweeps that go last
status: ready
opened: 2026-09-11
group: infra/sweeps
prefix: comb/
tag: (COMB orchestrator)
ab_band: 4300-4399
paths: []
keep_out: [this program claims NO paths by construction - every row here is workspace-wide and collides with every fence, which is the reason all five L rows were written as not-takeable-while-a-track-is-open in the first place, no row here may be dispatched onto files another program has live units on - the ordering rule is decide-before-you-delete and delete-before-you-polish and this program is the polish, the roll-ups (S35 S11 S19 S43) are PARTITIONED not landed - a member that belongs to a live program is filed there as its own row and struck here, never fixed from this program, L4 and L5 audit other programs' closed records and produce findings and rides-along files - they never edit another program's live item, docs/DESIGN.md and memories/* are Ev's - a sweep that wants a standing sentence relocated proposes it on an [ev] PR, REROUTE merged in on 2026-11-20 (docs/doc-ledger/reroute-merges-into-comb.md) - it held no P0 or P1 row, from REROUTE: this program claims NO paths and inherits SUITE's reason - crates/*/tests/* and crates/test-utils/* are S-TCOST's and S-TINT's in every crate and crates/sweep/src/test_support.rs and crates/topo/src/test_support_impl.rs are BLEND's and TOPO's so the shared homes land there by announced seam, from REROUTE: S-TINT owns the ground and the test-integrity charter and this program owns only the duplication question - a row that cannot go red is S-TINT's and one thing spelled n times is this program's and a row that is both goes to S-TINT first, from REROUTE: demos/tour/* and demos/wild/* are never converted - they are evidence about the public API from an outside consumer's seat and a demo that reaches into a dev-gated test_support stops being evidence about the library (SUITE's ruling 2026-09-15 restating the tree's settled practice), from REROUTE: docs/GUIDE.md and rustdoc examples fall under the same rule for the same reason - they are written from a seat that cannot reach a doc(hidden) door, from REROUTE: a census is not a population until its SCOPE as well as its pattern is re-derived - git grep over every tracked file with no path argument is the only form that makes no path claim]
---

Code-quality's *Last, deliberately* group given a program of its own,
plus the four roll-ups no single track can hold. `L1`–`L5`, `S35`'s 62
live sub-rows, `S11`'s dead machinery, `S19`'s ~260 refusal sites,
`S43`'s remaining idiom, the two comment sweeps and the milestone-naming
pass. Nothing here is `E`, eleven of thirteen rows are `H`, and **none
of it is takeable while a track is open on the files it would touch** —
which is what the group has always said about itself. Charter and unit
order: `work/comb/plan.md`.

## From REROUTE (merged 2026-11-20)

S-REROUTE — route the rest of a homed class onto its door

**Route the rest of a class onto the door it already has.** Cut from
S-DUP on 2026-09-26 (Ev: "you can do such splits without asking"). S-DUP's
census-first method homes a class and then files the class's remainder as
a row; those remainders are the whole of this program. Each row here names
a door or a settled rule that already exists on main, and the work is to
route the remaining members onto it — no door to design, no decision to
take. A row that turns out to need a new door goes by `git mv` to the
program that owns the crate the door would live in (S-DUP, which minted
doors, closed on 2026-09-28).

Why a separate track: S-DUP measured 40.5 against its 30-point budget after
two drain batches, because each drain filed its class's remainder as fast
as it closed its head. A priority-seam cut was degenerate (all but two of
S-DUP's rows are P4), so the cut is along the one line that separates the
work by kind: a row that needs a door (S-DUP) against a row that needs
only routing (here). Method and discipline: S-DUP's method items 1–29,
moved into `work/comb/plan.md` when S-DUP closed, apply unchanged, and
that plan says what differs.

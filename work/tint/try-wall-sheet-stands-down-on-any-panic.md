---
id: try-wall-sheet-stands-down-on-any-panic
kind: issue
title: try_wall_sheet cannot tell an unmintable fixture from a broken builder, so its two rows pass over both
status: open
opened: 2026-09-20
priority: P3
cost: D
---


## Finding

`crates/topo/tests/probe_support/mod.rs`'s `try_wall_sheet` wraps the
cylinder-wall sheet builder in `std::panic::catch_unwind` and answers
`None` on a panic, so a row whose fixture the current ε cannot MINT
stands down instead of reporting. Its two callers —
`r1_mate5_probe::probe1_tilt_lever_omits_the_radius_and_certifies_a_separated_pair`
and
`r2_probes::r2_tilted_disjoint_hairline_pair_certifies_false_positive`
— take a `println!` + `return` on `None`.

**It cannot tell an unmintable fixture from a broken builder**, and
that is measured, not inferred. Planting a wrong descending-rim axis in
the shared builder (`topo::test_support_fixtures`'s
`cyl_wall_sheet_keyed`, the axis reversal removed) reds **22 of the 566
default-lane integration rows**, including seven other rows in
`r1_mate5_probe` and four in `r2_probes` — but these two print *"the
fixture cannot be minted at this ε — standing down"* and report **ok**.
Planting a `try_wall_sheet` that returns `None` unconditionally reds
**zero** rows. So these two are the only two rows in the crate that no
mutation of the builder they are built on can red.

Measured 2026-09-20 on branch `dup/src-cyl-sheet` at merge base
`cd9fdfd6b`.

## Why this is S-TINT's and not S-DUP's

The duplication was real and is fixed: the wrapper was token-identical
in both suites and now has one home. What is left is a row that cannot
go red, which is this program's charter and not that one's.

## What a unit here owes

The stand-down's premise is that the MINT is a bit-lottery in ε, so a
row that loses it is not evidence. That premise is about ONE failure —
`mint_pcurves` refusing to mint exact structure — and `catch_unwind`
catches every failure there is. Either narrow it (a builder that
answers `Result` at the mint, so the wrapper stands down on that
refusal alone and lets every other panic through), or make the
stand-down itself observable (a row that asserts the fixture DID mint
at the default ε, so the silent branch is not the only evidence).
Deciding that the stand-down is right as it stands is also an answer —
but then say what makes these two rows' greens mean anything.

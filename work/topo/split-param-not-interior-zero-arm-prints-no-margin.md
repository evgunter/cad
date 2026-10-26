---
id: split-param-not-interior-zero-arm-prints-no-margin
kind: issue
title: topo: SplitParamNotInterior's Zero arm says the crossing lands on an end, and prints no margin
status: closed
opened: 2026-10-13
priority: P3
cost: E
closed: 2026-10-26
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from its sweep for Zero/collapse payload Displays.)

Since that unit, geom-brep's Zero-arm payloads print their margin and claim no size: whether a reading shows none is the ending's to say, from `geom_core::MarginDiag`'s one classification (D4 ¶1 (i): the margin rides the payload as data). The Displays below are on this program's ground and still read the arm instead.

## What

`crates/topo/src/euler.rs`, the `Self::SplitParamNotInterior { verdict, .. }` arm of the euler error's text: on `Refused::Zero(_)` it reads "a crossing lands on an end of its edge at this tolerance, not strictly inside it", with no margin. The reading's distance from the end is the user's one number here (D4 ¶1 (i)).

## Repair shape

Print `verdict.margin()` in the Zero arm's sentence; leave the ending to `split::split_param_ending`.

## Closed (2026-10-26, easy-fixes lane)

`crates/topo/src/euler.rs` `EulerOpError`'s `SplitParamNotInterior` Zero arm now reads "a crossing is not shown strictly inside its edge (margin {m} m from an end)", with the margin through `verdict.margin().readable()`. The ending is still `split::split_param_ending`. The margin is the metered sub-span from whichever end the gate refused first (`split_edge`), so the sentence says "an end", not "the nearer end". The Negative arm is unchanged. Pinned in `euler::tests::split_param_arms_tell_one_story`, which parses the printed margin and compares it to the payload's. Verified: `cargo nextest run -p topo --lib -E 'test(/…|euler::/)'`: 367 passed.

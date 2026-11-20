---
id: told-is-boxed-per-number-where-boxing-the-large-variants-would-keep-copy
kind: issue
title: Told is boxed per number; boxing the large variants or the error would keep it Copy
status: open
opened: 2026-11-20
priority: P2
cost: M
design: true
---


Found building the told quantity (PR 7159); raised as open by its
review. For whoever specs the notation-taking rendering (unit 2), before
more crates build on the type.

`geom_core::Told` is `Box<…>`, one pointer wide. Held inline it is 56
bytes, which took `profile::PathError` from under 128 bytes to 336 and
tripped `clippy::result_large_err` at 660 sites and
`large_enum_variant` in `profile::LiftOutcome` (the PR's first hosted
run, 44725093100).

The cost of boxing each number: `Told` is `Clone` and not `Copy`, so
every payload type that holds one loses `Copy` too
(`PathNoCornerReason`, `LegFit`, `FilletLegCarrier`), which rippled
through `crates/profile` (closures rebuilt per use in
`arc_fillet::resolve`, `.clone()` in `CornerReason::recourse`, borrowed
matches across 25 suite files); and a refusal allocates once per number
it tells.

The alternative not taken: keep `Told` inline and `Copy`, and box the
large variants' payloads, or the error where it is returned
(`Result<_, Box<PathError<T>>>`). That is one allocation per refusal and
no `Copy` loss, at the price of a pattern change at every site that
destructures a boxed variant, and of deciding it again in each crate
whose error grows. Not measured: how many variants across the remaining
crates would cross the 128-byte line with an inline `Told`.

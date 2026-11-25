---
id: contact-refusal-undeclared-is-raised-by-no-door
kind: issue
title: ContactRefusal::Undeclared is constructed only by a test sample: the undeclared-contact refusal's vocabulary outlived its producer
status: closed
opened: 2026-10-29
priority: P4
cost: C
refs: [5997]
closed: 2026-11-25
---


Found by the D10 exit walk (`work/intent/d10-exit-walk-report.md`, the undeclared-contact refusal).

`topo::ContactRefusal::Undeclared` (`crates/topo/src/contact.rs:183`;
its `Display` at `:231` says "with no declaration behind them") is
constructed only by `test_support_samples.rs:183`. Every other mention
is a match arm: `census.rs:6530`, `census.rs:10191`,
`boolean/mod.rs:7648` and `validate.rs:4096`. Stage 4 E and J (#4496,
#4733) retired the undeclared refusals, and the census now reports the
touch as `UnrecordedContact` rows.

Fix: delete the variant, its `Display` arm, the four match arms and the
sample. Ride it with `boolean-refusals-for-door-glued-pairs-keep-declaration-names`
or `atrest/the-contact-recourse-names-a-class-nobody-can-declare`,
which touch the same file.

## Closed

Closed 2026-11-25 as **fixed**, in the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`), against main at `6003877ad`. `ContactRefusal` has only `Escalated` and `NotCertifiable` (`crates/topo/src/contact.rs:145-160`), and `ContactRefusal::Undeclared` gets no hit in `crates/`. The removal is in PR #6079's fix pass, `7e7627d53` (`docs/dual-review-log/rows/PR-6079.md:10`: "`ContactRefusal::Undeclared` removed") Close it, citing #6079.

---
id: a-retrys-zero-has-no-cross-check-against-the-numeric-channel
kind: issue
title: a retry attempt's zero is never cross-checked against a definite numeric sign in any profile: the contradiction assertion runs the first attempt only
status: open
opened: 2026-09-24
---



## What is missing

The tier has one instrument that catches a bug in its own algebra: the
contradiction ASSERTION in `Decide for Sym<T>` (`sym.rs`, the
`definitely_nonzero` arm). Wherever debug assertions are on — dev, test
and this workspace's release profile — it runs the discharge on every
margin the numeric channel has proved NON-ZERO, and a theorem there is
two channels contradicting each other, so it panics.

SYM-9's retry ladder (`geom_core::SymRetry`) runs its attempts on the
DECISION path only (`discharge_retried`); the assertion runs
`discharge`, the first attempt alone. So a retry attempt's zero is never
set against a definite enclosure in any profile. Nothing wrong is
DECIDED because of it — a definite margin returns before the ladder is
ever consulted, so a retry cannot answer one — but a defect in the
algebra a retry runs (a mask producing a rule set that was never
measured, a ring bound leaking into a later walk) would be caught by
nothing that runs on every margin.

Why the ladder is not simply put in the assertion: the assertion asks a
form of every definite margin, which is 95 % of the early walk's forms
on the M10-3 slab (`sym.rs`, `# Cost`), and almost every one of those
refuses — so the ladder would run on nearly every margin a document
has, instead of on the refusals the decision path has (at the nominal:
0 on the plate, 74 of 2,021 on R2's bracket).

## What would answer it

1. **A sampled cross-check**: the assertion runs the ladder on one
   definite margin in `N` (a deterministic sample by node id, so D9
   holds), at a cost the leaf instrument can state.
2. **A probe row**: the evidence rows drive `SymRetry::kept_atom` over the
   measured documents with the ladder forced into the assertion, once,
   and pin that nothing contradicted — a measurement, not a per-margin
   guard.
3. Or the argument that none is owed: each attempt is the same
   algebra under fewer rules or a wider ring, both of which the first
   attempt's assertion already exercises on its own forms. That is an
   argument and not an execution, and this row exists so it is made on
   purpose.

## Home

`crates/geom-core/src/sym.rs` (`discharge_retried`, the assertion in
`Decide for Sym<T>`). Filed by SYM-9's fix pass (deviation 5 of that
unit), no unit takes it yet.

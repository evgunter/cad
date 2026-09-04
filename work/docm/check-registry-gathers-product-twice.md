---
id: check-registry-gathers-product-twice
kind: issue
title: The check registry derives its own subject, so the viewer gathers the product twice per landing
status: open
opened: 2026-08-29
github: 1181
refs: [1162]
---

## From GitHub issue 1181

Opened 2026-08-29; 0 comments.

Raised by the review of [#1162](https://github.com/evgunter/cad/pull/1162) (findings m14 + S7 + the residue of m9). Not a defect in that PR's behaviour — it is the structural cause behind two of its findings, and the fix is an API-shape decision rather than a mechanical dedup, so it is filed rather than folded in.

## The shape

`editor_core::checks::separation` reaches out and computes its own subject:

```rust
// crates/editor-core/src/checks.rs
let gathered = match product::product_recorded(doc, ev, tol) { … };
```

A resident that derives its own subject cannot be composed with one that derives the same subject differently, and `DocSession::land` now pays for that literally — per landed evaluation:

| # | call site | gather |
|---|---|---|
| 1 | `session.rs:1052` — `product(...)` for `landed_fault` | always |
| 2 | `session.rs:1492` — `at_rest_of` → `assemble` → `product_recorded` | assembly-shaped documents |
| 3 | `session.rs:1064` — `run_checks` → `separation` → `product_recorded` | always |

So two gathers for a part document and three for an assembly, where one would do. Each `product_recorded` also re-runs pass-2 `validate_geometric` over every source whenever the product holds more than one solid (`product.rs`), so this is not a cheap duplicate.

The same decision produced `ChecksError::Product` — a gather refusal became a *registry* refusal, which is what made an empty document sink the whole report (fixed in #1162 by special-casing `NoBodyRoots`, but the arm only exists because the resident owns the gather).

## What makes this more than a dedup

The obvious fix — hand `run_checks` a `&Product<T>` — does not compose, because **`assemble` gathers internally too** and then *consumes* what it gathered:

```rust
// crates/editor-core/src/assembly.rs
let product = product_recorded(doc, evaluation, tol)?;
let Product { body, names, mut contacts, .. } = product;
let minted = mint(doc, evaluation, &names, &mut contacts)?;
```

So sharing one product across all three consumers needs a decision, not a refactor:

- Does `assemble` grow a variant taking a pre-gathered product? That is a second door onto a ratified A5 gate, and "which is canonical" then needs answering.
- Does `Product` become cheap to share (`Arc`), or does the session clone it?
- Does `run_checks` take the product as a parameter — changing a public signature, 5 call sites — or does the registry grow a "subject" concept that residents are handed?

I'd expect the last to be the right shape (`run_checks` computes the product once and passes each resident what it needs), but it is a registry-design question and belongs in a design conversation, not a follow-up commit.

## Also rides here: the measurement (#1162's m9)

Style-lane Q6 says a claim resting on a measurement owes a mechanical guard, a scheduled re-measure, or a written reason it can have neither. `checks.rs` and `product.rs` carry a measured pair (tier-3′ census ~1.1 s vs the whole registry ~28 ms, at 161 solids / 966 faces) with none of the three: no row in `benches/benches/kernel.rs`, no register in `ci.yml` that re-takes it, and `PERF-PLAN.md`'s resurvey covers only claims in that file.

It is deliberately unguarded **for now** because the number moves when this issue is fixed — a doubled gather is inside the 28 ms. Both claim sites were narrowed in #1162 to state only what was actually measured, and the withdrawn sentence ("the gather rather than the pair walk dominates") points here. Whoever fixes the gather should re-take the measurement and then discharge Q6 properly.

## Acceptance

- One gather per landing, for assembly-shaped and part documents alike.
- `ChecksError::Product` either gone or reduced to a genuine registry refusal.
- The perf claim re-measured against the fixed shape and given a Q6 disposition (guard, scheduled register, or a written reason at the claim site).

## Home

The registry, `product.rs` and `DocSession::land` sit in `crates/editor-core/src` outside every open program's territory (M10 owns only the analysis lane and the Dual arms in `product.rs`), so it lands unowned under `work/issues/`.

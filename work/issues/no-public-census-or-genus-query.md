---
id: no-public-census-or-genus-query
kind: issue
title: API gap — no public census/genus query, so the Euler-Poincare identity is hand-written about 13 times
status: open
opened: 2026-08-20
github: 758
refs: [S79]
---

## From GitHub issue 758

Opened 2026-08-20; 0 comments.

Found by a style-lane scan of `demos/` (out of scope for the original SMELL-SCAN per `docs/SMELL-SCAN-2026-08.md` §B). Filed per **Protocol v5 / A1**.

## The gap

A caller wanting *"tell me this body's census and its genus"* has no door. `genus` exists in the tree only inside `#[cfg(test)]` review modules:

- `crates/topo/src/review_m1_pr3.rs:127` (`genus_inputs`) and `:138` (`genus`)
- `crates/topo/src/review_m1_pr4.rs:686` (`genus` as an inherent method on a test-local type)

There is no public query on `Body`. The kernel discusses genus at length — `euler.rs`, `euler_ring.rs`, `euler_kill.rs`, `fixtures.rs` all reason about `h` — and exposes no way to ask for it.

## What that costs

Callers write the five `.count()`s, the `.map(|(_, face)| face.rings.len()).sum()`, and the `s − (v − e + f − r)/2` identity by hand. `rg 'rings\.len\(\)\)\.sum'` over `crates/` + `demos/` returns **13 sites**, in several different return-tuple shapes. Known instances:

| Where | Note |
|---|---|
| `demos/tour/src/main.rs:186-194` | |
| `demos/wild/src/main.rs:175-183` | **byte-identical to the tour's**; its doc-comment calls it *"the tour's own narration identity"* |
| `crates/step-import/tests/common/mod.rs:126` | |
| `crates/topo/tests/review_m3_pr1.rs:64` | |
| `crates/topo/tests/graft_disjoint.rs:147` | |
| `crates/topo/tests/m3_pr3_split.rs:81` | |
| `crates/topo/src/seqgen.rs:242` | |
| `crates/topo/src/fixtures.rs:1006` | |
| `crates/sweep/tests/m6_tube.rs:59` | |

Two demo crates independently re-deriving the same identity is the sharpest evidence: the second author read the first, named it in a comment, and copied it anyway, because there was nothing to call.

## Why it matters beyond tidiness

The identity is a **correctness statement about the topology store** — it is the soundness theorem the Euler operators are checked against. Thirteen hand-written copies means thirteen chances to get `r` (ring count) or the shell/solid term subtly wrong, in exactly the places that are *asserting* the kernel is sound. A wrong copy in a test asserts a wrong thing and passes.

This is a **class, not an instance**: per the scan's C13 lesson, the fix should sweep every `rings.len()).sum()` site, not just the demo ones.

## Not asserted

Whether the door is `Body::census() -> Census`, a `genus()` method, or both is a design question. Note the two existing test implementations differ in shape (free function over an inputs struct vs inherent method on a per-component type), so *which* granularity — whole body, per shell, per connected component — is part of the question rather than an afterthought.

## Home

`work/issues/`: the door would sit on `topo::Body` beside `euler.rs`/`fixtures.rs`/`seqgen.rs`, which no open program's `paths` covers (S-MATE owns `topo/src/census.rs` only, a different census), and the code-quality register parks its `S79` row on this issue precisely because it is kernel API work rather than a track row.

---
id: no-public-census-or-genus-query
kind: unit
title: API gap — no public census/genus query, so the Euler-Poincare identity is hand-written about 13 times
status: dispatched
opened: 2026-08-20
github: 758
refs: [S79]
branch: topo/census-door
pr: 2131
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

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/topo/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Ruling proposed (TOPO, 2026-09-06) — for Ev

Re-measured on main `e6fe16f7`: the identity is hand-written at
**26 sites in 22 files** (`rg 'rings\.len\(\)\)\.sum' crates/ demos/`),
in `topo` (`euler_kill.rs`, `seqgen.rs`, `fixtures.rs`, four `tests/`
files), `sweep`'s tests (ten files), and all three demo crates. Two
already disagree on the shell term: `seqgen.rs`'s `Ledger` counts
SOLIDS as `s` while `review_m1_pr3.rs`'s `genus_inputs` counts SHELLS —
the identity `v − e + f − r = 2(s − h)` wants shells; `S69` (PR 2014,
merged) moved the ledger onto shells. That is the cost the row names,
realised: thirteen chances to get `r` or `s` wrong became one actual
wrong `s` in the instrument that checks the operators.

**The question is the door's shape.** Three viable answers:

- **(A) A typed read in `readback`, whole body.**
  `readback::euler_counts(body) -> EulerCounts { v, e, f, r, s }` with
  `s` = shells, and `EulerCounts::genus(self) -> Result<i64, EulerParityError>`
  — parity failure is a typed refusal (the store is torn), never an
  `assert`. Whole-body only; per-shell or per-component genus needs the
  connectivity walk `review_m1_pr4.rs` does and no caller today asks
  for it. Every hand-written site becomes `euler_counts(body)` plus a
  field read or `.genus()`.
- **(B) `Body::census()` as a method on the arena.** Same content, but
  the name collides with `topo::census`, the tier-3′ coincidence census
  (CURVED's), and a method on `Body` puts a derived read on the store
  where every other derived read lives in `readback`.
- **(C) Per-component from the start.** `review_m1_pr4.rs:686`'s shape
  (a per-component type with an inherent `genus`). Answers a question
  no caller asks yet, at the price of shipping a component walk as
  public API before its consumer exists.

**Recommendation: (A).** It is where the kernel's other typed reads
live (`face_carrier_kind`, `face_pose`, the vertex read), it names the
shell term once so the solids/shells slip cannot recur, it refuses
typed, and it is the smallest surface that retires all 26 sites.
(C) stays available as an addition when a consumer arrives.

**Scope of the unit, if ratified:** the door in `readback.rs` with its
rows (red-first: a torn store's parity refuses typed; the cube, the
box-with-hole and a two-shell body give the expected counts and genus);
`topo`'s own sites converted (`euler_kill.rs`, `fixtures.rs`,
`seqgen.rs` after `S69` lands); `topo/tests/*` and `sweep/tests/*` by
announced seam to S-TCOST (test rows are ordinary tests); the demos'
copies by announced seam to whoever owns `demos/` per `paths`, else in
the same PR with the demo-purpose rule kept (the demo calls the public
door, which is exactly the "real usage" the demos exist to show). The
class receipt: every site converted or listed with its owner.
Code-quality's `S79` parks on this item and closes with it.

## Ruled (2026-09-06, PR 2010)

Ev: "sounds good!" — (A) ratified. The item is now a unit:
`readback::euler_counts(body) -> EulerCounts { v, e, f, r, s }` with
`s` = shells, `EulerCounts::genus(self) -> Result<i64, EulerParityError>`
refusing typed on a parity violation; whole body only. Scope as the
proposal states: the door and its rows in `readback.rs`; `topo`'s own
sites converted; `topo/tests/*` and `sweep/tests/*` by announced seam
to S-TCOST; the demos' copies by announced seam to `demos/`' owner,
else in the same PR under the demo-purpose rule. Branch
`topo/census-door`. Dispatches when a lane frees (two are running);
single style review plus a correctness arm on the door's refusal —
the door is a new public answer, so it draws a dual at block TOPO-B1
slot 2. Code-quality's `S79` closes with it.

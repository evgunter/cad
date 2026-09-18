---
id: sweep-test-support-brick-is-still-a-second-box-construction
kind: issue
title: sweep::test_support::brick still builds the box a second way, and three measured things block the delegation
status: open
opened: 2026-09-18
refs: [brick-has-two-constructions-and-two-homes]
---

## Finding

- **Where**: `crates/sweep/src/test_support.rs` (`brick`, `block`,
  `cube`, over `prism_at` → `prism_on` → the extrude machinery)
  against `crates/topo/src/test_support_fixtures.rs` (`brick`, over
  `prism_ops`).
- **Importance**: medium
- **Confidence**: sure about all three blockers; each is read off a
  manifest, a signature or a committed header, not inferred
- **Raised by**: the link-3 lane (`dup/move-the-fixture-family`),
  2026-09-18, which was dispatched to delegate or delete this door and
  could not

`work/dup/brick-has-two-constructions-and-two-homes.md`'s link 3 says
*"`sweep::test_support::brick` delegates to it or is deleted, and `stl`
and `step-export` follow their `pub use`. No manifest edge is added at
any step."* The move it depended on has landed — the family is
`crates/topo/src/test_support_fixtures.rs`, re-exported as
`topo::test_support` — and the delegation still does not fit. Three
things block it, and none of them is the thing the 2026-09-16
measurement was about (that measurement stands: the two builders make
the same solid).

## 1. The two doors do not have the same domain

`sweep::test_support::brick<T: Decide>(x: (T, T), …)` takes its extents
**at the lane's scalar**; `topo::test_support::brick<T: Decide>(x: (f64,
f64), …)` takes them as `f64` constants and lifts them through
`T::from_f64`. There is no conversion in the `Decide` direction that
delegation could use.

Narrowing `sweep`'s side to `(f64, f64)` is not local to `brick`:
`block` and `cube` in the same file pass `T` extents into it, `cube` is
the door most of `sweep`'s blend suites build on, and at least one call
site passes a genuinely `T`-typed value —
`crates/sweep/tests/m6_surgery_interval.rs`'s `cube(iv(DIE_L), …)`,
where `iv` is `Interval::from_f64`. Widening `topo`'s side to `(T, T)`
is worse: `prism_ops` takes an `f64` profile and a point map, so a
`(T, T)` extent has to be re-expressed as a unit-square profile under a
scaling map, and `x.0 + u·(x.1 − x.0)` is not `x.1` in floating point.

That module's own header states the genericity as a property of the
whole extrusion family (*"All of them are generic in the scalar,
because the `Interval` and `Probe` lanes build the same bodies as the
`f64` one"*), so narrowing three of its members is a change to what
that paragraph says, not only to three signatures.

## 2. `sweep/src` cannot name `topo::test_support` without a manifest change

`crates/sweep/Cargo.toml` carries `topo = { path = "../topo" }` under
`[dependencies]` and `topo = { path = "../topo", features =
["sweep-testing"] }` under `[dev-dependencies]`. `sweep`'s
`test_support` is a **`src/` module**, so a `use topo::test_support::…`
in it needs `topo/test-support` on a *library* edge. Putting it on the
`[dependencies]` line is the exact defect
`scripts/gates/test-features-dev-only.sh` exists to refuse — its header
names that very line as the case it was written for. The only other
spelling is a feature forward (`sweep`'s `test-support` enabling
`topo/test-support`), which is a manifest change the link-3 brief ruled
out and whose standing under that gate needs settling before it is
written.

`stl` and `step-export` are not in that bind — their `pub use
sweep::test_support::brick` lives in `tests/`, so a dev-dependency
feature would do — but they cannot move ahead of `sweep` without
building their boxes from one family and their `cube()` from another.

## 3. The swap re-authors committed bytes

`crates/sweep/src/test_support.rs`'s header says it, under *"Editing a
fixture here re-authors committed bytes"*: `step-export`'s
`examples/export_fixtures` regenerates that crate's committed `.step`
corpus from these builders, and `committed_fixtures_are_byte_golden`
runs on every PR. The two constructions are the same solid but assign
the curve arena's twelve keys to edges in a different order and write
`Intersection`'s `(s1, s2)` the other way round on the four bottom-rim
edges (measured, `brick-has-two-constructions-and-two-homes`,
2026-09-16). So the delegation is a re-baseline of checked-in `.step`
files, which is legitimate under the repo's baseline rule and is work
that has to be budgeted, decided and named — not a side effect of a
one-line body change.

## What would settle it

The cheapest first measurement is whether the swap moves the committed
STEP bytes at all: the arena permutation may or may not reach the
exporter's entity numbering, and nobody has run it. If it does not, 1
and 2 are the whole cost. If it does, the unit is a delegation plus a
re-baseline with its own argument about which body the corpus should
show.

Not blocked on anything: the home it needed exists now.

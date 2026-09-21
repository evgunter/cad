---
id: sweep-test-support-brick-is-still-a-second-box-construction
kind: issue
title: sweep::test_support::brick still builds the box a second way; one measured thing blocks the delegation and two scope it
status: closed
opened: 2026-09-18
refs: [brick-has-two-constructions-and-two-homes]
branch: dup/sweep-brick-delegation
pr: 2877
closed: 2026-09-19
---

## Finding

- **Where**: `crates/sweep/src/test_support.rs` (`brick`, `block`,
  `cube`, which when this row was opened went over `prism_at` →
  `prism_on` → the extrude machinery) against
  `crates/topo/src/test_support_fixtures.rs` (`brick`, over
  `prism_ops`).
- **Importance**: medium
- **Confidence**: sure about §1, which is read off two signatures and
  a call site. §2 was overstated when this row was opened and is
  corrected below, by execution. §3 is a scoping argument, not a
  blocker
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

## 2. `sweep/src` needs a manifest CHANGE — settled 2026-09-19, and it is not a gate problem

`sweep`'s `test_support` is a **`src/` module**, so a
`use topo::test_support::…` in it needs `topo/test-support` on a
*library* edge. Two spellings, and only one of them is the defect:

- **Featuring the `[dependencies]` line** (`topo = { path = "../topo",
  features = ["test-support"] }`) is exactly what
  `scripts/gates/test-features-dev-only.sh` refuses — route R1, the
  live leak it was written for, on that very line.
- **A forward from `sweep`'s own `test-support`**
  (`test-support = […, "topo/test-support"]`) is **not** refused, and
  this row's earlier claim that its standing "needs settling" was
  wrong. The gate skips a forward whose SOURCE feature is test-only, by
  construction: its scan reads
  `if is_test_feature(feature) …: continue` over the `[features]`
  table before it looks at any entry. **The precedent is in the same
  manifest**, in the `[features]` table rather than beside the
  dependency it forwards to: `crates/sweep/Cargo.toml:28` already reads
  `test-support = ["profile/test-support"]`, with a comment saying the
  forward is not optional — `src/test_support.rs` names
  `profile::RawLoop`, so a feature that turns this module on without
  turning that door on does not compile from the crates that name it.

**Measured, not read** (2026-09-19, on `dup/move-the-fixture-family`):
with `"topo/test-support"` appended to that list,
`scripts/gates/test-features-dev-only.sh` passes (26 manifests),
`cargo check -p sweep --lib --features test-support` compiles a
`src/test_support.rs` naming `topo::test_support::arena_counts`, and
`cargo check --workspace --all-targets` is clean. The probe was
reverted; nothing of it is committed.

**So what actually blocks it is the brief's own sentence**, not the
gate: link 3's plan says *"No manifest edge is added at any step."* A
feature appended to an existing forward list is a manifest **change**,
not a new **edge**, so whether that sentence reaches it is a reading
of the brief and a small one — it is the cheapest of the three
blockers by a wide margin and should not be counted alongside 1.

`stl` and `step-export` are not in this bind at all — their `pub use
sweep::test_support::brick` lives in `tests/`, so a dev-dependency
feature would do — but they cannot move ahead of `sweep` without
building their boxes from one family and their `cube()` from another.

## 3. The swap re-authors committed bytes — a SCOPING argument, not a blocker

`crates/sweep/src/test_support.rs`'s header says it, under *"Editing a
fixture here re-authors committed bytes"*: `step-export`'s
`examples/export_fixtures` regenerates that crate's committed `.step`
corpus from these builders, and `committed_fixtures_are_byte_golden`
runs on every PR. The two constructions are the same solid but assign
the curve arena's twelve keys to edges in a different order and write
`Intersection`'s `(s1, s2)` the other way round on the four bottom-rim
edges (measured, `brick-has-two-constructions-and-two-homes`,
2026-09-16). So the delegation is a re-baseline of checked-in `.step`
files.

**That is not a reason not to do it and must not be read as one.**
`docs/prompts/implementer-discipline.md` §3, `crates/sweep/src/test_support.rs`'s
own header and CLAUDE.md all say the same thing: a golden exists to
report what the kernel does, and when it moves the only question is
whether the new bytes are right. What this section buys is **scope** —
the unit is a delegation plus a re-baseline with its own argument about
which body the corpus should show, so it is not the one-line body
change the brief's sentence makes it sound like. Reason 1 is what
carries the deviation on its own.

## Measured 2026-09-19 (branch `dup/sweep-brick-delegation`): the STEP bytes do not move, and §1's premises do not hold

### The STEP corpus: zero bytes move, and the probe was live

`cargo run -p step-export --example export_fixtures` at the merge base
reproduces all **17** committed `.step` files byte for byte. Re-run
with `sweep::test_support::brick` delegating to `topo`'s construction,
all 17 are **still byte-identical**. So the curve-arena permutation and
the swapped `(s1, s2)` never reach the exporter's entity numbering, and
§3 evaporates: the delegation is not a re-baseline.

**A green diff is not evidence unless the probe is live**, so the
delegation was mutated (`+0.001` on the mapped z) and re-run: **5 of
the 17** fixtures moved — `cube`, `die`, `die_pips`, `composed_die`,
`kiss_assembly`. The zero-movement result is therefore about the
exporter, not about a probe that never executed.

This closes the blind spot
`brick-has-two-constructions-and-two-homes`'s measurement disclosed by
name: *"Nothing downstream of the body. No STL, STEP or mesh output was
compared."*

### The two constructions still differ, on this head

Dumped side by side at the unit cube: **identical** arena counts, edge
keys, surface keys and witnesses; **different** curve keys (the same
twelve permuted) and `(s1, s2)` reversed on the four bottom-rim edges —
exactly the 2026-09-16 finding, still true. `topo::mapped_cube` under a
selector map and `topo::brick` are arena-for-arena identical, so the
delegation reproduces `topo`'s body and nothing else.

### §1, premise by premise

- **"At least one call site passes a genuinely `T`-typed value" — false.**
  Type-directed census: narrow `brick`, `block` and `cube` to `f64`
  extents and compile every target at every lane. Result: **zero**
  `E0308` on the default lane, **3** on `interval`
  (`m6_surgery_interval.rs`'s `cube(iv(DIE_L), …)` and
  `review_fillet_e1_probes.rs`'s two `cube(iv(1.0), …)`), **1** on
  `probe` (`review_fillet_e1_probes.rs`'s `cube(Probe(1.0), …)`). Every
  one is an **f64 constant lifted at the call site** — `iv` is
  `Interval::from_f64` — which is precisely what `topo::brick` does
  inside the door. No call site in the tree passes a computed `T`. Two
  of the four were not named by this row; the row's instrument could
  not see them because they are behind `interval` and `probe`, which
  `cargo check --workspace` does not compile.
  The remaining cost is **7** inference sites needing a turbofish or an
  annotation, listed in the PR.
- **"A change to that paragraph too" — false.** The module header
  claims the family is *"generic in the scalar"*, and a door that
  returns `Body<T>` from `f64` extents still is. The same file's
  [`corners`] already takes `f64` pairs at every scalar and states the
  reason: *"a fixture's outline is a set of chosen constants, and a
  chosen constant is an `f64` whatever the lane's arithmetic is."* The
  extents are the outlier in this module, not the convention.
- **The widening objection is against a formulation, not the approach.**
  `x.0 + u·(x.1 − x.0)` is indeed not `x.1` in floating point — but
  `prism_ops` evaluates its map only at the profile's own corners and
  the two z stations, so a **selector** map (`if t == 0.0 { lo } else
  { hi }`) is exact, with no arithmetic at all. Measured: it builds a
  body arena-for-arena identical to `topo::brick`, and it moves none
  of the 17 STEP files. The approach works; it is simply not needed,
  because narrowing is the direction the module's own convention
  already points.

## Settled (2026-09-19, branch `dup/sweep-brick-delegation`)

All three sections are measured and none of them blocks. The remedy is
**narrow `sweep`, then delegate**: `brick`, `block` and `cube` take
`f64` extents like every other constant-taking door in that module,
and `brick` is one line of `topo::test_support::brick`.

- §1 dies on its own premises: no call site passes a computed `T`, and
  the header paragraph it cites claims genericity **in the scalar**,
  which the narrowed doors keep.
- §2 is a manifest change, cleared by Ev, and the gate passes on the
  head that makes it.
- §3 evaporates: the committed STEP bytes do not move.

Cost, measured: **11** call-site edits — 4 that got shorter (the
call-site lift is now the door's job) and 7 turbofishes for `T`
witnesses the extent argument used to supply. The suites are
**8231/8231** on both sides of the diff at default features, 3081 on
`interval` and 2923 on `probe`, and a `+1e-3` mutation on the
delegated extent reds **79** rows in `sweep`, so the green is about
the fold rather than about a fixture nothing executes.

The residue the unit's own structural needle turned up is
`work/dup/private-extruded-box-builders-outside-the-brick-door.md`.

## Closed (2026-09-19, PR #2877)

`brick`, `block` and `cube` are `topo::test_support::brick`. **One
construction of the axis-aligned box in the tree.**

All three sections of this row were stated as costs and **none had ever
been run**:

- **§3** — the delegation moves **zero** of the 17 committed `.step`
  bytes, with the probe proved live (a `+0.001` mutation moves 5).
- **§1's first premise** — *"at least one call site passes a genuinely
  `T`-typed value"*. Not one does. All four are an `f64` constant lifted
  at the call site, which is what the door now does internally, and two
  of the four were invisible to this row because they sit behind
  `interval` and `probe`.
- **§1's second premise** — *"a change to what that paragraph says"*.
  The header claims genericity **in the scalar**, which a door returning
  `Body<T>` from `f64` extents keeps; `corners`, twenty lines below in
  the same file, already takes `f64` pairs at every scalar.
- **§2** was already corrected here on 2026-09-19 and Ev cleared the
  manifest change.

The row's numerical objection to the other direction is **true of the
formulation it names and false of the approach**: `prism_ops` evaluates
its map only at the profile's corners, so a selector map is exact. That
option was built, measured arena-identical, and rejected for a better
reason — it serves zero call sites.

Residue: `work/dup/private-extruded-box-builders-outside-the-brick-door.md`.
The `(s1, s2)` blind spot this row inherited is now measured and lives
on `work/carve/intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself.md`.

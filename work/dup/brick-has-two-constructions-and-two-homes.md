---
id: brick-has-two-constructions-and-two-homes
kind: issue
title: The axis-aligned box is built two ways in two homes; the shared home is topo's, not sweep's
status: closed
opened: 2026-09-16
closed: 2026-09-18
refs: [topo-tests-brick-copies]
---


## Finding

- **Where**: `crates/sweep/src/test_support.rs` (`brick`, over `prism_at`
  → `prism_on(sketch_at(z0), …)`, i.e. the extrude machinery) against
  `crates/topo/tests/common/mod.rs` (`prism_z`/`Prism`, hand-built from
  `mvfs`/`mev`/`mef`).
- **Importance**: medium
- **Confidence**: sure that there are two constructions in two homes;
  **unmeasured** whether the two produce equal bodies, which is what
  decides the remedy
- **Raised by**: the S-DUP orchestrator, 2026-09-16, out of
  `work/dup/topo-tests-brick-copies.md`

Two spellings of "the plainest body in the kernel". `sweep`'s is
reachable from `sweep`, `stl` and `step-export` (both of the latter
`pub use sweep::test_support::brick` from their `tests/common`);
`topo`'s is reachable only from `topo`'s own test binary. Neither can
reach the other today.

## Why the shared home is `topo`'s

`topo`'s `src/test_support_impl.rs` module docs state the crate's three
homes for test vocabulary, and `tests/common/mod.rs`'s header cites
them:

| home | reachable from |
| --- | --- |
| in-crate `mod tests` | private sites only |
| `src/test_support_impl.rs`, gated `feature = "test-support"` | `topo`'s `tests/` **and any crate above `topo`** |
| `tests/common/mod.rs` | `topo`'s test binary only |

`prism_z` and `Prism` sit in the third. Moving them to the second makes
them nameable from `sweep`, `mesh`, `stl`, `step-export` and
`editor-core` through the `topo = { path = "…", features =
["test-support"] }` spelling — **no new manifest edge and no
dev-dependency cycle**, because all of those already depend on `topo`.
Every import `prism_z` needs (`geom`, `geom_brep`, `geom_core`) is
already a normal dependency of `topo`, so the move compiles where it
lands.

The rule is the tree's own: an item lives at the narrowest home all of
its consumers can reach. An axis-aligned box **is** a `topo` value;
building one by extruding a rectangle uses a `sweep` operation to
produce a lower layer's value, which is why the shared home was
reached for uphill and found to need a cycle.

This supersedes option 1 of `work/dup/topo-tests-brick-copies.md` (add
`sweep` to `topo`'s `[dev-dependencies]`): that edge is cargo-legal and
has three precedents in `topo`'s own manifest, but it is unnecessary
and it would make `cargo test -p topo` build the extrude/revolve/blend
stack, which nothing does today.

## What this row owes before it is dispatched

**The measurement, first, not the fix.** Build the same box both ways
at the same `(x, y, z)` and compare: face/edge/vertex counts, arena
order, the keys, and `mass_properties` bits. The answer decides the
remedy and the remedy is different in each direction:

- **Equal** → one fixture. `prism_z`/`Prism`/`brick` move down to
  `topo::test_support`; `sweep::test_support::brick` becomes a
  re-export or a delegation, and `stl`/`step-export` follow it.
- **Not equal** → two legitimately different fixtures that must be
  **named for their construction** rather than both called `brick`.
  `sweep`'s suites keep the extruded one wherever extrusion is the
  thing under test; the rest take whichever the suite actually means.
  Renaming is then the fix, and the duplication claim is retired as
  false.

Do not let the lane choose the shape before the measurement is in.

## Measurement (2026-09-16)

**Not equal — but they differ in exactly one axis, and it is an arena
bookkeeping axis, not a geometry one.** The two bodies are the same
solid, the same topology, the same keys in the same arena order for
every topological arena, and the same `mass_properties` bits. They
differ in (a) which curve-arena slot each edge's carrier landed in, and
(b) the ORDER of `s1`/`s2` inside `EdgeDescription::Intersection` on
four of the twelve edges.

### What was compared, and how

Two throwaway probes, one in each crate's `tests/` tree, both
`include!`ing ONE dump function from outside the worktree so the two
sides could not be dumped by two copies that had drifted:

- `crates/topo/tests/` — ran inside `topo`'s own test binary and called
  the REAL `crate::common::brick` (a one-line wrapper over `prism_z`).
  **Nothing was vendored**: the Euler side is the tree's own
  `tests/common/prism_z`, not a copy of it.
- `crates/sweep/tests/` — named `sweep::test_support::brick` directly,
  at `Tol::witness()`, the tolerance `prism_z` mints internally.

The dump is `format!("{body:#?}")` — `Body`'s DERIVED `Debug`, so it
carries every field of every arena including the `SlotMap` slot
versions, `free_head` and `num_elems`, plus a counts line and the four
`mass_properties` fields as hex bit patterns. Deriving rather than
hand-picking fields is the point: nothing could be left out by the
person choosing what to look at. Both probes and the shared dump are
deleted; this row's file is the only thing this branch touches.

Two boxes, to defeat an accidental agreement at the unit ranges:
`(0,1)×(0,1)×(0,1)` and `(-0.5,2.25)×(1.0,1.5)×(-2.0,0.75)`. **Both
give the identical verdict**, as do both eps rows run (`default` and
`CAD_TOLERANCE_EPS=1e-12`), at `f64`, default features.

### The numbers

Counts, both boxes, both builders: `shells=1 faces=6 loops=6
half_edges=24 edges=12 vertices=8 points=8 curves=12 surfaces=6`.

`mass_properties` bits are equal on the nose. Unit box:
`volume=0x3ff0000000000000 surface_area=0x4018000000000000
volume_pad=0x0 area_pad=0x0`. Off-origin box:
`volume=0x400e400000000000 surface_area=0x4034a00000000000`, pads zero.

Section by section over the derived dump — `solids`, `shells`, `faces`,
`loops`, `half_edges`, `vertices`, `points`, `surfaces`, `pcurves`,
`null_faces`, all seven provenance maps, `point_origins`,
`curve_origins`, `surface_origins`, `surface_field_sources`, `surgery`
— **byte-identical**, keys and arena order included. Face surfaces are
`[7v1, 2v1, 3v1, 4v1, 5v1, 6v1]` in face-arena order on both sides and
`sense: true` on all six of both.

Two sections differ:

- **`edges`** — `he_plus`/`he_minus` identical on all twelve; only the
  `curve: CurveKey` differs. Both arenas hold the SAME SET of curve
  keys (`13v1` plus `1v3…11v3`, 14 slots, 12 live, `free_head: 12`);
  the two constructions assign them to edges in a different order.
  `prism_z` lays them out `[13v1, 1v3, 2v3, 3v3, 4v3, 5v3…11v3]` in
  edge-arena order; `brick` lays them out `[8v3, 9v3, 10v3, 11v3,
  13v1, 5v3, 6v3, 7v3, 1v3, 2v3, 3v3, 4v3]`.
- **`curves`** — per edge, the two carriers are identical field for
  field (`Line` origin, `dir`, `param_start`, `param_end`,
  `Certificate { samples: 9, max_residual: 0.0 }`), the witness point
  is identical, the description arm is `Intersection` on all twelve of
  both, and the authority is `Derived` on all twelve of both. **There
  is no `Scaffold`/`Declared` anywhere in either body** — the axis that
  separated `brick` from `geometric_cube` does NOT separate these two.

### Where they first differ

Edge `1v1` of the edge arena — the first bottom-rim edge. `prism_z`
writes `Intersection { s1: 3v1, s2: 2v1 }`; `brick` writes
`Intersection { s1: 2v1, s2: 3v1 }`. Same pair, swapped. It happens on
edges `1v1`–`4v1`, which are exactly the four bottom-rim edges (bottom
cap `2v1` against a side wall); the four struts and the four top-rim
edges agree on the order.

The rule behind it: `prism_z`'s `describe_as_intersections` sets
`s1 = surface(face(he_plus))`, `s2 = surface(face(he_minus))`, and it
holds on **12/12** edges of the Euler body. On the extruded body it
holds on **8/12** — the four bottom-rim edges carry the pair the other
way round. `he_plus`/`he_minus` are identical between the two bodies,
so this is the extrude machinery naming the pair by something other
than the edge's own direction on the cap it closes.

**Nothing in the tree reads that order.** Every consumer of the pair
compares it unordered and says so in code: `validate.rs`'s
`DescriptionNotAdjacent` check is
`(s1 == fs_plus && s2 == fs_minus) || (s1 == fs_minus && s2 == fs_plus)`,
`boolean/ops.rs`'s staleness test is the same shape, and
`Body::description_surfaces` is consumed only by `contains`, by a
refcount and by a `for` (`splitting/finish.rs`, `validate.rs`,
`body.rs`). `EdgeDescription::Intersection`'s own docs say only "the
first surface" / "the second surface" — there is no stated convention
for the order, so neither builder is violating a ratified rule. That is
a reading of the code, not a measurement.

### What this means for the remedy

This is **one fixture with a cosmetic divergence**, not two legitimately
different fixtures. Nothing a suite can assert on today separates them:
same counts, same keys, same arena order, same descriptions, same
authorities, same surfaces, same senses, same mass-property bits. The
row's "equal" branch is the one that applies, with one carried residue:
whichever spelling survives, the (s1, s2) order on a cap rim is
unpinned, and a body built one way is not `Debug`-equal to a body built
the other. Any future row that compares two bodies by their dumps will
trip over it.

### The set that would have to move

Every item in `crates/topo/tests/common/mod.rs` — the whole file —
names only `geom`, `geom_brep`, `geom_core` and `topo`, all normal
`[dependencies]` of the `topo` **library**. No member reaches
`proptest`, `test-utils`, `strum`, `mesh`, `stl`, `step-export` or
`sweep`, so **nothing in the family is blocked by a dev-only crate**.

The entanglement is at the leaves, not at the cubes. `line` and `plane`
are called by all three builders (`geometric_cube` 7 + 6,
`prism_z` 5 + 3, `cube_into` 7 + 6), and
`describe_as_intersections` by `prism_z` and `cube_into` but **not** by
`geometric_cube` — which is precisely the step whose absence
`geometric_cube`'s rows assert on. So moving `prism_z` alone still
moves `line`, `plane` and `describe_as_intersections` down with it, and
`geometric_cube`/`GeoCube` would then be a `tests/`-only fixture built
out of `topo::test_support` leaves. The honest set is the file:
`line`, `plane`, `describe_as_intersections`, `Prism`, `prism`,
`prism_z`, `brick`, `GeoCube`, `geometric_cube`, `mapped_cube`,
`cube_into`, and the three members the row did not list —
`StraddleSeat`/`straddle_seat` (over `prism_z` +
`graft_disjoint_all_keyed`), `flush_declarations` (over `topo::flush`)
and `assert_every_chord_named_by_both_rules`.

Moving `geometric_cube` and `cube_into` as they stand would relocate
the duplication the row
`topo-tests-geometric-cube-and-cube-into-are-one-sequence-twice`
on S-TINT's slate names, so that row reconciles BEFORE or WITH the
move, not after it.

**Two gates a mover meets at the door**, neither of which the row
anticipated:

1. `scripts/gates/witness-not-ambient.sh` forbids `Tol::witness()`
   anywhere under `crates/*/src`, exempting only `#[cfg(test)]`.
   `test_support_impl.rs` is mounted `#[cfg(any(debug_assertions, test,
   feature = "test-support"))]`, and `gate_test_only_mounts`'
   `GATE_CFG_TEST_NOT_RE` explicitly excludes an `any(...)` cfg from the
   test-only narrowing — so the file IS in the gate's production set.
   Verified by planting one `Tol::witness()` in `test_support_impl.rs`
   and running the gate: it fired, naming the line. The family calls
   `Tol::witness()` **24 times**. A mover either threads `tol: Tol`
   through every one of those doors — which would make `prism_z`'s
   signature match `sweep::test_support::brick`'s, and is arguably the
   right shape — or argues for a new exemption.
2. Workspace clippy has `unwrap_used`/`expect_used`/`panic` at `warn`,
   which the gate runs as `-D warnings`. `tests/common/mod.rs` carries
   a file-level `#![allow(...)]` for all three; in `src/` that allow
   lands inside the library tree.

Neither is a blocker; both are work the "move it down" branch has to
budget for.

### What the probe could not see

- **`f64` only.** Both builders are generic over `Decide`; nothing was
  run at `Dual`, `Interval` or `Probe`. A divergence that only appears
  under interval arithmetic is outside this measurement.
- **Default features, two eps rows** (`default` and `1e-12`), local
  build, `CARGO_TARGET_DIR=/home/user/dup-measure-target`, `Compiling
  topo`/`Compiling sweep` confirmed on both. `1e-6` was not run.
- **Two boxes.** Axis-aligned, four-corner profiles only. Nothing says
  the two agree on a reflex profile — `sweep`'s `prism_on` takes
  `ProfileVertex` with a bulge and `prism_z` takes bare `(f64, f64)`
  corners, so the two builders' domains are not the same set and only
  their overlap was measured.
- **Whether the (s1, s2) order matters** is read, not measured: the
  consumers were inspected by eye and all compare unordered. No
  mutation was planted to prove a swapped pair changes no verdict.
- **Nothing downstream of the body.** No STL, STEP or mesh output was
  compared; the claim is about the arenas and the mass-property
  certificate, not about what an exporter makes of them.

## Adjudicated (2026-09-16): the "equal" branch, and the unit is three

**The measurement above settles the fork this row was opened on.** The
two builders produce the same solid — same counts, same keys, same
arena order in every topological arena, same face surfaces and senses,
same `mass_properties` bits. They differ on two axes and neither is
geometry: the curve arena holds the same key set permuted, and four of
twelve edges carry `Intersection`'s `(s1, s2)` swapped. **Nothing in
the tree reads either.** So this is one fixture, not two, and the
remedy is the move down rather than a rename.

The pair-order asymmetry is a finding about the kernel, not about
fixtures, and it outlives this row: filed as
`work/blend/intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself.md`
on the ground it lands on.

**But the move is three units, not one**, and the measurement is what
shows that. The row imagined moving `prism_z`/`Prism`; the honest set
is the whole of `crates/topo/tests/common/mod.rs`, and two gates sit in
front of it that this row did not anticipate — both of which the lane
**measured rather than guessed**, one by planting a violation and
watching the gate fire.

1. **Reconcile first.** `geometric_cube` and `cube_into` are the same
   ninety-line Euler sequence written twice
   (`work/tint/topo-tests-geometric-cube-and-cube-into-are-one-sequence-twice.md`).
   `line`, `plane` and `describe_as_intersections` are called by all
   three builders, so moving `prism_z` alone already drags them down
   and leaves the duplication behind at a second address. **Relocating
   a duplication is not a move, it is a second copy with a forwarding
   address.** This reconciles before or with the move.
1b. **Unify `prism_z` and `cube_ops`** — inserted 2026-09-16 on link
   1's full review, which hand-traced the two at `n = 4` and found
   them one function: same `mvfs`, same `MevSite::Lone`, same `Fan`
   chain, same reversed bottom corner list, same strut anchors
   including the `f_bottom.he_plus` special case at `i == n-1`, same
   four side planes, same `first_side_he_plus` closing.
   **`prism_z`'s own doc has said so since `0765b4617`** — *"the
   geometric_cube construction generalized to N corners"* — and
   `cube_doors_agree.rs` now proves it by execution at four boxes.
   Link 1's stated reason for keeping two cores does not survive:
   *"`prism_z` can neither write into an existing body nor take a tilt
   map"* describes a signature link 1 had just changed on the other
   function, and three of the four direct `cube_into` call sites take
   axis-aligned affine maps rather than tilts. **This runs before link
   2**, so that the tolerance is threaded through one builder family
   rather than two.

2. **Thread the tolerance.** `scripts/gates/witness-not-ambient.sh`
   forbids `Tol::witness()` under `crates/*/src`, exempting only
   `#[cfg(test)]` — and `gate_test_only_mounts`' `GATE_CFG_TEST_NOT_RE`
   explicitly excludes an `any(...)` cfg from that narrowing, so
   `test_support_impl.rs`'s
   `#[cfg(any(debug_assertions, test, feature = "test-support"))]`
   mount puts it squarely in the gate's production set. The family
   calls `Tol::witness()` **24 times**. A mover threads `tol: Tol`
   through every one of those doors or argues for a new exemption.
   **Thread it**: it is right on its own merits — a fixture should not
   reach for an ambient tolerance, which is what the gate exists to
   say — and it lands `prism_z`'s signature on
   `sweep::test_support::brick`'s, which is convergent evidence that
   the two really are one door. Also in this unit: the file-level
   `#![allow(unwrap_used, expect_used, panic)]` that is unremarkable in
   `tests/` lands inside the library tree on the move.
   A fifth member of the family that neither this row nor the
   measurement listed surfaced during link 1:
   `crates/topo/src/cert_m3r1_probes.rs` holds a **verbatim in-src copy
   of `GeoCube`, `line`, `plane`, `geometric_cube` and
   `describe_as_intersections`** — its own doc says so twice, and it
   cannot be fixed from `tests/` because nothing in `src/` can name
   `tests/common` and its header explains that it must be in-crate
   (`Body::surfaces` is `pub(crate)`). **It unblocks with this link
   exactly**, which is the strongest argument yet that the move is
   worth making: `work/dup/topo-src-cert-m3r1-probes-holds-a-fifth-copy-of-the-cube-family.md`.
   Carry also link 1's own residue — `cube_ops` is generic in the
   scalar while `cube_into` stays `f64`-only so that no call site had
   to annotate, and that asymmetry is worth revisiting once the family
   has a home in `src/`.

   **Named for link 3, so they are scheduled rather than disclosed**
   (link 1b's review, Q6): three in-`src` census candidates nobody has
   opened — `crates/topo/src/review_m1_pr2/cube_independent.rs`,
   `.../atomicity.rs`, and `crates/topo/src/review_m1_pr3.rs`'s
   `build_box`. The census that found them counts call sites rather than
   operators, so it undercounts any builder that loops and cannot settle
   their membership either way; someone has to read them, and link 3 is
   the unit standing in that code. Also for link 3 to decide:
   `crates/topo/tests/fixture/mod.rs` is a **second** `tests/`-only
   vocabulary home in the same binary, 393 lines, with no stated
   boundary against `tests/common/mod.rs` — the move has to say which of
   the two it is moving.

3. **Move, then unify.** The family goes to
   `crates/topo/src/test_support_impl.rs`; `sweep::test_support::brick`
   delegates to it or is deleted, and `stl` and `step-export` follow
   their `pub use`. No manifest edge is added at any step — every
   consumer already depends on `topo`.

Each link stands on its own merits whether or not the next one
happens, which is the test that the decomposition is real rather than
a way of making a large unit look small.

**Sequencing.** Ev ratified "A now, B next unit" on 2026-09-16, before
the measurement existed. B is now the third link of three; the first
link is an existing row on S-TINT's slate. Taken as a sequencing
decision with a recommendation rather than put back to Ev, per
`memories/orchestration-model.md`, and reported to him in the same
sitting.

## Link 2 is done (2026-09-18, branch `dup/thread-the-tol`) — what link 3 now faces

**The tolerance is threaded.** Every member of
`crates/topo/tests/common/mod.rs` that minted a witness now takes
`tol: Tol` as its LAST parameter, and every call site in
`crates/topo/tests/` passes `Tol::witness()`. `prism_z`'s signature is
now `(profile, z0, z1, tol)` and `brick`'s `(x, y, z, tol)` — which is
`sweep::test_support::brick`'s, as this row predicted.

**The "24 times" above is wrong in a way worth naming**, because it is
the kind of error this program exists to catch: it is one file read at
two commits, not a family total against a file subtotal.
`crates/topo/tests/common/mod.rs` held 24 `Tol::witness()` calls at
`01ca2ead7` (before S-DUP touched it), 17 after link 1 (`6b092272a`) and
**10** after link 1b (`244a6bb83`, PR #2812). The family-wide number at
`bcc2e6c6f` was **10, all in that one file** — no member of the family
is defined anywhere else, by a `git grep` with no path argument for each
member's definition.

**Gate item 2 (the `#![allow]`) is decided, and the tree had already
decided it twice.** Two files hold fixture vocabulary inside `src/` and
both carry
`#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` at
file scope: `crates/sweep/src/test_support.rs:110` and
`crates/topo/src/fixtures.rs:44`, the latter with the argument written
out — *"Test-support code: panicking is a test's failure mechanism (L5),
and fixture unwraps are on keys the fixture itself just minted."* So the
allow travels with the family, carrying that comment.

**But not into `test_support_impl.rs` itself.** An inner `#![allow]` in a
module file scopes to that module, and `test_support_impl.rs`'s existing
resident `ArenaCounts` compiles clean without it and is the one item
there with a non-test consumer (the D1 debug postcondition). The family
should land in a sibling module file re-exported through `test_support`,
so the allow stays exactly as wide as the code that earns it. That is a
recommendation to link 3, not a decision taken for it.

`dead_code` and `unreachable_pub` need nothing new: `fixtures.rs` already
handles the first per-item (*"key bundles expose every minted key; tests
pick what they need"*) — the shape `PrismOps`, `Prism` and `GeoCube` will
want — and `test_support_impl.rs:110` already handles the second.

**One correction to the fifth-member note above.**
`crates/topo/src/cert_m3r1_probes.rs` is mounted `#[cfg(test)] mod
cert_m3r1_probes;` (`crates/topo/src/lib.rs:169`), so
`witness-not-ambient.sh` does **not** reach it —
`gate_filter_test_only_paths` takes a `#[cfg(test)] mod x;` module out of
the scan set entirely. Its 12 `Tol::witness()` calls are therefore
legal where they sit, and the gate is not what unblocks that copy.
What unblocks it is namability alone, exactly as this row says: nothing
in `src/` can name `tests/common`. Link 3 should not expect the gate to
force the issue there.

## A home already exists in `src/`, and link 3's destination question is sharper than this row states

Found while adjudicating link 2 (2026-09-18). **`crates/topo/src/fixtures.rs`
exists** — 40 KB of fixture vocabulary, mounted `pub(crate) mod fixtures;`
at `lib.rs:158`, carrying the file-level
`#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]`
with its argument written out (*"Test-support code: panicking is a
test's failure mechanism (L5), and fixture unwraps are on keys the
fixture itself just minted"*), and exporting a `prism(n, tol)` whose
signature link 2 just converged the `tests/` family onto.

It is **`pub(crate)`**, which is exactly why `tests/common/mod.rs`
exists separately: a `tests/` binary is a separate crate and cannot
name a `pub(crate)` item. So the two homes are not rivals by accident —
one is unreachable from the other by construction.

**This row says the family moves to `src/test_support_impl.rs`. That is
now one of at least three options**, and link 3 has to choose rather
than inherit:

1. `test_support_impl.rs` — this row's original answer, gated and
   already re-exported as `topo::test_support`.
2. **`fixtures.rs`, made nameable** — it already holds fixture
   vocabulary, already carries the `#![allow]` with its reason, and
   already has the post-link-2 signature. The change is its visibility,
   not its contents.
3. A **sibling module** re-exported through `test_support`, which is
   what link 2's lane recommended on the ground that an inner
   `#![allow]` scopes to its module and `ArenaCounts` — the one item in
   `test_support_impl.rs` with a non-test consumer, the D1 debug
   postcondition — compiles clean without it and should not inherit a
   blanket allow it does not earn.

Option 2 was invisible to this row when it was written, and it is the
one that would make the move smallest. **Measure before choosing**: how
much of `fixtures.rs` duplicates what `tests/common/mod.rs` holds is
unknown and is the number that decides it.

## Home measurement (2026-09-18, branch `dup/home-measurement`)

**The crux is settled and it kills option 2.** `topo::fixtures::prism(n, tol)`
is **not** the same builder as `tests/common`'s `prism_ops` — it is not a
weaker version of it, not a permutation of it, and not the same body. The two
agree on exactly one thing, the ten arena LENGTHS, and disagree on every other
section of the derived dump. `fixtures.rs` and `tests/common/mod.rs` hold
**disjoint families that happen to share two names**, so "how much of
`fixtures.rs` duplicates what `tests/common/mod.rs` holds" — the number the
previous section said would decide the choice — is **zero items**, and the two
shared names (`prism`, `Prism`) are a collision to be resolved rather than a
duplication to be folded.

### How the two were named from one place

A `tests/` binary cannot name `pub(crate)`, and `src/` cannot name
`tests/common`, so no existing seat sees both. The probe made one: inside
`topo`'s library, under `#[cfg(test)]`,

```rust
extern crate self as topo;
#[path = "../tests/common/mod.rs"] mod probe_vendored_common;
```

**Nothing was vendored** — that is the tree's own `tests/common/mod.rs`,
compiled from its own path, so a drift between a copy and the original is not
possible. Both builders were then dumped by ONE function: a counts line, the
four `mass_properties` fields as hex bit patterns, and `format!("{body:#?}")`
— `Body`'s DERIVED `Debug`, carrying every field of every arena including
`SlotMap` slot versions, `free_head` and `num_elems`. Inputs: `fixtures::prism`
at `n = 3, 4, 5`; `common::brick` at the unit box and at
`(-0.5,2.25)×(1.0,1.5)×(-2.0,0.75)`; `common::prism_z` at a regular triangle
and a regular pentagon, which is the non-4-corner row both domains admit
through `n`. `f64`, default features, `Tol::witness()`,
`CARGO_TARGET_DIR=/home/user/dup-home-measure-target`, `Compiling topo`
confirmed. Every probe is deleted; this branch touches this file only.

### Where they agree

**The counts, exactly, at every `n` run.** `fixtures::prism(n)` and
`common::prism_z` over an n-gon both give
`solids=1 shells=1 faces=n+2 loops=n+2 half_edges=6n edges=3n vertices=2n
points=2n curves=3n surfaces=n+2` — verified at `n = 3, 4, 5`. Both bodies'
`solids` and `shells` arenas are byte-identical, as are `pcurves`,
`null_faces`, `point_origins`, `surface_field_sources` and `surgery` (the
first two empty, the rest trivial). That is **7 of the dump's 24 top-level
sections**.

Face `sense` agrees too: `true` on all six of both at `n = 4`.

### Where they diverge

The other **17 sections**. Not a permutation — a different construction.

- **`mass_properties` does not exist on the `fixtures` side.** `common::brick`
  gives `volume=0x3ff0000000000000 surface_area=0x4018000000000000`, pads zero.
  `fixtures::prism` REFUSES, at every `n`:
  `Face { face: FaceKey(1v1), source: QuadratureUnsupported { what: "the mvfs
  Nurbs placeholder reached the quadrature lane — a mid-surgery body has no
  mass properties (tier 2 refuses it at rest)" } }`.
- **`points`** — `fixtures::prism(4)`'s eight points are
  `(0,0,1) (1,0,1) (2,0,1) (3,0,1)` and the same four at `z = 0`: **collinear
  in `y = 0`**, an index-derived placeholder, not a box. `common::brick`'s are
  the eight corners of the unit cube. The module header says so in advance
  ("the prism's points are collinear, not an n-gon"); this measures it.
- **`surfaces`** — `fixtures`: six `Nurbs` placeholders whose control points
  are `NaN`. `common`: six `Plane { origin, normal, u_ref }` with real values
  (`origin (0.5,0.5,0.0)`, `normal (0,0,-1)`, …). 329 dump lines against 135.
- **`curves`** — `fixtures`: twelve `Circle` carriers,
  `description: Scaffold(RevolvedPoint …)`, `authority: Declared`, all twelve
  identical (the self-loop circle at the origin `test_curve` mints).
  `common`: twelve `Line` carriers, `description: Intersection`,
  `authority: Derived`. 1415 lines against 483. **This is the
  `Scaffold`/`Declared`-against-`Intersection`/`Derived` axis that the
  2026-09-16 measurement found did NOT separate `prism_z` from
  `sweep::test_support::brick`.** Here it separates cleanly, 12/12 against
  12/12.
- **All seven provenance maps** — `fixtures` writes
  `Primordial { op: "fixture" }` on every entity; `common` writes the real
  operator records (`Mvfs`, `Mef { site: Chords { he1, he2 } }`, …). The
  `fixtures` maps are about half the dump lines of the `common` ones for that
  reason.
- **`faces`, `loops`, `half_edges`, `edges`, `vertices`** — same line counts,
  different key wiring throughout: the first `half_edges` slot alone differs in
  three of its key fields, and the diff runs to 436 lines over that one section.
  Face surface keys are `[1v1 2v1 3v1 4v1 5v1 6v1]` against
  `[7v1 2v1 3v1 4v1 5v1 6v1]`.
- **`curve_origins`, `surface_origins`** — differ by one entry each.

**This is not the divergence shape the row's earlier measurement found.** That
one was two spellings of the same solid differing in a curve-key permutation
and an `(s1, s2)` order. This one is a real solid against a topological
skeleton with placeholder geometry, and no assertion a suite can make about
geometry survives the swap.

### The overlap, item by item

`fixtures.rs` holds 17 items; `tests/common/mod.rs` holds 17. **No item is
held by both.** The table is the near misses, which are what option 2 rested
on.

| `fixtures.rs` item | `tests/common` counterpart | same? | how settled |
| --- | --- | --- | --- |
| `prism(n, tol)` | `prism(profile, height, tol)` / `prism_z` / `prism_ops` | **No** | **by execution** (above) |
| `Prism` (21 key-vector fields, `f64`) | `Prism<T>` (6 fields, generic) | **No** | by reading — disjoint field sets, different scalars |
| `test_curve(anchor, tol)` → certified self-loop CIRCLE | `line(p0, p1)` → `EdgeCurveSpec::line_between` | **No** | **by execution** — the dumped carriers are `Circle` against `Line` |
| `test_surface(_)` → `Surface::nurbs_placeholder()` | `plane(corners, tol)` → `newell_plane(…)` | **No** | **by execution** — `Nurbs`/`NaN` against `Plane` |
| `plane_surface(origin, normal, u_ref)` → literal `Surface::Plane` | `plane(corners, tol)` → Newell-certified from a corner list | **No** | by reading — different inputs, no certification on the `fixtures` side |
| `prov()` → `Primordial { op: "fixture" }` | — (no counterpart; `common` lets the operators write provenance) | n/a | **by execution** — the provenance maps |
| `arena_snapshot` / `ArenaSnapshot` | — (`tests/` uses `topo::test_support::arena_counts`) | n/a | by reading |
| `deep_snapshot` | — | n/a | by reading |
| `ngon_pillow`, `pillow`, `mvfs_state` | — | n/a | by reading |
| `ops_cube` | `geometric_cube` | **the same Euler sequence, minus face geometry** | **by execution** (see below) |
| `ops_holed_box`, `ops_genus2`, `ops_ring_bridge`, `ops_strut_cube` | — | n/a | by reading |
| — | `PrismOps`, `brick`, `GeoCube`, `describe_as_intersections`, `mapped_cube`, `cube_into`, `StraddleSeat`/`straddle_seat`, `flush_declarations`, `assert_every_chord_named_by_both_rules` | no counterpart | by reading |

So the two homes are not rivals for the same vocabulary at all. `fixtures.rs`
is the **raw-insertion** home — bodies assembled by writing arena entries and
patching null keys, with placeholder geometry, for structural/atomicity/kill
tests that never read a coordinate. `tests/common/mod.rs` is the **Euler-op**
home — bodies built through the public operators with real certified geometry,
for suites that do read coordinates. Merging them would put two unrelated
disciplines behind one `#![allow]` and one module path.

### What cannot move: the set is empty, and this time it is proved

The 2026-09-16 section asserted this from the import list. It is now proved by
execution: with `tests/common/mod.rs` mounted into `topo`'s **library** target
(`#[path]`, under `cfg(any(test, feature = "test-support"))`),

```
cargo check -p topo --lib --features test-support
```

**succeeds with zero errors and zero warnings.** `--lib` does not put
dev-dependencies in scope, so a member reaching `proptest`, `test-utils`,
`strum`, `mesh`, `stl` or `step-export` could not have compiled. The file's
whole import list is `geom::Surface`, `geom_brep::{EdgeCurveSpec,
EdgeDescriptionSpec, newell_plane}`, `geom_core::{Tol, Band, Point3, Real}` and
`topo::{…}` — all normal `[dependencies]` of the `topo` library
(`crates/topo/Cargo.toml`). `git grep` over the file for each dev-only crate
name returns nothing outside prose. **Blocked set: ∅.**

### The gate, re-measured — and item 1 is already discharged

`crates/topo/tests/common/mod.rs` now contains **zero** `Tol::witness()` calls
(link 2 threaded them all). So `scripts/gates/witness-not-ambient.sh` is a
**non-issue for this family under all three options** — there is nothing left
for it to fire on. The gate's reach was nonetheless re-measured by planting a
witness and watching it fire, three ways:

| mount | in the gate's scan set? | evidence |
| --- | --- | --- |
| `#[cfg(any(debug_assertions, test, feature = "test-support"))] mod test_support_impl;` | **yes** | planted witness → `ERROR`, naming the line |
| `#[cfg(test)] pub(crate) mod fixtures;` (today) | **no** | planted witness → gate passes, 436 files scanned |
| `pub mod fixtures;` (option 2's change) | **yes** | planted witness → `ERROR`, naming `crates/topo/src/fixtures.rs:62` |

The third row is a cost option 2 pays and options 1 and 3 do not:
**un-gating `fixtures.rs` newly subjects 1130 lines to the gate forever.** It
costs nothing today (`fixtures.rs` also has 0 witness calls) and everything
the day a raw fixture wants one.

`crates/topo/tests/fixture/mod.rs` has **4** `Tol::witness()` calls, so that
file is the one place in this neighbourhood where the gate would actually bite
on a move.

### The `#![allow]`, measured

Stripping `#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]`
from `tests/common/mod.rs` and clippy-ing it inside the library gives
**31 lints: 25 `unwrap_used`, 6 `expect_used`, and 0 `panic`.** That is the
exact width the family earns. Two consequences:

- The `panic` arm is **unearned by this family today** — it travels as
  insurance, not as a debt.
- 31 lints is what option 1 would put a file-level allow over, in a file whose
  other resident (`ArenaCounts` + `Body::arena_counts`) earns 0 of them and
  compiles clean without any allow at all today.

`dead_code` and `unreachable_pub` need nothing new: the file's own
`#![allow(dead_code)]` and `#![allow(unreachable_pub)]` travel with it, and
under them the library check above is warning-free.

### The third home: `crates/topo/tests/fixture/mod.rs`

**It is not a vocabulary home and it has no overlap with either of the other
two.** It is a single-purpose acceptance fixture: ONE cylinder×sphere rung-3
SSI edge at rest carrying a fitted chart image, traced at `f64` and lifted to
the caller's scalar. Its public surface is three items — `build::<T>()`,
`foreign_cache`, `certify_at_dual` — plus the `Built<T>` bundle; the other
eleven functions are private helpers (`cylinder`, `sphere`, `trace_branch`,
`restrict`, `sub_arc3`, `sub_arc2`, `lift3`, `lift2`, `assemble`, `chart_of`,
`branch_or_budget`).

**Who calls it:** two suite files, `tests/m6_2_fitted_at_rest.rs` and
`tests/review_m6_2_probes.rs`. Nothing else, in either direction — it names
nothing from `tests/common` and `tests/common` names nothing from it.

**The boundary the row says is unstated is in fact structural**, and worth
writing down rather than discovering again: `common/` is the shared vocabulary
every suite may draw on (`use crate::common;` appears in **74** files);
`fixture/` is one fixture for one acceptance row, mounted beside it only
because `tests/all.rs` is the single test binary and helper modules are
declared there once.

**Where it lands under each option: nowhere.** It should not move under any of
the three, and the move does not have to "say which of the two it is moving"
— it is moving `common/`, and `fixture/` is out of scope by subject, by
consumer set and by the gate (its 4 witness calls are legal where they sit and
would not be in `src/`). Its own header says the whole hand-assembly should go
when the cyl×sphere fitted-chord join lane lands, which is a stronger argument
against moving it than anything about homes.

### The in-`src` census candidates, read

**`crates/topo/src/review_m1_pr2/cube_independent.rs` — NOT a member, and must
not become one.** Its module header is explicit: *"These are **independent
derivations** — do not 'simplify' them to match shipped fixtures; the
independence is the regression value. Promoted per Ev's request (PR #17
thread)."* And it is genuinely a different addressing: it seeds at the **top**
corner `A'`, builds the TOP chain, closes the TOP face with the first `mef`,
drops struts **down**, and the seed face ends as the **bottom** — the mirror
of the shared builder, which seeds at the bottom and raises struts up. Folding
it onto the shared builder would delete the thing it exists to prove. **Does
not fold. Ev has already ruled on this file.**

**`crates/topo/src/review_m1_pr2/atomicity.rs` — NOT a member.** It builds no
cube. Its one builder is a local `pillow()` — the digon pillow, 2 vertices,
2 edges, 2 faces — and the rest of the file is 300 lines of error-path sweeps
asserting `deep_snapshot` equality after every `EulerOpError`. The census
flagged it on `mvfs`/`mev_line`/`mef_chord` call sites, which is exactly the
undercount-by-shape the row predicted, in the other direction. **Does not
fold.**

**`crates/topo/src/review_m1_pr3.rs`'s `build_box` — IS a member, and the
measurement is stronger than "a member".** `build_box` is **byte-identical to
`fixtures::ops_cube` modulo a uniform 2× scale**. Built both into fresh bodies
and dumped: `solids`, `shells`, `faces`, `loops`, `half_edges`, `edges`,
`vertices`, `surfaces`, all seven provenance maps, `point_origins`,
`curve_origins`, `surface_origins`, `surface_field_sources`, `surgery` —
**every one byte-identical**. The only two sections that differ are `points`
(`1.0 → 2.0` on every nonzero coordinate, checked entry by entry) and `curves`
(which carry those same coordinates). Its own doc admits it: *"the 2×2×2 box
(cube-test sequence; PR 2 material, re-verified here via loop-walk assertions
rather than trusted)"*. **Folds** — onto a builder that takes an extent.

### A sixth copy the census did not name: `fixtures::ops_cube`

Measured while settling `build_box`, and it is the largest single finding here
after the crux. **`fixtures::ops_cube(tol)` and `common::geometric_cube::<f64>(tol)`
produce bodies that are byte-identical in every section of the derived dump
except three.**

Identical: `solids`, `shells`, `loops`, `half_edges`, `edges`, `vertices`,
**`points`**, **`curves`** (all 1199 lines), `pcurves`, `null_faces`, all seven
provenance maps, `point_origins`, `curve_origins`, `surface_field_sources`,
`surgery`.

Different: `faces` (surface keys `[1v1 ×6]` against `[7v1 2v1 3v1 4v1 5v1 6v1]`),
`surfaces` (one `Nurbs` placeholder shared by all six faces, against six
Newell `Plane`s), and `surface_origins` (11 lines against 32).

That is the whole of the difference: **`ops_cube` is `geometric_cube` with the
face geometry declined.** Reading the two confirms the sequence is the same
operator for operator — same `mvfs` at the origin corner, same `MevSite::Lone`
seed chord, same `Fan` chain, same
`find_half_edge(seed.face, v[n-1], v[n-2])` bottom-cap site, same four strut
anchors **including** the `f_bottom.he_plus` special case on the last, same
four side `mef`s **including** the `f_front.he_plus` close. It differs from
`prism_ops` only by calling `mev_line`/`mef_chord` where `prism_ops` calls
`mev`/`mef` with a curve spec and a `FaceSurface::New(plane(…))`, and by
skipping the seed-face `set_face_surface`.

So the cube sequence is written, in this crate, **at least five times**:
`prism_ops` (`tests/common`), `cube_ops` (unified into it by link 1b),
`cert_m3r1_probes.rs`'s copy, `fixtures::ops_cube`, and
`review_m1_pr3::build_box` — of which the last two are in `src/`, are the same
function as each other, and are the same function as `geometric_cube` up to
one axis (supply face geometry or not) and one scale. **`cube_independent.rs`
is the one that is genuinely independent and is exempt by Ev's own ruling.**
This wants its own row on S-DUP's slate; it is not scheduled by being written
here.

### Each option, costed

Common to all three: the family compiles inside the library with no
dev-dependency (proved above); the witness gate has nothing to fire on; the
`#![allow]` is 25+6+0 lints wide; `dead_code`/`unreachable_pub` are already
handled by the file's own inner attributes.

**Call sites that move: ONE, under every option.** `tests/common/mod.rs` is
declared exactly once — `mod common;` at `tests/all.rs:54` — and every
consumer reaches it through `use crate::common;` (**74 files**) or
`use common::X` (a further 48 lines). All 279 `common::` references keep
working if that one line becomes a `use … as common;` alias. No suite file
changes under any option. *(This is a reading of the resolution rules, not an
execution: the aliased build was not run.)*

**Option 1 — append to `test_support_impl.rs`.**
Visibility: nothing new; the file is already `mod` + `pub use … as
test_support` under the two gates. Gate: in the witness scan set (measured) —
harmless at 0 calls. `#![allow]` scope: the 3-lint file-level allow now covers
`ArenaCounts` and `Body::arena_counts`, which earn **0** of the 31 lints and
compile clean without it today. Consumer above topo writes
`topo::test_support::brick(…)` after adding `features = ["test-support"]` to
its existing dev-dep `topo` edge. The file goes from 147 lines to ~720, and
its module docs — which are the crate's statement of the three-homes rule —
end up as the header of a fixture file.

**Option 2 — `fixtures.rs` made nameable. Falsified by the measurement.**
It rested on "how much of `fixtures.rs` duplicates `tests/common/mod.rs`"; the
answer is nothing. Its costs, measured:
- A **name collision on `prism` and `Prism`**, two pairs with disjoint
  meanings. Renaming the incumbents costs 9 + 3 = **12** call sites across
  `crates/topo/src/`; renaming the incomers costs 68 (`prism_z`) + 48
  (`Prism`) + 9 (`prism`) = **125** in `tests/`.
- Un-gating the module puts **1130 lines** into the always-compiled library
  and into the witness gate's scan set (measured: the gate fires there once
  the `#[cfg(test)]` comes off).
- `cargo check -p topo --lib` with the `#[cfg(test)]` removed produces
  **17 `dead_code` warnings** (measured), one per `pub(crate)` item with no
  non-test consumer, under a workspace that runs clippy as `-D warnings`. The
  only fix that clears them is making all 17 `pub` and re-exporting — which
  publishes `ngon_pillow`, `deep_snapshot`, `ops_genus2`, `ops_strut_cube`
  and the rest through `test_support` to every crate above `topo`, for the
  benefit of nobody.
- It would put the raw-insertion family and the Euler-op family behind one
  module path and one allow, which is the opposite of what the three-homes
  rule in `test_support_impl.rs` asks for.

**Option 3 — a sibling module re-exported through `test_support`.**
Visibility: one new file mounted under the same two cfgs as
`test_support_impl`, plus one `pub use` line. Gate: in the witness scan set —
harmless at 0 calls. `#![allow]` scope: **exactly the 31 lints that earn it**;
`ArenaCounts` keeps its clean file. Consumer above topo writes the same
`topo::test_support::brick(…)` as under option 1 (if the re-export is flat) —
**option 3 is invisible to every consumer**, inside `topo` and above it. Cost
over option 1: one file and one `pub use`.

### The verdict

**Option 2 is out, on a measurement rather than a preference**: the number it
was waiting on — shared items between the two files — is **0**, and the two
names they share are a collision costing 12 or 125 call sites to break, on top
of 17 new dead-code warnings and 1130 lines newly under the witness gate.

**Between options 1 and 3 there is no genuine fork.** They are identical in
every consumer-visible respect (same `topo::test_support::X` spelling, same
feature edge, same gate posture, same call-site cost of one line) and differ
only in whether a 3-lint blanket allow — 31 lints wide, of which the `panic`
arm is unearned — also covers `ArenaCounts` and `Body::arena_counts`, which
earn 0 of them and compile clean without it. Option 3 buys that for one file
and one `pub use`. **Take option 3.**

The number that would change this: if `ArenaCounts` ever earned any of the 31,
the two options would collapse into one and option 1's single file would be
the tidier answer. It earns 0 today.

### What this measurement could not see

- **`f64` only.** `prism_ops`, `prism_z`, `brick` and `geometric_cube` are
  generic over `Decide`; `fixtures::prism`, `ops_cube` and `build_box` are
  `f64`-only, which is itself a reason the two families cannot merge, but it
  means nothing was run at `Dual`, `Interval` or `Probe`. The `interval` lane
  was not built.
- **Default features, one eps row.** Local build, default eps only.
  `CAD_TOLERANCE_EPS=1e-6` and `1e-12` were NOT run — the 2026-09-16
  measurement ran two eps rows and this one ran one, because the divergence
  found here is categorical (a `Plane` against a `NaN` `Nurbs`) rather than
  numeric. An eps-dependent agreement is not a shape this finding could have.
- **Inputs not run.** `fixtures::prism` was run at `n = 3, 4, 5` and
  `common::prism_z` at a triangle, a pentagon and two boxes. No reflex
  profile, no tilted `map`, no second solid grafted into one body, no
  `n = 2` digon cap (`fixtures::prism` admits it; `prism_ops` asserts
  `n >= 3`, so the domains differ at exactly that point and it was not
  measured).
- **`straddle_seat`, `flush_declarations`, `cube_into`, `mapped_cube` and
  `assert_every_chord_named_by_both_rules` were not executed** — they were
  read. Their membership in the family is inherited from the 2026-09-16
  section, not re-established here.
- **The one-line `all.rs` swap was not compiled.** The claim that 0 of 279
  `common::` references move is a reading of module resolution, not a build.
- **No consumer above `topo` was built against a moved family.** `sweep`,
  `stl`, `mesh` and `editor-core` each carry a plain `topo = { path = "../topo" }`
  dev edge today; that adding `features = ["test-support"]` to it compiles is
  inferred from `step-export` and `step-import`, which already carry exactly
  that, not measured.
- **`fixtures::ops_cube` against `cert_m3r1_probes.rs`'s copy** was not
  compared. The five-copies claim above counts `geometric_cube`,
  `fixtures::ops_cube` and `build_box` by execution and the other two by
  reading.
- **Nothing downstream of the body.** No STL, STEP, mesh or validator output
  was compared, and `validate()` was not run on either side.

## Adjudicated (2026-09-18): option 3, and option 2 died on a number

**The home is settled and it is option 3** — a sibling module under
`src/`, re-exported through `test_support`. Options 1 and 3 are
indistinguishable to every consumer (same `topo::test_support::X`, same
feature edge, same gate posture) and differ only in whether the
three-lint `#![allow]` also covers `ArenaCounts`, which earns none of
it. Option 3 costs one file and one `pub use`. The number that would
flip it back to option 1 is `ArenaCounts` earning any of the 31 allowed
lints; it earns zero.

**Option 2 is dead, and not narrowly.** The row proposed it on the
ground that `fixtures.rs` "already holds fixture vocabulary, already
carries the `#![allow]` with its argument, and already exports a
`prism(n, tol)` whose signature link 2 just converged the `tests/`
family onto" — and named the deciding number as *how much of
`fixtures.rs` duplicates what `tests/common/mod.rs` holds*. **That
number is zero.** The two files share no item. What they share is a
name: `prism`/`Prism` exists in both with disjoint meanings.

`topo::fixtures::prism(n, tol)` is not a weaker `prism_ops`, it is a
different artifact. The two agree on the ten arena lengths and on 7 of
the derived dump's 24 top-level sections and diverge on the other 17.
`fixtures::prism` has no mass properties at all
(`QuadratureUnsupported { "the mvfs Nurbs placeholder reached the
quadrature lane" }` at every `n`); its eight points are
`(0,0,1)(1,0,1)(2,0,1)(3,0,1)` and the same at `z = 0`, **collinear in
`y = 0`** — not a box; its surfaces are six `Nurbs` with `NaN` control
points against six real `Plane`s; its twelve carriers are `Circle`,
`Scaffold(RevolvedPoint)`, `Declared` against twelve `Line`,
`Intersection`, `Derived`. The file says so itself at `:492` — *"Coordinates
are indexed placeholders standing in for the documented picture …
structural validation never reads them."*

Note which axis that is. The 2026-09-16 measurement above recorded that
`Scaffold`/`Declared` was **not** what separated `prism_z` from
`sweep::brick` — there was no `Scaffold` anywhere in either body. Here
it separates 12/12 against 12/12. A real solid against a skeleton, not
a permuted arena.

### One correction to the measurement's framing

The lane characterised `fixtures.rs` as "the raw-insertion home" and
`tests/common` as "the Euler-op home". **That is right about
`fixtures::prism` and wrong about the file.** `ops_cube` (`:765`),
`ops_holed_box`, `ops_genus2`, `ops_ring_bridge` and `ops_strut_cube`
are all built through the operators with real coordinates —
`ops_cube`'s own doc says *"the §9.4.2-minimal sequence; same
construction as the PR 2 acceptance test"*, and it is 1 `mvfs` + 7
`mev_line` + 5 `mef_chord` including the `f_bottom.he_plus` strut
anchor and the `f_front.he_plus` close. `fixtures.rs` holds **both**
kinds side by side.

This does not disturb the verdict — the item overlap with
`tests/common` is still zero and the `prism` collision is still
disjoint — but it changes what the two files' boundary *is*, which
matters to anyone working the move. The boundary is not
raw-against-Euler. It is **reachability**: `fixtures.rs` is
`pub(crate)` (and `#[cfg(test)]`, `lib.rs:157–158` — the row's earlier
note that option 2 changes "visibility, not contents" understated it;
the cfg has to come off too, which is what pulls 1130 lines under
`witness-not-ambient.sh` forever and raises 17 `dead_code` warnings in
a `-D warnings` workspace), so `tests/` cannot name it, and
`tests/common` is a `tests/` binary, so `src/` cannot name it. Each
file exists because the other is unreachable. That is the wall link 3
is taking down, and it is the same wall holding
`cert_m3r1_probes.rs`'s copy in place.

### The methodological finding, which is this orchestrator's to own

I wrote option 2 into this row on 2026-09-18 on the strength of two
observations: `fixtures.rs` holds fixture vocabulary, and it exports a
`prism(n, tol)` matching the signature link 2 had just converged the
`tests/` family onto. Both were true. Neither was evidence. **A
signature match is not a sameness claim** — `prism(n, tol)` names two
functions that produce a certified solid and a `NaN`-surfaced skeleton
respectively, and the convergence I read as a sign the two were one
door was a convergence onto `(count, tol)`, which is what almost any
fixture builder in this tree takes after link 2.

This is the program's standing trap running backwards. S-DUP normally
catches *different names for one thing*; here I nearly landed a unit on
*one name for two things*, and what saved it was that the row demanded
a measurement before the choice. Put beside the five instruments
result — no single instrument has ever found even half of any class —
the companion rule: **a name, a signature and a neighbourhood are
three readings of the same surface, and three surface readings do not
make a measurement.**

### Carried to link 3 as settled

- **Nothing is blocked from `src/` — proved, not read.** `cargo check
  -p topo --lib --features test-support` with the family mounted in the
  library: 0 errors, 0 warnings. `--lib` excludes dev-dependencies, so
  no member can be reaching `proptest`, `test-utils`, `strum`, `mesh`,
  `stl` or `step-export`. The row's "the set that would have to move"
  section is confirmed and its blocked set is empty.
- **The witness gate is already discharged.** `tests/common/mod.rs`
  holds **0** `Tol::witness()` calls after link 2. Gate item 1 of this
  row's "two gates at the door" is spent; only the `#![allow]` remains,
  and option 3 is the answer to it.
- **The `#![allow]` is 31 lints wide**: 25 `unwrap_used`, 6
  `expect_used`, **0 `panic`**. The `panic` arm is unearned and should
  not travel with the family.
- **One call site moves.** `mod common;` at `tests/all.rs:54` is the
  sole declaration; 74 files say `use crate::common;` and all 279
  references survive a `use … as common;` alias. (Read, not compiled —
  link 3 compiles it.)
- **`tests/fixture/mod.rs` moves nowhere.** Not a second vocabulary
  home: one cyl×sphere rung-3 SSI acceptance fixture, 3 public items, 2
  consumers, zero overlap with either other home in either direction,
  and its own header says the hand assembly should be deleted when the
  fitted-chord join lane lands. Out of scope by subject, by consumers
  and by gate. This retires the destination question the row raised for
  link 3.
- **The three unopened in-`src` census candidates are settled.**
  `review_m1_pr2/cube_independent.rs` is **not** a member and is
  **exempt by Ev's ruling** — its header reads *"independent
  derivations — do not 'simplify' them to match shipped fixtures …
  Promoted per Ev's request (PR #17 thread)"* — and it genuinely
  differs in addressing. `review_m1_pr2/atomicity.rs` is not a member
  (a digon pillow, no cube; the census flagged it on `mvfs`/`mev_line`
  call sites, its undercount-by-shape running the other way).
  `review_m1_pr3::build_box` **is** a member and folds — it is
  `fixtures::ops_cube` at a uniform 2× scale, every dump section
  identical but `points` and the curves carrying them.
- **A new row, filed rather than left in prose**:
  `work/dup/the-cube-sequence-is-written-five-times-and-twice-inside-src.md`,
  parked behind this one. `fixtures::ops_cube` is `geometric_cube` with
  the face geometry declined — byte-identical dumps in `points`, all
  1199 lines of `curves`, `half_edges`, `loops`, `edges`, `vertices`,
  all seven provenance maps, `curve_origins` and `surgery`. It parks
  behind link 3 because nothing in `src/` can name the shared builder
  until link 3 lands.

### What the measurement could not see

`f64` only; one eps row (default — the divergence is categorical,
`Plane` against `NaN` `Nurbs`, so an eps-dependent agreement is not a
possible shape here); no reflex profile, no tilt map, and **no `n = 2`
digon**, which is exactly where the two domains differ (`prism_ops`
asserts `n >= 3`, `fixtures::prism` accepts `n >= 2`);
`straddle_seat`, `flush_declarations`, `cube_into` and `mapped_cube`
were read, not executed; the `all.rs` alias was not compiled; no
consumer above `topo` was built against a moved family;
`cert_m3r1_probes.rs`'s copy was not dumped.


## Closed (2026-09-18, `dup/move-the-fixture-family`) — link 3 landed, with one bullet carved out

**Option 3, as adjudicated.** The family is
`crates/topo/src/test_support_fixtures.rs`, a sibling of
`test_support_impl.rs` mounted `#[cfg(any(test, feature =
"test-support"))]` and re-exported flat through `topo::test_support`.
`ArenaCounts` keeps its allow-free file. The `#![allow]` carried is
`unwrap_used` + `expect_used` with `fixtures.rs`'s argument comment
adapted; **the `panic` arm was dropped**, since the measurement had it
earning zero.

**The one call site moved, and it compiles.** `mod common;` at
`tests/all.rs` is now `use topo::test_support as common;`. All 279
`common::` references still resolve — a crate-root `use` is private but
visible to descendants, which is every suite module. No suite file
changed for the move. This retires the row's *"read, not compiled"*
caveat.

**`tests/fixture/mod.rs` moved nowhere**, as adjudicated.

**The collision was resolved by renaming the INCUMBENT, not the
incomer.** `crates/topo/src/fixtures.rs`'s `prism`/`Prism` are now
`raw_prism`/`RawPrism` — 12 call sites inside `crates/topo/src/`, the
number this row measured for that direction. So `prism` and `Prism` now
have exactly one definition each in the crate and no `use` can bind the
wrong one. `test_support_fixtures`'s own
`the_two_prism_families_build_different_bodies` is the guard: it asserts
the two agree on every arena length and then separates them by surface
kind, carrier description and mass properties — the axes a counts check
cannot see.

**`cert_m3r1_probes.rs` is folded** and its row
(`topo-src-cert-m3r1-probes-holds-an-in-src-copy-of-the-cube-family`)
is closed. A sixth copy the row named in passing folded with it:
`face_surface_of_he` was written four times (a closure in the family, a
free function in `cert_m3r1_probes.rs`, a closure in
`tests/bool4_material_containment.rs`, a free function in
`tests/m4_pr2_transform.rs`) and is now one exported function.

**Link 1's carried residue is fixed.** `mapped_cube` and `cube_into`
are generic in the `Decide` scalar, and **no call site needed an
annotation** — the closures they take already fix `T`. The payoff is
executed, not asserted: `tests/cube_doors_agree.rs`'s interval row now
runs the whole four-door roster instead of the two generic ones.

**Two `src/` guards had to learn about the new home**, which is a fact
the move turned up and nothing predicted. `crate::source_walk::mutation_doors`
walks `topo/src` for `pub fn` taking `&mut Body`, so `prism_ops`,
`describe_as_intersections` and `cube_into` entered the population of
`review_m1_pr5_internal::ALLOWED` and
`pcurves::staleness_posture::DECLARED`. They were given entries in both,
with the reason tier 1 and the pcurve map survive each — the guards'
own prescribed remedy. **Narrowing either walk to exclude test-support
sources was considered and refused**: it would shrink a guard's
population so that new code escapes it, and the entries are true and
checkable.

### What did NOT land: the `sweep` bullet

*"`sweep::test_support::brick` delegates to it or is deleted"* is the
one part of link 3 this unit could not do, for three measured reasons —
a scalar-domain mismatch (`(T, T)` against `(f64, f64)`, which `block`
and `cube` propagate), a `src/`-to-`src/` feature edge that
`scripts/gates/test-features-dev-only.sh` polices, and a committed
`.step` corpus regenerated from those builders. Filed with its evidence
as `work/dup/sweep-test-support-brick-is-still-a-second-box-construction.md`
rather than left in a PR body. Nothing blocks it now that the home
exists.

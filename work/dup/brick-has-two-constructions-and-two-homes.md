---
id: brick-has-two-constructions-and-two-homes
kind: issue
title: The axis-aligned box is built two ways in two homes; the shared home is topo's, not sweep's
status: parked
opened: 2026-09-16
refs: [topo-tests-brick-copies]
blocked_on: [topo-one-builder-subsumes-the-cube-and-prism-sequences]
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

# LANE-4P — the door values' pins in one shape, and a census that counts doors as well as scalars

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-24).** Binds
the implementer of unit LANE-4P; deleted at merge with a note under
`docs/doc-ledger/`. Read `docs/prompts/implementer-discipline.md` in full
first. The item is `work/scalar/lane-4p-door-pins-in-one-shape.md`; the
filed row it takes is
`work/scalar/the-shell-door-is-a-third-door-value-the-certified-enclosure-census-does-not-know.md`.
The survey is `/home/user/scalar-briefs/survey-lane4.md` §7 (at
`6709bc3229`); re-derive every count and line at your merge base.

**The cut (orchestrator, 2026-09-24).** H5 ruling 3's last trait,
`PcurveFittedLane`, becomes a fifth door value in LANE-4. Before it does,
this unit puts the four existing door values' wiring pins into ONE shape
and teaches the census the axis it lacks, so LANE-4 adds its door in that
shape and the census reds if it does not. The alternative, folding this
into LANE-4, was declined: LANE-4 is broad (≈66 bound sites, five
crates) and waits on #3160; this is small, independent and dispatchable
now.

## 0. What the tree has

Four door values, each a `Copy` struct of private fn-pointer fields with
one constructor:

| door | crate | constructor | pin today |
|---|---|---|---|
| `OffsetFitLane<T>` | geom-brep (`offset_fit_lane.rs`) | concrete at `f64` | `offset_fit_lane.rs` `wiring_rows`, three concrete-`f64` `fn_addr_eq` rows, no helper |
| `QuadLane<T>` | topo (`props.rs`) | `certified()` at `Decide + CertifiedBounds` | `props.rs` `mod wiring_rows`, helper `holds_the_certified_quadrature::<T>()` |
| `RegionLane<T>` | topo (`chart_region.rs`) | `certified()` | `chart_region.rs` `mod wiring_rows`, helper `holds_the_certified_region_doors::<T>()` |
| `ShellDoor<T>` | topo (`props.rs`) | `certified()` at `Decide + CertifiedBounds + AtRestPolicy` | inline in `props.rs` `at_rest_policy_tests::certifying_arms_are_the_doors` (+ the `Sym<f64>` row) |

`crates/topo/tests/certified_enclosure_impl_census.rs` reads every
`impl … CertifiedEnclosure for X` in `crates/*/src` and checks the TWO
helpers named in `ROSTERS` are called for each. It does not know the
other two doors, and it cannot see a NEW door at all: the failure the
filed row records is that a door appeared and nothing reddened.

## 1. What this unit delivers

1. **One pin shape per door.** Each door gets one in-crate `#[cfg(test)]`
   helper of `RegionLane`'s shape — `fn wired<T: …>() -> Result<(),
   &'static str>` (or the existing helper names kept, if renaming only
   churns: say which) that compares every fn-pointer field against its
   intended target by `fn_addr_eq` and names the field that moved — and
   one row per scalar that calls it. The shell door's pin moves out of
   `at_rest_policy_tests` into that shape (the gate rows there keep
   testing the policy's `Some`/`None` answers, not the wiring). The
   offset-fit door gets a helper at `f64`.
2. **The census learns both axes.** `ROSTERS` becomes `(file, needle,
   expected)` with `expected` = the certifying scalars (every
   `CertifiedEnclosure` implementor, as today) or `F64Only` (the offset
   fit — its constructor is concrete, so a new `CertifiedEnclosure` impl
   forms no `OffsetFitLane`; state the reason in the entry). AND the
   census enumerates door VALUES: every `impl<…> X<T>` block in
   `crates/*/src` that defines a door constructor (`certified()`, and the
   offset fit's concrete constructor — find the exact spelling) must have
   a `ROSTERS` entry, or the census reds naming the type. Make the
   door-value matcher robust to the shapes in the tree (generic bound
   lists across lines, `where` clauses) and state its blind spot.
3. **Red-first.** Show, and record in the PR body: (a) a planted fifth
   door type with a `certified()` constructor and no roster entry reds the
   census; (b) dropping one row per door reds the census; (c) mutating
   one fn-pointer field per door to a same-signature forwarder reds that
   door's helper row; (d) a planted `CertifiedEnclosure` impl reds every
   certifying door's roster and NOT the offset fit's. Restore each.
4. **The filed row closes** at merge with this PR; its body says what
   LANE-4 still owes (the fifth door lands in this shape).
5. **The plan.** `work/scalar/plan.md`'s LANE-4 row is split into
   LANE-4P and LANE-4, and LANE-4's ground list is corrected to the
   survey's §8 owners (PCERT, REACH, CARVE/BAND, ATREST, OFFSET/SHELL,
   TANG/ZIP, TQUERY, CHART, ISO, CURVED, STRUT, WIRE, PROPS, GUARD,
   TCOST/TINT) — this commit does it; leave it.

No production code changes: every edit is `#[cfg(test)]`, a test file,
or prose. No `decide(` moves; no golden moves.

## 2. Not this unit

The `PcurveFittedLane` fold itself (LANE-4); any change to a door's
constructor, bound or field set; `NOT_A_DOOR_SCALAR` (RING-3, #3153,
empties it — if #3153 has landed at your merge base, build on it; if not,
leave that constant as it is and expect a trivial conflict at merge).

## 3. Fence

topo test modules in `props.rs` and `chart_region.rs` (unowned / CHART),
`geom-brep/src/offset_fit_lane.rs` (unowned), the census
`crates/topo/tests/certified_enclosure_impl_census.rs` (TCOST/TINT), the
test ledgers if a file is added. Nothing else; if more is needed, stop at
the count and file.

## 4. Verification and PR

Merge `origin/main` before opening. `cargo check --workspace
--all-targets` default and `--all-features`; clippy `-D warnings` at
default and `--all-features` on topo and geom-brep; `cargo fmt --check`;
every gate touched (`scripts/gates/*.sh`, `probe-suite-census.sh` if a
test file is added) and `work.py lint`; nextest of topo and geom-brep
(with `--features interval` if the feature still exists at your base).
Open the PR titled `LANE-4P: …` (not `[ev]`: nothing ratified moves),
body per the discipline: what landed, the red-first table, the census's
blind spot, deviations. Watch the hosted run to green on your final
head. Review tier: SINGLE (orchestrator dispatches it).

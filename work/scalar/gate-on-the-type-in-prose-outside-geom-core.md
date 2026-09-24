---
id: gate-on-the-type-in-prose-outside-geom-core
kind: issue
title: prose sites outside geom-core still put the interval gate on the TYPE
status: closed
opened: 2026-09-21
closed: 2026-09-24
priority: P4
cost: E
refs: [ring-1-interval-type-ungated, H5]
---


## What

RING-1 ungated `geom_core::interval`: the type, its arithmetic and its
`Real`/`Decide`/`Bounds`/`SpanLocate` impls compile in every build, and
what the `interval` feature gates is the lane-trait impls in the crates
above `geom-core` and the interval test files. Prose sites outside that
unit's fence still say the gate is on the TYPE, or name the backend as
the FEATURE's, and each is false as written:

- ~~`crates/geom-brep/README.md`, clause **C9**~~ — **FOLDED by
  RING-2**, which re-worded the clause anyway (the ring's arithmetic
  is the backend's now), and said in the same sentence what the
  `interval` feature gates: the lane impls above `geom-core` and the
  interval test files, not the type. Retiring the clause is still
  RING-3's; this bullet was only its staleness.
- `interval-transcendentals/README.md` ("`geom-core`'s `interval`
  feature depends on this crate", `:11`) — `geom-core` depends on it
  unconditionally now; no feature activates the edge.
- `interval-transcendentals/docs/inventory.md` ("the interval scalar
  … `crates/geom-core/src/interval.rs`, behind the `interval`
  feature", `:4`).
- `crates/test-utils/src/lib.rs` ("`interval-transcendentals/` … is
  path-depended on by `geom-core` (the `interval` feature's backend)",
  `:41`) — the same naming class: it is the interval scalar's backend,
  and the path dependency the sentence argues from is unconditional.
- **WIRE's**: `crates/editor-core/src/lib.rs` (`:30-32`), the `drive`
  module's gate — "Gated on `interval` because the leaf protocol
  replays at the certified interval scalar: without that scalar there
  is no leaf to certify". The GATE is right and stays; its stated
  REASON is not, because the scalar is in every build now. What keeps
  `drive` gated is the instantiation cost of replaying the kernel at
  the scalar, which is what the reason should say.

## Sweep, and what it could not match

`rg -i "behind the .interval. feature|interval. feature's backend|feature-gated"`
over `*.rs *.md *.toml *.yml` (R2's shape), read hit by hit. The
`feature-gated` arm is mostly other features and the interval TEST
files, which stay gated and stay true: the four consumer manifests'
"exercised by the feature-gated test modules in CI's interval lane"
(`crates/{geom,sweep,profile,geom-brep}/Cargo.toml`), the `cfg(test)`
mod comments in `crates/geom/src/{curves,surfaces}.rs` and
`crates/geom-core/src/dual.rs`, `docs/GUIDE.md:2111` (the wheel's
uncertified half — the E6/E4/E5/E10 modules are gated), `demos/tour/
src/plate.rs:14,95` (the tour's tolerance cell), `crates/pncad/tests/
all.rs:4375` (`crate::analysis`), and `crates/geom-core/src/k_stats.rs:107`
(the `probe` feature, a different gate).

Two blind spots, stated rather than claimed away: the pattern matches
a PHRASE, so a site that argues the gate in its own words is invisible
to it — WIRE's site above was found by reading, not by the pattern —
and it matches no source outside those four extensions (no python, no
shell). `crates/editor-core/src/measure.rs:525` ("its own type, which
lives behind the `interval` feature this door does not") reads as the
gated engine module rather than the scalar and is left; the WIRE row
above is the place to settle it.

## Why not fixed where it was found

`docs/RING-1-SPEC.md` §5 fences that unit to `geom-core`, `ci.yml`,
`scripts/gates/*`, `docs/DESIGN.md` and `docs/GENERICS-BUILD-COST.md`;
these sit outside it, and C9 belongs to a ruling that already named
the PR that re-words it. Three sites of the same class that the fence
DOES cover were fixed in RING-1's PR rather than filed here:
`docs/DESIGN.md:1018`'s crate-landscape row and
`.github/workflows/ci.yml:3596,3997` — all three named the backend as
the feature's. `docs/CI-MINUTES-2026-08.md` is a dated measurement
record, correct as of its date, and stays.

## Closed (2026-09-24) — mooted by RING-4, PR 3154

Moot: the `interval` feature is deleted (RING-4, PR 3154) and its fix pass swept the live prose that named any interval gate, on the type or otherwise; no gate is left to misplace.

# RING-1 — `geom_core::interval` compiles unconditionally; the feature gates only the instantiation

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-21).** Binds
the implementer of unit RING-1; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/ring-1-interval-type-ungated.md`; the ruling is
`work/scalar/H5.md` §RATIFIED, ruling 1 (PR 2701), cut (i).

## 0. The finding, and what the tree says

`pub mod interval` in `crates/geom-core/src/lib.rs` is behind
`feature = "interval"` (`crates/geom-core/Cargo.toml`: `interval =
["dep:interval-transcendentals"]`), while the same crate is already an
unconditional dev-dependency, in-repo and libm-only (the LGPL ground for
the gate left with PR 127; `docs/GENERICS-BUILD-COST.md` §3/§7 records
what the gate was measured to be for — build cost of the
INSTANTIATION, not of the type). Measured 2026-09-15 on a clean dev
build of the workspace: default 107 s, the type ungated 106 s, the full
instantiation 146 s; `cargo build -p geom-core` 8.0 s vs 8.3 s. The
type is free. Ruling 1 retires `RingInterval` in three cuts; this is
cut (i), and RING-2 (the newtype over `DInterval`) needs the type in
every build to exist.

## 1. What this unit delivers

The measured patch, made real (it compiled clean at `-p geom-core`,
`--workspace` and `--workspace --tests`; re-derive it against your
merge base rather than applying it blind):

- `crates/geom-core/Cargo.toml`: `interval-transcendentals` a normal
  dependency; the `interval` feature stays declared as `interval = []`
  so every `--features interval` invocation in `ci.yml`, the other
  crates' `interval = ["geom-core/interval", …]` forwards and the 111
  `#![cfg(feature = "interval")]` test files keep working unchanged.
  Re-word the feature's comment to what it gates now: the kernel's
  instantiation at `Interval` (lane impls, tests), not the type.
- `crates/geom-core/src/lib.rs`: `pub mod interval` and the two
  re-exports (`Interval`, `DualInterval`) unconditional.
- `crates/geom-core/src/dual.rs` (the `use` and `DualInterval`),
  `bit_identity.rs` (`repr_bits`'s `Interval` arm), `spline/locate.rs`
  (the sealed impl): the four `cfg(feature = "interval")` lines go.
- Inside `geom-core` only: any other `cfg(feature = "interval")` that
  gates the TYPE or its impls of `Real`/`Bounds`/`Decide`/
  `CertifiedEnclosure`/`SpanLocate` goes with them; the ten `cfg(test)`
  sites in `geom-core` stay unless a test of the type itself is now
  unconditional by construction (say which). **The 47 cfg sites in the
  23 other `src` files and the gated test files stay** — they are the
  instantiation, RING-3's.
- **Prose that leans on the gate, re-worded with the change** (present
  tense, no history): `crates/geom-core/src/ring_interval.rs:10-13`
  ("behind the `interval` cargo feature — deliberately unlinked: the
  module does not exist in a default build") — the module exists in
  every build now, so the intra-doc link can be a real link;
  `docs/DESIGN.md` Q1's instantiation bullet (~`:1308-1309`, "behind
  the `interval` feature") — a naming-only re-wording caused by this
  approved change (run `git log -S'behind the `interval` feature' --
  docs/DESIGN.md`, cite the commit in the PR body, change only the
  phrase: the feature gates the instantiation); `docs/GENERICS-BUILD-COST.md`
  gains a dated addendum with the 2026-09-15 numbers
  (`/home/user/scalar-briefs/interval-cost.md` on the box has the table
  and method; copy the table, not the file). The crate-landscape row
  `DESIGN.md:1018` stays true and stays.

**What must not change:** every bit anywhere — this unit changes what
is compiled, not what any door computes; the k-lint predicate counts
(read `docs/K-REPORT.md`'s method: if the sweep counts `Decide` sites by
source rather than by compiled configuration, nothing moves; if
compiling `Interval`'s `Decide` impl unconditionally adds names to a
default-build roster, say so with the before/after and why it is
right); the wasm32 check (`ci.yml`'s `cargo check --workspace … --target
wasm32-unknown-unknown` already passes `--features interval`, so the
backend already compiles for wasm — confirm the default wasm build does
too); GUARD's `scripts/gates/test-features-dev-only.sh` (read it: if it
asserts that `interval-transcendentals` is dev-only or optional, that
assertion is what the ruling retires — re-word the gate's expectation
in this PR and say so, do not delete the gate).

## 2. Docs

`crates/geom-core/README.md`'s scalar list, if it names the gate,
follows. One sentence each; nothing about the ring's future (RING-2/3
write their own).

## 3. The pin

- A row (a doctest on `geom_core::Interval` or a `crates/geom-core/tests`
  row NOT under `cfg(feature = "interval")`) that `Interval` is
  nameable, constructible and `Decide`-capable in a default build.
- `cargo build -p geom-core` (default) compiles the backend: the
  dependency graph is 85 crates where it was 84 (`cargo tree -e normal
  --prefix none | sort -u | wc -l`), stated in the PR body.
- D9: every suite green unchanged at default and `--features interval`;
  both `span_bit_identity` suites untouched; no render cell moves.
- The measurement, re-taken once on your box the way `interval-cost.md`
  did (clean `cargo build --workspace`, default, before and after),
  reporting not gating.

## 4. Sweep

The class: `cfg(feature = "interval")` sites in `crates/geom-core/src`
that gate the TYPE (go) versus the instantiation or a test (stay). List
every one of the crate's sites with its disposition in the PR body;
count the sites outside the crate (they stay) so RING-3 starts from a
number.

## 5. Fence

This program claims no paths. This unit reaches PROPS'
`crates/geom-core/{Cargo.toml,src/*}` and `crates/geom-core/README.md`,
CIW's `.github/workflows/ci.yml` only if a step must change (say why),
GUARD's `scripts/gates/*` if an expectation moves, `docs/DESIGN.md` (the
phrase) and `docs/GENERICS-BUILD-COST.md` (the addendum). Announced by
the orchestrator in `work/scalar/log.md` and on the PR. Merge
`origin/main` immediately before opening the PR; `python3
scripts/work.py territory --base origin/main` in the PR body.

## 6. Verification and report

Local: `cargo build -p geom-core` default; `cargo nextest run -p geom-core`
at default and `--features interval`; `cargo nextest run -p geom -p
topo -p geom-brep` at default; `cargo clippy --workspace --all-targets
-- -D warnings` at default AND `--all-features`; `cargo check
--workspace --exclude pncad --exclude pncad-py --exclude viewer --target
wasm32-unknown-unknown` at default AND `--features interval` (as
`ci.yml` does); `cargo fmt --all --check`; `scripts/doc-gate.sh`; every
script under `scripts/gates/` that names the crate or the feature.
Private `CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`; disk is shared.
Hosted CI is the verification of record; poll to conclusion in the
foreground and report the run id. Report ≤100 lines: the diff by file,
the cfg-site table of §4 with the outside-crate count, the two prose
re-wordings quoted, the gate scripts read and what each expects, the
measurement, the K-roster statement, deviations, rows filed.

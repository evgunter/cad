---
id: probe-interval-lane-has-no-clippy-row
kind: issue
title: the probe+interval feature combination has no clippy row anywhere, and four unused imports have accumulated in it
status: open
opened: 2026-09-04
---


Found during M10-7's fix pass, by running a lint the tree does not run.

## What is missing

`.github/workflows/ci.yml` carries thirteen `cargo clippy` invocations.
Their feature sets are: default (`ci.yml:1751`), `interval`
(`ci.yml:2748`), `viewer --features app` (`1488`), `pncad-py --features
python` (`3344`), `mesh --features budget` (`3783`), and the excluded
roots at their own defaults (`3060`, `3601`, `3604`, `3718`, `3732`,
`3751`). **None of them enables `probe`**, at any combination.

`probe` is not an inert feature: `scripts/k_probe_sweep.sh` builds the
whole K population under `--features probe,interval`, and
`scripts/gates/probe-suite-census.sh` exists precisely because a
`probe`-gated suite is invisible to every default row. That gate answers
"does it COMPILE"; nothing answers "does it lint".

## What has accumulated there

`cargo clippy --workspace --all-targets --features probe,interval --
-D warnings` fails on four unused imports, all in `geom-brep` test
files whose uses are fully qualified at the call site:

- `crates/geom-brep/tests/rim_dim_review_probes.rs:36` — `use
  geom_core::Tol;`, while both uses at `:91` and `:105` write
  `geom_core::Tol::witness()`
- `crates/geom-brep/tests/rim_dim_scale_twins.rs:61` — `use
  geom_core::Band;`
- `crates/geom-brep/tests/span_meter_dim_twins.rs:47` —
  `EdgeDescription` in the `geom_brep::{…}` list
- `crates/geom-brep/tests/span_meter_dim_twins.rs:51` — `Band` in the
  `geom_core::{…}` list

They are warnings under `cargo test`, which is what the sweep runs, so
the sweep's log has carried them for as long as they have existed and
nothing reads it. Under `-D warnings` they are errors.

The imports themselves are trivial. The point of the item is the row
that would have caught them: this is the same shape as M10-7's D12 —
a lane whose evidence nobody lints — and the fix is a clippy row over
the feature combination the sweep already builds, not four deletions.

## Recourse

Add a `probe`-enabling clippy row (`--features probe,interval` matches
what `k_probe_sweep.sh` compiles) to the `interval` job, which already
owns the interval lane's own clippy pass, and delete the four imports in
the same change. `scripts/check-ci-mirror-parity.py` will want the local
half in `local-scripts/ci-local.sh` alongside it.

Filed by M10-7 rather than fixed there: the four files belong to
geom-brep's dimension review lane, and the row is a CI-discipline change
that wants its own review.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/ciw/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

---
id: tour-scalar-trait-bundles-certifiedbounds-outside-the-allowlist-scan
kind: issue
title: demos/tour's Scalar trait bundles CertifiedBounds outside bounds-allowlist.sh's crates/*/src scan
status: open
opened: 2026-09-21
priority: P4
cost: E
---

## Finding

`scripts/gates/bounds-allowlist.sh` scans `crates/*/src` only
(`scripts/gates/lib.sh`, `gate_require_crate_sources`), so a compound
bracket bound written under `demos/` is invisible to it.
`demos/tour/src/scalar.rs`'s `Scalar` trait is one: it bundles
`pncad::geom_core::CertifiedBounds` as a supertrait beside
`AtRestPolicy` (LANE-1, PR 3010, deviation 5 — before that change it
bundled `Bounds` the same way, equally unscanned). Both tour impls
(`f64`, `Probe`) certify, so the bundle is honest today; what is
missing is the instrument. A tour scene at `Dual64` would need the
trait split first, and nothing would say so before the compile error.
Raised by the LANE-1 R1 review as a fresh instance of the class H5's
own sweep named (a compound bound outside the gate's scan root); the
lane filed it rather than widening the scan, which is GUARD's call —
`demos/` is a separate cargo root and the gate's roots are argued at
`lib.sh`.

## What to do

Either widen the scan root to the cargo roots `scripts/doc-gate.sh
--print-roots` derives (with an allowlist entry for `scalar.rs` citing
its ratification), or state at the gate why `demos/` is out of scope
and put the tour's bundle in a register the gate's prose names.

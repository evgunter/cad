---
id: offset-fit-mignitude-floor-on-norm-e
kind: issue
title: offset_fit small-|d| certificates are floored by the componentwise mignitude lower bound on ‖E‖, not by rounding
status: open
opened: 2026-08-31
github: 1320
refs: [1008, 1319]
---

## From GitHub issue 1320

opened 2026-08-31, 0 comments.

(S-CERT orchestrator) Filed from CERT-7 (PR 1319), whose recentring work measured issue 1008's hoped-for gain to its actual mechanism.

At `d = 1e-6` on the quarter cylinder, the sup-carrying cell's certified bound is 96% `τ²/‖E‖` (3.103e-4 of 3.222e-4), and the `‖E‖` lower bound is a **componentwise mignitude assembly reading 1.581e-8 where `‖E‖ ≈ |d| = 1e-6`** — each component of `E ≈ d·n` straddles zero as the normal rotates across the cell, so the componentwise floor collapses even though the vector's norm never leaves ~|d|. Recentring (issue 1008, landed) cannot reach this; it was never a rounding problem at the origin.

The proposal with digits behind it: a bound reading the three components *together* — the projection `‖E‖ ≥ |D|/(w̃·w³·sup‖m‖)`, which the composite already carries `D` for — is a small change to `cell_bound` and is the real content of what issue 1008 hoped for. The micron row (currently 3.222e-4 at tol 1e-9, refusing with `achieved = 3.791e-7` post-CERT-7) is the acceptance instrument.

S-CERT fence (`offset_fit.rs`); note CERT-10 edits this file later — sequence accordingly.

## Home

`work/cert/` — `crates/geom-brep/src/offset_fit.rs` is an S-CERT territory glob and the issue names the S-CERT fence, filed from CERT-7.

---
id: offset-fit-mignitude-floor-on-norm-e
kind: issue
title: offset_fit small-|d| certificates are floored by the componentwise mignitude lower bound on ‖E‖, not by rounding
status: closed
closed: 2026-09-15
opened: 2026-08-31
branch: props/mignitude-floor
pr: 2469
github: 1320
refs: [1008, 1319]
---

## From GitHub issue 1320

Opened 2026-08-31; 0 comments.

(S-CERT orchestrator) Filed from CERT-7 (PR 1319), whose recentring work measured issue 1008's hoped-for gain to its actual mechanism.

At `d = 1e-6` on the quarter cylinder, the sup-carrying cell's certified bound is 96% `τ²/‖E‖` (3.103e-4 of 3.222e-4), and the `‖E‖` lower bound is a **componentwise mignitude assembly reading 1.581e-8 where `‖E‖ ≈ |d| = 1e-6`** — each component of `E ≈ d·n` straddles zero as the normal rotates across the cell, so the componentwise floor collapses even though the vector's norm never leaves ~|d|. Recentring (issue 1008, landed) cannot reach this; it was never a rounding problem at the origin.

The proposal with digits behind it: a bound reading the three components *together* — the projection `‖E‖ ≥ |D|/(w̃·w³·sup‖m‖)`, which the composite already carries `D` for — is a small change to `cell_bound` and is the real content of what issue 1008 hoped for. The micron row (currently 3.222e-4 at tol 1e-9, refusing with `achieved = 3.791e-7` post-CERT-7) is the acceptance instrument.

S-CERT fence (`offset_fit.rs`); note CERT-10 edits this file later — sequence accordingly.

## Home

`work/cert/` — `crates/geom-brep/src/offset_fit.rs` is an S-CERT territory glob and the issue names the S-CERT fence, filed from CERT-7.

## Re-homed (2026-09-06)

Moved from `work/cert/` to `work/props/` on S-CERT's exit walk PR
(#1924, its handoffs ledger; merged by Ev 2026-09-06 = ratified), before
`work/cert/` was deleted at sweep 7 of `docs/DOC-LEDGER.md`. Id, body
and header are unchanged; the directory is the claim (`work/README.md`).
The `## Home` section above naming `work/cert/` is superseded by this
line and is kept as the record of why the file was filed there.

## Closed

Landed on PR #2469 (head `f050e2d6f`, run 35015861638 green on the full
matrix). `Composite::cell_bound` no longer floors `‖E‖` with the
componentwise mignitude assembly alone: the three components are read
TOGETHER through the sign witness the composite already carries,
`‖E‖ ≥ mig(D)/(w̃·sup‖M̃‖)` with `D = Ẽ·M̃`, and `e_lo` is the max of
that, the componentwise assembly and `|d| − dist`, feeding both the
`dist` divisor and the `τ²/‖E‖` term. `M̃`'s three channels are kept on
`Composite` instead of being discarded after `Y` and `D`.

The micron row, which this item named as the instrument: `e_lo`
1.5798e-8 → 5.6056e-7 at the sup cell (0.56·|d| where it read
0.016·|d|), the round-4 bound 3.2219e-4 → 1.7072e-5, and `τ²/‖E‖`'s
share of the sup 96.4% → 51.3%. Over the 70-request corpus 24 bounds
move, the largest by 807.7×, and eight requests change face. The
`d = 1e-9` arm still refuses at the sample cap, now at `3.7544e-7`:
one round finer the componentwise assembly recovers on its own and the
sup is carried by `τ`, the tangential term that divides by the
regularity floor — which is a different item's ground, and this one
does not touch it.

Residues filed rather than folded in: the door-level bound is not
monotone in the cell bound (one request's bound rose 1.8% through the
marking schedule — `offset-fit-door-bound-is-not-monotone-in-the-cell-bound`),
and `RefinementStalled` lost its only door-level fixture
(`offset-fit-stall-face-has-no-fixture`). The dual's own class finding,
an upper bound on a norm assembled by an `f64` fold of ring endpoints
and used as a divisor, is fixed here in `offset_fit` and filed on the
owning slates for `offset_meters::cell_normal` (SHELL) and
`ssi/certify.rs`'s `stretch`, now `Box3::speed_sup` (TRIM).

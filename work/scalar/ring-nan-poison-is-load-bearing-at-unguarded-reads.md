---
id: ring-nan-poison-is-load-bearing-at-unguarded-reads
kind: issue
title: The ring's NaN poison is load-bearing at 17 unguarded reads: RING-2's decoration swap launders a refusal into a pass there
status: open
opened: 2026-09-21
priority: P2
cost: D
refs: [H5]
---


## What

RING-0's deliverable (c) swept every production read of one side of a
`RingInterval` bracket. Most are guarded — the site asks `is_poison()`
first, and `is_poison()` survives H5 ruling 1 cut (ii) as `dec < Def`.
**Seventeen are not**, and each of them refuses today only because a
poisoned endpoint is NaN and every comparison against NaN is false. A
`DInterval` at `Trv` carries REAL endpoints, so at each of these the
same code takes the certifying branch on a value that may not certify.

Two shapes.

**(1) One side read without asking poison — 11 sites.** The refusal
IS the NaN comparison.

- `geom-brep/src/offset_meters.rs`, `mig` — `i.lo() > 0.0` /
  `i.hi() < 0.0`, documented as answering `0.0` for poison "whose
  comparisons are all false". A `Trv` half-line answers a nonzero
  mignitude.
- `geom-brep/src/props/quad.rs`, three: the area gauge's fallback arm
  (`area.lo() > 0.0`) and the two rational-weight gates
  (`g_w.lo() <= 0.0 || !g_w.lo().is_finite()` and its per-cell twin).
  The `is_finite` arm catches NaN and ±inf, not a finite `Trv`
  bracket.
- `geom-brep/src/offset_fit.rs`, `cell_bound`, three: `w_lo` and
  `wt_lo` bound to locals and tested `!(w_lo > 0.0) ||
  !w_lo.is_finite()`, and the sign witness `dh.lo() > 0.0` /
  `dh.hi() < 0.0` with no guard at all.
- `mesh/src/nurbs_cert.rs`, `cell_component` — `sq.hi()` consumed as
  `hi.sqrt().next_up()`, with "poison answers NaN, which every
  consumer treats as unbounded/poisoned" as the stated contract.
- `mesh/src/chords.rs`, two — the `sum_sq.hi().sqrt().next_up()` and
  `s.hi().sqrt().next_up()` collapses, same contract.
- `topo/src/props.rs`, the exact-structure read
  `x.lo() == x.hi() && x.lo().is_finite()`.

**(2) A ring value re-minted from its own endpoints — 6 sites.**
`RingInterval::from_bounds(x.lo(), x.hi())` is poison-preserving only
because the endpoints are NaN; under cut (ii) it mints a `Com`
bracket out of a `Trv` one, which is laundering of exactly the kind
`from_certified` exists to prevent.

- `geom-brep/src/props/quad.rs`, three: `cos_step`'s and `sin_step`'s
  `from_bounds(lo.lo(), hi.hi())`, and `sin_step`'s sign mirror
  `from_bounds(-unit.hi(), -unit.lo())`.
- `geom-brep/src/ssi/enclose.rs`, two: `Box3::split`'s `half`
  closure, `from_bounds(i.lo(), m)` and `from_bounds(m, i.hi())`.
- `topo/src/props.rs`, one: `trig_at_start`'s `clamp` closure,
  `from_bounds(x.lo() - pad, x.hi() + pad)`.

`quad.rs`'s `widen` has the same shape and is **not** in the list: it
asks `is_poison()` first, which is what every site above should do.

## Disposition

Not RING-0's to fix: that unit changes no `src`, and the repair is a
decision about each door's contract that belongs with the newtype
itself. The fix at every site is one shape — ask the refusal first
(`is_poison()`, or `from_certified` where a value is being re-minted)
instead of relying on NaN. The RING-0 dry run
(`scalar/ring-0-dry-run`) reached **none** of these with a red row,
which is why they need naming rather than reading off a failure list.

Whether each can follow a ring DIVISION is the narrower question the
survey asked: by reading, `mig`, the weight gates and `cell_bound`
are fed by coefficient hulls (`PatchGrid::chan`,
`CellHulls::cell_hull`) that are sums and products only, while
`quad.rs`'s quadrature area and `trig_at_start` do divide. The
laundering does not need a division, though — any `Trv` reaches it,
and after cut (ii) a domain clamp anywhere upstream produces one.

Sweep pattern, and its blind spot: a comparison with `.lo()`/`.hi()`
on one side, plus an endpoint bound to a local and compared there,
plus a bare one-sided read consumed arithmetically, over the files
naming `RingInterval` and above each file's `#[cfg(test)]`. It cannot
match a ring endpoint handed to a helper that takes `f64` — the value
is no longer a ring value where the comparison happens — nor a read
inside a macro body.

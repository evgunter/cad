---
id: the-pinned-profile-lift-launders-at-sym
kind: issue
title: The pinned profile lift enters profile geometry as constants at Sym, and the re-valuation rows run it, so profile's generic code never reaches them
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [the-from-f64-gate-reads-every-generic-crate]
---

`ProfileLift::Pinned`, the `EvalOptions` default (`eval/mod.rs`), lifts the
profile's nominal `f64` elaboration through `from_f64`. The two lifts are
`eval/wire.rs`'s `placement.map(T::from_f64)` and `profile/src/validate.rs`'s
`ValidatedSegment::lift` and `ValidatedLoop::lift`. Both are listed
`launders(the-pinned-profile-lift-launders-at-sym)` in
`scripts/gates/from-f64-sites.tsv`.

At `Sym`, every profile length is therefore a `Lit`, while the same
variables are symbols in the node magnitudes (an extrude distance). A
`Zero` decided over that mix is a theorem about a function the document
does not compute.

`drive::lane_opts` and the Monte Carlo lane choose `Guided`, which
elaborates the profile at `T`. Nothing stops another `Sym` evaluation
from taking the default. `crates/editor-core/tests/revalue_corpus.rs`'s
`options` takes it, and so do the gating rows and the census:
- Both builds resolve the profile at the nominal
  (`program.resolve(op_env.lane.nominal)`), so they agree, and no row
  can see a laundering in the profile lowering.
- `profile`'s generic code, the lowering the symbolic lane registers
  arcs through, is never run at `Sym` by the instrument.
- The in-step move is not a dilation of the document, because a
  profile length does not move under it. That is why `plate_param`'s
  `bool_join_nearest` distance is exactly zero at `p0` and moves at
  `p1` under both moves.

Owed:
- The pinned lift refuses, or is not offered, at an analysis scalar.
- The gating rows and the census run `Guided`, each row's move and
  floor re-pinned. A row that parts where it compared before is read
  as a finding, not dropped.

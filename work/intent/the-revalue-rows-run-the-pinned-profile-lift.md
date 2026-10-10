---
id: the-revalue-rows-run-the-pinned-profile-lift
kind: issue
title: The re-valuation rows and census evaluate under ProfileLift::Pinned, so profile geometry never reaches them at Sym; the symbolic lane runs Guided
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [the-from-f64-gate-reads-every-generic-crate]
---


`crates/editor-core/tests/revalue_corpus.rs`'s `options` builds
`EvalOptions { param_box, ..EvalOptions::default() }`, and the default
`profile_lift` is `ProfileLift::Pinned` (`eval/mod.rs`): profile
geometry is the nominal `f64` elaboration lifted through `from_f64`
(`eval/wire.rs`'s `placement.map(T::from_f64)`, `profile/src/validate.rs`'s
`ValidatedProfile::lift`). Both builds resolve the profile at the
nominal (`program.resolve(op_env.lane.nominal)`), so they agree, and no
row can see a laundering in the profile lowering.

The symbolic lane the stage 4 door reads runs `ProfileLift::Guided`
(`drive::lane_opts`), which elaborates the profile at `T`. So the
`profile` crate's generic code (about 55 `from_f64` occurrences, all
listed `constant` in `scripts/gates/from-f64-sites.tsv`) is never run
at `Sym` by the instrument.

Owed: run the gating rows and the census under `Guided` too, and re-pin
each row's move and floor. Under the pinned lift a profile length does
not move, so the in-step move is not a dilation of the document. A row
may part where it compared before; it is then read as a finding, not
dropped.

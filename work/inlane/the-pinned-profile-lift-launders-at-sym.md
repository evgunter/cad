---
id: the-pinned-profile-lift-launders-at-sym
kind: issue
title: The pinned profile lift enters profile geometry as constants at Sym, and the re-valuation rows run it, so profile's generic code never reaches them
status: closed
opened: 2026-10-10
priority: P2
cost: M
refs: [4596]
closed: 2026-10-17
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

## Lofts are now in this row's family (EDIT, 2026-10-11, PR 4741)

A loft reads its sections' profile values, so under the pinned lift it
launders the way an extrude does. A `Dual` seed on a parameter a
section reads gives a zero derivative where main refused typed
(`SeedPinnedSection`, retired), and a box over a section lofts the
nominal. Under the guided lift both are correct: the seed rides to the
walls, and the box lofts the family it spans
(`crates/editor-core/tests/eval10_section_reads_the_nominal.rs`).


## Closed

Closed by ruling A of the INLANE orchestrator: the evaluator derives the lift, and `ProfileLift` leaves `EvalOptions`.

**The lift.** `EvalOptions::profile_lift_over(env)` reads it off what the evaluation binds, and the evaluator takes exactly that:
- **Guided** when the evaluation's box varies an axis (`[0, 0]` included), it carries a seed, it binds every variable a symbol, or its environment binds any variable as a parameter symbol: an ANALYSIS. `EvalOptions::profile_lift()` is the options' half of this.
- **Pinned** otherwise, at any scalar: the build.

The rule reads what the environment binds. An empty or all-`Fixed` box holds every variable it names at its nominal: at every numeric scalar it is the build, and an unused box seam changes no decision. At `Sym`, a `Fixed` axis binds a CONSTANT (`analysis::var_env_over` sends only a VARYING axis through the identified door), the INLANE orchestrator's option (d), adopted after PR 5248's review found the all-`Fixed` box laundering at `Sym`. A toleranced variable the box does not name stays its id's symbol there (VR8). The environment records whether it binds any symbol (`VarEnv::binds_symbols`), and one that does is guided whatever the box's shape. A consumer that wants the nominal as a symbol asks for it with `Varying { lo: 0, hi: 0 }` (`ParamBox::at_nominal`) or with `symbols`. The M10 harness's `nominal_box`, stackup's worst-case hull prior and the M10.7 evidence probes are restated that way.

**Where analyses meet the build:**
- `ParamBox::at_nominal()` (every varying axis spanning `[0, 0]`) makes three evaluations read the document as their leaves do:
  - the drive's witness;
  - Monte Carlo's nominal build;
  - stackup's unseeded base.
- A part runs at its own nominal, so it is pinned under an analysis.
- The coincidence door and the revalue instrument share one replay session (PR 5247), whose replays are guided: they bind every variable a symbol or vary a box.

**What else changed with the field:**
- PP6 (`crates/editor-core/README.md`) says the evaluator chooses.
- The Python surface loses the field.
- The two `from-f64-sites.tsv` sites that were `launders(...)` are `constant`: the build's `f64` elaboration, run only where every variable is bound at its nominal.

**What the guided reading turned up** (each filed, each row naming its item):
- The revalue rows' findings: PR 5247's three items.
- `work/carve/the-guided-derived-frame-extrude-escalates-as-a-kernel-defect.md`.

## After Ev's ruling A on #5068 (2026-10-14, merged 2026-10-19)

Ev ruled that the pinned lift retires outright: every evaluation elaborates
profile geometry at its own scalar, guided by the `f64` record, and the `f64`
lane over the nominal takes the first pass's validated form internally
(`editor_core::ProfileLane`). The derived lift above (`profile_lift_over`) left
with it, and so did the frame's way up (`RigidMotion::embed`), so no evaluator
site enters the `f64` elaboration through `from_f64`. The one residue,
`ValidatedProfile::lift_onto` in sweep's loft at an enclosure scalar, is filed
as `the-loft-enclosure-arm-re-enters-its-sections-canonical-form-as-constants`.

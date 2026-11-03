---
id: onedoor
kind: program
title: ONEDOOR — one boolean door with two homes - the description ladder, the operand gates, the sweep trace, the chart walk, the carrier door's placement, and the shared cores of the boolean and split pipeline
status: active
opened: 2026-11-02
group: kernel/boolean/one-home
prefix: onedoor/
tag: (ONEDOOR orchestrator)
paths: [crates/topo/src/boolean/*, crates/topo/src/splitting/*, crates/topo/src/chord_join.rs, crates/topo/src/census.rs, crates/topo/src/validate.rs]
keep_out: [opened 2026-11-02 by BOXES' cut on its semantic seam per work/README.md Track size - BOXES measured 62 budget points against 30 and kept the box and extent rows (the region a face or edge can occupy), these rows are its other one-home doors and ask nothing of the extent question - every row moved by git mv with its id and body unchanged - this track shares crates/topo/src/boolean/* splitting/* and chord_join.rs with BOXES ONEHOME ONEREAD CLEAVE and the HONE family by design, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled - ONEHOME holds rows of the same class from CLEAVE's residue and is the place to look before filing a new one here - CORES (TOPO's 2026-10-26 cut, PIPE's two P1 rows) merged in 2026-11-02 when the budget came to count only P0 P1 and unbanded rows (docs/doc-ledger/cores-merges-into-onedoor.md) - its two rows move shared code that several boolean and split programs have live units in, so neither is dispatched before two designers weigh the layering question (crate-root shared cores, two peer lanes, docs/DESIGN.md's Layering topo row) and an [ev] PR opens it if it is a fork - a layering decision ratified after the diff is one the diff made]
budget: 31
---

**One door, two homes, outside the box rules.** The description
staleness ladder is spelled at the split, the boolean and tier 3; the
operand-kind gate twice; the sweep trace as two types; the chart-window
walk lives in `chord_join.rs`, whose chords no longer read it; the
carrier door's curved half sits inside `boolean/`; `combine.rs` answers
one proof two ways; a face's shell is scanned for where a back-pointer
answers.

**The boolean and split engines as one pipeline** (from CORES, merged
2026-11-02): the two engines in `crates/topo` share a pipeline and two
cores (`finish` and `plane_crossing_lane`), and the shared code is
hosted inside one half (`topo-shared-cores-hosted-in-one-half`); the
pipeline is built twice (`S5`, parked on it). It is this track's
question at the scale of the engines: one logic, and the home it
belongs in.

P1 by `work/README.md`'s Priority: two implementations of one logic
entrench with every new caller.

**Budget 31, not 30.** ONEDOOR and CORES measured 23 and 8 points when
the budget came to count only P0, P1 and unbanded rows. Ev accepted a
coherent pair landing just over the ceiling when one of them is under
12, as preferable to leaving the pair apart (in chat, 2026-11-02).

Before dispatching a row, sort it the way ONEHOME's plan does: a copy
that exists because nothing says where the logic belongs is a design
signal (designers first), not a deletion for a lane to choose.

Slate: `work/onedoor/plan.md`; narrative in `work/onedoor/log.md`.

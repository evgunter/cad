---
id: restfront
kind: program
title: RESTFRONT — what tier 3 passes at rest that it should refuse: a face's region on the plane and off it, and a body's shells and solids
status: ready
opened: 2026-09-27
group: kernel/topo/certify
prefix: restfront/
tag: (RESTFRONT orchestrator)
paths: [crates/topo/src/validate.rs, crates/topo/src/tier3_tests.rs]
keep_out: [split on 2026-10-29 into RESTFRONT RESTCURVE RESTSHELL RESTGATE RESTDATUM RESTWORD, all on validate.rs - the files are shared and the questions are not, so a lane announces the seam on a sibling's log only where the changes are coupled - opened by ATREST's close (docs/doc-ledger/atrest-leaves-the-tracker.md) on ATREST's ground, and inherits its seam - TOPO keeps the Euler operators proper and the sibling cuts are ORIGIN TQUERY WALKS PROBE, a row here that has to edit another program's ground announces the seam in the PR that lands it where the changes are coupled rather than drawing a second fence, RESTCURVE merged in on 2026-11-02 (docs/doc-ledger/restcurve-leaves-the-tracker.md) - its notes: split from RESTFRONT on 2026-10-29 and shares validate.rs with its five siblings RESTFRONT RESTCURVE RESTSHELL RESTGATE RESTDATUM RESTWORD - so a lane announces the seam on a sibling's log only where the changes are coupled (work/README.md Territory) - ATREST owns the census (topo census.rs) and TOPO the Euler operators - FOREIGN owns import's normalization of a chart singularity (import-normalizes-the-rim-only-cap - closed) and filed the wrap-tip grow on the P0 from [ev] PR 6014, RESTSHELL merged in on 2026-11-02 (docs/doc-ledger/restshell-leaves-the-tracker.md) - its notes: ATREST's census is tier 3' - and two rows here are a tier-3 reading of what the census already refuses, RESTDATUM merged in on 2026-11-20 (docs/doc-ledger/restdatum-restgate-and-restword-merge-into-restfront.md) - it held no P0 or P1 row, from RESTDATUM: split from RESTFRONT on 2026-10-29 and shares validate.rs with its five siblings RESTFRONT RESTCURVE RESTSHELL RESTGATE RESTDATUM RESTWORD - the files are shared and the questions are not, from RESTDATUM: so a lane announces the seam on a sibling's log only where the changes are coupled (work/README.md Territory) - ATREST owns the census (topo census.rs) and TOPO the Euler operators - geom's convention module (frame_margins) and geom-brep's EdgeCurve::certify are read by these checks and are other programs' ground, RESTGATE merged in on 2026-11-20 (docs/doc-ledger/restdatum-restgate-and-restword-merge-into-restfront.md) - it held no P0 or P1 row, from RESTGATE: split from RESTFRONT on 2026-10-29 and shares validate.rs with its five siblings RESTFRONT RESTCURVE RESTSHELL RESTGATE RESTDATUM RESTWORD - the files are shared and the questions are not, from RESTGATE: so a lane announces the seam on a sibling's log only where the changes are coupled (work/README.md Territory) - ATREST owns the census (topo census.rs) and TOPO the Euler operators - props.rs's at-rest policy and the operand-gated doors (boolean reduce, from RESTGATE: split, from RESTGATE: blend) read this gate, from RESTGATE: so a change to what it admits is announced to their programs, RESTWORD merged in on 2026-11-20 (docs/doc-ledger/restdatum-restgate-and-restword-merge-into-restfront.md) - it held no P0 or P1 row, from RESTWORD: split from RESTFRONT on 2026-10-29 and shares validate.rs with its five siblings RESTFRONT RESTCURVE RESTSHELL RESTGATE RESTDATUM RESTWORD - the files are shared and the questions are not, from RESTWORD: so a lane announces the seam on a sibling's log only where the changes are coupled (work/README.md Territory) - ATREST owns the census (topo census.rs) and TOPO the Euler operators - geom-core's MarginDiag and geom-brep's recourse module are the one home of the ending rules (ENCL's), from RESTWORD: and a row here moves validate onto them rather than adding a rule]
---
**Whether a planar face's loops bound the region they claim.** ATREST
made the at-rest validator right in both directions on the ground it
reads. Every row here is a planar face that tier 3 still passes: a ring
inside another ring, a loop that meets itself, a zero-area slit or
membrane, a carrier check 6 does not wind, or check 9's line and arc
arms answering wrong at the edge of their band. The same questions off
the plane, and across shells and solids, came in with RESTCURVE and RESTSHELL (below).

Charter and order: `work/restfront/plan.md`; narrative in
`work/restfront/log.md`.

## Merged in 2026-11-02

RESTCURVE asks RESTFRONT's region question on a curved chart, and RESTSHELL asks it across shells and solids; every row of the three is a body tier 3 passes that it should refuse, on `validate.rs`. The track budget now counts only dispatchable P0, P1 and unbanded rows (`work/README.md`, Track size), so these tracks had fallen far under it; their rows moved here by `git mv` with ids and bodies unchanged.

**From RESTCURVE** (RESTCURVE — the face region off the plane: chart singularities, curved containment, curved meeting arms):

**Where a face's region is read on a curved chart.** RESTFRONT reads
a planar face's loops as a region: winding, ring nesting, meeting arms,
corners. Every row here is the same question on a curved surface or a
chart with a singularity, where that reading is silent today: a pole or
apex inside a face (Ev's rule, now DESIGN `[topology]`'s), a curved
face's loops bounding a region of its surface, check 9's meeting and
corner arms off a plane.

**From RESTSHELL** (RESTSHELL — solids and shells at rest: checks 7 and 10, overlapping material, orientation):

**What tier 3 reads across a solid's shells and a body's solids.**
Check 7 (each solid's sign) and check 10 (shell winding 0/1) answer
only where their walks answer, and nothing in tier 3 reads across two
solids. Every row here is a body tier 3 passes for that reason, an
early stop that hides a finding, or the cost those checks pay.

## From RESTDATUM (merged 2026-11-20)

RESTDATUM — checks 1 and 4: a carrier read against its datum and against its faces

**How tier 3 reads a carrier.** Check 1 reads each carrier's datum
(frames, unit conventions, D2's declared/derived chart) and check 4
reads an edge's carrier against its faces (the material arm, the
transverse/smooth classification, the carrier's ends). Every row here
is a convention certified nowhere, a band that reads the wrong
quantity, or an edge class the arm never judges.

## From RESTGATE (merged 2026-11-20)

RESTGATE — what the at-rest gate admits a finished body and an operand on

**The gate, not the checks.** `AtRestBody` and `gate_unverdicted`
decide which tiers a finished body or a dual's operand must pass, and
what verdict rides with it. Each row here is a body admitted because the
gate asks less than the checks can say, or a verdict whose provenance
the gate does not keep.

## From RESTWORD (merged 2026-11-20)

RESTWORD — how validate decides and words its refusals: one funnel, one ending per reading

**The endings.** D4 ¶1 says a refusal's ending follows from its
reading's class, and ENCL gave that rule one home. `validate.rs` still
classifies and words several refusals by rules of its own: a lead keyed
on a variant, a `&'static str` that cannot carry a margin, a
hand-minted `INVALID`, a poisoned margin ended two ways, k_stats called
past topo's own funnel. Each row moves one site onto the shared reading,
or asks the ruling that says which reading is true.

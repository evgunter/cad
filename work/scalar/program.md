---
id: scalar
kind: program
title: SCALAR — the scalar doors and the readers over them: the certified-door family, margins and their recourse, unit directions, and the box driver's readings
status: active
opened: 2026-10-10
group: kernel/geom/numeric
prefix: scalar/
tag: (SCALAR orchestrator)
ab_band: 12400-12499
paths: [crates/geom-core/src/predicate.rs, crates/geom-core/src/real.rs, crates/geom-core/src/k_stats.rs, crates/geom-core/src/tolerance.rs, crates/geom-core/src/linalg/unit_vec.rs, crates/geom/src/curves/boxes.rs, crates/editor-core/src/drive.rs, crates/editor-core/src/analysis.rs, crates/editor-core/tests/m10*, crates/editor-core/tests/e4_dual*, docs/ERROR-DESIGN.md, docs/DUAL-DESIGN.md, crates/geom-core/src/linalg/frame.rs]
keep_out: [opened 2026-10-10 by FLUX's priority-seam cut (FLUX measured 125.5 budget points against 30 - work/README.md Track size) - the rows moved by git mv with ids and bodies unchanged and FLUX kept the curved closed-form arms. Shared ground is expected (Ev in chat 2026-09-20) - crates/geom-brep/src/props/* is shared with QUAD and FLUX's successors CONEARM NEARPOINT SPIRICARM SPANFOLD and MEASUREDOOR, crates/geom-core/src/* with FLUX's siblings KNOT SCALAR and with FRAME NURBS SYM ENCL - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, FRAME merged in on 2026-11-20 (docs/doc-ledger/frame-merges-into-scalar.md) - it held no P0 or P1 row, from FRAME: CUT FROM PROPS 2026-09-20 with Ev's agreement in chat — the family's head is PROPS' sign-hull unit (PR #2468, from FRAME: Ev's option-1 ruling on #1944), from FRAME: which PROPS lands and this program inherits the rest of the family from, from FRAME: exactly as PROPS inherited from S-CERT at its exit walk, from FRAME: and it has landed (the sign hull in Vec3::orthonormal_basis); crates/geom-core/src/* and crates/geom/src/* stay PROPS' paths and this program reaches them by announced seam, from FRAME: which is the honest record of a family whose code is spread across two crates' door sets; SHARED GROUND IS EXPECTED and is not a conflict (Ev, from FRAME: in chat, from FRAME: 2026-09-20) — what is owed is awareness when a lane is live on the same file]
---

**The scalar doors, and every reader that spells their contract by
hand.** `CertifiedEnclosure`'s unswept fourth implementor and a bracket
that holds its postcondition by hand; `sqrt_up` re-spelled; the
mignitude and refuse-then-read readers written twelve times; the
sampler's error with three spellings; unit-direction doors that drop a
decided margin; an interval `sin θ` that loses the shared magnitude;
the K floor a band-decided ordering needs; an invalid margin that
cannot say "not implemented"; float noise in `PathError`'s text; and
the box driver's readings of a decided gate rejection, its stored axis
order and its race row's witness.

Slate, order and posture: `work/scalar/plan.md`; narrative in
`work/scalar/log.md`.

## From FRAME (merged 2026-11-20)

FRAME — who answers "give me a frame", and by what rule

One question asked eight ways: who answers "give me a frame", and by
what rule. The kernel has at least four doors that turn a direction
into a basis — `Vec3::orthonormal_basis`, `path_start_frame`,
`frame_from_unit_aim`, the exact-frame mints — and no rule saying
which a caller should reach for, what each guarantees, or what any of
them does when the direction is not decided.

Cut from PROPS on 2026-09-20. The family's head is PROPS' sign-hull
unit: Ev ruled (option 1, on #1944) that the frame constructor crosses
the normal with a decided world axis and transfers no sign, and that
unit is built and holding on a fold SYM owes it (see
`work/sym/the-decision-door-is-opaque-to-the-tier.md`). PROPS lands it;
this program takes the rest of the family from that point. **Read the
sign-hull unit's PR and that SYM row before specifying anything**: they
establish that a frame chosen by a DECISION is opaque to the symbolic
tier, which is a constraint on every door this program might mint.

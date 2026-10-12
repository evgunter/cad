---
id: the-box-driver-carries-no-part-resolver
kind: issue
title: The box driver carries no part resolver, so no drive, and so no stackup, runs over an assembly
status: parked
priority: P2
cost: M
design: true
opened: 2026-10-03
refs: [MSOLVE-14, a-box-independent-mate-fault-bisects-the-whole-leaf-budget]
blocked_on: [intent-stage3-is-built]
---

## Finding

Found by MSOLVE-14's fix pass (PR 3986), which threads a part resolver
through `stackup::sensitivities` and `stackup::stackup` (the anchor,
every `Dual64` pass, and each leaf replay of `bind_verdict` and
`worst_case`, through `leaf_opts`), so both doors evaluate a document
that instantiates parts.

`stackup()` takes a `ParamBoxVerdict`, and the only producer of one is
`drive::drive`, whose evaluations (`drive::lane_opts`, `classify`)
carry no resolver: `DriveConfig` has no field for one. Over any
assembly the drive's witness does not build:
`DriveRefusal::WitnessDoesNotBuild`, its cause the mate's `Unleverable`
"no part resolver was given" (pinned by
`msolve11_mate_log::a_box_run_over_an_escalating_mate_refuses_at_its_witness`).
So no verdict, and so no stackup report, exists over an assembly, though
the stackup door's own evaluations now resolve its parts.

## Why it is not a one-line field

`DriveConfig` derives `PartialEq, Eq` and `Debug`, and an
`Arc<dyn PartResolver>` has no equality; the verdict's leaves would also
have to record which resolver they were certified over for `stackup`'s
tie (`stackup::bind_verdict`) to replay them over the same one. Where the
resolver lives (the config, the call, or the verdict) is the design
question.

## Re-homed from FLUX to FLUXHOLD (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXHOLD holds FLUX's rows on the D10 hold (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`). Each waits on the INTENT unit that rebuilds its ground, named in `blocked_on`. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

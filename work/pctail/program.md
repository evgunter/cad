---
id: pctail
kind: program
title: PCTAIL — the P3 tail of pcurve certification - its refusal vocabulary, its guards, and the coverage it claims
status: ready
opened: 2026-10-01
area: kernel
prefix: pctail/
tag: (PCTAIL orchestrator)
paths: [crates/geom-brep/src/pcurve_cache.rs]
keep_out: [cut from PCERT along its priority seam on 2026-10-01 per work/README.md Track size - PCERT keeps the P0 and P1 spine and runs it, and this track holds the P3 and P4 rows behind it - both claim crates/geom-brep/src/pcurve_cache.rs and that shared ground is legitimate by the README's 2026-09-20 rule, so run scripts/work.py territory on your branch and announce the seam in the PR while PCERT is active]
priority: P3
---
**What `PcurveCertifyError` says when it refuses, and the guards and
coverage claims around the certifier.** The spine of this ground — that
`validate_pcurves` can answer a clean bill on a body whose
certification failed — is PCERT's. This track is what stands behind it:

- **The refusal vocabulary.** Seven of `PcurveCertifyError`'s arms stop
  at the condition, two of them through `&'static str` payloads minted
  at fourteen sites in two crates; its two escalation arms offer a
  coincidence declaration the boundary's own fit has no object for; one
  arm runs past the viewer's word budget.
- **The guards.** The pcurve posture guard walks only `&mut Body`
  doors, so every `&self -> Body` producer's row posture is prose; an
  SSI seam refusal is unreachable by construction and its acceptance
  row asserts a forced identity; the sole-`T: Bounds` doors the D1
  census enumerated in `geom-brep` are undisposed.
- **A coverage claim with no caller**: the LINE-carrier seam limb's
  partial and rational column corners.

Charter and order: `work/pctail/plan.md`; narrative in `work/pctail/log.md`.

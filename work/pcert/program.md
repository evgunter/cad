---
id: pcert
kind: program
title: PCERT — pcurve certification: what validate_pcurves actually checks, and the refusal vocabulary, guards and coverage claims around the certifier
status: active
opened: 2026-09-20
group: kernel/topo/certify
prefix: pcert/
tag: (PCERT orchestrator)
ab_band: 7600-7699
budget: 36
paths: [crates/geom-brep/src/pcurve_cache.rs]
keep_out: [opened by CHART's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CHART measured 105 budget points and was cut into four tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CHART SSI PCERT ISO and they share crates/geom-brep by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled rather than drawing a fence, TRIM keeps nurbs_iso.rs and docs/PCURVE-P2-SPEC.md until its exit, SHELL owns editor-core/src/clearance.rs, TESS and CHORD own crates/mesh, QUAD owns props/quad/, PCTAIL merged in on 2026-11-02 (docs/doc-ledger/pctail-leaves-the-tracker.md) - its notes: cut from PCERT along its priority seam on 2026-10-01 per work/README.md Track size - PCERT keeps the P0 and P1 spine and runs it - and this track holds the P3 and P4 rows behind it - both claim crates/geom-brep/src/pcurve_cache.rs and that shared ground is legitimate by the README's 2026-09-20 rule - so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled while PCERT is active]
---
**A certifier that answers a clean bill on a body whose certification
failed.** `validate_pcurves` is the door the rest of the kernel reads
to know a body's pcurves are sound; three of this program's rows say it
cannot tell success from failure — it answers `Ok` after a mint that
failed, skips its re-certification and continuity passes on any face it
finds incomplete, and reads a face a door EMPTIED exactly as it reads
one never minted.

Beside them, `chart_stretch_sup` answers unit arms for a placeholder
chart while its sibling refuses — a bound that is a placeholder wearing
a bound's name — and `Pcurve::chart_box` is twice the true span of a
Harmonic image.

P0 because a certifier that cannot fail is worse than no certifier:
the kernel is built to trust it.

Charter and order: `work/pcert/plan.md`; narrative in `work/pcert/log.md`.

## Merged in 2026-11-02

PCTAIL was cut from PCERT on its priority seam (2026-10-01) and holds the P3 and P4 rows behind PCERT's spine on the same file. Ev, in chat 2026-11-02: dumping PCTAIL into PCERT is ok, since adding rows to a track with a live orchestrator is safe. The track budget now counts only dispatchable P0, P1 and unbanded rows (`work/README.md`, Track size), so these tracks had fallen far under it; their rows moved here by `git mv` with ids and bodies unchanged.

**From PCTAIL** (PCTAIL — the P3 tail of pcurve certification - its refusal vocabulary, its guards, and the coverage it claims):

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

`budget: 36` (2026-11-02): PCTAIL's merge adds 6 points to PCERT's 30. Ev, in chat 2026-11-02: dumping PCTAIL into PCERT is ok, because adding rows to a track with a live orchestrator is safe (taking rows away from one may not be), so the budget is set to the merged load rather than the track being split.

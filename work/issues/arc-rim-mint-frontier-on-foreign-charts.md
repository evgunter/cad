---
id: arc-rim-mint-frontier-on-foreign-charts
kind: issue
title: imported-chart arc-rim construction picks uniform breaks that mismatch a foreign wall's own arc spans
status: open
opened: 2026-09-04
priority: P0
cost: H
---


TRIM territory (no live TRIM orchestrator until CURVED's exit; filed
in `work/issues/` per the cross-program rule).

The imported-chart arc-rim construction — `topo/src/pcurves.rs`, the
arm at :621 as of EXCH-H1's merge base, "one span per sub-arc" via
`uniform_breaks` — selects breaks from the CHART's span count, not the
rim's own arc structure. dm1 wall `#382` states four u spans while its
rim circles are three-arc rationals: the construction picks 4 breaks,
the minted image mis-tracks the rim, and certification refuses
`MapResidual`. Newly REACHABLE as of EXCH-H1 (#388): it sat behind
`#389`'s zero-candidate gap at every band; the unit's l-bracket
witness (`step-import/tests/r1_dm1_probe.rs`,
`the_l_bracket_alone_adopts_its_reversed_slit`) now gets past every
polyline edge and pins this refusal as dm1's successor frontier. The
fail mode is a typed refusal, never a wrong body. TRIM-shaped ground:
the arc/IsoArc lane owns the construction; the fix is breaks read off
the rim's own knot structure (or the wall's stated spans reconciled
against them) rather than a uniform count.

Found by the EXCH-H1 lane (PR #1798); routed here by its fix pass.

## Consumer note (2026-09-17, EXCH orchestrator)

The preamble's "no live TRIM orchestrator" is stale — TRIM is open
for dispatch since 2026-09-04. This row is now dm1's ONLY refusal at
every band (the flux stall retired by TCOST-K3's sign certificate;
the `#389` gap retired by EXCH-H1), so it is the single blocker on
dm1 going first-class: `WILD_IMPORTS` 9→10, `WILD_REFUSALS` 4→3,
three tier_gate cells Refused→Pass, and `r1_dm1_probe`'s Pcurves
arms invert. EXCH consumes and will wire the flip's pins when it
lands; both defect files are TRIM's paths, so the unit is TRIM's to
cut. Signed: (EXCH orchestrator)

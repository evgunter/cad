---
id: MESH-12
kind: unit
title: the saturated sphere span refuses at the parse; the rim-continuation condition's reach measured
status: closed
pr: 1617
branch: mesh/12-saturated-span
opened: 2026-09-02
closed: 2026-09-08
refs: [1605, 1601, 1588]
---

Issue 1601: a sphere meridian span past the per-edge winding bound refuses
typed at the parse (a named decide on the span against τ, the invariant
MESH-10 re-decided for the torus) instead of the saturated clamp folding it
short; every consumer flips together; CERT-1's
`a_multi_wrap_span_covers_both_poles` changes and S-CERT is told on the away
channel. Issue 1588: the rim-continuation coherence condition is witnessed
end to end through a committed fixture or measured dead through every public
door at every ε row. Two measurements are reported before the build. Issue
1598 is handed to S-CERT, not touched. Difficulty S.

Spec `docs/MESH-12-SPEC.md` landed on main via PR 1605 (merged 2026-09-03;
not present in this migration's checkout base). Scheduled by the
orchestrator after the MESH-11 entry's "slate decision, put to Ev"
(`work/mesh/log.md`); the unit PR is 1617.

## Closed

PR 1617 merged on 2026-09-08 at the landing head `267e2510f` (main merged
forward by S-MESH; PROPS's fix pass `6859ece19` underneath, green since
2026-09-06). Two duals ran under ordinal 1210 — S-MESH's on `0e053a727`
(2026-09-03; its fix pass died with a rate limit) and PROPS's on
`3daab7d80` (2026-09-06; its fix pass landed); the A/B row MESH12 in
`docs/MODEL-AB-LOG.md` (sample #157) records both and says which the
sample is. Landed: the saturated span refuses at the parse under
`props_meridian_span_winding` with `props_meridian_span_forward` beside
it, both decided once inside the pole helper; the clamp deleted; the
rim-continuation condition measured dead through the import door and
pinned so by rows, its one live Euler-door witness pinned in topo.
Residue, each its own file on this slate: `rim-only-sphere-cap-panics-at-
census` (issue 1615, un-parked by this closure), `stored-spans-read-raw-
past-winding-bound` (issue 1618), `period-headroom-margin-has-no-shared-
home` (filed by the fix pass).

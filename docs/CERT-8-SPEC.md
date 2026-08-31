# CERT-8 — chart-stretch honesty (issue 501, then issue 528)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged at spec: **M/L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issues 501 and 528 are the primary specifications; this document
fixes scope, order, and fences. Branch `cert/8-chart-stretch`.

## The cluster's shape

Two defects with one root: the kernel's chart-stretch bounds exist
only on the geom-brep side (`pcurve_cache`'s Floater
`nurbs_stretch_bounds`, sup-side), so `topo` sites that need a
stretch meter either default to 1 (issue 501 — an UNDER-statement
on NURBS charts, the wrong-but-green shape) or refuse wholesale
(issue 528 — no inf-side bounds exist, the uselessly-narrow shape).
Fix the honesty defect first, then extend the certified lane.

## Part 1 — issue 501 (do this first, commit-separable)

The sites, in `crates/topo/src/pcurves.rs`:

- `azimuth_arm`'s final arm (~:968): `Surface::Plane` answering 1
  is CORRECT (a plane chart's parameters are metres — the audit
  ledger records it OK); `Surface::Nurbs(_)` and
  `Surface::Approx(_)` answering 1 are the defect — the chart's
  metre stretch is whatever the net says, and 1 under-states it.
- both `v_meter` fallbacks (~:1386, ~:1631):
  `unwrap_or_else(T::one)` is right where v is already a length,
  wrong on a NURBS chart.

The retirement shape (the issue's own): give `topo` the same
stretch bounds the geom-brep charts use, through a properly-layered
export, and meter both channels. `topo` already depends on
`geom-brep` and `pcurve_cache` is a `pub` module, so this is an
API-surface decision, not a dependency change: make the bound
reachable under a name and doc contract that states exactly what it
bounds (sup-side stretch, safe for escape metering, NOT a
lower bound — issue 528's distinction, stated at the export so the
next caller cannot misuse it). These are `Margin::metered` sites
already; once the bound is reachable the change is local.

**Direction-of-error discipline**: these meters divide margins for
ESCAPE claims, where a sup arm is the conservative side. State at
each changed site which side the claim needs and why sup is sound
there. That argument is the unit's core content — the review will
re-derive it.

**Red-first**: a NURBS-chart loop whose stretch is far from 1
(a deliberately stretched net, e.g. a chart whose image spans 100×
its parameter span), pinned showing the gap margin the old
`1`-meter certifies and the honest metered result — red under the
old arm reading, green under the new. Plus the issue's scale twin
(same loop at a large uniform scale) and the three-outcome posture
(certify / refuse-typed / escalate) for any row the real arm newly
refuses.

**The ledger**: `docs/predicate-dimension-audit.md` carries the
narrowed row (analytic + plane OK, NURBS flagged, pointer to 501).
Record the row OK with the same precision the flag has.

## Part 2 — issue 528 (after part 1 is locally green)

Extend `chart_region`'s certified positive-area lane beyond
plane/cylinder by deriving certified LOWER stretch bounds
(inf |S_u|, inf |S_v|) per chart kind, per the issue's own
derivations:

- NURBS: lower bound on the derivative net's column norms with the
  rational weight-ratio factor taken opposite to
  `nurbs_stretch_bounds` (Floater's bound is two-sided);
  zero-crossing nets honestly have inf 0 and KEEP REFUSING —
  `ArmUnbounded` stays typed for them, do not fake a positive
  bound.
- sphere/torus: the azimuth arm's inf over the face's certified
  v-window (window-dependent — the reason `exact_arms` could not
  quote it).
- cone: v_inf·sin α over the window (the `chart_arms_at` shape,
  inf-side).

Meter the 2-D machinery's margins by the inf arm for positive
claims; keep sup arms for escape claims. Each new inf bound needs
its derivation argued at the site (why it lower-bounds the true
stretch over the whole window) and a row that would red if the
inf and sup readings were swapped — the confusion this lane exists
to prevent.

Acceptance rows: at least one sphere-window and one NURBS-chart
face that today refuse `ArmUnbounded` and now certify positive
area, with the certified margin's digits in the PR body; and one
zero-crossing NURBS net that still refuses, pinned typed.

## Fences / posture

- **The pcurve_cache seam is PCURVE-adjacent**: review the layering
  with P-2's resume state in view — PR 1177 is MERGED, so no
  keep-out remains on `geom-core/src/linalg/vec.rs`; issue 1195
  (interior-iso de Boor collapse extractor) is NEW capability and
  NOT in scope; issue 1316 (the exact-equality seam comparison at
  `pcurve_cache.rs:~3444`) is its own unit's fix — do not take it,
  but your layering decision should state where the exported bound
  sits relative to that seam so 1316's taker inherits a map.
- Issue 1305's open half (typed refusal on a non-singleton
  `shift_branch` shift) sits on ground you may visit: if your
  metering diff reaches `chord_join.rs`'s pole arm, decide it with
  the D2-addendum classification; if not, state explicitly that
  the ground was not touched.
- **Sibling lane fence**: CERT-10 runs concurrently on
  `geom_core::spline`, `geom-brep/{patch_bound.rs, props/quad.rs,
  offset_fit.rs}`, and `mesh/nurbs_cert.rs`. Do NOT edit those
  files. If your export wants a shared home in `geom_core::spline`,
  do not create it — export from `geom-brep` and note the wish; the
  orchestrator owns the collision.
- ε posture per the issue-1356 practice: metered margins are
  band-sensitive; run the three-ε local sweep on every new/changed
  row, state per-band premises, and pin the trailer ε with the
  argument. `CI-Config: lane=both` (interval metering is live
  here).
- No `Co-Authored-By`; issues spelled out ("issue 501"); push early
  to `cert/8-chart-stretch`; the gate runs when the orchestrator
  opens the PR — report local evidence as local.

## Acceptance

- Part 1: both channels metered through the exported bound on NURBS
  charts, plane behaviour bit-identical, red-first digits in the PR
  body, ledger row OK, scale twin + three-outcome rows.
- Part 2: the per-kind inf bounds with argued derivations, the
  newly-certifying rows' digits, the still-refusing row typed.
- Sweep obligation (assume a class): other `T::one()` /
  `unwrap_or_else(T::one)` default meters in `topo`, and any other
  site reading a sup bound where its claim needs an inf (or vice
  versa) — hit list with dispositions; state what the pattern
  cannot match.
- Deviations stated; any refusal minted/changed/retired classified
  per the D2 addendum (part 2 retires refusals — the addendum walk
  is owed, not optional).

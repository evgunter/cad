# M5 PR 14 — the exit sweep (binding spec)

Branch `ev/m5-pr14-exit` from current main (post-#166: the die is
in). Plan line 14 (M5-PLAN :313-319). This unit closes M5. Its PR
is a DESIGN CONVERSATION at the end: the exit walk carries two
items for Evan's explicit sign-off, so the PR opens complete and
WAITS (no self-merge).

## 1. The T5 K-telemetry snapshot (+ the #89 kernel half)

- Run the K-probe machinery over the CURVED corpus (every Band-4
  document incl. boss_union, cut_cylinder, die_fillet, the S11/S13
  bodies) at the standard ε rows; produce the snapshot in the
  K-REPORT format (per-family margin distributions, band landings,
  the counterfactual-K table K ∈ {3, 10, 30, 100} re-run on the
  new corpus — the FIRST with computed SSI/tangency/quadrature
  margins; K-REPORT Finding 4's revisit condition fires HERE).
- Write the results as a dated addendum section in docs/K-REPORT.md
  (do not rewrite the M2-era findings — append the M5 snapshot).
- The #89 decision material: state plainly whether the curved
  corpus changes the raise-K calculus (zero-landings? new in-band
  populations? which families are the closest to their bands?).
  DECISION IS EVAN'S — the addendum presents the table + a
  recommendation with grounds, and the PR body carries it as
  sign-off item 1. Note for the addendum: the M7-first resequencing
  (#161) means imported-geometry evidence arrives NEXT milestone —
  if the recommendation is "hold K=10 pending the import corpus,"
  say exactly that.

## 2. The envelope / DESIGN.md sweep

- DESIGN.md roadmap: the M5 line → done (with the two-piece shape
  (v) disposition stated honestly); frontier entries updated to the
  as-built state (the banked units by name: composition surgery,
  SSI generic lift, loft assembly, canal blend, cyl×sphere chords,
  NURBS extent lift, curved REST).
- Quarantine text: verify the PR 1-era inari/LGPL retirement text
  is fully consistent (S7 did the sweep; this is the verify pass).
- New conventions proposed during M5 that deserve DESIGN.md
  residence: the two-tolerance-including-definite-arms rule (S9
  lesson), the equivariance principle (memories → a DESIGN
  convention line, marked premise-unaudited), the tessellation
  ruling (distance-only certified; angular = future display lane —
  the 2026-08-02 in-session ruling, recorded verbatim-scoped).
- The M5 envelope's typed frontiers enumerated as they ACTUALLY
  shipped (union-only→per-class curved booleans, sphere-class
  extent scan, NURBS re-gates, touching-refusals, planar census).

## 3. Band-4 corpus green + the exit walk

- Every R5 shape's corpus rows verified green at the standard
  matrix (this is reading CI truth, not re-running locally).
- The exit walk: every criterion from M5-PLAN's exit list quoted
  and dispositioned (met / met-with-recorded-honesty / carried).
  Known carried items to state exactly: shape (iii)'s full loft
  BODY (substrate row met via PR 7b; body = banked assembly unit);
  shape (v) composed die (two-piece + banked surgery unit — Evan
  sign-off item 2); NURBS-at-rest tessellation/props (the honest
  doors); anything else the walk surfaces.
- docs/M4-EXIT-WALK.md is the format precedent → docs/M5-EXIT-WALK.md.

## 4. State-doc trim + A/B readout

- M5-LOG gets its closing entry (not a rewrite — the log is the
  record); MODEL-AB-LOG gets the M5-close readout row block
  (n≈42 rows: the milestone-level honest summary in the M4-close
  format — stratified by difficulty, silent-dev counts, the
  fix-pass-size distribution; NO significance overclaims).
- memories/ index audit: stale pointers fixed (the session added
  ~8 memories).

## 5. Process

Implementer runs the mechanical parts (snapshot, sweeps, walk
drafting); the orchestrator reviews the walk personally (this
unit's reviewer IS the orchestrator — exit honesty is central
planning, per the orchestration model); Evan's sign-off on items
1 (#89/K) and 2 (shape (v) disposition) happens ON the PR (👍 or
comments), and the PR merges only after both. fmt-all --check;
iteration-speed local scope; no blinded lane (no code semantics
change — telemetry, docs, and walk text only; any code change
discovered necessary is a numbered deviation that STOPS for
orchestrator review).

# M7-3 spec — NURBS-face import (binding)

Mandate (docs/M7-PLAN.md unit 3, unblocked by #192): import the
NURBS-face bodies the export writer now emits. Substrate: the
measured gap at `~/.local/share/cad-work/m7-3-substrate/`
(inventory.md — the ranked gap with file:line for every
certification door, the emitted files, the tier-3 traversal
map) — read it FIRST; every scope item below is one of its
measurements. This spec is binding: deviations are REPORTED
(numbered, with the executed blocker), never improvised.

## 0. Fence and bounds

All work in `crates/step-import` unless an item names another
file. The #207 weight-drift kernel bug is OUT of scope (uniform
lofts are the exportable class today — the unit round-trips
those; the bug bounds coverage, honestly stated, not fixed
here). Export writer untouched.

## 1. Scope (the inventory's ranked gap, verbatim order)

1. **Parse both B_SPLINE_SURFACE_WITH_KNOTS arms** (non-rational
   simple entity + RATIONAL complex instance) →
   `Surface::Nurbs`, mirroring the curve twin
   (entities.rs:713/:731 precedent).
2. **Fix the `surface_sig` trap FIRST** (adopt.rs:235:
   `Nurbs(_) → vec![5u64]` — after arm parsing lands, all walls
   would silently share ONE surface key): hash knot/control/
   weight bits. A pin proves two distinct NURBS walls get
   distinct keys (the trap is the silent-wrong-body class).
3. **The IsoCurve adoption rung** (new): bitwise-match the edge
   carrier against `boundary_iso_u(wall, false|true)` of the
   adjacent walls → adopt as `IsoCurve { wall, u: 0|1, 0, 1 }`,
   certified through the iso residual lane (`resolve_iso`
   admits described NURBS — measured open). No existing rung
   serves (measured: certify.rs:787/:798 refusals).
4. **Rim edges (cap-plane × NURBS wall)**: Nurbs-adjacency
   exemption on the conventional rung's coincidence gate
   (mirror check-4 flip B, validate.rs:1774 — the Seam-idiom
   precedent), and route the mint blocker per the inventory's
   two options (synthesize PlacedSegment vs an ExtrudedPoint
   arm in `nurbs_iso_derive`, pcurves.rs:386) — implementer's
   call from reading both sites, reported with the executed
   comparison.
5. **Rational arm (firm proposal, flagged for Evan in the PR):
   ARM B — import-with-typed-limitation.** The writer exports
   arc_loft today (measured; t3 refuses RATIONAL-patch flux and
   the export happens anyway), so refuse-at-import would
   reproduce the writer/reader asymmetry one arm down. Arm B is
   measured feasible: seams certify, rational walls skip minting
   (`chart_mints` = false), and the imported body lands in
   EXACTLY the native state — t1/t2 valid, the identical typed
   t3 refusal. `StepImport::Solid`'s doc contract conditions
   honestly: tier-3-valid for bodies whose native twin is
   tier-3-valid; a body whose native twin refuses t3 imports
   carrying the same typed refusal. A pin asserts the imported
   arc_loft's t3 refusal is the SAME variant the native body
   produces.
6. **Outerness**: loft faces are single-bound (the definitional
   arm suffices — measured); multi-ring foreign NURBS faces
   keep refusing typed (no NURBS `uv_of`; the frontier is
   named, banked with stage-1 recognition).

## 2. Acceptance rows

1. **Round-trip row**: loft_prism (the committed fixture) and a
   built arc_loft import; censuses/validity match the native
   bodies (arc_loft per item 5's conditioned contract); the
   non-rational loft reaches FULL tier-3 with certified volume
   (V = 9 m³ for loft_prism, quadrature-bounded).
2. **Byte-identity**: loft_prism joins the M7-1 fixed-point
   suite (`SOLID_FIXTURES` 14→15) — the inventory measures this
   feasible (bit round-trip printer, file-order reconstruction,
   the sig fix restoring key sharing; no new ordering source).
   If a divergence class emerges, report it with the honest
   fallback (fixed point without committed-byte-identity), the
   M7-4 D4 precedent.
3. **Flip row**: review_k3_probe's loft_prism-refuses-typed pin
   FLIPS to acceptance (S9, history carried).
4. **Refusal preservation**: dm1-id-214 STAYS REFUSED (7/19
   rational surfaces, multi-span, 11 trim rings — stage-1
   territory), but re-anchor its pinned substring if the new
   arms change the first refusal site (the inventory flags the
   risk); TAIL_TURBINE/io1/OCC rows unaffected (assert).
   NOTE: the work order's nist_ftc_10 premise was stale (not in
   the repo) — no row.
5. **surface_sig pin** (item 2's trap) and the same_sense/
   orientation controls re-run (M7-2's box/sphere/torus probes
   stay green).
6. **Regressions**: full step-import crate + step-export suites
   green; hosted matrix is the gate.

## 3. Constraints

M7-1-SPEC §3 + the flat-ε_in amendment carry over. The #205
margin convention is PROPOSED, not ratified — any new comparand
follows its clauses in SUBSTANCE (honest lengths, per-kind
payloads) without introducing the newtypes. Doc honesty: where
this unit's coverage is bounded by #207 (curved-path sweeps,
non-uniform lofts), say so at the site; do not overclaim
"NURBS import" beyond the measured exportable class.

## 4. Local battery

Targeted crate suites foreground (pin cleared, normal scope);
one workspace check; hosted CI is the gate.

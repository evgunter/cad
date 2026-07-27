# M5 orchestrator log

Running record of the M5 milestone (curved geometry: NURBS depth,
SSI, fillets). Plan: `docs/M5-PLAN.md`; design contract:
`docs/CURVED-DESIGN.md` (C1–C12, ratified #85). Convention as in
M0–M4 logs: newest entries append at the bottom; the tail snapshot
is the resumption contract for any successor.

## Session start (2026-07-27 — cad-implement-m5 orchestrator)

Handoff from cad-implement-m3-6plus received at the drained M4→M5
seam (M4 CLOSED: exit walk 12/12, #119/#121 merged, final snapshot
at M4-LOG tail). Worktree fast-forwarded to main `2f2b3b3`;
monitors installed from `scripts/monitors/` and armed (away-channel,
disk watchdog, hourly check-in); sign-off watchlist empty;
predecessor stopped via `mngr stop` after orientation confirmed.

**M5-PLAN drafted and opened without overnight block** per Evan's
explicit authorization at handoff (2026-07-27: "it seems safe for
it to just write that and then go … i don't want it to wait
overnight just for my ok"; patchable-not-fatal stance invoked by
him). The plan is sequencing-only — CURVED-DESIGN C1–C12 is the
design layer and every OQ1–OQ9 carries a #85 DECIDED entry; the
plan resolves only the planning residue R1–R6 (cone-arm staging,
ring-vs-adoption split, schema v2 mechanics, banked-opener
scope-boxing, the acceptance-shape set, scope guards). PR 0
obligations folded into the plan PR: the D2 sharpening
(`TangencyLocus` → `TangentIntersection` rename sweep in
DESIGN.md), the inari quarantine transition-state text +
LGPL-before-publish exit condition, this log seeded.

Immediate dispatch order (fork-independent first, per handoff):
PR 1 (interval-transcendentals adoption, green-lit), PR 2 (C9
ring), PR 3 (linalg + NURBS substrate part 1), S3 (curvo audit —
must report before PR 4). A/B block 4 opens with the first
implementation dispatch (pre-flip difficulty logged per protocol
v2; MODEL-AB-LOG is the log).

**Openers dispatched (2026-07-27, same session)**: #123 opened
with the sign-off comment watchlisted. Two seam surveys ran
(interval seam: swap surface is exactly geom-core/src/interval.rs
+ two Cargo.tomls, zero inari use elsewhere; curve/surface
substrate: no Curve2 exists, Surface::Nurbs is a unit placeholder,
Real trait confirmed ring-complete for de Boor, no linear-solve
code anywhere). **S3 curvo audit COMPLETE** (same day —
docs/CURVO-AUDIT.md @ 1fbf0ad): Q5 default stance CONFIRMED,
vendoring REJECTED on all four candidate areas (hardcoded
epsilons/fail-quiet branches at every invariant-relevant surface;
no A9.10 stack and NO SSI in curvo at all — the DESIGN.md
landscape row's "incl. SSI" was wrong, corrected); oracle scope =
evaluation/basis/derivatives, pinned at 47d19d5. Q5 lean revision
applied to DESIGN.md per the OQ9-decided path. Plan amendments at
spec time (recorded in #123): PR 1 takes a path-dep (crate keeps
its own workspace so the gmp dev-oracle stays out of kernel
builds); C12.8's LSQ/SVD move to their consumer PRs (4/7).
Binding specs committed: M5-PR1-SPEC.md (API mapping, D1-D8
divergence handling incl. the D8 floor pin flip, width-pin
inventory), M5-PR3-SPEC.md (geom-core::spline home, Arc payloads
with accepted Copy-loss, the SpanLocate seam, clamped-v1,
Tiller-bound honesty test). **PR 1 implementer DISPATCHED (A/B row
11: M, OPUS — block-4 draw byte 63) and PR 3 implementer
DISPATCHED (row 12: L, fable remainder)** — the two cargo lanes;
PR 2 (C9 ring) queues for the next free lane. Branches:
ev/m5-pr1-interval-adoption, ev/m5-pr3-nurbs-substrate.

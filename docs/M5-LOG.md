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

**M5-PLAN RATIFIED (2026-07-27): Evan "lgtm!" on #123 → MERGED to
main `4642619` on 18/18 green**, with one rider folded in pre-merge
(R3: the v1→v2 migration is NOT a commitment — no users yet; clean
break if cleaner; the PR 10 spec records the call). Same sweep:
Evan triaged open issues — #95 verified landed at #102 (memo.rs
recursive naming key + grandparent pin) and CLOSED as a
bookkeeping slip; #89 answered open-by-design until the M5 exit
K-snapshot; #104 answered unresolved/banked for v2 (offered an M5
design-unit slot if wanted). His `test (interval)` CI-duration
question answered with run data: 8–12m is the historical norm
(feature-set recompile + gmp on cache miss + genuinely slower
interval instantiation); recommendation on record — no split
before PR 1's gmp removal lands, shard per-crate only if it still
dominates after. One waiter-parking incident: the PR 3 implementer
stopped on background builds mid-battery; nudged back to
foreground rows with the cd-prefix rule (standard playbook), lane
recovered and progressing.

**PR 1 implementation COMPLETE (2026-07-27, ev/m5-pr1-interval-
adoption @ 2dc8860 + report) — opus row 11.** Shape A: ring ops +
transcendentals both from the in-repo crate; inari/gmp/rug gone
from every kernel path incl. Cargo.lock. Headline: ONE predicate
verdict flip found and fixed at the ROOT — split_join_order_u
escalated because div_lo/div_hi padded exact quotients (axis-
aligned v/||v|| stopped being exactly unit); fix = division
exactness witness (fma residual == 0 above the 2Prod floor)
mirroring mul_exact, D1 updated. 5 reported deviations, 0 silent
(div witness beyond sanctioned scope; oracle-computable moved to
sibling crate — cargo resolves path-dep manifests even disabled,
would break builds without Evan's ~/projects/computable checkout;
root exclude addition; acceptance item 3 skipped to avoid
reintroducing the LGPL dev-dep — ruled ACCEPT conditional on
reviewer verifying the wrapper is a pure delegating newtype;
consts path nit). One tripwire consciously inverted:
powi_diverges_from_the_tight_enclosure → containment pin (pad
backends contain their own f64 lane; flagged for review
assessment). Battery: 1342-1343/0 ×3ε default, 1498/0 ×3ε
interval, clippy clean, certify 12/0 vs warm-cache inari oracle.
Build delta measured PAIRED: workspace interval clean 665→478s
(1.39×), isolated geom-core 280→47s (6.0×) — #115's 93× was
crate-vs-crate, not claimed. **Blinded reviewer DISPATCHED**
(F1-F10: div-witness soundness attack incl. subnormal/2Prod-floor
edges, containment fuzz, with_dec_capped laundering, verdict-flip
repro both directions, tripwire-loss assessment, 4 pin
re-derivations, LGPL-free repro, doc-claims audit, e2e consumer
run, battery honesty).

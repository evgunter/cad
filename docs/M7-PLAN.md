# M7 — STEP import as adoption (plan)

Ratified by construction: the boundary was decided at #169 (the
renumbering: "M7 should stay as just adopting STEP files") and the
content is DESIGN.md **D7** ("Import is adoption, not admission") —
this document assembles them; nothing here is newly proposed.
Core kernel work that import happens to *want* belongs to M6, not
here (#161 §2 relocated the census/declared-contact design work to
M6 by exactly this rule). M8 = error propagation.

**Concurrency note (2026-08-04; the two-orchestrator fence is
RETIRED — the M6 session wound down 2026-08-05 and the M7
orchestrator is sole orchestrator, see M6-LOG).** Still standing
from it: any round-trip disagreement that tempts a change to an
export fixture, an `.expect` sidecar, or `check_step.sh` semantics
goes to a design-conversation PR seen by Evan — never a direct
edit.

## The contract (D7, restated as obligations)

Import **reconstructs the intensional description** the extensional
data satisfies; imported bodies end first-class, their caches
recomputed and certified at ε by the kernel's own machinery.
Interpretation is governed by a separate per-import **ε_in**
(defaulted from the file's `uncertainty_measure_with_unit`,
overridable per call); healing may move geometry by up to O(ε_in)
as a **reported** model change, never a loosened certification.
Data ambiguous at ε_in fails with a typed ambiguity error; the
unhealable fail loudly, naming entities (D4 ¶5). Feature
recognition is a non-goal.

*Design consideration (Evan, #180 comment, 2026-08-04):* the
adoption machinery will likely be reused to offer GUI users the
appropriate **remedy** instead of an error — refusal types carry
structured data (entity, failed interpretation, what would be
needed), so a future remedy flow never parses messages.

## First slice: import what we export

The export corpus (17 solid fixtures + `nurbs_wireframe` under
`crates/step-export/tests/fixtures/`) covers the kernel's whole
geometry vocabulary as **native, exact AP214 entities** (M5 PR 13;
`docs/CURVED-DESIGN.md` + the STEP writer identity mapping (M5 PR 13 record)): PLANE / CYLINDRICAL_ / CONICAL_
/ SPHERICAL_ / TOROIDAL_SURFACE surfaces; LINE / CIRCLE / ELLIPSE
/ B_SPLINE_CURVE_WITH_KNOTS carriers. For this subset, D7 stage 1
(NURBS→analytic recognition) is mostly the identity — the entities
arrive already analytic — so the first slice exercises **stage 2
(edge adoption) and the certification path**, which is where the
inverse problem actually lives.

### Units, in order

1. **M7-1 — import crate skeleton + own-corpus round-trip**: new
   crate `crates/step-import` (own tests; workspace member —
   root-Cargo.toml member line is the only out-of-crate edit).
   Parse the AP214 subset the writer emits; adopt per D7 into
   kernel bodies; acceptance: for every solid fixture,
   export → import → **censuses, certified volumes, and validity
   match the source body**; the committed fixtures import to their
   `.expect` counts. `nurbs_wireframe` (curve-only) gets the
   disposition its geometry supports, stated, not skipped.
2. **M7-2 — foreign corpus: FreeCAD-authored files** of the same
   entity subset (FreeCAD 1.1.2, the version-matched oracle —
   `scripts/check_step.sh (the oracle path + version are baked in)`): the first geometry this kernel
   adopts that it did not write. Validity + expected censuses /
   volumes; ε_in exercised for real (OCC's default write
   uncertainty is coarser than kernel ε).
3. **Later M7 (blocked on M6 units, not started early)**: NURBS
   *faces* (`B_SPLINE_SURFACE_WITH_KNOTS` arrives with M6's
   loft/sweep assembly — its import waits for its export);
   genuine stage-1 recognition (foreign NURBS within ε_in of an
   analytic surface, promoted); the healing ladder beyond what
   M7-2's corpus forces.
4. **Wild corpus (late; may defer past the slice — Evan, #180
   comment, 2026-08-04)**: suitably-licensed STEP files found in
   the wild that fall inside the supported subset (no NURBS), as
   a demonstration that import works on files nobody here
   authored. Sequenced near the end of the work, and deferrable
   until the underlying kernel support is more mature.

## K telemetry (standing, #89)

The import corpus is the **designated re-open trigger** for #89
(K=10, CLOSED — `docs/K-REPORT.md`): the expected first source of
IN-BAND LANDINGS, detected by k-lint rule 1 at the next hosted run.
A landing is a headline finding — **report it to Evan; never
quietly retune**. Known stale item, not this milestone's to fix
silently: the large-K lint's 1.5e-3 baseline floor (named M6
pickup).

## Exit shape

Every STEP file the kernel can currently export imports back to a
first-class body whose censuses, certified volumes, and validity
match; FreeCAD-authored files of the same entity subset adopt
cleanly or fail with the typed errors D7 promises. Anything beyond
that subset is a typed refusal naming the unsupported entity —
the S9 flip pattern applies when later units retire refusals.

*Directional note (Evan, #191 comment, 2026-08-05):* the eventual
goal is for adoption to support **all** of the wild corpus —
today's refusal fixtures (NURBS faces, trimmed splines) are
waypoints that flip as the vocabulary grows, potentially after
applying recommended geometry fixes where the foreign data
carries slivers (ties to the banked #89 sliver-lint idea,
LONGTERM-IDEAS I1(0)).


## Runway addendum (2026-08-09 — the orchestrator's live plate; updated at seams)

*(Terminology: "block M7-N" in MODEL-AB-LOG is an A/B arm-assignment
block, not a unit. Units are M7-1…M7-7 below. This section exists so
the plate is visible on main between seam entries; the log tails
remain the authoritative narrative.)*

**Units as executed** (the plan's four grew to seven — each growth
ruled on a thread, none silent):

| unit | what | state |
|---|---|---|
| M7-1 | import crate + own-corpus round-trip | MERGED #183 |
| M7-2 | FreeCAD foreign corpus | MERGED #189 |
| M7-3 | NURBS faces (+ARM B) | MERGED #209 |
| M7-4 | wild corpus + dialects | MERGED #193 |
| M7-5 | band-seam re-mint (the plan's S9-flip pattern executed) | MERGED #252 |
| M7-6 | stage-1 recognition (always-promote, #256; whole-patch envelope) | MERGED #264 (pair struck from variance sample — same-head amendment) |
| M7-7 | tier-at-import (the #260 (a) ruling; shared gate) | MERGED #276 (per-solid gating earned at review; ε-row-honest pins; delta re-review 0 MAJ) |
| M7-8 | plane×NURBS intersection certification (ruled on #264's thread) — the last M7 code unit | MERGED #288. The #276 union collision RULED (option (c), re-fixture — preserves both the #260 (a) no-verdict-filter law and the stretch sequencing): the seam-orphan CLASS is retired (declare-and-check certifies, envelope mutation-pinned post-fix); the arc prism re-pinned as an ADVANCED waypoint (seam certifies, body refuses the banked rational quadrature — flips at stretch item 1b); the integral twin offset_square_prism is tier-valid at rest and measured the SECOND gap (nurbs_iso_derive has no Intersection arm — foreign-restated seams refuse IsoUnsupported at pcurve minting), both flip conditions named in-code |
| demo | wild-corpus montage, KERNEL-TESSELLATION lane only | MERGED #283; post-#276 re-verify DONE (byte-stable on the union) |
| demo | calochortus (globe-lily) PARTIAL refresh: tube_along_arc exact-intent stems (finding 11's workaround retired), curved-path sweep_body leaves (KITE section — the rational-section frontier refused the crescent, pinned in crates/sweep with flip-when-fixed), twisted-duct spine vocabulary. The FULL rebuild stays M8 (findings 1/2/7 are C7 machinery) | MERGED #294 (27/27 incl. k-lint gate; both lanes eyeballed; byte-stable re-render) |
| demo | Utah(-ish) teapot | RE-SEQUENCED (Evan, 2026-08-09): designated as the SHELL verb's demo — queues behind shell/offset (see docs/KERNEL-VERBS.md), not an M7/M8 item |

**Then**: the M7 EXIT WALK (docs/M7-EXIT-WALK.md — only M7-8's
cells + the tally remain open) → presented for Evan's closure
ruling → **the DEMO-HARDENING STRETCH** (Evan's directive,
2026-08-09, in-session + "after the exit walk in another chunk"):
iron out the demo-raised issues before M8 — #284 (mesh chart-frame
Newell re-anchor; flips two wild tessellation refusals), the
rational-carrier span-meter speed bound (unblocks rational-section
sweeps — the calochortus crescent; flips the crates/sweep pin),
then the rational-patch-flux quadrature (arc-loft round-trip,
tier-3 on rational walls; the M7-3 Arm B bank). #298 (pre-push
hook lock) is LIB's #299. **Then M8 opens with C7** (the join
lane; contact/signed-clearance co-design — ruled at #223) and the
M8-PLAN gets drafted against ERROR-DESIGN + CONTACT-DESIGN; the
lily FULL rebuild rides there as the demo moment.

**Adjacent/banked (not on the critical path)**: the M6 carried-items
register (#250) incl. the gate-minted-validity-currency design
(#260) and the structural analytic-mint sidecar (#256); Q9 naming
(fresh availability data banked; Evan's call); k-lint gating landed
(#253); render/slot infra landed (#266, #269 — the latter LIB-side).

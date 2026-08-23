# TESS-SPLIT — the aspect-capped split schedule (binding spec)

Executes the SPLIT half of #320 as corrected by #547 (the
dominant ~4.1x factor) under the RATIFIED aspect policy
(TESS-BUDGET.md, PR #568): the schedule takes the cell-minimizing
point on the certified ellipse subject to a first-fundamental-form
3-D aspect cap at **A = 16** (a named spec-time constant with its
reasoning at the definition site). Pre-logged **M / NUMERIC**.
**DISPATCH GATE: TESS-SPAN merged** — OPEN since #594.
Deviations reported, never absorbed.

**Reconciled with TESS-SPAN's landed state (2026-08-23).**
TESS-SPAN's binding spec was deleted at unit close
(`docs/DOC-LEDGER.md`); its constraints that still bind this unit
are restated here from the shipped code sites, which are the
statement of record — chiefly the sliver lesson, which D-1a binds
alongside the ratified cap. They are DIFFERENT quantities; the
cap does not discharge the sliver constraint and neither implies
the other.

## D-1: the point selection

`grid_steps`' AM-GM grouping `(muu+muv, mvv+muv)`
(`mesh::nurbs_cert::NurbsFaceBound::grid_steps`) is replaced by
the constrained minimizer: over the cell's certified ellipse
`muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`, choose (h_u, h_v)
minimizing the cell count subject to the 3-D aspect — the ratio
of the grid cell's edge lengths mapped through the FIRST
FUNDAMENTAL FORM (sampled where the Hessian bound is sampled;
state the sampling choice and its conservatism) — not exceeding
A = 16. Closed form where it exists; a guarded 1-D search over
the ellipse boundary is acceptable if closed form is unwieldy
(deterministic, derivative-free, iteration-capped — REPORT the
choice). Degenerate directions (muu or mvv ≈ 0, the ruled-wall
case that motivated the unit) must be exact, not limits of the
generic formula — decided predicates, never raw comparisons, for
any case split. The cap is the ratified policy's dial, not the
only bound: D-1a binds independently, and on ruled walls it may
bind first.

## D-1a: the two aspects (the TESS-SPAN reconciliation)

Two aspect quantities, and this unit must bind BOTH:

1. **3-D cell aspect** — the chosen cell's edge-length ratio
   through the first fundamental form; capped at A = 16
   (ratified, #568). A property of the cell SHAPE the optimizer
   picks.
2. **Realized lattice aspect** — the post-`ceil` spacing ratio
   `s_u/s_v` that `NurbsCellGrid::band_schedule` judges against
   `mesh::nurbs_cert::SAFE_ASPECT` (read the value and its
   derivation THERE, never a copy). A property of the emitted
   LATTICE: beside any off-lattice point (a foreign column at a
   band interface, a trim-chord vertex, an anchor top, a
   refinement centroid) an anisotropic lattice admits a
   Delaunay-legal sliver of certificate ~`(aspect²+1)/8·δ_s`,
   and no local insertion cures it — TESS-SPAN's lesson, paid
   once per failed variant (the `band_schedule` doc block is the
   statement of record; the failing faces are in asm/tess-span's
   commit messages).

The A = 16 point on a ruled wall will generally exceed the
sliver-safe line in realized spacing. That is not license to
weaken either bound; it means the schedule's safety there is
ALIGNMENT — every band and the chord pass on one column schedule,
so no off-lattice u-points exist where the aspect is high —
exactly the mechanism the malign-band snap restores today. So:
the snap machinery REMAINS IN FORCE over the new selection (same
realized-spacing test, same site), and the unit demonstrates by
execution (rows 9–10) that the schedule it emits does not enter
the sliver regime. If sliver safety effectively caps ruled walls
below what A = 16 alone would allow wherever alignment cannot be
established, that is a REPORTED finding with numbers — D-3's
indicator is the instrument — never an absorbed narrowing of the
ratified policy.

## D-2: scope and composition

Applies per-cell: `band_schedule`'s per-v-band tensor is the
substrate. The snap target `patch_nuc` derives from the
whole-patch bound through the SAME new selection — one
derivation, no AM-GM residue surviving as a second copy anywhere
(sweep for callers of the old grouping; hit list in the PR). The
chord pass keeps the whole-patch schedule (TESS-SPAN's deliberate
safe arm at `chords::nurbs_tighten`) — extend it consistently
through the new selection, do not re-litigate it; grid and chords
must remain one derivation so the alignment D-1a leans on holds
by construction, not by phase accident. The certificate is
untouched: every chosen point is inside the SAME ellipse; the
two-grid-cells-per-triangle-axis budget, half-open knot cells,
grid determinism, T-junction/retry ladder, and δ split all bind
exactly as shipped (`mesh::nurbs_cert` cert/band_schedule docs
are the statement of record).

## D-3: the meter and baseline

Post-fix the split ratio reads ~1.0 at cap-inactive cells; keep
the meter sighted per TESS-SPAN's precedent (the counterfactual
columns — TESS-BUDGET.md "The columns after TESS-SPAN"), and ADD
the cap's own visibility: a per-face indicator (or column) for
constraint-ACTIVE cells that DISTINGUISHES which constraint bound
the optimum — the A cap, or the sliver/snap machinery — so a
future A re-tune has data and D-1a's "sliver binds first" report
is read off the meter rather than asserted. Baseline re-cut
WITHOUT --sizing-only, rationale in the commit, the TESS-BUDGET
ritual.

## Acceptance rows

1. Ruled/degree-1 walls (the universal case): measured cell
   reduction consistent with the baseline's split column
   (~4x class-wide); exact numbers reported per scene.
2. Aspect honesty: on leaf_a f2 (the 1×4905 poster child) the
   chosen grid's measured 3-D aspect (through the FFF) is ≤ 16;
   a planted A=∞ run reproduces the strip (executed, reported,
   reverted) — the cap demonstrably binds. Report the realized
   `s_u/s_v` distribution too: which faces sit above the
   sliver-safe line, and under which protection (snap alignment,
   or spacing below it).
3. Certification: the planted-violation falsifier stays red on
   a schedule pushed OUTSIDE the ellipse (execute, restore).
4. Degenerate-direction exactness: a ruled wall's flat direction
   gets its exact arm (test pins it against the generic formula's
   limit — bitwise or with a stated bound).
5. Renders re-blessed through the pipeline; determinism
   two-process row; T-junction suite green (as TESS-SPAN rows
   4-6, repeated at this head).
6. Baseline re-cut + tess-lint green; the constraint-active
   indicator populated and sane (spot-check named faces, both
   constraint kinds represented or their absence explained).
7. Cold clippy: CI scope + interval + budget/probe lanes.
   k-lint fires → report, never silence.
8. M9-5 coupling (the lily rebuild): if M9-5's spec-freeze has
   happened by this unit's merge, re-pin against ITS landed
   state per the #569 agreement; else the at-merge entry carries
   the fresh-state pointer for M9-5.
9. Sliver safety, executed: at the new schedule, the
   per-triangle certificate falsifier
   (`probe_review::z1_per_triangle_certificate_falsification`)
   is green over its fixtures, AND a dense-resample pass on the
   known reproducers — leaf_a f2 and a trimmed NURBS face whose
   trim boundary crosses columns off-lattice — shows the worst
   certificate inside δ with the retry ladder inside its shipped
   bound (no escalation storm). `band_schedule`'s malignity row
   (`band_schedule_snaps_on_realized_aspect`) stays red-able:
   the raise-the-constant probe executed and reverted.
10. The finding-13 table (#782): diagnose the standing
   +4.1%/+3.4% swept-blade drift BEFORE re-pinning — attribute
   it to its cause (TESS-SPAN's realized-aspect snap is the
   suspect; verify, don't assume), then re-pin
   `demos/tour` `finding_13_tessellation_table_reproduces` at
   this unit's schedule and post the disposition on #782 so the
   demo-test arming decision unblocks. Arming demos/ tests in CI
   is NOT this unit's scope; the diagnosis + honest re-pin is.

## Standing brief lines

Read `docs/prompts/implementer-discipline.md` by path before
starting — output discipline, CI-first verification, baselines,
comment style, and sweep receipts live there and bind. Beyond it:
merge origin/main immediately before opening the PR, re-merge on
movement while open, and confirm checks actually STARTED from the
workflow runs list (CONFLICTING = silent CI outage); build under
`local-scripts/with-build-slot.sh` with your own
`CARGO_TARGET_DIR`; kill only by your own recorded PIDs; commit
AND push after every coherent unit; PR bodies authored at
lane-private paths, never the shared session scratchpad; NO
Co-Authored-By trailer in lane commits (blinding overrides the
harness convention — if one lands in a pushed commit, note it in
the PR body and carry on; never rewrite history).

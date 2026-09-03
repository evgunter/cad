# TCOST-K1 — the patch-flux lanes' exhausted-budget cost (spec)

**Program:** S-TCOST (`docs/S-TCOST-PLAN.md`, the kernel-logic track).
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge). **Pre-draw fields, logged here before the block
draw:** difficulty **L**, task-class **NUMERIC**.

## The finding (TCOST-2, PR 1609, measured locally at default ε)

`geom-brep::props::quad::nurbs_patch_face` and its rational twin
`rational_patch_face` cost **22–33 s per call when they exhaust their
round budget**, against 3–5 s when they certify or refuse early; on a
3×3 single-span dome the rational lane costs **~90× the integral
lane** for the same face (unit weights 0.23 s; one weight at 1.25 →
21.7 s). This is not a test-side cost: every rational face the real
program cannot certify — an import, a validate, a mass-properties
call — pays the same seconds. TCOST-2 cut the probe families ~47 %
and found the residual is almost entirely this.

## Why it costs what it costs (read from the code, to be MEASURED first)

Both lanes refine a knot-aligned composite midpoint rule on a FIXED
schedule (D9): pieces per axis double per round from
`QUAD2_INIT_PIECES = 8`, up to `QUAD2_MAX_ROUNDS = 6` (integral) and
`QUAD2_RATIONAL_MAX_ROUNDS = 7` (rational), so cells grow 4× per
round and the rounds at 512 and 1024 pieces per axis are ~94 % of the
cells the whole schedule can execute. A face that certifies exits at
the first round whose width clears `target_len = 1024·ε`; a face that
cannot pays every round.

The certified width is two terms: a Taylor remainder
`Σ hull(f_dd)·h³h/24` over per-block hulls fixed before the loop,
which QUARTERS per round exactly (each cell's remainder splits into
four children at 1/16 each), and the midpoint-sum term, whose width
is the accumulated interval-arithmetic width of the cell evaluations
— a FLOOR that does not shrink with refinement (the `floor (m)` table
in `rational_patch_face`'s header is that quantity). A face refuses
at exhaustion when the floor exceeds the target; the remaining rounds
after the remainder has fallen under the floor add cells and change
nothing a caller can use.

## The unit

**Phase 1 — measure before touching anything** (`memories/refusal-text-is-not-cause.md`:
measure-first is a mandatory checkpoint). Instrument the two loops
(locally; nothing committed prints) to record, per round, the wall
time, the remainder total, the midpoint-sum width and the reported
width, on: TCOST-2's `width_versus_gap_from_a_block_edge` fixture,
the cert6 dome at weight 1.25, the header table's refusing carriers
(the two-span half cylinder, the quarter torus), the certifying ones
(the sphere octant, the single-span cylinder), and dm1's wall
(`stepcode/dm1-id-214.stp`, `FaceKey(3v3)`, the widest honest
bracket the tree pins). Put the table in the PR body. If the picture
above is wrong — the floor is not where the cost sits, or the
remainder does not quarter — say so and stop at a report; the lever
below rests on the measurement, not on this file.

**Phase 2 — the lever: exit the schedule when it can no longer
certify.** After each round that does not certify, compute from
quantities the loop already holds whether ANY later round of the
fixed schedule could: the remainder term at round `j > k` is exactly
`R_k / 4^(j−k)`, and the floor term is bounded below by what this
round measured (state the bound you use and why it is sound — the
floor is a sum of rounding widths and is not monotone by theorem, so
either prove the monotonicity you rely on for this rule and this
arithmetic or use a margin that makes the exit conservative). When
`floor_bound + R_k / 4^(remaining) > target_len` for the last round
of the schedule, return the typed `QuadratureBudget` refusal NOW,
carrying the width the schedule would have reached, or the width at
this round with the payload's doc saying which — the choice is the
implementer's and must be argued in the PR: a caller reads
`width_len` as "how far off this face is", and a width from an
earlier round is looser than the schedule's last one.

Constraints, binding:

- **D9 holds.** The cut schedule is unchanged; certification is a
  function of (face, ε, band) alone; the exit is deterministic. The
  existing certify exit is already data-dependent, so an early
  REFUSAL exit is the same shape, and the comment that says "never a
  data-dependent iteration" is about the cut points — re-word it to
  say exactly that.
- **Every face that certifies today certifies with a bit-identical
  bracket.** Prove it with the two-build digest instrument the MESH
  units used (a roster + FNV digest of every certified `FaceCutBounds`
  over the shipped corpus, the STEP fixtures and every probe suite, at
  the three ε rows and on both lanes), md5-identical across the merge
  base and the head.
- **Every refusal keeps its typed class.** `QuadratureBudget` stays
  `QuadratureBudget`; `DegenerateFace` and `QuadratureUnsupported`
  are untouched. Rows that pin a refusal's `width_len` (dm1's
  5.2477e-4 among them, and the area-gauge headroom table in
  `quad.rs`) are re-read against the new value and re-baselined WITH
  THE REASON in the row, never adjusted to preserve the number
  (`docs/prompts/implementer-discipline.md` §3).
- **No change to the rule, the remainder, the hull blocks or the
  budgets** — Simpson, tighter area pads, more blocks and more rounds
  are the header's own "next levers" and each is its own unit.
- **The k-lint gate** (`k-lint (gate)`, the margins ledger) is the
  distribution evidence for the predicate `props_quad_converged`;
  ask for the `dev-probe` row (`CI-Config: klint=dev-probe`) on the
  head and read it.

## Acceptance

- The Phase 1 table in the PR body, from the merge base.
- Hosted: both lanes drawn or asked for (`CI-Config: lane=interval`
  on one head, `lane=default` on another, each at `eps=1e-12` where
  the informative bands are), green; the digest identical across
  builds on every certifying face; every re-baselined pin carries its
  reason.
- Measured: the four TCOST-2 rows that read ≥ 97 % kernel
  (`width_versus_gap_from_a_block_edge` and the three BUDGET rows of
  `cert5_r2_probes`) and the cert6 dome in the hosted cost report,
  before and after, plus dm1's wall face locally — a refusal that took
  the whole schedule now takes the rounds the exit rule needs.
- The unit's own suite: a row per exit reason (certified early;
  refused early on the remainder bound; refused at exhaustion because
  the bound never fired) with a labelled assertion each, and a row
  proving the early refusal reports a width no smaller than the
  schedule's own would have been (or exactly it, per the choice
  above) on a face that exhausts.

## Out of scope

The rule's order, the area pad, the hull-block count, the budgets;
the 90× rational-vs-integral ratio on CERTIFYING faces (a Simpson or
exact-arm question, its own unit if Phase 1 shows it matters); any
test-content change (TCOST-2's remainder and TCOST-5 own those).

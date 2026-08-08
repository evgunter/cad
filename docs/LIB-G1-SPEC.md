# LIB-G1 spec — PATHS vocabulary growth, cheap set (binding)

Mandate: PROFILES-V2-DESIGN §V7 VQ1 ruling ((b)-direct: the algebra
grows until the persisted corpus authors fully, BEFORE the schema
switch) — this unit lands the cheap set: **circle primitive,
arc_via, arc_center, far-end anchor, exact directors**. Evidence
base: the U2 PR-2 wall list (W1/W2/W5, docs/LIB-LOG.md accumulator)
and PATHS-DESIGN §§2–4. The arc-carrier fillet modes are G2, NOT
this unit. This spec is binding; deviations numbered and REPORTED.
Where PATHS-DESIGN's existing text under-specifies an interaction,
that is a finding to report, not a silent fix.

## 0. Output discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
report ≤150 lines. Every heavy cargo row wrapped in
`scripts/with-build-slot.sh -- cargo ...`, synchronous FOREGROUND,
long timeouts, one at a time; NEVER background or park on waits.
Builds queue behind other lanes' — a wait is not a failure.

## 1. The fence

In scope: `crates/profile` (path.rs + sugar as needed),
`demos/tour` (the migration, §4), `docs/PATHS-DESIGN.md` (the
addendum, §3). OUT: schema/persistence (the switch unit's);
editor-core; crates/sweep (U3's, possibly still in flight — if
its PR has merged, rebase onto it; coordinate by merging
origin/main, never by touching its files yourself); arc-carrier
fillets (G2); the lift tool (switch unit); no CI edits; no
render regeneration.

## 2. The five constructors (design; the addendum text's source)

All five: closed forms only, no iteration; authored points stored
verbatim, derived quantities (bulges, rays) computed at lowering —
nothing computed is ever re-typed by the author. Typed refusals
through the existing predicate funnel. §5's one-struct
representation is preserved (extend `PosData`/the ang slot's
payload as needed; fields stay private, binders stay the only
constructors, off-lattice stays unreachable — compile-fail rows
extend accordingly).

1. **`circle(center, r)` — a one-step complete-loop program form,
   not a chain.** It authors NO seam, so PQ4 (mid-carrier seams
   refused for chains) is untouched — the conventional split is
   the primitive's PRIVATE lowering, exactly the M2
   closed-carrier precedent. Lowering MUST reproduce the corpus's
   existing circle convention bit-for-bit (read the demos'
   circle helpers first; byte-identity of migrated scenes is the
   acceptance). `r` classifies Positive (typed refusal),
   consistent with the U2 sign gates. Multi-loop profiles: a
   circle is one loop among others, composable with chain loops
   in the same profile.
2. **`arc_via(via, end)`** — from a positioned tip: the arc
   through (current, via, end). Bulge derived via the existing
   `sugar::bulge_from_via` closed form. Junction semantics
   identical to `arc_to` (a free arc; the standard
   definitively-non-tangent junction check under the band
   machinery). Collinear/degenerate (via on chord, zero-radius)
   → typed refusals, funnel-classified.
3. **`arc_center(center, end, winding)`** — the arc from the
   current point about `center` to `end`; `winding` (Ccw|Cw, a
   structural argument, not a number) selects which of the two
   arcs. Equidistance is CHECKED (|tip−center| vs |end−center|
   through the funnel): definite mismatch refuses typed — never
   silently re-project the center (no repair). Bulge derived via
   `sugar::bulge_from_center`. This is the lantern/ball spelling;
   the lantern's documented centre-intent comment is the
   acceptance narrative.
4. **Far-end anchor (W5)** — the smallest faithful spelling per
   PATHS §3's anchor semantics that lets a post-fillet side END
   at its authored far vertex (the natural authoring PR-2 found
   impossible: today needs a synthetic mid-side anchor + a
   length). Design this against the doc's anchor/trim rules and
   REPORT the exact form chosen (e.g. an arrival-side
   `to(p)`-family verb whose anchor is the far endpoint) with
   its junction/trim semantics stated for the addendum. If the
   doc's anchor rules genuinely conflict, that is a
   finding-back, not an improvisation.
5. **Exact directors (VQ4)** — a direction-valued alternative to
   `.angle(θ)`: `.toward(dx, dy)` (components stored exactly;
   the ray built from them without a trig round-trip, so
   axis-aligned directions are EXACT — the bracket's `.angle(PI)`
   1-ulp drift class dies). Same lattice slot as `angle`
   (binds the angular DOF); `(0,0)` refuses typed. The
   representation may widen the ang payload to angle-or-direction
   but must keep the one-struct §5 shape. Acceptance: the
   BRACKET moves to the algebra bit-identically (the loop PR-2
   measured and kept raw — its migration is this constructor's
   proof).

## 3. The PATHS-DESIGN addendum

The PR adds a §2-addendum section to docs/PATHS-DESIGN.md (title:
"G1 vocabulary growth, 2026-08-08 — ratified via PROFILES-V2-DESIGN
VQ1(b)") documenting the five constructors with the same register
as §2: what each consumes, what it determines, its refusal
inventory, and the two exactness contracts (stored-verbatim points;
direction-exact rays). Cite the wall evidence (W1/W2/W5, rocker
excluded to G2). This is a high-confidence elaboration of the
ratified VQ1 ruling — the PR self-merge rule does NOT apply to you;
the orchestrator merges after review, as always.

## 4. Corpus migration (the acceptance)

Move to the algebra, wholesale per loop: the four circle loops
(bodies::circle ×2 uses, bossplate::boss, curvedcut::disc,
lily::circle_loop), the via-point loops (vase, sheave, lily
leaves), the centre-first loops (lantern, ball), the bracket (via
`.toward`), and the W5 case if one exists in the corpus (report
if none does). Rocker stays raw (G2), the bowtie stays raw
(kernel-layer demo, permanent). Gap comments on remaining raw
loops updated to name G2 or the permanent reason.
**Zero geometry diffs**: every pin, volume, ε row, and export
byte-identical vs the merge-base — build the base in a scratch
worktree inside your lane and diff the export trees yourself at
all three ε rows. The lantern/ball/bracket loops specifically must
lower bit-identically to their raw predecessors (that is what
"derived at lowering from stored authored points" buys; any ulp
divergence is a defect or a numbered, measured deviation kept raw
as PR-2's bracket precedent).

## 5. Tests

Extend the three U2 families: differential (each new constructor
vs its hand-built raw twin, bit-level), property (authored points
on path; junction totality; sign gates), compile-fail (new
off-lattice rows for the widened surface). Circle: one-step loop
validates, r≤0 refuses, PQ4 pin still green (chains still refuse
mid-carrier seams — the primitive changed nothing). arc_center:
equidistance refusal row + both windings differential. toward:
exactness pin (axis directions produce exact rays; the bracket
loop bit-identical). No new [[test]] binaries.

## 6. PR discipline

One PR. Commit AND push after every coherent chunk. NO
Co-Authored-By trailer, no model names in commits (blinding).
Merge origin/main immediately before opening (U3's #245 may land
mid-unit — re-merge and re-run the profile battery) and re-merge
if main moves; confirm checks STARTED after any push. PR body:
constructor semantics as landed, migration census, byte-diff
proof, addendum text, numbered deviations. Report ≤150 lines to
`~/.local/share/cad-work/lib-g1-report.md` — include your own
per-phase token/wall figures if you can observe them (the A/B log
now records impl/fix/review separately). Open, do NOT merge.
Final message: PR number + report path, nothing more.

# CERT-4 — issue 1191: period-fold widening at Interval

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged at spec: **L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1191 is the primary specification; this document fixes scope,
the ratified constraint restatement, and acceptance.

## The defect

A floor-based period fold evaluated at `Interval` widens toward the
whole period whenever the argument box straddles a step of the
`floor` — a correct enclosure of the function *as written*, which is
the point: the function must stop being written that way. The live
instance (the rocker eye's anchor-coincident corner) passes through
two composed folds and comes out `[−τ, τ]` where the truth is `0`.
The tree-wide hit list with per-site dispositions is the issue's
table; it is the scope.

## The constraint, as ratified (supersedes the issue's framing)

The issue was filed under an f64-bits-must-not-move constraint. Evan
RESTATED it semantically (in-chat, 2026-08-29, recorded at the plan's
Q2 seam and in `memories/output-stability-as-justification.md`):

- The unit MAY reformulate both lanes — f64 included — where that is
  the cleaner shape; a flipped classification is fine when
  semantically correct and the code cleaner.
- PROVIDED the exact-fit guarantee survives: a true tangency must
  still classify as an exact fit, by a **preserved structural zero**
  (today: `extent − setback` bit-zero when a tangent point lands on a
  leg's far end) or by a **re-derived gate** — never by a
  re-baselined near-miss.
- If you take the both-lanes rewrite, the PR body carries the
  re-derived gate's design and its argument explicitly — the
  orchestrator adjudicates it at review; do not silently re-baseline
  any exact-fit row.
- Any f64 bit that moves is measured and REPORTED (the CERT-3
  precedent: sweep a corpus, count moved coordinates, state
  magnitudes); affected pinned rows are re-derived with the argument
  in the PR body, never re-baselined silently.

## The deliverable

1. **A fold whose interval enclosure is computed from the true
   angular difference** rather than a `floor` over a straddling box —
   the issue's own shape of the answer. Where the argument is derived
   from `atan2` of enclosures of separately-rounded points, the
   reformulation must make the straddle-at-zero case come out at the
   width of the inputs, not the period.
2. **The profile sites**: `arc_fillet.rs` `FilletSide::travel` (the
   live instance), `sugar.rs`'s `signed_swept` / `swept` composition
   (the doubling), `bulge_from_center` (guarded today; fix or record
   why the refusal-guard posture stands).
3. **The topo sites** (the bulk of the list): `splitting/classify.rs`,
   `chord_join.rs` (window recentres, azimuth gap, arc spans, the
   open-coded `floor()±1` arms), `pcurves.rs` branch-shift integers,
   `replace_face.rs:1914`'s open-coded `signed_swept` fold — **plus
   straddle-driving rows**: nothing currently exercises these sites
   into a straddle at Interval; each fixed site gets a row that puts
   a box across its boundary and asserts input-width, not
   period-width. Red-first where the harness allows driving the
   straddle before the fix.
4. **The two standing pins flip, owned by this unit**:
   - `profile/tests/generic_replay.rs`'s
     `exactly_one_corpus_row_escalates_at_interval`: when the eye
     stops escalating, the census's set changes by design — re-pin to
     the new truth (zero rows, or whatever survives), keeping the
     census's shape-watching prose accurate.
   - `editor-core/tests/m10_3_driver_interval.rs`'s
     `a_macroscopic_box_refuses_all_of_its_mass_as_budget_today`:
     pinned red-the-day-the-widening-closes, per its own doc. If this
     unit closes the widening for the driver's certification
     predicates, the row fails BY DESIGN and the number it asserts
     becomes a real answer — re-pin it to that answer and report the
     measured certified/refused split in the PR body. This is
     M10-3's test: the PR flags the M10 orchestrator on the pin flip
     (fence etiquette, not permission — the class home is this
     issue's and the cut assigns it here).
5. **Re-sweep at merge base**: the issue's stated blind spots
   (macro/cfg_attr-generated folds, `rem_euclid`, hand-rolled sign
   branches) are re-swept with a differently-shaped pattern; hit list
   with dispositions in the PR body; state what the new pattern
   cannot match.

## PR shape

Two PRs are permitted and mildly preferred if the seam is clean
(profile sites + the census flip first; topo sites + their new
straddle rows second) — each merges on its own green head. One PR is
acceptable if the reformulation lands as one shared helper both
halves consume and splitting would duplicate its review.

## Keep-outs and fences

- `geom-core/src/linalg/vec.rs` is PCURVE P-2's until PR 1177 lands —
  read freely, edit never (check whether 1177 has landed at your
  merge base; if it has, the keep-out is void).
- M10's fences: `dual.rs`, the `AtRestPolicy` seam, `product.rs`'s
  Dual arms, the bvh interval lift — not this unit's ground. The
  m10_3_driver pin flip above is the one sanctioned touch on M10's
  test ground, flagged on the PR.
- The `editor-core` m10-p bit-identity fence: if your f64 movement
  reaches the corpus walk, follow the CERT-3 precedent exactly —
  coordinate-dump differential on both trees, structural/coordinate
  counts reported, digests re-derived with the procedure recorded in
  the fence header, M10 flagged on the PR.
- Merge origin/main before opening the PR and again if main moves.

## Order of work

1. Red-first Interval rows at the live instance (the eye) and at
   least one topo straddle nothing drives today.
2. The reformulated fold (one home; the open-coded copies collapse
   onto it or are individually respelled with the same argument —
   say which and why).
3. The remaining hit-list sites, each with its straddle row.
4. The two pin flips, measured and argued.
5. Blast radius: touched-crate suites at default + interval locally;
   k-lint per the K-REPORT runbook if it fires; any margin population
   that moves is re-derived with the argument in the PR body.

## Acceptance

- The eye's advance gate classifies at Interval with input-width
  enclosure (the `[−τ, τ]` census row retired/re-pinned); exact-fit
  rows green by structural zero or the adjudicated re-derived gate.
- Every hit-list site fixed or dispositioned; new straddle rows for
  the topo sites; ε-three-outcome honesty on rows that consult ε
  (width-assertion rows state that they consult none, per the CERT-3
  precedent).
- f64 movement measured and reported; affected pins re-derived, not
  re-baselined; fence procedure followed where digests move.
- Hosted CI green on the head; a change under `topo`/`profile` is
  not basename-matched for the interval lane unless the filename
  says interval — use `CI-Config: lane=both` on the head commit (the
  CERT-3 precedent) so the lane your claims rest on is gated.
- Sweep receipt with blind spots stated; issue-keyword hygiene
  (write "issue 1191" spelled out; the orchestrator closes it after
  merge); no `Co-Authored-By` in lane commits.
- Any refusal minted or changed classified per the D2 addendum.

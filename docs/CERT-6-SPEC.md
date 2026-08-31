# CERT-6 — issue 870: the area-gauge tripwire and its calibration (under the Q1 ruling)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged at spec: **S/M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 870 is the primary specification; this document fixes scope to
the RATIFIED Q1 ruling — the issue's always-on metering proposal is
explicitly NOT what this unit builds.

## The ruling (Evan, in-chat, 2026-08-29 — cited, not re-litigated)

No always-on area metering: the intent is that any realized geometry
everywhere within ε of correct is valid, so the wide-but-sound
default bracket stands and no funnel target is built (the O(h) cost
arithmetic independently supports this — an ε-scale area target is a
~10³–10⁴× piece-count multiplier). The check lands as a **hefty
`debug_assert` on the A2 gauge** under the D2 addendum's
row-5-boundary class: expensive checks whose failure PROBABLY
indicates a bug — currently on in every profile
(`debug-assertions = true` in release), eventually debug/CI-only.

## The deliverable

1. **The A2 gauge as a row-5-boundary `debug_assert`** at the patch
   lanes' area pass: `area.width()` against a certified perimeter
   lower bound — a mean edge displacement, the direct analogue of the
   flux funnel's mean-boundary-displacement gauge. Fall back to the
   relative gauge on `area.lo()` where a certified perimeter is not
   cheaply reachable in the lane (say which lanes fall back and why).
2. **A GENEROUS ceiling calibrated from the corpus**, with the
   calibration documented IN-FILE (the `closing_column` model is the
   named precedent; its nine-orders-off estimate on the issue-723
   input is the cautionary half — the ceiling must clear the honest
   wide cases by a stated margin, not hug the observed population).
   The assert is a tripwire for probably-a-bug widths, not a
   tightness meter.
3. **The calibration record**: issue 873's ceilings re-derived as the
   record; `review_m6_3_chart_probes.rs:354`'s deliberate lower-bound
   row re-derived, not deleted (its location may have drifted — find
   it by its own comment, not the line number).
4. **The opt-in refinement door FILED, not built**: a demand-triggered
   valve (caller-requested area target, per-round resolution, typed
   refusal) — file the issue with the design sketch and the Q1 ruling
   cited; no consumer asks today.
5. **Optional, only if it falls out cheap**: the order bump on
   `area_midpoint_taylor`.
6. **S26/S230 pointers updated at merge** (SMELL-table hygiene per
   §D's conventions — the rows this unit's landing affects leave the
   table in the landing PR, per rule 3).

## Fences and keep-outs

- CERT-5 may be editing `props/quad.rs` concurrently (knot-aligned
  composite cells). The A2 gauge sits at the AREA pass; coordinate by
  merge order — merge origin/main before opening the PR and again if
  main moves; if CERT-5 has landed, calibrate on the post-CERT-5
  corpus (its cells change area enclosures). If your diff and
  CERT-5's genuinely collide on a hunk, stop and report.
- No new refusal is minted by the debug_assert itself (an assert is
  not a refusal); if you DO mint or change any typed refusal in a
  fallback lane, classify it per the D2 addendum.
- Consolidation (C3/C-m) remains Track R's.

## Order of work

1. Measure the corpus first: the gauge value distribution across the
   patch lanes' existing fixtures at both scalar lanes — the
   calibration evidence, committed as the in-file record.
2. The gauge + assert, ceiling from (1) with the stated margin.
3. Planted-corruption check: a deliberately degraded enclosure (the
   issue-723-shaped nine-orders case, or a synthetic widening) must
   FIRE the assert; the honest wide cases must NOT. Both directions
   red/green-verified and reverted.
4. Items 3–6.

## Acceptance

- The assert fires on probably-a-bug widths and is silent on the
  corpus, with the calibration and its margin documented in-file.
- Issue 873's ceilings re-derived; the lower-bound row re-derived.
- The valve issue filed with the ruling cited.
- Hosted CI green on the head (debug-assertions are on in CI, so the
  gate exercises the assert on every fixture it walks — say so in
  the PR body). **ε posture (the issue-1356 lesson, binding on this
  unit)**: a gauge ceiling is band-sensitive by construction, so the
  final head carries `CI-Config: lane=both` AND pins the ε row by
  trailer at the tightest band your calibration claims cover
  (`eps=1e-12` unless you argue otherwise), and every new
  assert/row states its premise per band or states that it consults
  no tolerance — "green at the drawn point" is not coverage for a
  band-dependent ceiling. Run the three-ε local sweep on your new
  rows before the final push.
- Fresh calibration context from CERT-5's merge (read its PR 1314
  close-out): dm1's wall carries a certified flux beside a ~30×
  area bracket (`[1.9358e-5, 6.0906e-4]` on a ~3.14e-4 face) — a
  live honest-wide case your GENEROUS ceiling must clear; and the
  area rule you calibrate against now intersects the padded
  midpoint with the cell hull (CERT-5's fix), so calibrate on the
  post-merge corpus, not on any number quoted in issue 870 or 873
  from before it.
- ε-three-outcome honesty on any new rows; sweep receipt not owed
  (no defect class fixed) but the gauge's blind spots stated: which
  lanes fall back, and what the fallback cannot see.
- No `Co-Authored-By`; "issue 870" spelled out (orchestrator closes
  it after merge); deviations stated.

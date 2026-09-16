# EDIT-PICK3 — the pick door answers a `t` interval (spec)

**Unit:** `pick-door-answers-a-t-interval`, building the ruling `what-t-the-pick-door-answers-and-with-what-width` (kernel
unit, v6 dual, block EDIT-B1 slot 2). **Ruled by Ev** on `[ev]` PR
#2764 (2026-09-16); the ruling is the row's `## RULED` section and is
restated here as the spec's premises. This file is deleted at merge and
recorded in `docs/DOC-LEDGER.md`, as EDIT-PICK's and EDIT-PICK2's were.

Branch `edit/pick-t-interval`. Read first: `crates/editor-core/src/resolve/pick.rs`
(`crossing`, `ray_triangle`, `pick_face` and its determinism/tie-break
contract paragraph), the row and the three rows parked on it, the two
EDIT-PICK2 review summaries' measurements as the row's "Measurements it
inherits" restates them, and `docs/DOC-LEDGER.md`'s two "Per-merge
deletion — EDIT-PICK…" entries (what the last two specs got wrong on
this door: a box-entry guard false in `f64`; MEET ∧ INFORM breaking the
early-out).

## The ruling, as premises

1. **Interval.** `ray_triangle` answers the hit's parameter as an
   interval `[t_lo, t_hi]` around the rounded `t`, derived: the hit
   point `p = a + u·e1 + v·e2` moves by up to `err_u·|e1| + err_v·|e2|`
   under the barycentrics' certified bounds (`Crossing::barycentrics`),
   and the projection `t = (p − o)·d / d·d` adds its own rounding,
   counted like the siblings (an `*_ERROR_UNITS` constant derived from
   the operation count, with the derivation at ONE site, `crossing`'s
   doc or a sibling paragraph). No factor is chosen.
2. **Order.** Candidate A precedes B when `t_hi(A) < t_lo(B)`.
   Overlapping intervals are a CERTIFIED tie.
3. **Tie-break.** A certified tie is decided by the NARROWER interval
   first, then `(target position, flat triangle position)` as today.
   `pick_face`'s contract paragraph is rewritten to say so, present
   tense, with the reason: when the geometry cannot order two
   candidates the door prefers the better-certified claim; the viewer's
   GPU id pass picks what is displayed by construction and this is what
   makes the ray path agree with it in the edge-on class.
4. **Clamp.** The hit point is the nearest point of the CLOSED triangle
   to the rounded barycentrics (`u ∈ [0,1]`, `v ∈ [0, 1−u]`, the
   per-coordinate projection; state the rule at the site). Under the
   closed acceptance the clamp cannot move an admitted point beyond its
   own rounding; under MEET it is what keeps the hit on the triangle.
5. **The box.** Enters only as the traversal's early-out:
   `t_hi(best) < cand.t_enter` breaks the scan. The docs' near-tie
   class (a rounded entry a few ULP off) is restated for the interval.

## Left to the unit, measured

**Closed ∧ INFORM stays, or MEET ∧ INFORM returns.** Under premises
1–5 run the three-rule table (the tie-break aim, 19 296 rays:
beyond-or-miss / `Pruned ≠ Every` / winners with a bound ≥ 1) and the
wide aim (441 126 rays; `crates/viewer/tests/review_pick2_r1.rs`'s
shape) for BOTH acceptances. Take MEET only at `0 Pruned ≠ Every` and
no lost aim; otherwise closed stays and the 149 grazes remain the
stated class (`pick-closed-acceptance-loses-a-graze-to-rounding`,
which then closes as "measured, stays"). Report the two tables in the
PR body. The corner-labelling row: measure whether the asymmetry
survives the clamp; if it does, it stays its own row and you say so.

## Rows

- The wide-but-informative fixture
  (`a_wide_but_informative_candidate_answers_before_the_rings_aimed_vertex`,
  `crates/viewer/tests/index_memo.rs`) now answers the vertex at
  `t = 1.48` to the bit — RED today at `1.4488`; write it first as the
  row it becomes.
- `index_memo`'s reference loop calls the door, never restates it
  (as now); its tally columns re-baseline with the shape stated (which
  moved, which did not, why).
- A ray down a shared edge still ties two narrow intervals and the
  order decides; a row pins the order and that both answers are true.
- The early-out: a row where the interval's upper end decides the
  break, and a mutant that compares the rounded `t` instead of `t_hi`
  reds it.
- Mutants, each named with the rows it reds: compare rounded `t`
  instead of intervals (the wide fixture reds); swap the tie-break
  (narrower last) — a row must red; drop the clamp (under MEET, a row
  reds; under closed, say why none can).
- `PickHit` carries the interval (`t_lo`, `t_hi` beside `t`); the
  façade and `pncad-py` follow mechanically (LIB's, say so); the viewer
  reads `point` and is untouched beyond the type.
- No tolerance, anywhere: every width is the operation count's. A
  reviewer will grep for a chosen constant.

## Protocol

v6 dual: one implementer (this block's slot 2), two concurrent
blinded reviewers on the frozen head, union fix pass by the
implementer. Ordinals claimed on `main` at review dispatch from the
EDIT band (next free: 4802, 4803); recorded at merge as sample #215.
Blinding verbatim: no `Co-Authored-By`/`Claude-Session` lines in lane
commits; the PR body is the record.

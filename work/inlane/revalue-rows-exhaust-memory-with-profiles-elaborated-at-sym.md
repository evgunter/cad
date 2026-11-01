---
id: revalue-rows-exhaust-memory-with-profiles-elaborated-at-sym
kind: issue
title: With the pinned lift retired, revalue_corpus's boss_union row exhausts a 16 GB runner at Sym, and five other rows hold several GB each
status: closed
pr: 5068
opened: 2026-10-15
priority: P1
cost: M
refs: [the-pinned-profile-lift-launders-at-sym, the-top-residual-reduction-dominates-a-discharging-session-over-symbolic-frames]
closed: 2026-10-26
---


Found applying Ev's ruling A on #5068 (PATHS' `the-sketch-plane-is-its-frame`):
`ProfileLift::Pinned` retired, so `crates/editor-core/tests/revalue_corpus.rs`
now replays each corpus document at `Sym<f64>` with its profiles elaborated
at the lane (the guided second pass), as this program's
`the-pinned-profile-lift-launders-at-sym` owed.

Measured on #5068's head (dev profile, the hosted 4-vCPU / 16 GB runner and
a shared 4-core / 15 GB box):

- `revalue_boss_union` alone ran 3409 s on the hosted runner and the runner
  died (shutdown signal, then SIGKILL): run 38647943553, job
  116139800609, step "test (the slow set of the crates this diff seeds)".
  Four rows at once killed the runner after about 600 s on the previous head
  (run 38635923344).
- Locally the cgroup OOM-killed `revalue_boss_union` at about 5 GB RSS, and
  `revalue_cut_cylinder`, `revalue_die_chamfer`, `revalue_declared_tangency`,
  `revalue_plate_param` and `revalue_heat_sink_fins` with it, each holding
  3.5 to 6 GB.
- `revalue_loft_prism` (646 s), `revalue_face_sketch` (1053 s) and
  `revalue_measured_web` (2353 s) passed, on the shared box.

`.config/nextest.toml` now runs the re-valuation rows one at a time; that
was not enough for `boss_union`. Until this is answered the PR gate's slow
step cannot finish for any diff that seeds editor-core's slow set.

Owed: find where the symbolic replay's memory goes once profile geometry is
elaborated at `Sym` (the loops' arcs and the guided validation's
predicates are now symbolic expressions, where the pinned lift made them
literals), bound it, and re-pin each row's move and floor as
`the-pinned-profile-lift-launders-at-sym` asks. A row whose result changes
is a finding.

## Where the cost went (measured, release, `revalue_cost_profile`)

**This is a pre-existing production cost that the row now exercises, not
a regression.** On main, the coincidence replay and the E6 drive already
elaborated profiles Guided. Only this row took Pinned, which hid the cost
from it.

**The cost is the identity tier, and it scales with how many symbols
reach the profile.** The DAG is not the cost:
- loft_prism's `p0` build at `Sym<f64>` takes 61 s and 4.6 GB in a
  session that discharges each decision as it is taken (the row's
  session before this fix, at the drive's 4096 terms).
- The same build at a zero-term budget, or in a deferring session, takes
  0.6 s.

The mechanism:
- A corpus frame's six axis components are free variables, and the
  sketch plane is minted at the lane by Gram-Schmidt (`wire::frame_axes`).
  Each profile point is therefore a rational function of nine symbols
  over two `sqrt` atoms.
- The predicates over several points multiply those forms up to the
  4096-term budget (largest 4088 terms, degree 84 on loft_prism), and
  then most of them freeze (7508 freezes on loft_prism).
- **Time:** the top-residual reduction (`algebra::reduce`, not memoized)
  is 35 s of loft_prism's 61 s and 136 s of face_sketch's 149 s. The
  walks are most of the rest. On loft_prism 18 s of the walks are the
  inexact witness's dispute check, which runs on every definite decision.
- **Memory:** the session's form memos, about 80 000 forms of about 200
  terms each.

The full table and the tier-side options (bound or memoize the top
reduction, the dispute check at an inexact witness, a forms cap) are on
SYM's slate as
`the-top-residual-reduction-dominates-a-discharging-session-over-symbolic-frames`.

## The fix

The row records its builds in a **deferring session**, which is the
session the coincidence door replays in (`coincide/replay.rs`):
- **Why it reads the same thing.** Re-valuation reads each decision's
  DAG, its value and its numeric sign, never the tier's verdict. A
  deferring session builds the same DAG without discharging every margin
  as it is decided.
- **Where the two sessions can differ:** only at a decision the `f64`
  channel escalates over a real margin, where a discharging session
  could prove `Zero`. The row counts those (`escalated`) and asserts
  none.
- **The door is unaffected.** It never paid this cost: `boss_union`'s
  door replay takes 131 ms.

Every row now runs in at most 3 s and about 110 MB (release). None
launders.

**The re-pins are findings of the ruling.** With profiles elaborated at
the lane, the profiles move:
- `plate_param` now parts under the apart move at `point_in_loop_arm`,
  and `boss_union` at `pcurve_chart_azimuth_frame`. A frame's axes tilt
  under that move. Both rows are pinned in step, as `cut_cylinder`
  already was.
- Under the apart move, `boss_union`'s prefix shows `bool_pair_gap`
  mismatches at #4037/#4040, just before the builds part. The margin at
  #4037 re-values to exactly p1's #4040, so this is a reordering of pair
  checks, not a laundering.
- The floors are re-pinned at half of what each row now moves: loft_prism
  moves 518 margins, where it moved 3 when its profiles were literals.
- `boss_union` decides `chart_bound_outer_span` three times on a NaN
  margin. A NaN margin is a domain violation, which no session
  discharges.

## Closed (2026-10-26)

Fixed by PR 5068, merged 2026-10-19 (`paths/sketch-plane-is-its-frame`). This row stayed in `review` after that merge. Its `## The fix` section is what that PR landed: the row records its builds in a deferring session, the session `coincide/replay.rs` replays in, with no escalation over a real margin. Every row runs in at most 3 s and about 110 MB, and the parting rows are re-pinned (`revalue_plate_param_apart_to_its_parting`, `revalue_boss_union_apart_to_its_parting`, named in the PR's M2). The `revalue_corpus` rows stand in editor-core's slow set (`.config/nextest.toml:32`), which the nightly runs.

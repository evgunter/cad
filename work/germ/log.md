# GERM — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

6 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7000-7099 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-25 — an orchestrator picks the track up

Status `ready` → `active`. Orchestrator branch `germ/orchestrator`, unit
branches `germ/<unit>` (the #396 prefix convention, which holds over a
harness-pinned branch name per PR 3247, approved by Ev in chat).

Opening order, a sequencing call: two lanes run in PARALLEL from the start,
as the cut intended.

- `ray-torus-root-search-finds-a-counterexample-at-eps-1e-12` — the plan's
  first row and the only one with a recorded counterexample. Measure-first:
  replay the seed, and decide which of the three candidates is wrong (the
  root chain, the oracle, the agreement window in band units) before
  touching `line_torus_roots`.
- `torus-operand-gate-admission` — raised beside it rather than behind it,
  because Ev hit it as a user on 2026-09-17 (the dumbbell union in the viewer)
  and asked for it at high priority. Its first step is a measurement, not a fix:
  the row itself says to confirm that the (torus, plane) germ-pair join is
  what blocks before scoping, and a refusal's text is not its cause
  (`memories/refusal-text-is-not-cause.md`). That lane is read-only and
  reports a scope. The unit is cut from what it finds.

Why this beats the plan's strict serial order: the second lane's first step
is read-only and does not touch the root search the first lane may change.
Running it first gives the unit the user is waiting on a measured spec while
the root-search lane is still underway, and costs the root-search lane nothing.

## 2026-09-25 — the dumbbell measured; three lanes out

The torus-gate measurement lane reported (in two rounds; transcript-only,
so the load-bearing findings are here and in the item files):

- **The row's premise is refuted.** On every revolved-dumbbell spelling,
  the torus meets the other operand's plane only at its BOUNDARY (a rim or
  meridian lying in the plane). In the transverse spelling it also carries a
  coincident same-carrier torus pair. No walk reached `join.rs`'s germ-pair
  dispatch. So the (Plane, Torus) join arm and `plane_torus_section` are not
  on this path, whatever the refusal text named.
- **A wall ahead of every torus door, and not ours.** A full revolve of an
  axis-touching profile emits each planar wall as two same-key halves, and
  F7 refuses it as `NonMaximalFaces`. This holds for a plain cylinder too.
  Filed on CARVE's slate as `full-revolve-emits-split-planar-walls` (P0).
  It changes F7 or N3/N4 text, so it is a fork for Ev. The designer pair
  runs on it now (blinding recorded on
  `analysis/design-fork/full-revolve-split-planar-walls`).
- **The torus doors behind it**: (a) the gate; (e) the carrier-identity rung
  before the sampled clearance; (c) the containment torus arm; a line×torus
  crossing in `wall_crossing`; a torus arm in `sector_face::resolve`. Past
  them the union stops at `Join(UnpairedLooseEnds{4})`, the same as a
  torus-free control.
- Evidence added to TANG's `arc-aware-point-in-loop`: a torus-free
  standalone `contfp` fixture.

Dispatched, with review tiers named at dispatch:

- `germ/torus-doors` carries `torus-operand-gate-admission`, the coincident
  pair row, and the TORUS half of the containment row (the cone half is
  split off as `curved-face-containment-lacks-a-cone-arm`). Its fixture
  pre-merges the operands, pending the fork. **Tier: dual.** It is a
  reduction-order change plus three new certified arms on P0 ground,
  where the failure mode is a confident wrong answer.
- `germ/ray-torus-root-search`: **tier decided at its report.** It is a
  full single review if the fault is the oracle or the criterion, and a
  dual if the certified enclosure is found to exclude the truth.

Sequencing call (logged, not asked): I judged admitting the torus at the
gate (a) a faithful elaboration, not a fork. It is the ratified
CURVED-TORUS spec's own next door, and its stated honesty condition (torus
arms in the crossing layer) lands in the same PR. The measurement lane
thought it wanted a ruling. If the review finds the `class_of` ordering
premise shaky, it goes to Ev.

## 2026-09-25 — the full-revolve fork goes to Ev

Designer pair on `full-revolve-emits-split-planar-walls`, blinded, with the
mapping on `analysis/design-fork/full-revolve-split-planar-walls`. First
reports, recorded before any reconciliation:

- **A**: the revolve runs the structural merge as its own final stage; F7's
  sweeps clause is widened by one clause; the merged planar wall is plain
  `Band(s)`; π revolve a SEPARATE question (lean: recognise a half turn like
  `Full`, or park). Seat likely. Did not reject the framing. Changes ratified
  text (F7, pole-valence bullet, N1 poles, `RoleSeg::BandPi`).
- **B**: the same seat; F7 rewritten as the general rule; `Band(s)` (unsure,
  Ev's call); π answered NOW (B1: decide θ vs π at the band and mint the end
  cap Shared; B2: a declared `Half`). Did not reject the framing. Changes
  ratified text.

They agree on the final state for the main question and split only on F7's
wording and on the π timing, so no reconciliation round was run. The split
is stated for Ev. Put to Ev as an `[ev]` PR (`germ/ev-full-revolve-maximal-walls`,
stacked on the claim PR). The design-fork log row waits for
`docs/DESIGN-FORK-LOG.md` (PR 3247).

Orchestrator-side actions from the reports:
- ONE final-stage merge seat for the continuation and the band twins.
  A note is left on BAND's log (below, announced seam).
- Before implementing: measure the merge on a full-revolve corpus (pole
  disc, off-axis annulus, joint disc), and check the lamina (off-axis)
  plane annulus, which keeps an interior seam edge (two canonical forms for
  one annulus).
- The `UndeclaredCoincidence` diag's `margin: Invalid` rides
  `germ/torus-doors` as a drive-by (already in that brief).

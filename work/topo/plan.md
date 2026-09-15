# TOPO — the topology core (plan)

**STATUS: OPEN (2026-09-04).** Live state is `log.md`'s tail and the
item files beside this plan, never this file.

Branch prefix: **`topo/`**. Away-channel tag `(TOPO orchestrator)`.
A/B band **2700–2799** (`docs/MODEL-AB-LOG.md` owns every live
experiment number).

## Why this program exists

`docs/WORK-TRACKS-2026-09.md` (the 2026-09-03 cut) recorded that **no
program owns 37 `crates/topo/src` files**, and cut eleven tracks around
that gap without closing it. Re-measured on 2026-09-05 against merged
main: **55 of the 102 `.rs` files under `crates/topo/src/` fell in no
open program's `paths`** — 35 of them still do, this program having
taken the other 20. (PR #1899 said "47", a figure read off one line of
a directory-grouped scan that dropped the `review_m1_pr2/` and
`seqgen/` subdirectories; corrected here by re-deriving.) It is the
largest unowned kernel territory in the tree, and it is where the
Euler surgery, tier-3 validation and the birth-identity channel live —
the layer every other kernel program builds on.

The same gap shows up on the code-quality side. **Track P** — fourteen
open rows whose fence is exactly `euler*.rs`, `validate.rs`, `live.rs`,
`seqgen.rs`, `merge_faces.rs` and the review/fixture readers — is
listed as claimed by "this program" (code-quality itself) and **has
never had a lane**: no `smell/p-*` branch has ever existed, while K, X
and T lanes have run. The rows are not stalled on a ruling or a
dependency. They are stalled on an owner.

This program is that owner, and it takes Track P whole, per the cut's
rule that a program claiming a code-quality letter takes the whole
letter so the schedule stays single-owner.

## The fence

The `paths` list is **enumerated file by file**, not globbed. That is
deliberate and it is the one thing a reader should not "tidy":
`crates/topo/src/*` is an `fnmatch` glob that crosses `/`, so it would
silently double-claim `boolean/`, `splitting/`, `query.rs`,
`coherence.rs`, `shell.rs`, `pcurves.rs` and `props.rs` — five
programs' ground plus two code-quality letters — and
`scripts/work.py territory` **cannot see a double claim** (that is the
finding in `work/meta/territory-cannot-see-a-path-two-programs-both-claim`,
raised by exactly this failure between FIX and SHELL). Until that lint
exists, the only honest fence is an enumeration.

What this program does **not** take is written in `program.md`'s
`keep_out` in full. The two seams worth naming here:

- **`query.rs` is SEAT's.** `face-kind-read-has-two-homes` is a
  question about a pair of doors, one of which is SEAT's. The ruling
  can be made here; the edit on SEAT's side is announced on SEAT's
  board before it lands, and never taken silently.
- **The remaining 35 `topo/src` files are unowned and NOT finished.**
  `body.rs`, `entity.rs`, `geometry.rs`, `instance.rs`, `null.rs`,
  `lib.rs`, `contact.rs`, `separation.rs`, `ray_parity.rs`,
  `offset_axial.rs`, `iso.rs`, the four `sector_*`/`chart*` files, the
  eleven `review_m1_*`/`review_d21`/`r2_probes` readers,
  `test_support_impl.rs`, and `param_source.rs` (new on main with
  SEAT-7). Three of the 35 — `chart_region.rs`, `face_normal.rs`,
  `props.rs` — sit inside a code-quality letter's fence (Q, Q, M) while
  no *program* claims them, which is a different kind of unowned and is
  the letter-holder's to take. A row landing on one of them draws the fence
  in the PR that mints the row — the rule the code-quality plan states
  for the `geom-brep` seam. Extending this program to the whole crate
  is a later decision and not this opening's.

## The slate

**Three issues, re-homed at opening** (header edit and `git mv`, ids
unchanged):

1. **`validate-tier3-curved-boundary-containment`** (H) — face-boundary
   containment on curved surfaces, the last unmarked deferral in
   `validate.rs`'s not-yet-checked list. Its own `## Home` said
   `crates/topo/src/validate.rs` is in no open program's `paths`; it is
   in this one's now.
2. **`no-public-census-or-genus-query`** (D→E) — the Euler–Poincaré
   identity is hand-written above eight call sites with no public
   census or genus door. Its `## Home` names the door's site as
   "beside `euler.rs`/`fixtures.rs`/`seqgen.rs`, which no open
   program's `paths` covers"; `readback.rs` is the typed-read module
   and the natural seat. Code-quality parks `S79` on it.

**Track P's fourteen rows, claimed whole** — they keep their ids and
their `track: P` letter, and this directory is where they are open,
dispatched and closed from now on. The code-quality plan already runs P
as three disjoint sub-lanes, and that partition is inherited unchanged:

| sub-lane | rows | files |
|---|---|---|
| Euler surgery and validation | `S93`, `S94`, `D265`, `D262`, `D263`, `S330`, `S69` | `euler.rs`, `euler_ring.rs`, `euler_kill.rs`, `merge_faces.rs`, `validate.rs` |
| the review and fixture readers | `D107`, `D261`, `D264` | `review_d18.rs`, `review_d18_probes.rs`, `fixtures.rs`, `source_walk.rs` |
| liveness and the generator | `D50`, `D260`, `D20` | `live.rs`, `seqgen.rs` |

The three share no file, so they can run at once.

## Order

The first block (TOPO-B1) is concluded: `S330`+`S94`, `D261`+`D264`,
`D50`+`D260`, the two-homes ruling, `D265`, `S69` and the census door
are closed, and the slate is now mostly what other programs placed
here while those ran — SHELL's six diagnoses on TOPO's files, plus the
rows that carried over (`S93`, `D107`, `D263`, `D262`, `D20`, the H
item and the residues).

**Block TOPO-B2** is concluded (2026-09-14): the ring-nesting decide,
`split_edge`'s carried pcurve rows and `revert`'s mirrored chart
images, three duals, no tally candidate.

**Block TOPO-B3** is concluded (2026-09-14): the loop-re-parenting
rows, D263's placeholder regime and S93's `mev` half; three duals, one
doc-class tally candidate; `S93` stays open on its `kev` half.

**Block TOPO-B4** takes three kernel answers on this program's own
files, in this slot order (fixed before the block byte is drawn):

- slot 0 — `geom-source-absence-conflates-four-origins`: a bare
  `None` from `surface_source` covers imported, hand-built,
  kernel-derived and a failed re-stamp, so Ev's question "how do we
  know when there is no provenance" has no answer. The unit makes
  absence typed on the identity channel (`source.rs`), with the
  cleared-and-not-re-stamped case distinguishable from never-stamped.
- slot 1 — `revert-leaves-a-periodic-charts-loop-wrap-mid-chain`:
  `revert` re-states plane rows with their frames but a periodic
  chart's one-period wrap, parked at the forward loop's closure, sits
  mid-chain once the loop runs the other way. The unit re-parks the
  wrap (a `shift_branch` per row, no re-certification) or re-mints,
  phase 1 deciding with the involution as the test.
- slot 2 — `edge-carrier-kind-has-no-readback-door`: the two-homes
  ruling gave faces a `readback` twin and left edges with only the
  query-side door; the unit adds the readback door for the edge side
  in the ratified shape.

**Block TOPO-B4** is concluded on slots 0 and 1 (2026-09-14: the typed
provenance absence and the revert wrap re-park, ordinals 2710 and
2709, one code-class tally candidate) with slot 2 (the edge-side
readback door, ordinal 2711) in its fix pass; `D107` merged non-dual.

**Block TOPO-B5** takes three kernel answers on this program's own
files, in this slot order (fixed before the block byte is drawn):

- slot 0 — `set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left`:
  `set_face_surface` swaps a face onto a chart that mints nothing and
  leaves its complete row set stated in the chart it left, which the
  tier-3 pass then skips. The unit gives the setter the loop doors'
  answer — drop the rows on a chart change through `same_chart`,
  deriving nothing — and makes its `DECLARED` posture note true.
- slot 1 — `mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`:
  the loop-re-parenting fix one level down — `mef`'s chord surgery and
  `kef`'s unsplice move a RUN of half-edges between two faces' loops
  and leave the run's rows on the chart they left. `mef` is the
  two-line carry-or-drop on the run it already holds; `kef` has to
  resolve the surviving face inside a kill operator's precondition
  order, which phase 1 decides.
- slot 2 — `a-null-edge-can-be-re-based-onto-a-distinct-point`, taken
  together with `the-re-basing-gate-refuses-m7-8-where-nothing-moves`
  (two rows wanting one answer): the fan `mev`'s re-basing gate skips
  null scaffolding unconditionally, so a null edge can be left
  spanning two distinct points, and refuses the plane × NURBS class
  even where nothing moves — because the exact "is `p_new` the point
  `p_old`" question has no door at `T: Real`. The unit answers
  structurally (a moved run refuses a null edge; a no-move `mev` is
  spelled as the copy `mev_null` already is) or says which shape and
  why.

**Ruled on PR 2527 (2026-09-14), cut at the next block** — three
kernel answers now units on this slate:

- `a-chart-spans-solids-after-move-shells-to-new-solid` — (A): the
  mover re-mints the moved shell's shared surface keys, the invariant
  stated at `Body` and checked in tier 1, SHELL-8's arm retired by
  announced seam.
- `kevs-fan-merge-needs-a-re-describing-kill-door` — (c) as two doors:
  `kev(he)` refusing typed where a merged carrier would go stale, and
  `kev_describing(he, specs, tol)` for the callers that re-describe;
  `S93` closes with it.
- `half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`
  — the operator mints the row at the mint site; phase 1 is whether
  the closed-form derivation splits from the fitted lane under
  `Decide`; the closing-mint convention retires.

`attach-postconditions-validate-the-whole-body-and-panic` closed on
the D9 reading.

**Not in TOPO-B5**, and why: `check-9-nesting-is-line-bounded-only`'s
disc third is S-BOOL's `disc_side` to open and its arc third waits on
#1076; `split-edge-cannot-carry-a-fitted-or-general-pcurve-row` and
`two-provenance-free-keys-holding-one-surface-read-as-two-charts` are
the bound ripples on Ev's `[ev]` PR; `euler-characteristic-has-three-carriers`
is the next block's structural slot.

Beside the block, non-dual: `D107` (a `kemr` fixture past its plan
phase for `review_d18`'s hammer — test support, style review only).

The `kev` half of `S93` (three landing shapes on its row, each
constrained by the generator's kills) and the half-edge-minting
operators' pcurve posture (a bound ripple or a declared primitive
posture) are Ev's questions, added to the open `[ev]` PR.

**Not in the block**, and why:

- `a-chart-spans-solids-after-move-shells-to-new-solid` is a design
  choice (is "a chart lives in one solid" a `Body` invariant the mover
  re-mints for, or is the shell doors' grouping per (solid, surface)?)
  with S-BOOL holding the disconnecting-subtract half — an `[ev]`
  ruling proposal, written by the orchestrator.
- `tier-3-does-not-check-shell-roles-per-solid` is the right check
  and lands red today: the boolean's hollow-operand subtraction files
  two `Outer` shells under one solid
  (`work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`,
  open). It waits on that row, announced on S-BOOL's log.
- `attach-postconditions-validate-the-whole-body-and-panic`: its cost
  half is answered — PERF-4 (Ev's ruling on
  `work/perf/d1-per-op-tier1-sweep-price`, PR 2305) made the setters'
  tier-1 sweep once per public door through the surgery scope. What
  remains is the panic half, and it is D1's question, not the
  setters': a postcondition asserting "kernel bug" fires on a body
  that was torn BEFORE the write. That goes to Ev with the chart
  ruling, not to a lane.
- `S93`, `D107`, `D263`, `D262`, `D20`, the H item and the residues
  keep their earlier placement: `D262` waits on `work/meta`'s
  `decide_flagged` register, `D20` on the lane budget, the H item on a
  tier-3 read that check 9's unit now provides.

## Review posture

**The program's first question to itself rather than to Ev.** Track
P's rows are mixed: `D260` and `D264` are prose, `S330`/`D262` move
kernel answers, and `D50` is a guard with no test today. Settled at
opening (log, 2026-09-05), matching FIX's on the same kind of mixture:
one style review per unit, plus a full v6 dual where a unit moves a
kernel answer rather than its rendering; non-dual units record no row
and the band is drawn only by the duals. Revisit at the first H
dispatch.

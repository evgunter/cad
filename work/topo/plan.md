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

The opener is **`S330`** — tier-3 check 1 has no arm for a described
Nurbs surface, so a poisoned net falls to `_ => {}` and is refused only
by accident through check 2. It is small, it is a wrong answer rather
than a tidy, it is in `validate.rs` where the H item and half the track
also sit, and it puts this program's first read on the file everything
else here depends on. `S94` (the two hand-kept `VARIANTS` ladders)
rides with it, per the code-quality plan's fold-in rule.

(`S331`, the plan's original opener, is on TRIM's slate: its mechanism
is `crates/topo/src/pcurves.rs`, TRIM's file, beside TRIM's `D36`.)

Then, in parallel across the three sub-lanes: `D261`+`D264` (the reader
collapse, which owes the shared `UNCONVERTED_TODAY` re-derivation) and
`D260`+`D50` (the `live.rs` pair, where `D260` is one sentence and
`D50` is the guard that sentence is about; `D50` follows `D261` because
the guard reads source through the shared lexer).

`face-kind-read-has-two-homes` is the one D and runs on its own clock:
it is a ruling plus a one-door delegation, and its gating cost is
SEAT's agreement rather than a lane.

`validate-tier3-curved-boundary-containment` is the H unit and does not
open first: it is a marker for genuinely unbuilt work, it wants the
tier-3 file read end to end, and `S330` is that read.

Two rows carry a stated dependency and are not takeable early:
`D262` (whose twelve silent sites overlap the `decide_flagged` F2
family the register in `work/meta/` tracks) and `D20` (which closes on
an attribution measured off hosted CI, so it queues on the lane
budget rather than on a reviewer).

## Review posture

**The program's first question to itself rather than to Ev.** Track
P's rows are mixed: `D260` and `D264` are prose, `S330`/`D262` move
kernel answers, and `D50` is a guard with no test today. Settled at
opening (log, 2026-09-05), matching FIX's on the same kind of mixture:
one style review per unit, plus a full v6 dual where a unit moves a
kernel answer rather than its rendering; non-dual units record no row
and the band is drawn only by the duals. Revisit at the first H
dispatch.

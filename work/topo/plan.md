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

Blocks TOPO-B1 through TOPO-B5 are concluded, and blocks end there:
the model A/B protocol was suspended on 2026-09-23, so units no longer
draw an arm. Their records live in `docs/MODEL-AB-LOG.md`, and the
log's entries carry what each unit closed. B5 (2026-09-24) closed the
setter's rows (PR 2594), the run doors' rows (PR 2603) and the fan
`mev`'s null-edge hole (PR 3148, the first `docs/DUAL-REVIEW-LOG.md`
row). The M7-8 question went to Ev as PR 3156.

**Next, run two at a time** (the box holds two fresh builds), each
with its brief and tier on its item:

- `kevs-fan-merge-needs-a-re-describing-kill-door`: Ev's (c), two
  doors. `S93` closes with it. DUAL.
- `half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`:
  mint the row at the mint site. Phase 1 is the closed-form split
  under `Decide`. DUAL.

**After them, by priority:**

- `mint-face-surface-and-sense-reads-key-equality-where-same-chart-reads-provenance`
  (P1, E; a single review).
- `three-spellings-of-one-chart-answer-the-same-question-differently`
  (P2).
- `set-face-surface-hands-the-caller-an-ordering-obligation-in-prose`
  and `movefac-row-skips-three-component-shells` (P3, E).

**Not yet, and why:**

- `D262` waits on `work/meta`'s `decide_flagged` register.
- `the-re-basing-gate-refuses-m7-8-where-nothing-moves` waits on Ev
  (PR 3156).

## Review posture

Per unit, at spec time, on the tiers in
`memories/orchestration-model.md`. The orchestrator's read covers
mechanical changes; a single review (style, or full with claims to
falsify) is the default; a dual review covers especially tricky logic
or an architectural decision that would be hard to change later. The
tier and its one-line reason sit on the unit's item or in the log at
dispatch. A dual is a row in `docs/DUAL-REVIEW-LOG.md`, run by
`docs/DUAL-REVIEW-PROTOCOL.md`.

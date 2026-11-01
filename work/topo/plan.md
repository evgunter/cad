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
  `lib.rs`, `contact.rs`, `separation.rs`, `ray_walk.rs`,
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

Twelve live rows, P2 to P4, three of them with no `cost` (`python3 scripts/work.py
status --program topo` is the live view, and this file does not restate
it). The opening slate (the three re-homed issues and code-quality Track
P's fourteen rows) has closed or moved to the 2026-09-20, 2026-10-11 and
2026-10-26 successors; the last of those sent the describing-door family
to RESTATE (`work/restate/plan.md`) and PIPE's two layering rows to
CORES. What is left falls into three groups:

- **P2, on the goal's ground.** `revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet`
  (H: the re-pairing CLEAVE's PR 4795 ruling unblocked),
  `mfkrh-leaves-an-orphan-surface-under-the-per-op-scalpel`,
  `movefac-torn-bodies-fuzz-seed-panics-at-the-door-postcondition` and
  `a-lone-site-placeholder-has-no-type-of-its-own` (a representation
  change; designers first, and Ev weakly prefers to hold that round until
  D10 settles).
- **Endings and the lift** (P3): `split-param-not-interior-…`,
  `merge-door-bend-ending-…`, `merge-faces-undescribed-ending-…`,
  `offset-derive-swallows-…` (E each but the bend ending), and
  `euler-driver-lift-is-held-by-convention` (M).
- **Cost and hygiene** (P4 and unpriced):
  `a-joined-finisher-mints-the-whole-body-twice`,
  `topo-internal-stored-curve-ladders-…`, and topo's lib-test compile
  cost (`topos-unit-tests-recompile-…`).

## Order

By priority, with the cheap E rows taken as drive-bys:

- The P2 rows first. `revert-re-pairs-…` is a unit of its own.
  `mfkrh-…` and `movefac-…` are a replay and a fix each.
- The endings and the lift next; the E endings ride beside any unit.
- Cost and hygiene last. Price the three rows with no `cost` at pickup.

## Review posture

Per unit, at spec time, on the tiers in
`memories/orchestration-model.md`. The orchestrator's read covers
mechanical changes; a single review (style, or full with claims to
falsify) is the default; a dual review covers especially tricky logic
or an architectural decision that would be hard to change later. The
tier and its one-line reason sit on the unit's item or in the log at
dispatch. A dual is a row in `docs/DUAL-REVIEW-LOG.md`, run by
`docs/DUAL-REVIEW-PROTOCOL.md`.

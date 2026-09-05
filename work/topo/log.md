# TOPO log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/topo/plan.md`. A/B band 2700–2799
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opened (2026-09-04)

Opened on Ev's direction (in-chat, 2026-09-04) after a re-read of
`work/issues/` and the code-quality K–X partition against every open
program's `paths`. The measurement that produced this program:

- **55 of the 102 `.rs` files under `crates/topo/src/` are in no open
  program's `paths`** (re-derived 2026-09-05; the opening commit said
  47, off by the two subdirectories a directory-grouped scan dropped) —
  the largest unowned kernel territory in the tree, and the one
  `docs/WORK-TRACKS-2026-09.md` named ("37 `crates/topo/src` files")
  and then cut eleven tracks around without closing.
- **code-quality Track P has never had a lane.** Fourteen open rows,
  fence `euler*.rs`/`validate.rs`/`live.rs`/`seqgen.rs`/`merge_faces.rs`
  and the review-and-fixture readers, listed as claimed by code-quality
  itself; `git branch -r` shows `smell/k-*`, `smell/x-*` and
  `smell/t-*` lanes and no `smell/p-*` ever. The rows were waiting on
  an owner, not on a ruling.
- Two `work/issues/` files name this ground as unowned **in their own
  `## Home` sections** — `validate-tier3-curved-boundary-containment`
  ("`crates/topo/src/validate.rs` is in no open program's `paths`") and
  `no-public-census-or-genus-query` ("beside `euler.rs`/`fixtures.rs`/
  `seqgen.rs`, which no open program's `paths` covers").

Seventeen items at opening: three issues, re-homed by header edit and
`git mv` with ids unchanged, and Track P's fourteen rows, which keep
their ids and their `track: P` letter per `work/code-quality/program.md`
("a row leaves the moment a program claims it, by moving into that
program's directory keeping its id, `track:` letter and body").

**The fence is an enumeration, not a glob**, and the reason is on file:
`crates/topo/src/*` would double-claim five programs' ground, and
`scripts/work.py territory` is blind to a double claim
(`work/meta/territory-cannot-see-a-path-two-programs-both-claim`, the
FIX/SHELL collision on `transform.rs`). The 35 remaining `topo/src`
files are recorded as unowned-and-not-finished rather than swept in.

No unit is cut and no branch exists yet. The first dispatch is `S331`
(the vacuous green through `validate_pcurves`) and claims its ordinal
from the band above, recorded in `docs/MODEL-AB-LOG.md`.

**One seam to announce before it is crossed**: `face-kind-read-has-two-homes`
is on this slate because `readback.rs` is this program's, but its other
door is `query.rs`, which is **SEAT's**. The ruling can be made here;
the edit on SEAT's side is announced on SEAT's board and agreed there
before it lands. Nothing has been taken from SEAT.

## Orchestration opens (2026-09-05)

Picked up on Ev's direction (in-chat, 2026-09-05: "look at work/topo
and see if it's ready to orchestrate; if so, get started"). Single-
orchestrator remote box, no away-channel, no `gh`: GitHub goes through
the MCP tools, lanes are Agent-tool worktrees with private
`CARGO_TARGET_DIR`s seeded from one warm `topo` test build, at most two
heavy lanes at once (four cores). The orchestrator branch is the
session's designated `claude/work-topo-orchestration-mm0itt` rather
than `topo/orchestrator`; unit branches keep the plan's `topo/` prefix.

Decisions taken unilaterally at opening:

- **The opener is `S330`, not `S331`.** The plan cut `S331` first on
  the belief it sits in `validate.rs`; it does not. `validate_pcurves`
  is `crates/topo/src/pcurves.rs:1602`, which `program.md`'s `keep_out`
  and TRIM's `paths` both give to TRIM, and the mechanism the row asks
  about (`mint_pcurves` swallowing `UnsupportedCarrier` and clearing
  the face, `pcurves.rs:1261-1267`) is next to TRIM's own `D36`. So
  `S331` is a question TOPO holds — what at-rest validation may claim —
  whose edit, if any, lands on TRIM's file by announced seam. TRIM's
  board carries the heads-up (`work/trim/log.md`, this date); the
  proposal comes after `S330` lands and before any edit. `S330` is the
  same silence class in the file this program actually owns.
- **Review posture, settled for now** (the plan left it open): one
  style review per unit; a full v6 dual only where a unit moves a
  kernel answer. `S330` moves one (a refusal check 1 does not make
  today) and draws the band's first ordinal at its review dispatch.
  The reader collapse (`D261`+`D264`) and the `live.rs` pair
  (`D260`+`D50`) run outside the experiment with one style review each
  and record no row, S-TCOST's and FILLET's precedent for non-dual
  units. `work/meta/ab-log-v6-stream-is-past-its-stopping-rule-unadjudicated`
  is read: until Ev rules, duals continue and each records "+N
  candidate" per the H4 precedent.
- **`S94` rides with `S330`.** The code-quality plan folds the two
  `VARIANTS` ladders into the first lane that opens `validate.rs`;
  that is this one, and the fold is bounded in the spec (compiler-
  derived count and index, one spelling for both files).
- **Block TOPO-B1 drawn** (three slots, {opus, opus, fable}; the
  record is branch-side on `topo/b1-block` until the block concludes,
  so no unstarted slot reaches a surface reviewers read). Disclosed:
  the byte was drawn a few minutes BEFORE `S330`'s difficulty was
  written down, so the S/M guess on that one row was made knowing the
  slot's arm. The guess follows from the diff's shape (one arm, one
  public method, one test row, a rider that swaps a count for a
  derive) and is recorded as contaminated for the covariate anyway.
- **One seam announced on S-CERT's board** (`work/cert/log.md`): the
  poison predicate the arm needs is `pub(crate)` in
  `crates/geom/src/net.rs`, S-CERT's ground; the unit adds one public
  method on `NurbsSurface` delegating to it and nothing else in `geom`.

Spec: `docs/TOPO-S330-SPEC.md`. `S330` dispatched on
`topo/s330-described-nurbs-arm`.

## Second lane out: the reader collapse (2026-09-05)

`D261` (+`D264` riding) dispatched on `topo/d261-reader-collapse`, the
brief in the item file. Outside the experiment (single style review,
no row). Two seams announced: S-TCOST's board for the census entries
and the ceiling re-derivation, S-BOOL's for the one call-site re-point
in `face_normal.rs` that deleting `fixtures::code_only` forces. Noted
while reading `live.rs` for the third lane: its header no longer names
`D50` by row (main reads "a source-level guard can, and is owed"), so
`D260`'s premise is already false; `D260` closes with `D50`'s guard, as
one sentence, and is not a unit on its own.

## S331 to TRIM; opening PR merged (2026-09-05)

PR 1915 merged (the spec is on main). `S331` moved to `work/trim/` with
Ev's in-chat concurrence — its mechanism and its sibling `D36` are
TRIM's; the direction TOPO would have argued (a refused mint leaves a
typed trace or refuses; never a silent clear) is recorded on the item.
Sixteen items remain on this slate.

## D261 delivered; style review out (2026-09-05)

`topo/d261-reader-collapse` delivered as PR 1919, head `3f41f605`,
full matrix green (run 33943083355: twelve test jobs, five k-lint
rows). Four conversions, all deletions or re-points; one planted
mutant per converted guard in the PR body; `UNCONVERTED_TODAY` 9 → 5,
re-derived on the merged tree. Five deviations disclosed, the one that
matters being that the brief was wrong about the census: a file that
still trips the reader detector cannot have its line deleted, so three
entries moved to `Shared` instead. One residue filed inside the fence
(`probe-message-carve`). Three findings outside the fence relayed to
S-BOOL's board (`boxes.rs`'s stale "two readers" doc; the two raw-text
guards in `face_normal.rs` and `sector_shape.rs`). Single style review
dispatched on the frozen head, claims C1–C6 in the brief.

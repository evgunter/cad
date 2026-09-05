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

## D261 adjudicated (2026-09-05)

Style review (single, Fable) on frozen `3f41f605`: MERGEABLE-AFTER-
FIXES. All six claims held under execution — eighteen planted mutants
across the four guards, block-comment, raw-string and `'"'` shapes
included, every one red; the collapse is a deletion; the census count
is right; the doc gate is real and catches the planted link. What it
found is prose rot in the unit's own header (three sentences still
describing the deleted lexer), one class — the item-body carve is
hand-rolled five times across four crates and the shared home has no
op for it — and the census's `Shared` check being satisfiable by any
mention of the crate. Fix pass dispatched to the same lane (seven
items; the shared op is a one-function widening of the S-TCOST seam,
announced); the class and the census weakness filed on S-TCOST's slate
with the reviewer's stricter row embedded; D287's two stale premises
relayed to S-BOOL. Two reviewer probes handed to the fix pass.

## S330 delivered; dual out at ordinal 2700 (2026-09-05)

`topo/s330-described-nurbs-arm` delivered as PR 1923, head `55931a3e`,
full matrix green (run 33944671747). Phase 1 corrected the spec: the
fixture named there (`ops_cube`) is not tier-3-clean — every face
carries the placeholder — so the unit built on `coplanar_pillow`; and
the measurement is stronger than the row claimed: the finite and the
poisoned described net draw IDENTICAL check-2 lists, so before this
unit nothing in tier 3 could tell corrupt described geometry from
honest — now a committed assertion. The arm, a new variant, named
no-op arms for every `Surface` variant, the interval row, and the S94
rider (both ladders replaced by a test-only `strum` derive) landed.
Five deviations disclosed; one forced outside the fence
(`editor-core/src/assembly.rs:913`, a wildcard-free classify that does
not compile until the variant is placed). One residue filed
(`quadric-datums-unchecked-at-rest`). One inherited red on main
(FILLET-ATTR merged under a cancelled gate; the roster line here is
the port; FILLET's board told). Outside-fence findings held for the
adjudication: `n2r1_probes.rs`'s prose is now false, `BooleanErrorKind`
is a hand-written discriminant companion (S-BOOL). Dual dispatched on
the frozen head: ordinal 2700 claimed, parity byte 119 ⇒ R1 FABLE,
R2 OPUS; briefs stored with sha256 privately.

## S330 adjudicated; fix pass out (2026-09-05)

Both blinded reviews delivered on frozen `55931a3e`. R1: MERGEABLE,
0/3/6, rubric 4/4/4. R2: MERGEABLE-AFTER-FIXES, 1/4/5, rubric 4/4/2.
Every behavioural claim held under execution in both lanes — the
state ladder, the arm removal, the interval row, the derive-backed
coverage, the reproduction of the inherited red on main's own tree.
R2's MAJOR is unilateral and executed: the `Plane` no-op arm's
justification is false (a zero or poisoned frame describes no locus,
check 1 says nothing, and escalations elsewhere refuse it — the very
shape this unit closed for NURBS), and the quadric arms' "no check
reads them" is loose the same way. Adjudicated as a claim-class
finding on a comment plus a residue-scope gap, not shipped behaviour:
recorded as a tally candidate under 3(b)'s doc exclusion for the
blinded adjudication, per the H4 precedent. Convergent style finding
from both arms, taken: the three-state read becomes a type
(`NetState`), the S-CERT seam widened by that enum and announced.
Union fix pass of eight items dispatched to the same lane, both
reviewers' probes handed over (one state-ladder row adopted). Filed:
FILLET's roster debt as `work/fillet/anchor-span-sole-bracket-bound-unrostered`
(the roster line now cites it), the thirteen sibling two-state reads
as a class in `work/issues/` with the `NetState` door as the tool.
`program.md`'s `keep_out` now says how this program treats
`crates/topo/tests/*` (S-TCOST's glob): rows added as ordinary tests,
said in the PR, no second fence.

## D261 merged (2026-09-05)

PR 1919 merged at `da6f159e` (green run 33947129437 on `aed0b564`,
full matrix). `D261` and `D264` closed; `probe-message-carve` open on
this slate. The unit's own account is the PR body. Two operations
notes for the record: the lane re-rolled a seed-dependent `mesh` gate
failure with an EMPTY commit, which the lane rules forbid (the run did
classify code-tier that time; the reliable re-roll is a real commit —
stated here so the next brief says it); and the branch went
conflicting twice against a main that moved under it (S-CERT's log,
then FILLET's own roster line for the inherited red), both resolved by
the orchestrator in a throwaway worktree and pushed by ref, the
second by taking main's line outright. The seeded floor is filed on
S-MESH's slate (`cert10-strict-gap-floor-gates-on-a-varying-seed`).

## S330 merged; block TOPO-B1 slot 0 concluded (2026-09-05)

PR 1923 merged at `d9b7b26d` (green run 33947772289 on `c90a6752`,
full matrix). `S330` and `S94` closed; `quadric-datums-unchecked-at-rest`
open on this slate, widened at review to the poison case and the
`Plane` frame. The A/B row is recorded at merge (ordinal 2700, sample
#138) in `docs/MODEL-AB-LOG.md`'s TOPO section; block TOPO-B1's record
on `topo/b1-block` marks slot 0 concluded, slots 1–2 banked. Sixteen
items on the slate, two closed today (`D261`, `D264`) plus these two.
Next: the `live.rs` pair — `D50`'s source-level guard on the shared
lexer now that `D261` has landed it, `D260` as one sentence in the
same PR, single style review, no row.

## Third lane out: the live.rs pair (2026-09-05)

`D50` (+`D260` riding) dispatched on `topo/d50-live-guard`, the brief
in the item file. Outside the experiment (single style review, no row).
The survey corrected the row's premise: the compiler already makes
`Live` unforgeable from outside `live.rs` (private field, private
`new`); what nothing guards is the header's real claim — that every
door INSIDE the file looks up before it hands a token out — and that
is what the row builds, on the shared lexer and item carve `D261`
landed.

## The two-homes ruling goes to Ev (2026-09-05)

`face-kind-read-has-two-homes`: recommendation (a) — the predicate
seat reads through the typed readback door and flattens; the
readback header's "one reading, not two" is the ratified rule this
elaborates. Three viable answers on paper, one dominant argument, so
it goes out as an `[ev]` PR rather than self-merging, per the
"when unsure, treat it as a fork" rule; `needs_ev: true` on the item,
the seam announced on SEAT's board as a heads-up.

## Two-homes ruled (2026-09-05)

PR 1948: Ev asked whether SEAT's PR 1902 had just ruled the same
question the other way; it had not (a decide site under two funnel
names versus a tag read, and #1902's "one kernel door, callers keep
their names" is this item's (a)). Ratified (a). The item is a unit,
dispatched on `topo/two-homes-face-kind` as a one-door seam on SEAT's
`query.rs`, announced; single style review, no row.

## D50 delivered; style review out (2026-09-05)

`topo/d50-live-guard` delivered as PR 1949, head `e69760e1`, full
matrix green (run 33950634858). The guard is one row in `live.rs` over
the shared `code_only` view: the declaration and `new` carry no
visibility, every door reaches a closed lookup vocabulary before its
first construction, and the doors and sites are pinned to the header's
list plus the crate-wide "no other file builds one". Three mutants red
by name. `D260` is one sentence. Two deviations: `source_walk.rs`'s
item scan widened from `pub fn` to every named `fn` (the doors are
`pub(crate)`), with its own row; and one `Shared` line in the reader
census, forced by the census's own detector (S-TCOST's file — noted on
its board at adjudication). One residue filed: the guard proves
ordering, not that the key looked up is the key wrapped. Single style
review dispatched on the frozen head.

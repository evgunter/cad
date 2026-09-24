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

## D50 adjudicated (2026-09-06)

Style review (single, Fable; the first attempt died on the account's
usage limit and was relaunched fresh): MERGEABLE-AFTER-FIXES. Every
planted mutant redded naming its door — the guard is real. Findings:
D260's sentence claims the header names the doors (it does not); the
PR body claims a cross-file check with `euler.rs`'s door list that the
row does not make; one of the new scan rows cannot fail on the mutant
it was written for; the residue's reason for declining the cheap
argument-spelling partial was falsified by execution (green on the
tree, red on the residue's own example) — so the partial is taken and
the residue narrowed to the two gaps that remain (arena-blind `.get(`,
nested items unscanned); a stale measured door count; two vocabulary
spellings no door uses. Ten-item fix pass to the same lane. The item
scan's missing shared home and two smaller siblings appended to the
S-TCOST issue filed from D261's review.

## Two-homes unit delivered; orchestrator meta-review (2026-09-06)

PR 1959, head `c1539db5`, full matrix green: two function bodies in
SEAT's `query.rs`, the mutant receipt showing the delegation is real
(the door made to refuse → one `topo` row and one `sweep` row red).
Deliberate deviation from the per-unit style-review posture, recorded:
for a six-line delegation with an executed receipt, the orchestrator's
own read is the review — a lane would re-derive the two disclosed
questions and nothing else. Both questions got files: the missing
positive row on SEAT's slate (`face-surface-kind-has-no-positive-row-in-topo`),
the edge side's absent readback door on this one
(`edge-carrier-kind-has-no-readback-door`). The lane's one-word
question in `readback.rs` ("twin" → "flattening") taken
orchestrator-direct on the branch; merges on green.

## Two-homes merged (2026-09-06)

PR 1959 merged at `332bc981` (green run 34014916153 on `bf42c02a`,
full matrix). `face-kind-read-has-two-homes` closed: the predicate
seat reads the face kind through the typed readback door and flattens;
`readback.rs`'s doc names the query seat as its flattening. SEAT's
`query.rs` edited by announced seam, two function bodies. Fifteen
items remain on the slate; six closed since opening (`S330`, `S94`,
`D261`, `D264`, this one, and `D50`/`D260` in fix pass).

## D50 merged (2026-09-06)

PR 1949 merged at `368089da` (green run 34015295339 on `57244699`,
full matrix). `D50` and `D260` closed; the residue
`live-guard-proves-ordering-not-identity` narrowed at the fix pass to
the two gaps that survive the argument check. The ten-item fix pass
landed whole, the cheap argument-spelling partial included; the
reviewer's `-> Self`-in-`impl Body` mutant is now correctly green
because the `Self` check is scoped to the `impl Live` block. One
census line noted on S-TCOST's board. Two notes for the next briefs:
a reviewer's probe script that ends in `git checkout --` against a
hard-coded worktree path discarded a lane's uncommitted fix edits
when re-run there (lanes commit before running foreign probes;
reviewer probes must not reset files); and the lane found a stale
count in prose ("two `macro_rules!`", it was three) — the prose now
carries the claim without the number. Slate: thirteen open items.
The liveness-and-generator sub-lane is empty except `D20` (queues on
the lane budget); the reader sub-lane is `D107` only; the Euler
sub-lane holds `S93`, `D265`, `D262`, `D263`, `S69`, the H item, the
census door, and two residues.

## Wave 3 out: D265 and S69 (2026-09-06)

`D265` dispatched on `topo/d265-door-corruption-class`, spec
`docs/TOPO-D265-SPEC.md`: the merge door's arena-fault class asks the
enum a question ("torn?") that is not the door's ("contradicts a fact
I established?"); two homes, the door enumerates its own. Moves a
kernel answer (an inventory skip becomes an escape for the (C)
variants) → block TOPO-B1 slot 1, dual at review. `S69` dispatched on
`topo/s69-kfmrh-fusion-form`, brief in the item: the fusion branch
generated, the ledger counting shells, the postcondition handed the
plan's constant. Outside the experiment (generator, ledger, debug
postcondition). Both briefs pre-log difficulty M; the D265 arm was
known when its guess was written (block drawn 2026-09-05), disclosed
in the spec.

## The census door goes to Ev (2026-09-06)

`no-public-census-or-genus-query`: recommendation (A) — a typed
whole-body read in `readback` (`euler_counts` + `genus() -> Result`),
shells as the shell term, parity refusing typed. Three viable shapes
on paper; the API shape is a design question by the item's own words,
so it goes out as an `[ev]` PR. Re-measured at 26 sites in 22 files,
and the ledger's solids-for-shells slip (`S69`'s lane) is the row's
cost already realised.

## Census door ruled (2026-09-06)

PR 2010: Ev ratified (A). `no-public-census-or-genus-query` is a unit
in `spec`, branch `topo/census-door`, queued behind the two running
lanes; it draws block TOPO-B1 slot 2 (a new public answer). Seams
announced: S-TCOST for the fourteen test files; the demos' owner at
dispatch once `paths` say who that is.

## D265 delivered; dual out at ordinal 2701 (2026-09-06)

`topo/d265-door-corruption-class` delivered as PR 2013, head
`4dc616d0`, full matrix green (run 34017473814). Phase 1 found
`merge_group` raising from eight sites, not the spec's two; fifteen
variants can arrive — eight torn, seven contradicting a fact the door
established, and NONE reachable on a valid body, so the door was
recording seven kinds of kernel bug as inventory skips. Executed:
`kef → FaceHasRings` through the public door recorded as a skip on the
merge base, escapes at the head. `OpPlacement` is the door's own
exhaustive classification; `Torn` delegates its verdict to the enum.
`euler.rs` untouched; `D262`'s helpers untouched. D263 gained a
sharper witness (the whole placeholder cube grouped as one curved
group), recorded on its file. Dual dispatched on the frozen head:
ordinal 2701, parity byte 229 ⇒ R1 FABLE, R2 OPUS; briefs stored with
sha256 privately.

## D265 adjudicated (2026-09-06)

Both blinded reviews on frozen `4dc616d0`. R1: MERGEABLE-AFTER-FIXES,
0/5/5, rubric 4/3/3 — re-derived all 27 variants across nine sites
and executed six of the seven contradicted cases under both regimes.
R2: NOT-MERGEABLE-AS-IS, 1/6/4, rubric 3/2/2 — **one unilateral
executed MAJOR**: `kef → SameFace` is reachable on a valid body
because the door's OWN ring drain re-homes the dying loop between the
establishing scan and the `kef` call, so the head turns a legal
recorded skip into a refused call (built by legal Euler operators,
run through the public door, `Ok` on the base and `Err` at the head).
Code class, executed, R1 never mentioned it: a tally candidate, and
the strongest this program has produced. The lesson generalises and
the fix pass applies it to every arm: a fact the door established is
a fact only until the door's next mutation, so each surviving
contradiction arm gets a re-check immediately before the operator
call, and the classification rests on execution rather than a
hand-derived reachability table. Convergent findings taken: the
torn/not-raised split collapsed (void, unpinnable), the fact string
becomes an enum, the ninth raising site, the regime asserted on a
regime-independent fixture, the stale "no test can do this"
sentences, the copied sample array shared from `euler.rs`, an RAII
guard on the tear flag, the six statements of one rule reduced to a
home. Twelve-item fix pass to the same lane; both reviewers' probes
handed over. CURVED's spec cite relayed to its board.

## S69 delivered; style review out (2026-09-06)

`topo/s69-kfmrh-fusion-form` delivered as PR 2014, head `a137b800`,
full matrix green (run 34018946634). The three defects closed: the
fusion form enters the catalog as `OpChoice::KfmrhFuse`; the ledger's
`s` counts shells; `kfmrh`'s postcondition takes the plan phase's
constant. The brief's premise was wrong and the lane said so: `mfkrh`
mints no shell ENTITY (it splits a surface), so multi-shell solids come
only from `mvfs` and `movefac`, and `movefac` had to enter the catalog
too for the fusion row to be drawn at all. A real finding along the
way: the fusion form's Euler vector is `Δh = 0, Δs = −1`, not the
same-shell form's `Δh = +1` the operator's doc asserted for both.
Three deviations declared, three residues filed inside the fence, the
fuzz row put on the effort dial. Single style review dispatched on
the frozen head.

## S69 adjudicated (2026-09-06)

Style review (single, Fable) on frozen `a137b800`: MERGEABLE, 0/1/4.
Every claim reproduced — the fusion vector derived by hand and by
counts, both ledger mutants, both halves of the postcondition mutant,
the rows drawn at the reviewer's own seeds, the re-pinned hash
recomputed, the coverage row red without the fusion row, sixteen
postcondition sites re-swept. Findings: the proptest residue names the
wrong regression path and the tracked corpus is orphaned
(pre-existing); an over-broad "only doors that mint a shell" premise
in three places; stale "two kill sites" prose in six; the two new
enumerators run every step where the probe slot exists for exactly
that shape (measured +40%); a third copy of the glue relation in test
support. Eight-item fix pass to the same lane; `fuzz::replay()`'s
wrong seed under `pinned()` relayed to S-TCOST.

## D265 merged; block TOPO-B1 slot 1 concluded (2026-09-07)

PR 2013 merged at `56af92aa` (green run 34167918624 on `dc2e996e`,
full matrix). `D265` closed; no residue rides. The A/B row is recorded
at merge (ordinal 2701, sample #155) in `docs/MODEL-AB-LOG.md`'s TOPO
section, with R2's unilateral executed MAJOR as the program's second
tally candidate — and the first of code class. Block TOPO-B1's record
on `topo/b1-block` marks slot 1 concluded; slot 2 (FABLE) is banked for
the census door. `S69`'s fix pass, resumed after the second usage-limit
cut, is the only lane running. Twelve items open on the slate.

## S69 merged; the census door dispatched (2026-09-08)

PR 2014 merged at `242e8375` (green run 34170947394, full matrix).
`S69` closed as a unit; five residues filed on this slate across the
unit and its fix pass (`fused-two-shell-body-doc-predates-movefac`,
`seqgen-proptest-row-logs-no-seed`,
`movefac-row-skips-three-component-shells`,
`orphaned-proptest-corpus-for-seqgen`,
`shell-glue-relation-has-three-implementations`). The fix pass
measured the reviewer's +40% down to ~7.4% on a deterministic walk
and recovered ~40% of that with probes; the suggested shell-count gate
was right for one row and fatal to the other, and the lane said so.
Two discipline gates fired on the merge with main and both were the
diff's own; the postcondition restructured to `movefac`'s
`cfg(debug_assertions)` idiom with the mutant re-run identically red.

`no-public-census-or-genus-query` dispatched on `topo/census-door`
(block TOPO-B1 slot 2; dual at review): the ratified (A) shape, the
door and rows in `readback.rs`, `topo`'s own sites converted,
`topo/tests/*` and `sweep/tests/*` by the announced S-TCOST seam, the
demo sources (in no program's `paths`) in the same PR under the
demo-purpose rule. Difficulty pre-logged M, task class STRUCTURAL —
again written knowing the slot's arm; disclosed.

## Census door delivered; dual out at ordinal 2702 (2026-09-08)

`topo/census-door` delivered as PR 2131, head `11257506c`, full
matrix green (run 34174452199). The door is the ratified (A) shape:
`readback::euler_counts` returning five `i64` counts with `s` =
shells, `EulerCounts::genus` refusing typed on odd parity before the
halving, `EulerParityError` its own type (every `ReadbackError` arm
is about one entity from a door that takes its key; this one is about
the whole store from a value already read). Rows: the cube, the holed
box, a two-shell body re-homed into one solid by raw write (pins `s`
= shells), the red-first torn store. The class receipt widened the
ruling's sweep from 26 sites to 42 converted plus one listed with its
owner (`review_m1_pr4`'s per-component `ShellComponent::genus`, shape
(C)'s consumer when one arrives); the ring-less spellings were the
sites the ruling's pattern could not see. Deviations disclosed: test
helpers in `sweep/tests` and `tour/tests` keep their names as one-line
delegates rather than rewriting seventy call sites; two test rows
changed type to the door's `i64`; three ring-less sites and
`assembly.rs`'s structural census converted. The demos call the door
directly and print the same narration. S79's disposition is the
orchestrator's: the door retires its #758 third only, #757 and #759
stay open elsewhere. Dual dispatched on the frozen head: ordinal 2702,
parity byte 70 ⇒ R1 OPUS, R2 FABLE; briefs stored with sha256
privately. The claim entry on main wrote the difficulty as S/M where
the dispatch entry above pre-logged M; the pre-logged value is the
record and the row will carry M with this line as its disclosure.

## Announced seam from SHELL (2026-09-08): an ownership re-partition op beside `movefac`

SHELL-5 (`shell/5-hollow-operand`, `docs/SHELL-5-SPEC.md`) — shell of
a hollow operand thickens every boundary, one thin solid per operand
shell — needs to move an operand void and its dilated twin out of the
operand's solid into a new solid. That is a re-partition of ownership
in `movefac`'s shape one level up (mints a solid, rewrites the moved
shells' `solid` back-pointers, asserts `ArenaDelta { solids: 1, .. }`),
so it lands beside `movefac` in `crates/topo/src/movefac.rs` with one
provenance variant in `crates/topo/src/provenance.rs` — both TOPO's
files. Additive; no existing op changes. The unit also measures whether
tier 3 sees a wrong shell-to-solid grouping (it does not check solid
membership today) and reports the answer for TOPO rather than adding a
check. Signed (SHELL orchestrator).

## Announced seam from SHELL (2026-09-08): two doc lines in `pcurves.rs` with SHELL-9

SHELL-9 (PR #2223) adds `shell`'s closing pcurve mint — the producer's
half of the `insert_voids` `Transfers` contract the posture table
states. The unit edits two DOC lines in `crates/topo/src/pcurves.rs`
and nothing else there: the `insert_voids` posture row ("both
producers' final mint passes" → every producer's, naming the
boolean's, the revolve's and `shell`'s) and the Maintains bucket's
producer list, which omitted the revolve/tube and `shell` (one list,
two homes — both now agree). No action asked. The class behind it —
the closing mint is a prose convention with thirteen copies, and it
launders a stale operand row — is filed for TOPO as
`producer-closing-mint-is-a-convention-with-thirteen-copies`, with
`revert-does-not-mirror-plane-chart-images` beside it from the same
diagnosis. Signed (SHELL orchestrator).

## Announced seam from SHELL (2026-09-08): `mint_pcurves_of` and two registry rows with SHELL-10

SHELL-10 (PR #2229) narrows the two simultaneous offset doors to
their scope. In TOPO's files it adds `pcurves::mint_pcurves_of(body,
faces, tol)` — the whole-body pass restricted to the named faces,
sharing a private `mint_faces` with `mint_pcurves` (which keeps its
opening `clear()`, the only thing that drops rows on dead half-edge
keys; the subset pass does not, stated in its doc and pinned by both
reviewers' rows) — a `staleness_posture::DECLARED` row for it with
the guard's needle reading either spelling, and a
`review_m1_pr5_internal::ALLOWED` tier-1 row; `lib.rs` exports the
new door. No action asked. Two findings for TOPO from the same unit:
`attach-postconditions-validate-the-whole-body-and-panic` (the
setters' whole-body postcondition, a panic under the release
profile — both reviewers' by execution) and, from the unit's own §3
STOP, that tier 1 has no per-shell entry (five of thirteen passes are
arena-global by construction), kept on SHELL's
`doors-still-read-the-whole-body-for-tier1` as a closure-check
limitation rather than filed here. Signed (SHELL orchestrator).

## The dual died at dispatch; four days idle; re-run on a merged-forward head (2026-09-12)

Both blinded reviewers dispatched on the census door's frozen head
`11257506c` (ordinal 2702) were killed by the account's usage limit
within minutes of launch, before either had read past its brief — no
report, no probe, no scratch file beyond the stored brief. The session
then sat idle from 2026-09-08 01:15 UTC until Ev reset the limit on
2026-09-12. Main moved by some 2,400 commits in between (SHELL's units
5–10, the code-quality register leaving the tracker, PERF opening, the
discipline documents rewritten) and `topo/census-door` now conflicts
with it in four converted files (`verbs_shell.rs`,
`verbs_shell_r2_probes.rs`, `demos/tour/src/main.rs`, `teapot.rs`),
with 285 commits on the sweep's ground since its merge base.

Decision: the implementer lane merges main forward FIRST — resolves
the conflicts, re-derives the class receipt at the new merge base,
files its two outside-the-fence findings per the rewritten
`implementer-discipline.md` §6 (a lane files on the owner's slate in
its own PR now; the census brief predated that rule), and rewrites its
citations by name per the new §7 — then the dual runs on that head.
Reviewing the stale head and merging forward afterwards would have
put the conflict resolution and any new sites outside both reviews.
Ordinal 2702 stays; the claim entry on main names `11257506c` as the
frozen head and the row will carry the superseding SHA with this entry
as its disclosure. Parity byte 70 stands (R1 OPUS, R2 FABLE); the
briefs are regenerated from the merged PR body and re-hashed before
dispatch. The dead reviewers' worktrees were removed; their scratch
dirs held nothing but the brief.

Rules that changed on main and bind this program's briefs from here:
findings outside the fence are filed by the lane on the owner's slate
(`work/README.md`, discipline §6); citations by name (§7); the style
lane checks whether a structural fix mints a fresh instance of the
defect it closes (`reviewer-style-lane.md`); PRs touching
`docs/prompts/` or an already-ratified decision wait for Ev
(`CLAUDE.md`). `S79` moved to `work/pipe/`; the nine-copies row to
`work/suite/`. Ten rows were placed on this slate by other programs
while the session was idle (SHELL's diagnoses on `revert`,
`split_edge`, `move_shells_to_new_solid`, the attach postconditions,
tier 3's shell roles and ring nesting; `D360`'s sweep rule;
`geom-source-absence-conflates-four-origins`; the stale D107 citation)
— read before the next unit is cut, not acted on here.

## Census door merged forward; dual re-dispatched on `b5ead3c9f` (2026-09-12)

The lane merged main (`0312083aa`) by merge commit, took main's side
in the four conflicts with the door conversions on top (main had
rewritten the teapot scene's `describe` and moved `band()` into the
sweep tests' `common::approx`), re-derived the receipt at the new
base — 49 converted, 1 listed with its owner, three sites new on main
(`shell5_r1_probes`, the tour's `skinned` scene, editor-core's
`lib_g17_shell_node`) — and filed per §6: the nine-copies row on
`work/suite/` closed with every copy named, a new
`work/tcost/census-tuple-rows-assert-chi-not-genus` for the rows whose
`counts()` tuple carries no `s`. Citations rewritten by name. Full
matrix green (run 34716391723). Nothing on main moved the door's
premises. Correction to the entry above: the dead reviewers' scratch
dirs were NOT empty — both had run differential tour builds and probes
before the limit killed them; that material is archived privately and
the dirs were cleared to the brief before re-dispatch, so the new
reviews start cold. Briefs regenerated for the new head and re-hashed;
ordinal 2702, byte 70 (R1 OPUS, R2 FABLE), concurrent on frozen
`b5ead3c9f`.

## Census door adjudicated (2026-09-12)

Both blinded reviews on frozen `b5ead3c9f`, both MERGEABLE-AFTER-FIXES,
both executed the same two mutants and converged on both MAJORs. The
row the PR names as the pin for `s` = shells cannot fail for that
reason: with the door's `s` read off the solid arena every one of the
door's own rows and both doctests stay green, because the row keeps
the emptied solid alive and solids = shells = 2 throughout; only S69's
own instruments (`seqgen`'s property row, `review_m3_pr1`'s connected
sum) go red. Both reviewers built the public-operator route the brief
asked for — pillow + planted ring → `mfkrh_plug` → `movefac` — and
both probes red the mutant. The class receipt is short: two
`m9_3_zip` rows (one deriving `r` as loops − faces, a second spelling
of the ring count no `rings.len()` pattern can see) converged; R1 adds
`blend4_concave_fillet`'s vent-mouth row, the declared twin of a row
the PR did convert, and `cube_by_hand`'s identity over constants.
Converged MINORs: `r` not pinned as a sum (no door row has two rings on
one face); the ring-less conversions changed their rows' claims and
the PR body says otherwise; eleven same-named delegates with identical
docs are a fresh instance of the closed duplication (the style lane's
new trap check, raised by both); the `s` paragraph narrates S69;
`contains("torn")` pins prose; `assembly.rs`'s doc says solids over
the door's `s`. Unilateral, taken: the raw-write row's doc re-asserts
the "reachable only by raw write" sentence this slate already holds
false (R1); the tcost residue row names two of five instances (R2);
`#759`'s item is closed so the PR body's S79 sentence is false and
S79 waits on `#757` alone (R1); three review-module headers forbid
the simplification the ruling ordered, undisclosed (R1 — disclose,
headers untouched); `EulerCounts`, the validator's `ComponentCounts`
and `ShellComponent` are three carriers of one characteristic (R1 —
filed as a residue here). Both established that an odd census is
unreachable from outside the crate (every arena writer `pub(crate)`)
and that the demos print byte-identical narration across the change
(78 topology lines, not the brief's 72). No unilateral executed
MAJOR: no tally candidate. Nine-item fix pass to the same lane.

## Census door merged; block TOPO-B1 concluded (2026-09-12)

PR 2131 merged at `1072c130a` (green run 34721329369 on `39098c046`,
full matrix, verified job by job). `no-public-census-or-genus-query`
closed. The fix pass took every item: the `s` pin now runs through
`mfkrh_plug` → `movefac` and is the only row red under the solids
mutant; a `ring_move` row pins `r` as a sum (the lane measured that
`ops_genus2` cannot — four rings on four faces — and said so); the
receipt re-measured at 53 converted / 1 listed with the
identifier-level patterns stated; the eleven delegates became one
helper pair per cargo root, each re-raising the typed refusal; the
suite row's closing restated truthfully; the tcost row widened to its
class; `euler-characteristic-has-three-carriers` filed here. The A/B
row is recorded at merge (ordinal 2702, sample #173) in
`docs/MODEL-AB-LOG.md`'s TOPO section — no tally candidate — with the
block TOPO-B1 CONCLUDED record published beneath it. `S79` on PIPE's
slate edited to wait on `#757` alone, announced on `work/pipe/log.md`.
The next kernel-answer unit draws block TOPO-B2. Twenty-seven items
open; nothing dispatched.

## Block TOPO-B2 cut; pre-draw fields logged BEFORE the byte (2026-09-12)

Three of SHELL's six placed rows are kernel answers on this program's
own files with no other program's decision ahead of them; they form
block TOPO-B2 in the order the plan fixes. Pre-draw fields, written
and committed before any byte is drawn this time (B1's three slots
were all disclosed as contaminated):

- slot 0 `tier3-accepts-a-ring-outside-its-outer-loop` — difficulty
  **M**, task class **STRUCTURAL-GEOMETRIC** (a decide added to a
  tier-3 check; one helper hoisted from `shell.rs`).
- slot 1 `split-edge-children-lack-pcurve-rows-on-curved-charts` —
  difficulty **S/M**, task class **STRUCTURAL-NUMERIC** (a mint inside
  an existing operator from a cached pcurve, or a stated caveat).
- slot 2 `revert-does-not-mirror-plane-chart-images` — difficulty
  **M**, task class **NUMERIC** (an image transform under a frame
  reversal, certified after).

The other three placed rows are not units: the chart-spans-solids
question is an `[ev]` ruling the orchestrator writes, the shell-roles
check waits on S-BOOL's hollow-operand row, and the attach
postcondition's cost half is already PERF-4's (its panic half is D1's
question and rides with the chart ruling). Block TOPO-B2 draws after
this entry is committed; the record goes branch-side on
`topo/b2-block`.

## Block TOPO-B2 drawn; slots 0 and 1 dispatched (2026-09-13)

The byte was drawn after the pre-draw entry above was committed
(`41edc9d2a`): 215 ⇒ 215 mod 3 = 2 ⇒ fable at slot 2. Record
branch-side on `topo/b2-block`. Slot 0
(`tier3-accepts-a-ring-outside-its-outer-loop`, branch
`topo/tier3-ring-nesting`) and slot 1
(`split-edge-children-lack-pcurve-rows-on-curved-charts`, branch
`topo/split-edge-pcurve-rows`) dispatched together on the box's two
lanes with brief sections in their item files; slot 2 (`revert`)
dispatches when one frees. Seams announced: SHELL (one comment in
`shell.rs`; the glue's second precondition is SHELL's call), TRIM
(`pcurves.rs` read, at most one helper by seam), S-BOOL (the
shell-roles check waits on the hollow-operand row). The briefs mark
their instruments as hypotheses — `ray_parity.rs` for the nesting
decide, the parameter split of a cached pcurve for the mint — for
phase 1 to verify; the usage-limit cut of 2026-09-12 23:00 to
2026-09-13 21:45 sat between the draw and the dispatch, with nothing
running.

## Both slots delivered; the split-edge dual out at ordinal 2703 (2026-09-13)

Slot 1 `topo/split-edge-pcurve-rows` delivered as PR 2531, head
`880654600`, full matrix green (run 34787766875). Phase 1 chose
closing (a): a `Pcurve` is a function of the carrier parameter with
no interval of its own, so each child's chart image is the parent's
restricted to its sub-interval; `PcurveCache::certify` needs only
`T: Decide`, so the op carries its rows in the plan phase with no
caller's bound moved. Phase 1 also found the parent halves' rows going
stale in CONTENT on the merge base with `validate_pcurves` skipping
any face it finds incomplete — filed on TRIM's slate — and that the
half-edge-minting Euler operators leave a minted curved face
incomplete (filed here, with the `Fitted`/`General` residue). SHELL-7's
`p4` probe row re-baselined from pinning the defect to pinning the
fix. Dual dispatched on the frozen head: ordinal 2703, parity byte 20
⇒ R1 OPUS, R2 FABLE; briefs stored with sha256 privately.

Slot 0 `topo/tier3-ring-nesting` delivered as PR 2529, head
`a8703bdc9`, full matrix green (run 34787728506). Phase 1 corrected
the brief's instrument — `ray_parity.rs` directly would have minted a
second home for the 3-D frame; the entry is
`splitting::containment::point_in_loop` — and showed `shell.rs`'s
`encloses` unsound at rest (a 10×0.2 plate with an end hole reads
un-nested: a false refusal), and the `shell.rs` comment already
repaired on main before the lane. Check 9 gains its nesting half on
planar faces whose outer loop is a polygon of line carriers; the
arc-bounded and curved cases are the enumerated residue, filed. Its
dual (byte 0 ⇒ R1 OPUS, R2 FABLE; briefs stored) dispatches when the
split-edge pair frees the box — two reviewer pairs at once would
starve four cores. The claim entry for its ordinal goes to main at
that dispatch.

## Split-edge adjudicated (2026-09-13)

Both blinded reviews on frozen `880654600`, both
MERGEABLE-AFTER-FIXES. R1: 1/4/2, rubric 3/3/3 — MAJOR: the carry's
mint-identity claim is false on spline charts (loft prisms' `IsoLine`
walls and `IsoArc` rims: the carry is exact and tier 3 reads `Ok`, but
`mint_pcurves`, the named recovery step, still refuses — a
pre-existing refusal now reached silently), unfiled. R2: 0/3/3, rubric
4/4/3 — the same fact executed on the bulged loft, filed as MINOR
("cylinder-only"). CONVERGED on the fact, DIVERGENT on severity: no
unilateral executed MAJOR, no tally candidate. Converged also: the
class receipt's re-parenting rows are wrong (`kfmrh` onto a plane
keeps four cylinder rows under a planar face with tier 3 silent — R2
executed; `ring_move` the same mechanism — R1 read), the
mint-identity rows compare intervals not bytes, `split_cache`'s window
hull is a third spelling of `validate_pcurves`' (the trap check, both),
the one-use alias, `Posture::Carries` a comment wearing an enum, and
four statements of one bound argument. Unilateral, taken: `IsoArc::
chart_box`'s premise sentence in geom-brep invalidated (R1); a
`Corrupt` swallowed into `Ok(None)` (R1); the residue row's caller
list short by `mesh` (R1); `split_cache` reads the carrier's interval,
not the row's (R2). Both reproduced the merge-base readings and the
plan-phase contract under two mutants. Seven-item fix pass to a fresh
lane on the inherited branch (the implementer's worktree was gone);
both reviewers' probes handed over. The ring-nesting dual dispatches
now beside it: claim for ordinal 2704 to main.

## Ring-nesting dual out at ordinal 2704 (2026-09-13)

Claim merged to main; both reviewers dispatched concurrently on frozen
`a8703bdc9` beside the split-edge fix pass: byte 0 ⇒ R1 OPUS, R2
FABLE; briefs stored with sha256 privately at the unit's delivery,
re-verified identical modulo lane paths at dispatch.

## Ring nesting adjudicated (2026-09-14)

Both blinded reviews on frozen `a8703bdc9`, both
MERGEABLE-AFTER-FIXES, neither with a MAJOR. R1: 0/9/4, rubric 3/4/2;
R2: 0/4/7, rubric 4/3/3. Both executed the arm through every honest
fixture they built and every inverted glue on a polygonal outer,
`Body::revert` on both orientations, an escalation plant (never read
as nested) and the classify obligations (each deletion reds by name).
CONVERGED: the seam sentence written into `shell.rs` claims check 9
refuses the inverted pick at the verb's closing validate, and both
found the SHELL-5 mutant dies earlier at `Corrupt` in the naming
record's walk — the arm is never reached through `shell_open`; the
polygon gate is a second, narrower spelling of `boolean::contain`'s
`loop_shape` (which admits arc-bearing loops over three vertices —
R1 executed the divergence; R2 measured a round-holed square inverted
and silent); the control row stays green with the gate forced shut
(R2 executed); `Inside` carries two meanings against the code; the
gate is computed for ring-free faces; the residue "every vertex on
the boundary while contact reads disjoint" is unreachable; the
red-first row's comment contradicts its assertion; the H item
understates tier 3's coverage. Unilateral, taken: the PR body's
`encloses` counterexample does not compute — R1 re-did the arithmetic
(outer mean 5.0, ring 2.0, nested) and built a cross fixture that
genuinely false-refuses; an empty `kemr` ring reads `Inside` (R1
planted one outside the square); `KERNEL-VERBS.md` and a second
`shell.rs` comment still state check 9 as contact-only (R1); the K
roster has no mechanical guard (both noted; filed on INSTR). No
tally candidate. Ten-item fix pass to a fresh lane on the inherited
branch: the gate reaches `loop_shape` by announced seam to S-BOOL
(one visibility change), which widens the decide to the loops the
walk is measured correct on and retires the copy; a valid body the
widened arm refuses falls back to the line-only gate, stated.

## Split-edge merged; block TOPO-B2 slot 1 concluded (2026-09-14)

PR 2531 merged at `dffd2bf34` (green run 34793938681 on `8ae4e0ec8`,
full matrix, verified job by job). The unit closed. The fix pass took
every item; its one disagreement was with the brief's "pre-existing,
no regression" framing — the unsplit loft controls show the pass is
fine with spline charts and the honest statement is that the iso
arms cannot re-derive a SUB-edge, which is how the TRIM row now reads.
Row recorded at merge (ordinal 2703, sample #186), no tally candidate;
slot 1 marked concluded on `topo/b2-block`. Three rows filed by the
fix pass (one on TRIM, two here); `split_edge` now declares
`Transfers` in the posture table. The ring-nesting fix pass is the one
lane running; slot 2 (`revert`, FABLE) dispatches when it frees.

## Slot 2 dispatched: `revert` mirrors the plane chart's images (2026-09-14)

Block TOPO-B2's last slot (FABLE) dispatched on
`topo/revert-mirrors-chart-images` with a brief section in the item:
the transform hypothesis `(u, v) ↦ (u, −v)` on every `Chart` image
and pcurve row of a reverted plane's faces, per image kind, with the
bitwise involution and SHELL's drum as the red-first rows; seam to
TRIM announced for the row transform; `chart.rs`/`chart_iso.rs` are
unowned ground where the PR draws the fence. Beside it, the
ring-nesting fix pass is the other lane.

## Revert delivered; dual out at ordinal 2705 (2026-09-14)

Slot 2 `topo/revert-mirrors-chart-images` delivered as PR 2542, head
`f8ced386b`, full matrix green (run 34798088825; a new advisory
`render drift (gui)` job sits neutral in the matrix). Phase 1
confirmed the frame derivation and found the transform exact in all
five image kinds — a sign flip on the `v` components, bitwise
involution — so nothing is re-minted; the geom-brep chart certifier
was right to refuse and is not edited; plane faces mint no pcurve
rows, so the row arm is reachable only through `attach_pcurve`.
Certificates travel verbatim through three new geom-brep doors. Four
red-first rows red on the merge base (the drum's reverted cavity now
tier 3 `[NegativeVolume]` only, `insert_voids` accepts it); three
SHELL probe rows flipped from pinning the refusal to pinning the fix.
Four deviations disclosed, two of them the ones to weigh (`revert`
cannot enter the posture guard's table — it walks `&mut self` doors
only; dead-key rows travel as found). Two rows filed (the periodic
chart's loop wrap under `revert`, here; a bug-proof drum row, SHELL).
Dual dispatched on the frozen head beside the ring-nesting fix pass:
ordinal 2705, byte 83 ⇒ R1 FABLE, R2 OPUS; briefs stored with sha256
privately.

## Ring nesting merged; block TOPO-B2 slots 0 and 1 concluded (2026-09-14)

PR 2529 merged at `dc868f1f9` (green run 34796525445 on `7e56ec2f9`,
full matrix, verified job by job). The unit closed. The fix pass's one
finding of its own is the important one: gating the arm on
`loop_shape`'s arc-parity class REFUSED a valid body (a bored D-rod
whose cap's major arc dips past its chord, the bore in the lune the
polygon excludes), so the gate fell back to the no-arc class with the
reason stated, and `LoopShape::Parity` split into `Polygon`/`ArcParity`
in S-BOOL's `contain.rs` (bit-identical for `contfp`; announced on
S-BOOL's and CURVED's boards). The round-hole-in-square inversion
therefore stays silent (the disc class, `disc_side`'s), which the
residue row now says exactly. Fourteen reviewer rows adopted. Row
recorded at merge (ordinal 2704, sample #187), no tally candidate; the
block's CONCLUDED record published beneath it with slot 2's row to
follow at its merge. The revert dual is the only work running.

## Revert adjudicated (2026-09-14)

Both blinded reviews on frozen `f8ced386b`, both
MERGEABLE-AFTER-FIXES. R1: 0/4/3, rubric 3/3/3. R2: 2/8/3, rubric
3/4/3. Both executed the transform in all five image kinds (exact,
bitwise involution), the certificate-verbatim pin under a certificate
mutant (live, not decorative), the four red-first rows and the four
flipped rows on a merge-base emulation, and the class receipt's frame
readers. CONVERGED on the facts, DIVERGENT on severity: R2's two
MAJORs — `mirror_v` re-implements `NurbsCurve2::map_points` and so
mints two `RevertError` variants nothing can produce (R2 executed the
equivalence on a rational net; R1 read `validate_counts` and filed it
MINOR), and the header contract still says "every certification
survives" where the lane's own filed row says the sphere's cavity
falsifies it (R1 MINOR) — so no unilateral executed MAJOR and no tally
candidate. Converged also: the `shell.rs` sentence saying reverted
rows go stale; four of five mirror arms untested (R2's `None`-on-every-
arm mutant left every suite green); the guard's blindness to
`&self -> Self` producers owes a TRIM row. Unilateral, taken: "three
postures exist" falsified in the same doc comment, an invented
certificate field in the load-bearing paragraph, the dead-key
exception absent from the posture's home, `MIRROR-DESIGN.md`'s recipe
for the future mirror unit now incomplete, the `Pcurve` type's own
variant miscount, the receipt's extent short by seven `.dv` consumers
(all invariant) (R2); `revert_plane_charts.rs` re-deriving the drum
fixtures and the graft meter a third time, the `is_err()` refusal
unpinned (R1). Both corrected the brief: `with_chart_v_mirrored` lives
in `certify.rs`; only `mirror_v` matches per kind; `attach_pcurve` is
the only origin of a plane-face row. Five-item fix pass to a fresh
lane on the inherited branch: one `map_points`-shaped door for the
image map, `shift_branch` swept onto it, the variants deleted, both
reviewers' rows adopted, fixtures shared.

## Revert merged; block TOPO-B2 concluded (2026-09-14)

PR 2542 merged at `7aaeb095d` (green run 34803151950 on `8e4d5a4e9`,
full matrix, verified job by job). The unit closed; row recorded at
merge (ordinal 2705, sample #188), no tally candidate; slot 2 marked
concluded on `topo/b2-block` and in the published record. The fix
pass's one disagreement was right: the affine door takes the
point-action and the linear part as two closures because deriving one
from the other is not bit-exact under translation — `shift_branch`'s
pin would have moved. Block TOPO-B2 closes with three duals and no
tally candidate; every pre-draw field was logged before the byte. No
lane is running. The next kernel-answer unit draws TOPO-B3.

## Block TOPO-B3 cut; pre-draw fields logged BEFORE the byte (2026-09-14)

Three kernel answers on this program's files, slot order fixed here
and the byte drawn only after this entry is committed:

- slot 0 `loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart`
  — difficulty **S/M**, task class **STRUCTURAL** (two Euler doors
  made honest about the rows of the loop they move; the loud pass
  already exists).
- slot 1 `D263` — difficulty **M**, task class **STRUCTURAL-GEOMETRIC**
  (the regime classification gains the placeholder arm; the door's
  grouping changes on the placeholder cube).
- slot 2 `S93` — difficulty **S**, task class **STRUCTURAL** (two
  prose-held obligations become checked preconditions).

Held: the periodic-chart wrap under `revert` and the half-edge-minting
ops' pcurve posture (the bound ripple is Ev's question, added to the
open `[ev]` PR's scope at its next revision).

## Block TOPO-B3 drawn; slots 0 and 1 dispatched (2026-09-14)

Byte drawn after the pre-draw entry was committed (`daa30ee99`): 19
⇒ 19 mod 3 = 1 ⇒ fable at slot 1. Record branch-side on
`topo/b3-block`. Slot 0 (`loop-reparenting-…`, `topo/loop-reparenting-rows`)
and slot 1 (`D263`, `topo/d263-placeholder-regime`) dispatched
together with brief sections in their items; slot 2 (`S93`)
dispatches when one frees. Seam announced to TRIM (two posture notes).

## D263 delivered; dual out at ordinal 2706 (2026-09-14)

Slot 1 `topo/d263-placeholder-regime` delivered as PR 2548, head
`a46911a20`, full matrix green (run 34806202964). The regime's
two-way question is three-way: `MergeKind { Plane, Curved,
Placeholder }` read through the two-homes door and S330's
`net_state()`; a placeholder run forms no group and the outcome names
every placeholder face; a source stamp joining a placeholder to a
described face refuses typed (`GroupKindSplit` with both kinds
named), because the boolean reads a `SkippedMerge` as a glue it
anticipated. Phase 1 corrected the brief: the D265 control row it
named does not exist; the direct red-first row is the witness (merge
base: one curved run over six placeholder faces with a
`PeriodClosure` skip). The rungs stay kind-agnostic; `merge_group`
takes `curved` from the contract and its `OpPlacement` table is eight
sites. Dual dispatched on the frozen head beside the loop-re-parenting
lane: ordinal 2706, byte 194 ⇒ R1 OPUS, R2 FABLE; briefs stored with
sha256 privately.

## D263 adjudicated (2026-09-14)

Both blinded reviews on frozen `a46911a20`, both MERGEABLE-AFTER-FIXES,
neither with a MAJOR. R1: 0/4/5, rubric 3/3/2. R2: 0/4/2, rubric
4/3/3. Both executed the red-first row on the merge base, the
`Curved`-for-placeholder mutant (seven rows red), the mixed case
through the public door (a source stamp joining a placeholder to a
cylinder refuses with both kinds named; key-sharing is unspellable by
type), every `Ok` path's census, the `OpPlacement` table after the
parameter change, and the no-live-producer claim through revolve and
the boolean. CONVERGED: the `unreachable!` in the role pass cites a
gate this diff deleted (R1 traced the real proof to `kemr`'s
`require_key`; R2 planted a dead survivor and `kef`'s postcondition
caught it first); the brief's named control row does not exist; the
placeholder decision is written five times; the poisoned-net arm lets
the door commit surgery over a description tier 3 refuses (both, as a
NOTE). Unilateral, taken: `merge_group` believes the contract's
`curved` bool with nothing re-checking it — the lie mutant leaves the
whole `topo` suite green and silently merges a curved cube (R1,
executed; the defence D265 gave every other established fact); the
`OpPlacement` prose's "remaining twelve variants" is wrong by six and
`StaleGeometry` is still raised (R1); the render appends the
placeholder recourse to every kind split (R2); `CURVED-MERGEDOOR-SPEC`
cites moved symbols and its plan needs `MergeKind` (R2); the module
header describes a two-kind world and `merged_outline_ring`'s comment
reaches its answer by accident (R1); two spellings of the placeholder
question across `topo` (R2, filed). No tally candidate. Seven-item fix
pass to a fresh lane on the inherited branch; a poisoned face refuses
typed unless the lane reads `NetState`'s rule otherwise and says so.

## Loop re-parenting delivered; dual out at ordinal 2707 (2026-09-14)

Slot 0 `topo/loop-reparenting-rows` delivered as PR 2549, head
`d57e8ba53`, full matrix green (run 34808121136). Phase 1 found a
third door (`mfkrh`/`mfkrh_plug` promote a ring onto a caller-named
surface and were silent the same way), a fourth direction the item
lacked, and the item's `kfmrh` citation wrong (`euler_ring.rs`, not
`euler_kill.rs`). Shape: carry on one surface key, drop on another —
re-stating is a derivation and every derivation door carries the
fitted-lane bound; refusing is ruled out by the producers reaching
`kfmrh` mid-pipeline. Thirteen rows, four red on the merge base; the
`mef`/`kef` half-edge-run siblings measured and filed; a second seam
into GUARD's discard register found by CI. Dual dispatched on the
frozen head beside D263's fix pass: ordinal 2707, byte 158 ⇒ R1 OPUS,
R2 FABLE; briefs stored with sha256 privately. `S93` (slot 2) waits
for the D263 fix pass to free its lane.

## Loop re-parenting adjudicated (2026-09-14)

Both blinded reviews on frozen `d57e8ba53`, both
MERGEABLE-AFTER-FIXES. R1: 1/5/4, rubric 4/3/3. R2: 0/3/6, rubric
4/3/4. Both reproduced the four red-first rows on the merge base and
the byte-for-byte carry, both ran the never-drop and
compare-by-surface mutants, both executed the empty-ring and
broken-cycle plants and the discard gate. CONVERGED on the central
fact at DIVERGENT severity: the helper drops on a KEY change where its
name and the item say CHART — a ring promoted onto a second key
holding the identical cylinder was complete and correct on the merge
base and is silently unminted at the head, and the compare-by-surface
mutant leaves every one of the 1,217 `topo` rows green (R1 MAJOR, R2
MINOR with the same measurement inside its class finding). No
unilateral executed MAJOR: no tally candidate. Converged also: the
loud-to-silent trade is a class (every rowless curved target, all
three doors) where the PR says "once" — the brief's own C3 premise
was wrong; `Transfers` now names a door that moves no row; the helper
is a fresh copy of the one-loop-cycle walk; the `mfkrh_plug` rule is
attributed to the placeholder instead of the fresh-key mint; three
header sentences the change invalidated stand. Unilateral, taken: the
`set_face_surface` blind spot is this unit's mechanism, mis-filed to
two rows that do not cover it (R1); the fixture is tier-3-invalid by
the structural battery while a row's name says clean (R1); the merge
door also moves runs through `kef` (R1); the loop-walk census row's
floor is 33 not 34 (R1); the item's `kfmrh` citation (R2). Seven-item
fix pass to a fresh lane on the inherited branch: the predicate
becomes surface equality with both mutants pinned, the trade scoped
truthfully with a TRIM row filed, one home for the loop's rows walk.

## D263 merged; block TOPO-B3 slot 1 concluded; S93 dispatched (2026-09-14)

PR 2548 merged at `4628d4e6a` (green run 34811005145 on `9584efdcc`,
full matrix, verified job by job). `D263` closed. The fix pass took
every item and decided the poisoned-net question the reviewers had
only noted: a poisoned description refuses before any surgery, under
`NetState`'s "must fail at every consumer's described arm". Row
recorded at merge (ordinal 2706, sample #189), no tally candidate;
slot 1 marked concluded on `topo/b3-block`. `S93` (slot 2, OPUS)
dispatched on `topo/s93-rebased-carriers` with a brief section in
its item, beside the loop-re-parenting fix pass.

## Loop re-parenting merged; block TOPO-B3 slot 0 concluded (2026-09-14)

PR 2549 merged at `7f249e2dd` (green run 34815095925 on `59e956b0d`,
full matrix, verified job by job). The unit closed. The fix pass's
disagreement was the substantive one: the reviewers' bit-structural
surface equality needs `Bounds`, and the bound ripples through forty
signatures in four crates without converging — so the predicate is
key-or-provenance, with the provenance-free two-keys-one-surface
case pinned and filed rather than silently regressed. Row recorded at
merge (ordinal 2707, sample #190), no tally candidate; slot 0
concluded on `topo/b3-block`. `S93` (slot 2) is the one lane running;
the box was reclaimed to twelve gigabytes free after the two fix
passes' targets went.

## S93 delivered; dual out at ordinal 2708 (2026-09-14)

Slot 2 `topo/s93-rebased-carriers` delivered as PR 2562, head
`cb400cfe1`, full matrix green (run 34821959531). `mev`'s fan site
re-certifies every re-based edge's carrier exactly in its plan phase
(one `EdgeCurve::recertify` per edge, no re-fit for any kind, no bound
moved) and refuses typed with the body untouched; `mev_null` carries
because its point is bitwise the run's old point, and no kernel code
reaches the fan-with-run site otherwise. The `kev` half did NOT land:
the lane wired the same gate and measured 129 of 1,394 `sweep` rows
red at the blend verb's two closure kills, which kill mid-surgery and
re-describe at the door's end — a precondition cannot see a promise
the caller keeps later — so `kev`'s paragraph now states the defect
and a row (`kevs-fan-merge-needs-a-re-describing-kill-door`) carries
the measurement and the door shape that would close it. Nine rows
re-baselined onto `mev_null`; `roundtrip`'s `Kev` arm skips every fan
merge; a seqgen stream pin re-pinned. Dual dispatched on the frozen
head with the deviation at its centre: ordinal 2708, byte 160 ⇒ R1
OPUS, R2 FABLE; briefs stored with sha256 privately. No other lane.

## S93 adjudicated; one tally candidate, doc class (2026-09-14)

Both blinded reviews on frozen `cb400cfe1`, both
MERGEABLE-AFTER-FIXES. R1: 1/6/5, rubric 4/3/3. R2: 2/4/4, rubric
4/3/2. Both reproduced the red-first row, the sweep measurement (129
of 1,394, at the blend's two closure kills, to the row), the last-edge
mutant, the `Kev` skip census on the pinned streams (136 selections,
39 executing, 69 newly skipped) and the seqgen filter still biting.
CONVERGED, severity divergent: the rewritten `kev` paragraph is the
prose-held obligation in a new coat — the trap the style lane names,
fired on schedule (R2 MAJOR, R1 NOTE plus its Q1); "a certified `mev`
cannot reach a run-moving fan site" is false — a self-loop closed
carrier reaches the splice through the certified door (R1 MAJOR, R2
MINOR, both executed); the advertised control pins `mev_null`'s
bypass, not the gate; the coverage trade disclosed but unmeasured;
the iso route row's name and comment false; the kill∘make taxonomy
stale. UNILATERAL and executed, R2: **the filed row's central
measurement is false** — "`cargo test -p topo` was green with the
gate on `kev`" — R2 wired the gate and got fourteen of the tree's own
rows red (the generator's walk and teardown among them), and found
the branch commit that claims the measurement does not compile its
tests. R1 reproduced the sweep half and did not test the `topo` half.
A false measurement in a tracker row and the PR body, on which the
follow-up's design rests: **tally candidate +1, doc class**, coded at
the blinded adjudication (S330's precedent; the code-class exclusion
noted in the row). Unilateral, taken: the receipt's blind-spot
sentence false and its sweep count stale (R1); the enumerator
narrowed where it could have asserted the refusal under fuzz (R1);
four `kev` fixtures re-baselined onto tier-2-invalid scaffold bodies
undisclosed (R1); the gate's coincident-point sentence false for the
M7-8 class and for pre-stale runs (R2). Both reviewers found a landing
shape the filed row does not name — the surgery-scope switch (R2, Ev's
PR-2305 shape) and a tier-3 postcondition at the blend door (R1) —
recorded in the row for the unit that lands `kev`'s half; `S93` stays
open with the `mev` half landed. Nine-item fix pass to a fresh lane
on the inherited branch; a bitwise coincident-point short-circuit
taken so the gate carries where nothing moved.

## S93 merged; block TOPO-B3 concluded (2026-09-14)

PR 2562 merged at `21a0de4ce` (green run 34831213029 on `8fc3a1e6a`,
full matrix, verified job by job). `S93` stays OPEN: the `mev` half
landed, the `kev` half is the filed row with three landing shapes
and the generator's dependence measured honestly (fourteen `topo`
rows at four coordinates, not "green"). The fix pass's divergence was
right: a bitwise coincident-point short-circuit is not buildable at
`T: Real`, so the gate re-asks against current endpoints instead.
Row recorded at merge (ordinal 2708, sample #191) with the program's
third tally candidate (doc class); slot 2 concluded on `topo/b3-block`
and the block's CONCLUDED record published. Block TOPO-B3 closes with
three duals and one doc-class candidate. No lane is running.

## Block TOPO-B4 cut; pre-draw fields logged BEFORE the byte (2026-09-14)

Three kernel answers on this program's files, slot order fixed here,
the byte drawn only after this entry is committed:

- slot 0 `geom-source-absence-conflates-four-origins` — difficulty
  **M**, task class **STRUCTURAL** (a typed absence on the identity
  channel; consumers in the recipe layer read it).
- slot 1 `revert-leaves-a-periodic-charts-loop-wrap-mid-chain` —
  difficulty **M**, task class **NUMERIC** (a branch re-park under a
  frame reversal, pinned by the involution).
- slot 2 `edge-carrier-kind-has-no-readback-door` — difficulty **S**,
  task class **STRUCTURAL** (a readback twin in the ratified shape).

Non-dual beside it: `D107` (test support; one style review). To Ev on
the open `[ev]` PR: the `kev` half's landing shape and the
half-edge-minting operators' pcurve posture.

## Block TOPO-B4 drawn; slots 0 and 1 and D107 dispatched (2026-09-14)

Byte drawn after the pre-draw entry was committed (`4b813113a`): 223
⇒ 223 mod 3 = 1 ⇒ fable at slot 1. Record branch-side on
`topo/b4-block`. Slot 0 (`geom-source-absence-…`,
`topo/geom-source-typed-absence`, OPUS) and slot 1 (`revert-leaves-…`,
`topo/revert-reparks-the-wrap`, FABLE) dispatched with brief sections;
`D107` (`topo/d107-kemr-hammer-fixture`, non-dual, OPUS — no row)
beside them as the third lane, test support only. Seams announced to
WIRE (the stamping call) and TRIM (a reverse-parking helper). Slot 2
(`edge-carrier-kind-has-no-readback-door`) dispatches when a lane
frees.

## Container restart killed all three B4 lanes; WIP recovered, lanes relaunched (2026-09-14)

A container restart (~10:40 UTC) killed the three live lanes about ten
minutes in. Their worktrees survived with uncommitted edits; each was
committed as-is as one `wip: uncommitted work recovered after a
container restart…` commit on its branch and pushed
(`topo/geom-source-typed-absence` 3b2d9fd49,
`topo/revert-reparks-the-wrap` 73b66589e,
`topo/d107-kemr-hammer-fixture` bebf5dc46), the worktrees unlocked and
removed, and fresh lanes launched on the pushed branches with the same
briefs plus a preamble: read the WIP, keep what is right, redo what is
not, say in the PR body that the branch starts from a recovered WIP.
Model assignment unchanged (slot 0 OPUS, slot 1 FABLE, D107 OPUS); the
draw is not re-rolled by a restart. Lanes are now told to push early
and often, since only pushed commits survive a restart. Not a review
event, so nothing in the block record moves.

## D107 delivered (PR 2575, green); style review dispatched (2026-09-14)

The relaunched D107 lane delivered on head `d695dffd0`, run
34837521903 green (twelve `test`, five `k-lint`, the release-profile
corruption job that runs the hammer rows). Phase 1 found `kemr`
blocked twice over: no fixture presents an edge whose two halves sit
in one loop, and the hammer never enumerated an edge's own mate pair
— each alone keeps `kemr` at zero while the aggregate floor stays
green (the anti-vacuity class, evidence added to S-TINT's existing
row rather than a duplicate). `ops_ring_bridge` (a holed box with one
`mekr_chord` bridge) plus the mate-pair enumeration takes `kemr` to
2 on the spent graft and ~45 under sampling; the exposure table is
now asserted, not printed, on all twelve test jobs. The module doc's
"every operator that reaches `link_half_edges`" claim was false both
ways (`mfkrh_plug` reaches it nowhere; `mekr` reaches it at twelve
splices and was never driven) — narrowed, with `mekr` filed on this
slate (`review-d18-drives-no-mekr-though-it-reaches-link-half-edges`).
The citation fix lands as briefed; the `--nocapture` question is
moot. Both items closed on the PR. Non-dual: one style review
dispatched on the frozen head (brief at
`/home/user/topo-d107r-scratch/brief.md`, sha256 e4096369…); merge
after its fixes ride.

## Slot 1 delivered (PR 2573); dual dispatched, ordinal 2709 (2026-09-14)

The relaunched slot 1 lane delivered `revert-leaves-a-periodic-charts-loop-wrap-mid-chain`
on head `725d3c204`, run green (twelve `test`, five `k-lint`). Phase 1
measured the finding at the source's `prev(first)` and found the
brief's mechanism unsound as a rule: a `shift_branch` per row
round-trips only where the addition is exact, so the lane moved each
curved loop's `Cycle::first` to its source predecessor instead — no
row touched, the involution proved by the prev/next swap. The torus
hypothesis is false (no azimuth-free joint; both tori are controls).
One golden moved (`voided_rod`'s census verdict hash — same multiset
claimed, different order), two `shell9_probe` rows re-baselined from
pinning the defect to pinning the fix, one discard registered. Ordinal
**2709** claimed on main (PR opened from `topo/claim-2709`); parity
byte 12 ⇒ R1 OPUS, R2 FABLE; briefs stored with sha256 in
`/home/user/topo-orch/brief-hashes.txt` (template ebc58bf6…, R1
27d51383…, R2 a52dd601…; diff 4 lines), both reviewers dispatched
concurrently on the frozen head. The brief's correctness lane leans
on the golden re-cut (verdicts diffed as multisets), the receipt of
`first` as an ORDER (exports, mesh, census, naming) rather than only
as branch parking, and the fresh-instance check.

## Slot 0 delivered (PR 2576); dual queued behind lane capacity (2026-09-14)

The relaunched slot 0 lane delivered `geom-source-absence-conflates-four-origins`
on head `52c1868e8`, run green. Shape: `GeomOrigin<'a>` (`Recipe`,
`Imported`, `KernelDirect`, `Cleared`) in three side maps beside the
N6 maps, holding only origins a description carries while it holds no
`GeomSource`; `revert.rs` and `transform.rs` unedited, so N6 decides
nothing differently. Two disclosed deviations: `Constructed`/`Derived`
collapse to `KernelDirect` (the arena doors carry no caller identity;
filed on this slate) and `Cleared` carries no `by` (one caller). The
dual dispatches (ordinal 2710, byte drawn then) when the D107 style
review frees its lane; three heavy lanes is the box's ceiling.

## D107 style review adjudicated; fix pass dispatched (2026-09-14)

Verdict MERGEABLE-AFTER-FIXES, fourteen findings, every PR claim
reproduced (exposure table exact both columns, four mutants behave as
stated, the release job's exact selection green). Adjudicated: two
fresh instances of the closed defect are the substance — the new
`FIXTURES` const is an enumerated dimension with no floor (dropping
`ops_cube` passes green; S5, sure) and the sampling row's aggregate
`require_nonzero_among(LINK_OPS, n-1)` keeps five operators under the
slack `kemr` sat under (S6, half-fix) — both go to the fix pass with
`require_each` as the shape. Three stale prose counts in the file
whose subject is unheld counts (S1–S3), an assertion narrower than
its comment (S7), the one-splice `kemr` strut undriven (S8), the
`NotSameLoop` sentence witnessed for one fixture of three (S10), an
unreachable guard (S9), two PR-body overpromises (S4 gating, S11
territory's globs not covering `work/`), one filed-row cost estimate
(S12). Fix pass dispatched as a fresh lane on the inherited branch;
merge and close when its run is green. The review's report is
archived at `/home/user/topo-orch/d107-style-report.md`.

## Revert-wrap dual concluded (ordinal 2709); fix pass dispatched (2026-09-14)

Both reviews MERGEABLE-AFTER-FIXES on `725d3c204`, converging. The
mechanism held under every mutant either side planted (`next` for
`prev`, anchor write dropped, graft carrying `next`): the anchor move
is exactly `prev(first)`, the involution bitwise, the rows untouched.
The substance: (1) the golden re-cut's account is false — both
reviewers dumped `voided_rod`'s forty verdicts and diffed them as
multisets: `props_rim_side` and `props_rim_dir_group` changed SIGN
(two each), the readings compensated downstream; the PR said "same
multiset, same answers"; (2) the anchor's new meaning lives only in
`revert.rs` while `LoopBoundary::Cycle::first` still says it carries
no meaning and `transform.rs`'s reversal tripwire lists only `sense`
— the fresh-instance shape; (3) the plane/curved partition is two
rules where one would do — R1 re-anchored every loop and the planar
battery and census stayed green, so nothing but the lane's own
assertion observes it; ruled: uniform rule, re-baseline what moves.
Also: an unresolved `.prev` in the plan phase (no typed `Corrupt`),
the receipt scoped to branch parking rather than `first` as an order
(R2 derived the order readers and exported the STEP corpus
byte-identical; R1 found `chart_boundary`'s torus lever unpinned),
stale sibling prose (`shell7_seam_corner.rs`, DESIGN's revert
clause), two duplicated fixtures, a `1e-9` literal in a test reader,
and the characterisation "only azimuth-free joints wrap" measured
sufficient, not necessary. Brief corrections: C4's "controls green on
the merge base" was wrong (all seven red on the anchor assertion);
the skipped `.next =` sweep is clean (both ran it). Fix pass
dispatched as a fresh lane on the inherited branch, same arm as the
implementer; reports archived at `/home/user/topo-orch/wrap-r{1,2}-report.md`.
Row recorded at merge.

## Typed-absence dual prepared, ordinal 2710 claimed; dispatch waits on lane capacity (2026-09-14)

Parity byte 219 ⇒ R1 FABLE, R2 OPUS. Briefs stored with sha256 in
`/home/user/topo-orch/brief-hashes.txt` (template c9deb907…, R1
b8f56a7d…, R2 811ec39b…; diff 4 lines); claim PR opened from
`topo/claim-2710`. The correctness lane leans on the fresh-instance
check (`KernelDirect` is "the state no door marked" — an absence read
as a positive claim, the closed defect one level down?), the
storage shape the brief did not name (one enum-valued map per kind
with the `Recipe` arm projected, exclusion by type, against six
discharge sites and a `debug_assert!`), the deviations' honesty, and
the import/graft path. Both arms dispatch together when the D107 fix
pass frees the box's third lane; the wait is disclosed in the claim
entry.

## Typed-absence dual dispatched (ordinal 2710) with two fix passes live (2026-09-14)

Claim merged on main. The wait for the D107 fix pass to free a lane
was cut short: the box's load was 1.5 on four cores with both fix
passes in their reading phase, so R1 (FABLE) and R2 (OPUS) dispatched
together on the frozen head `52c1868e8` with two fix-pass lanes live
— the condition the claim entry's shared-box note records, applying
to both arms equally.

## Typed-absence dual concluded (ordinal 2710); fix pass dispatched (2026-09-14)

Both reviews MERGEABLE-AFTER-FIXES on `52c1868e8`, converging. Every
PR claim either side could execute held (H1 reproduced at the merge
base by R1 and at the head by R2; the red-first row reds under the
merge base's `clear()`; the receipt and sweep re-derived exact; the
gate green on the SHA). The substance, found by both independently:
(1) the storage shape is the trap the brief did not name — three
`OriginMark` maps beside three `GeomSource` maps, exclusion held by
twelve to sixteen mirror sites and a `debug_assert!`, where one
enum-valued map per kind (`Recipe(GeomSource) | Imported | Cleared`,
the `Recipe` arm projected for every existing reader) makes the
exclusion a type; the PR's H2 argued against a shape nobody proposed;
(2) `KernelDirect` is "the state no door marked" — an absence read as
a positive claim, the closed defect one level down: R1's mutant
dropping every mark in `revert` and R2's deleting two of three graft
carries left the whole topo suite green. Ruled: one map per kind, and
`KernelDirect` written positively at the three mint doors so the map
is total over live keys and a forgotten carry or mark is loud (D9 row
4). Also: `mark_imported` overwrites the `Cleared` defect arm (R2,
sure); the "both channels" revert row and the re-stamp row run on
fully stamped bodies and pin nothing about the origin arms (both);
point/curve graft carries and the removals unpinned; a recipe stamp
over `Imported` lossy and undocumented; `carve`'s point/curve orphan
loops reach no side table (pre-existing); the stale-key row leans on
foreignness; the `source.rs` header says four origins are separated
where three are; `body.rs`'s header lists half its fields. R2 noted
the workspace's release profile keeps debug assertions on, so the
`debug_assert!` fires in both profiles today — moot once the type
holds the exclusion. Fix pass dispatched as a fresh lane on the
inherited branch, same arm as the implementer; reports archived at
`/home/user/topo-orch/geom-r{1,2}-report.md`. Row recorded at merge.

## D107 merged (PR 2575); slot 2 dispatched (2026-09-14)

The D107 fix pass worked all eleven items on head `0abfd3d91`, run
34845056772 green. `FIXTURES` is now floored per body with the exact
call count pinned beside it (a derived floor list cannot see an entry
deleted from the enumeration it derives from — the exact count can);
six seeds put every link operator's minimum far above one, so the
aggregate with slack became `require_each` and the "deliberate slack"
comment went; every hard-coded count in the file is now held or gone;
both `kemr` splices witnessed by the neighbours read before the call;
the one-splice strut arm driven by a new `ops_strut_cube` and counted
apart; the `NotSameLoop` converse over all three closed fixtures; the
unreachable guard gone behind one `mate_halves`. Merged; both items
closed; no A/B row (non-dual). Slot 2
(`edge-carrier-kind-has-no-readback-door`, OPUS,
`topo/edge-carrier-kind-readback-door`) dispatched on the freed lane
with the D107 lane's target retired and the typed-absence R2 target
re-seeded as its own. Live: the revert-wrap fix pass, the
typed-absence fix pass, slot 2.

## Revert-wrap merged (PR 2573); row 2709 recorded, sample #194; slot 1 concluded (2026-09-14)

The fix pass took all nine items on head `0b5000154`, run 34844909382
green: the golden account rewritten to what moved (four verdict
signs, flux-compensated) with a sorted-multiset row beside the census
golden; the anchor invariant stated at `LoopBoundary::Cycle` and in
the reversal tripwire; the uniform rule adopted — every loop's
`first` moves — and the two STEP fixtures it moved (`die`,
`kiss_assembly`) regenerated with entity and line counts and
id-stripped line multisets identical; `RevertError::Corrupt` now
carries a closed `RevertLink` and refuses a dead `prev` typed; the
receipt widened to `first` as an order with a `chart_boundary`
result-kind row across the reversal; two fixtures shared, the `1e-9`
literal replaced by an exact whole-period test; DESIGN's D1 revert
clause re-worded by code (its `git log -S` names only an editing
pass). Two rows filed: TINT's ordered-FNV verdict channel is
anchor-sensitive; PROPS' `props_rim_side`/`props_rim_dir_group` signs
are facts about cycle order. Merged at `cbf5d1a49`; the row (ordinal
2709, sample #194, no tally candidate — the converged golden finding
is doc-class) and the unit's closure are on a docs PR from
`topo/row-2709`; slot 1 concluded on `topo/b4-block`. The
implementer's target retired (20 GB free). Live: the typed-absence
fix pass and slot 2.

## Typed-absence merged (PR 2576); row 2710 recorded, sample #195; slot 0 concluded (2026-09-14)

The fix pass took all seven items on head `5fdba8217`, run
34852906131 green: one `GeomOrigin` map per kind with the `Recipe`
arm projected (write sites 30 → 23, maps 6 → 3, mirror pairs 16 → 0,
the `debug_assert!` and the stored/borrowed enum pair gone);
`KernelDirect` written at the three mint doors so the map is total
over live keys — R1's graft mutant now reds 21 rows and R2's revert
mutant 69, where each reddened one or none; `mark_imported` leaves
`Cleared` and `Recipe` alone, documented; the revert and re-stamp rows
rebuilt on a mixed fixture; `carve` removes the row in all three
orphan loops; the stale-key row minted-then-removed in one body; the
`verbs` README's stale table name re-worded. Merged at `1adddd884`.
Row (ordinal 2710, sample #195) recorded with one code-class tally
candidate: R2's unilateral executed MAJOR that `mark_imported`
overwrote the `Cleared` defect arm — a public door's contract, latent
today, never mentioned by R1. Tally: candidates +4 (code class D265
and this; doc class S330 and S93). Unit closed on the row PR from
`topo/row-2710`; slot 0 concluded on `topo/b4-block`. Slot 2 is the
block's last open slot. Targets retired; 17 GB free.

## Step 1's closure fired WIRE's park; re-parked, step 3 filed (2026-09-14)

Closing `geom-source-absence-conflates-four-origins` made WIRE's
`axis-shaped-identity-channel` (parked on it as step 1 of the ratified
axis-channel cut) fail lint — a fired trigger is not a blocker. The
cut names steps 2 and 3 as the next movers, so the row is re-parked on
EXCH's `step-import-discards-the-entity-ids-that-are-its-identity-channel`
and on step 3, which had no row: filed on this slate as
`axis-per-component-source-beside-geom-source` (the per-component
source beside `GeomSource`, TOPO's ground per the cut). Not yet cut
into a block; the program decides its slot when it takes it. Both
changes ride the row-2710 PR.

## Slot 2 delivered (PR 2587); dual dispatched, ordinal 2711 (2026-09-14)

The slot 2 lane delivered `edge-carrier-kind-has-no-readback-door` on
head `36d4b9e76`, run 34853345271 green. Phase 1: one walk
(`readback::certified_carrier`) shared by `edge_pose` and the new
`readback::edge_carrier_kind`; `CurveKind` stays in `query.rs` on the
ground that `crates/verbs/README.md` §1 S1 is ratified text (the
reviewers are asked to run CLAUDE.md's `git log -S` check on that
claim); the chain stops before Python, with the twin filed on LIB's
slate and a `B-EDGE-KIND` gap family added to the binding census.
Two rows filed (LIB: the Python twin; TOPO: the program's `keep_out`
names SEAT, which left the tracker). Ordinal **2711** claimed on main
(PR opened from `topo/claim-2711`); parity byte 127 ⇒ R1 FABLE, R2
OPUS; briefs stored with sha256 (template c8a19edc…, R1 1b741b52…, R2
da904e08…; diff 4 lines); both reviewers dispatched concurrently on
the frozen head. The correctness lane leans on the fresh-instance
check (any other edge → carrier walk left outside the one door), the
ratification claim, the census gap family's honesty, and the
delegation witness the query seat's own rows cannot give.

## Edge-door dual concluded (ordinal 2711); fix pass dispatched; the stale SEAT fence retired (2026-09-14)

Both reviews MERGEABLE-AFTER-FIXES on `36d4b9e76`, converging; no
unilateral MAJOR. The code held under every mutant either side
planted (the shared walk's arms swapped, a live NURBS reported
`NoCarrier`, the refusals collapsed, `CurveKind::of` one arm at a
time — each reds exactly the row whose name says why; `edge_pose`
bit-identical to the merge base). The substance: (1) the PR's
load-bearing reason for keeping `CurveKind` in `query.rs` — that
`crates/verbs/README.md` §1 S1 is ratified text — is false by
CLAUDE.md's own check, run by both: the sentence was agent-authored at
SEAT-2 and reached the README in the closing sweep; what Ev ratified
at #1388 says "`CurveKind` moves down beside `Curve3`" (R2 MAJOR, R1
MINOR); (2) the document twin's row is green under a constant-return
mutant — the box fixture's edges are all lines (R2 MAJOR, R1 MINOR;
both wrote the washer probe); (3) the walk's third refusal
`Dangling { Geometry(Curve) }` is pinned nowhere and IS plantable from
a crate-internal test (both); (4) the delegation is unwitnessed —
restoring the merge-base arena walk in the query seat leaves every
row green (both; R2 names `test_utils::source`'s built-in-one-place
instrument); (5) the same walk is open-coded on the face side of the
same file (`face_pose`/`face_carrier_kind`, no `certified_surface`)
and across the crate with its own vocabularies (`rim_of` flattening
the very pair the door tells apart, `attach.rs`, `coherence.rs`,
`props.rs`, `split.rs`, `revert.rs`, `euler_kill.rs`) — the
fresh-instance shape (R2 MINOR, R1 NOTE); (6) the §5 blind-spot
sentence is false for four tag-only `matches!` sites (R2); (7) the
interrogate ladder's fifth copy (both, Q1); (8) `edge_pose`'s rows
pin only the line success path (both). R2: the PR is dirty against
main (`work/wire/log.md`). Ruled: `CurveKind` stays in `query.rs` on
its true reasons (both readers in `topo`; the door imports its answer
type as it does `SurfaceKind`), the PR body corrected, and the
divergence between the ratified VERB-SEAT sentence and the code filed
on this slate for Ev rather than re-decided by a lane. TOPO's own
`keep_out` clause sending `query.rs` seams to SEAT's closed board
(R2 MINOR-8; the lane's filed row) is retired here in `program.md`;
the row closes on the fix pass. Fix pass dispatched as a fresh lane
on the inherited branch, same arm as the implementer; reports
archived at `/home/user/topo-orch/edge-r{1,2}-report.md`. Row
recorded at merge.

## Block TOPO-B5 cut; pre-draw fields logged BEFORE the byte (2026-09-14)

Cut while B4's last slot is in its fix pass (lane capacity allows one
implementer now, the rest when the fix pass frees). Three kernel
answers on this program's files, slot order fixed here, the byte
drawn only after this entry is committed:

- slot 0 `set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left`
  — difficulty **S**, task class **STRUCTURAL** (a setter takes the
  loop doors' carry-or-drop through `same_chart`).
- slot 1 `mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`
  — difficulty **M**, task class **STRUCTURAL** (a run's rows on two
  core operators; `kef`'s precondition order is the question).
- slot 2 `a-null-edge-can-be-re-based-onto-a-distinct-point` with
  `the-re-basing-gate-refuses-m7-8-where-nothing-moves` — difficulty
  **M**, task class **STRUCTURAL** (a gate's hole and its over-refusal
  answered without a point-identity door).

## Block TOPO-B5 drawn (2026-09-14)

Byte drawn after the pre-draw entry was committed (`41eb35f5c`): 7 ⇒
7 mod 3 = 1 ⇒ fable at slot 1. Record branch-side on `topo/b5-block`.
Slot 0 (`set-face-surface-leaves-…`, OPUS) dispatches now on a target
seeded from the edge lane's warm build; slots 1 (`mef-and-kef-…`,
FABLE) and 2 (`a-null-edge-…` + `the-re-basing-gate-…`, OPUS) dispatch
as the edge-door fix pass frees the box.

## B5 slot 0 delivered (PR 2594); dual prepared, ordinal 2712 claimed (2026-09-14)

The slot 0 lane delivered `set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left`
on head `71d846be4`, run 34868100811 green. Shape: a face door
`drop_face_rows_on_chart_change` in `euler_ring.rs` delegating per
loop to the loop doors' drop, called after the surface write and
before the old key's orphan removal (`same_chart` is private to that
file, so the setter does not spell the walk); `set_edge_curve` measured
not the same hole (pass 2 re-derives from the current carrier);
twelve production callers receipted, none reading rows after a swap
without re-minting; two `mesh` test callers repaired. Parity byte 72 ⇒
R1 OPUS, R2 FABLE; briefs stored with sha256 (template 14cb3598…, R1
f54bd8ad…, R2 d31d1c48…; diff 4 lines); ordinal **2712** claimed on
main (PR from `topo/claim-2712`). Dispatch waits for disk: the edge
fix pass holds the only other warm target and the box is at 5 GB;
both arms dispatch together when it frees. The correctness lane leans
on the order of operations (drop before or after the orphan
removal), `same_chart`'s rungs at this door, whether the `mesh`
re-attach papers over a kernel hole, and the fresh-instance sweep of
every other chart-swapping door.

## Ev's first reply on the [ev] PR; edge-door merged (PR 2587); row 2711; block B4 CONCLUDED; the set-face-surface dual dispatched (2026-09-14)

Ev on PR 2527: 1 ruled **(A)** — a chart lives in one solid is a
`Body` invariant and the mover keeps it; recorded on the item, a unit
for the next cut. 2 conditional on DESIGN.md's panic-versus-error
strategy: the D9 reading (a torn body at a setter is a kernel-bug
state, row 5, MUST panic; the graft's spent-body class is S14's) is
written on the item and the PR, close proposed on it. 3 and 4 asked
for elaboration: written on the items and the PR body, one comment
posted leading with the role tag — (a) puts the obligation on the
boundary the kernel already has (the surgery scope's close; the blend
already opens one scope for the whole blend) while (c) puts it on the
operator call with two doors for one kill; the minting posture's
context, with a third option (enforce the closing-mint convention at
the scope's close, sharing (a)'s list) recommended as the enforcement
unit beside the declared posture. Waiting on Ev's letter for 3 and
yes/no for 2 and 4.

The edge-door fix pass took all nine items on head `e7a4241c9`, run
34868453112 green: the ratification claim corrected with the pickaxe
result and the disagreement between the ratified VERB-SEAT sentence
and the code filed `needs_ev`; the washer row (a constant-return
mutant now reds exactly it); the third refusal pinned from a
crate-internal plant; the delegation pinned by a source-rules row
(the merge-base walk restored in the seat is the only red
workspace-wide); `carrier_surface` shared by both face doors and
`rim_of` routed through `edge_carrier_ref` with `CarrierAbsence` as the
shared refusal (the `NotAnArc { kind: None }` conflation left and
filed — an arm's meaning would change); the five interrogate ladders
collapsed onto one reader; the fence row closed. Merged at
`b84de2f1a`; the row (ordinal 2711, sample #196, no tally candidate —
the converged findings are doc/claim class or MINOR) and the unit's
closure ride the docs PR from `topo/row-2711`, which also publishes
block TOPO-B4's CONCLUDED record. Block TOPO-B4: three duals, one
code-class tally candidate (typed absence), all three implementer
lanes relaunched once after the container restart.

The set-face-surface dual (ordinal 2712, R1 OPUS on the implementer's
target, R2 FABLE on the edge fix pass's retired target) dispatched on
frozen `71d846be4` the moment the fix pass freed the box. Live: the
two reviewers. B5 slots 1 and 2 dispatch when disk allows a third
seed.

## `[ev]` PR: is "a chart lives in one solid" a `Body` invariant? (2026-09-13)

The chart-spans-solids row (SHELL-8's placement) is a design choice,
proposed to Ev on an `[ev]` PR from `topo/ev-chart-spans-solids`:
(A) the mover re-mints and tier 1 states the invariant (recommended —
what every other producer does; the shape the boolean's fix needs),
(B) the shell doors group per (solid, surface), (C) keep the typed
refusal. The attach postcondition's panic half rides in the same PR
with a proposed close: no public input can present a torn body to a
setter, so it is D1's contract firing late, not a D9 refusal owed.
`needs_ev: true` on the chart row; the attach row waits unedited.

## Ev's second reply on the [ev] PR: 2 closed; (c) becomes the default kev; the minting posture's long-term target (2026-09-14)

Ev: 2 "cool" — `attach-postconditions-validate-the-whole-body-and-panic`
closed on the D9 reading (on the `[ev]` branch, lands with the PR).
3: Ev asked why the re-describing kill of (c) cannot simply be the
default `kev`. It can, and it is cleaner: `kev(he, redescriptions)`
certifies each supplied spec against the endpoint the merge will give
its edge, else re-certifies the existing carrier, refuses typed before
mutating — no intermediate stale state, no scope list, no close-time
sweep. (a)'s deferral was for a caller that does not exist: the
blend's two sites already compute the carriers they hand to
`attach_contact`; the generator's roundtrip inverse is already the
two-op `kev` + `set_edge_curve` with a chord spec. Recommendation
revised to (c) as the default and only `kev`; the earlier (a) leaned
on a deferral nobody needs, and the log says so. 4: Ev asked for the
cleanest long-term shape ignoring churn — the operator completes the
face it touches: mint the row at the mint site (D9 row 0 on the
half-minted state), closed-form rows under `Decide` if the derivation
splits from the fitted lane, a typed refusal at the fitted frontier,
the full bound ripple only if it cannot split; the closing-mint
convention's thirteen copies retire; the declared posture is the
interim only. Both written on the items and the PR body, one comment
posted. Waiting on Ev's letter for 3 and yes/no for 4.

## Ev's third reply: 4 ratified; 3's signature question answered with the two-door shape (2026-09-14)

Ev: 4 "sounds good!" — the long-term minting target ratified (rows
minted at the mint site; closed-form split under `Decide` first,
typed refusal at the fitted frontier, ripple only if it cannot split;
the closing-mint convention retires); the row is now a unit for a
block slot. 3: Ev did not like the list-with-empty-default and asked
how the signature sits beside the other operators. Answer written on
the item and posted: the make-operators take geometry and a band, the
kills take keys only; `kev`'s fan merge is the one kill that changes
geometry. The harmonious shape is (c) as originally written — two
doors: `kev(he)` keys-only, refusing typed where a merged carrier
would go stale (the S93 gate inside the kill), and
`kev_describing(he, specs, tol)` shaped like `mev` for the callers
that re-describe — the `mev`/`mev_line`/`mev_null` variant-family
pattern. No default anywhere. Recommendation: (c) as two doors; the
"default" reading was the orchestrator's, not the shape's. Waiting
on Ev's letter.

## Ev ruled 3 = (c) as two doors; all four questions on PR 2527 answered (2026-09-14)

Ev: "(c) sounds good then!". Recorded on the item; `S93` closes with
the unit. The four rulings stand: 1 (A), 2 closed, 3 (c) two doors, 4
the mint-at-site target. The `[ev]` PR merges when its docs tier is
green on the recorded head; the three ruled units are listed in the
plan for the next block cut (after B5's slots 1 and 2 dispatch and
conclude).

## B5 slot 1 dispatched; set-face-surface R2 delivered (2026-09-14)

The typed-absence R2's retired target, freed again by the
set-face-surface R2 (MERGEABLE-AFTER-FIXES, no MAJOR, report
archived at `/home/user/topo-orch/sfs-r2-report.md`), is slot 1's
seed: `mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`
(FABLE, `topo/mef-kef-runs-carry-or-drop-rows`) dispatched. Its brief
is told that PR 2527 ruled the minting posture's long-term shape and
that this unit leaves the minted halves' posture alone. Live: the
set-face-surface R1 (finishing) and slot 1. Slot 2 waits for the next
freed target.

## [ev] PR 2527 merged: four rulings on main (2026-09-14)

Merged at `d439bdfe6` after Ev's word on all four: 1 (A) — the chart
invariant kept by the mover; 2 closed on the D9 reading; 3 (c) as two
doors (`kev` refusing typed, `kev_describing` for the callers that
re-describe); 4 the mint-at-site target with the closed-form split
tried first. The three ruled kernel answers are units on this slate,
listed in the plan for the next cut; `attach-postconditions-…` is
closed. The PR subscription is released.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `census-witness-string-repeats-the-subject`.** Two
`ValidationError` arms document `witness` as *"a debug rendering of the
witnessing position/site"* and render it after the preposition *at*; two
sites fill that locative slot with a repeat of the subject the same
sentence already named, so the message renders the pair twice and supplies
no position at all.

It lands on TOPO because the decision is about `ValidationError`'s shape,
declared and documented in `crates/topo/src/validate.rs`, which territory
says is yours: `witness` is `String`, not `Option<String>`, so *"this arm
has no position"* has no spelling — either the field becomes optional or
the arms carry a typed locus.

**The two defective sites are REACH's, and FIX's fence prose about them
was stale.** Both are in `crates/topo/src/census.rs`, which
`work/fix/program.md` called CURVED's; territory says **reach**, and
CURVED's `paths` no longer name the file. The site at `census.rs:1677`
also has no position available to supply — its evidence is
`ChartOverlap::PositiveArea`, a region verdict the chart-region predicate
returns without a point — so supplying one reaches CHART, which owns
`crates/topo/src/chart_region.rs`. Not a correctness gap: both variants
carry the pair typed, so a consumer resolves from the field and never
parses the prose. What it costs is the locus.

Signed (FIX orchestrator).

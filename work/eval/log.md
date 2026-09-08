# EVAL log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/eval/plan.md`.

## Opened (2026-09-06)

Opened in the tracker-wide cut of 2026-09-06 (Ev's direction,
in-chat; `docs/WORK-TRACKS-2026-09.md` addendum 2), on the ground
SEAT's sweep (DOC-LEDGER sweep 8, the same day) left in no program's
`paths`. Six `work/issues/` items re-homed here by `git mv` with a
`## Re-homed` record each — five open and `two-verb-seats-do-not-compose`
set to `deferred` with its ratification cited — and four code-quality
Track V rows claimed (`D360`, `D367`, `D368`,
`emit-blend-restates-the-kernels-own-arguments`). Track V's other
rows (`D364`, `G4` on `crates/profile`, `S190` on `assembly.rs`) sit
on S-BOOL's and DOCM's globs and stay in `work/code-quality/`;
`S190`'s trigger (#855) has fired and the row reads as verify-and-close
for DOCM.

The paths are enumerated, not globbed, for the reason TOPO's are. No
branch exists yet; the first dispatch is unit 1.

## Readiness pass (2026-09-08, remote session)

Read in full before this entry: `work/README.md`, the plan, every item,
`docs/prompts/implementer-discipline.md`, `docs/prompts/reviewer-style-lane.md`,
`docs/REVIEW-STYLE-DISPATCH.md`, the orchestration and lane memories.
**Verdict: the program is ready to dispatch.** `work.py lint` is clean,
the band is claimed, no `eval/` branch exists on origin, no open PR
touches EVAL's `paths`, and every site the five E units cite is live on
main at `063bf1dc8`. What the pass found, so the first lane does not
re-derive it:

- **Citations moved since filing.** `wire.rs:1132` (the `Pinned` lift)
  is `:1226`; `wire.rs:784` (`pinned_plane`) is `:878`; `D368`'s
  "hand-lifted `Vec3` at `anchor.rs:238`" is now the `v` closure inside
  `map_affine` (`:252`), and its `from_f64` instance IS `embed_affine`
  — so `D368` closes by construction when unit 1 retires
  `embed_affine`. `embed_profile` (`anchor.rs:281`) is a third caller
  of the walk the item did not list; it goes with unit 1. The rest of
  `embed_profile` (a per-vertex `Point2` lift, `:291`) is `D385`'s
  shape on `crates/profile`'s door and is NOT unit 1.
- **Unit 1 spec written**: `docs/EVAL-1-SPEC.md`, branch
  `eval/1-affine-lift`; both items set to `spec`. The `try_map`
  question goes to PROPS by note, with a `parked` row filed in the unit
  that writes the note.
- **Unit 2 sites**: sentinels at `eval/mod.rs:2950`/`:3069`, the
  census at `:4422`; tags written outside the sentinels today are
  `40`, `41` (twice, `:3122` and `:4232`), `42`, `43`, `44`, `45`.
- **Fence gap, DOCM's to close**: `names/merged.rs` (DOCM-8, PR 2073,
  merged after this program opened) is in neither EVAL's enumerated
  `paths` nor DOCM's. DOCM's by authorship; announced there rather than
  drawn here.
- **Remote-session posture.** No tmux, no `cad-work`, no local
  monitors: lanes are subagents sharing this checkout (the both-ways
  hazard in `memories/agent-lane-operations.md` applies — lanes commit
  and push before the orchestrator moves the shared ref), hosted CI is
  the gate, `[ev]` PRs get a PR subscription. The orchestrator's
  state-sync rides this session's designated branch
  (`claude/work-eval-readiness-au40qs`) in place of `eval/orchestrator`.

## Unit 1 dispatched; unit 2 specified (2026-09-08)

`eval/1-affine-lift` dispatched to an implementer lane in its own
worktree against `docs/EVAL-1-SPEC.md`. `docs/EVAL-2-SPEC.md` written
while it runs. **Decision, logged (sequencing, not waited on):** unit 2
CLAIMS `D365` from DOCM — the arc-mode tag census is item 3 of the
unit exactly, on `eval/mod.rs`, EVAL's path; DOCM took the row on
2026-09-06 with the memo files before EVAL existed. The unit moves the
file; this line and the unit's PR are the announcement to DOCM
((DOCM orchestrator) — object here or on that PR and it moves back).
Measured for the spec: 96 `write_tag` sites in `eval/mod.rs`; two
source-text censuses (node, seg), four enum-function censuses (verb
content, profile verb, contact class, dimension); the "one append-only
space" sentences at `:3186`/`:3561` contradict `:3958` (43 dead in one
vocabulary, live in another). No test pins a literal key value, so the
unit's bit-identity claim is proved by a key dump at base and head.

## Units 3 and 4 specified (2026-09-08)

`docs/EVAL-3-SPEC.md` (prose-only: `emit_blend` cites `sweep::blend::
naming`'s sentences instead of restating them; the consumer's coverage
sentence, which already named one test where the kernel names two,
goes) and `docs/EVAL-4-SPEC.md` (`declare`/`declare_all` return the
`Applied` whole; `pncad-py`'s second copy of the declare body — kept
BECAUSE the sugar dropped maintenance, its comment says — collapses
back onto the sugar; `SplitOutcome` carries the remainder's
maintenance). **Announced seam to FIX ((FIX orchestrator)):** unit 4
edits `refactor.rs`'s `rem_apply` closure and `SplitOutcome`, per the
keep_out's announced-seam clause. Unit 5 (the two `Verb` types) rides
no unit 1–4 — none opens `crates/verbs`' or `profile`'s public
surface — so it gets its own small PR after unit 2.

## [ev] 2137 open: the placers over `Instances` (2026-09-08)

`work/eval/transform-refuses-a-patterns-instances-value.md` carries
the question (`needs_ev`), PR 2137 is the vehicle, announced on
DOCM's board there. Three answers; recommended: the placers are
shape-preserving over the value (`Body → Body`, `Instances →
Instances`), no payload or schema change, one `denotes_body` arm for
CHROME. Subscribed to the PR from this session. Widened from the
item's `Transform` to both placers because MSOLVE-2's walk already
admits nested patterns and `a10_a_nested_pattern_head_is_a_member`
pins `Pattern` refusing the same way.

## [ev] 2138 open: what a node's verdict log is (2026-09-08)

`work/eval/bracket-scope-is-run-op-not-the-node.md` carries the
question (`needs_ev`), PR 2138 the vehicle; shared with PROPS (the
bracket; its escalation item already schedules the mate-solve frame)
and M10 (certification keys hash the log), announced on DOCM's board.
Recommended: the node's bracket opens at the top of `eval_node` so the
pre-pass decisions are the node's; the mate solve gets a document-level
frame on `Evaluation`. Named cost: every Profile node's log grows, the
R2 pin flips, verdict-log goldens and certification keys re-baseline.
Measured for the question: the pre-pass runs before the key because it
FEEDS the key, so on a memo hit its fresh frame is dropped and the
prior's log already carries the same verdicts (D9). Subscribed from
this session. Both `[ev]` PRs (2137, 2138) are now the program's two
D rows; the E units run meanwhile.

## [ev] 2137 RULED and merged: answer 1 (2026-09-08)

Ev: "1 sounds good!" — the placers are shape-preserving over the
value. The row is the unit: `docs/EVAL-6-SPEC.md`, branch
`eval/6-placers-over-instances`, correctness arm on (it moves what two
node kinds evaluate to). Two things the spec elaborates beyond the
letter and says so: the nested-pattern LAYOUT (output body `j·M + i`,
`Instance(j)` over `Instance(i)`), which `name_pattern`'s own doc was
waiting to have ratified; and the viewer's `denotes_body`, which under
the ruling must read through the placer chain rather than judge by
kind — **announced to CHROME ((CHROME orchestrator))** in the spec, not
edited by EVAL. **To MSOLVE ((MSOLVE orchestrator)):** `a3(b)` and
`a10` in `msolve1_transform_aware.rs` flip from "refuses" to "gathers"
in EVAL-6; the spec makes the walk's (instance, copy) numbering a
STOP condition if it does not coincide with the flat index.

## [ev] 2138 RULED and merged: answer 1 (2026-09-08)

Ev: "sure 1 seems fine" — a node's log is every decision made on its
behalf; the mate solve gets the document's frame. The row is the unit
for the node half: `docs/EVAL-7-SPEC.md`, branch `eval/7-node-bracket`,
correctness arm on. **To PROPS ((PROPS orchestrator)):** the
document-level frame for `mate::solve_document` is yours, on
`escalation-channel-misses-op-minted-indeterminates` family 3 — the
ruling settles the shape (`Evaluation` carries the document's
verdicts/escalations). **To M10 ((M10 orchestrator)):**
`m10_6_certifying_keys.txt` and the accounting goldens re-baseline
when EVAL-7 lands; the PR will list each moved key. Both `[ev]` rows
are now ruled; the program's slate is seven E units (1–7) and the
standing rule `D360`.

## EVAL-1 implemented; review and EVAL-2 dispatched (2026-09-08)

PR 2139 (`bd2fe4289`): hosted CI green on the full matrix (37 jobs:
twelve `test`, five `k-lint (gate)`), no golden or frame moved, the
render lanes redrew and matched. Style review dispatched on the frozen
head; EVAL-2 implementer dispatched in parallel (two builds on the
box, accepted). Filed from the lane's sweep:
`work/issues/placement-lifts-its-affine-by-hand-beside-affine3-map.md`
(unowned file per the keep_out). The `try_map` note for PROPS is
`work/issues/affine3-try-map-the-fallible-walk-has-no-kernel-door.md`
on the PR branch, with `map-affine-retires-into-affine3-try-map`
parked on it — **to PROPS ((PROPS orchestrator)):** the geom-core door
question is yours; claim the issue file when you take it. Two things
the lane surfaced for the orchestrator: `cargo doc -p editor-core`
under `-D warnings` has 58 pre-existing intra-doc-link errors (none
in touched files; CI's rustdoc gate does not run that flag), so
EVAL-3's doc-build claim is restated as "no NEW broken links in the
touched files"; and the permission classifier refuses a Bash command
carrying the bare token `eval`, so lanes hand-write tracker files in
`work.py new`'s format when `--program eval` is needed.

## EVAL-1 review delivered; fix pass on PR 2139 (2026-09-08)

Style review: MERGEABLE, no MAJOR; claims 1–5 held (the reviewer
re-derived the twelve-component identity from the kernel's `map`
chain and ran the digest rows). Fix pass, by the orchestrator in the
lane's worktree: the reviewer's `map_affine` probe adopted as a test
(S10); `embed_profile`'s vertex point through `Point2::map` (the
dispatch correction — D368's shape nine lines below the fixed site,
which the spec had filed under D385's heading); `map_affine`'s doc no
longer schedules a future (S5); the `try_map` note moved from
`work/issues/` to `work/props/` (README: file onto the owner's slate)
with S3 (readout walks want `Affine3::components()`), S6
(`SketchPlane::try_map`) and S7 (`SketchPlane::map`'s "a caller
chooses" rule with no home) folded in; S1/S4/S8 filed as
`work/issues/profile-embed-lift-has-two-homes-anchor-and-loft.md`
(owner undecided — profile's door, EVAL's and BLEND's consumers); S2
filed onto TCOST's slate as
`m10-p-lift-interval-plane-is-sketch-plane-map-by-hand`. The parked
row's body now says EVAL retires `map_affine` in its own PR once the
door exists. **Lesson for briefs:** the first fix-pass push went red
on `discipline (evaluation-code)` — `no-extra-real-bounds` reads
`#[cfg(test)]` modules too, and the probe's helper said `T: Real +
Copy`. `scripts/gates/*.sh` run in seconds and are the pre-push
check every lane brief names from now on.

## EVAL-1 MERGED (2026-09-08, PR 2139, merge 51820f3ce)

Full matrix green on the merged head (37 checks: twelve `test`, five
`k-lint (gate)`, discipline, render lanes; four habitual skips).
`embed_affine` is gone, the plane lifts go through `SketchPlane::map`
and the vertex point through `Point2::map`; `map_affine` keeps the
fallible direction with the reviewer's identity probe pinning it to
the kernel's walk; both items closed; `D368` closed by construction.
Residue with files: `map-affine-retires-into-affine3-try-map`
(parked on PROPS' `affine3-try-map-…`), the profile embed lift twice
(`work/issues/`), `m10_p_lift`'s hand-built interval plane (TCOST's
slate), `placement.rs`'s hand lift (`work/issues/`). EVAL-3 dispatched
next (prose-only; runs beside EVAL-2's CI).

## EVAL-2 implemented; two reviews dispatched (2026-09-08)

PR 2153 (`aecc5ffda`): full matrix green (37 checks); key dump at
base and head over 1053 rows diffs to zero; acceptance grep returns
nothing; `D365` claimed by `git mv` (**announced to DOCM ((DOCM
orchestrator))** in the PR body — DOCM's `plan.md:35`/`:65` still
list it under Track V, DOCM's file to fix). Correctness and style
reviews dispatched on the frozen head; the correctness lane
reproduces the dump on documents the implementer's did not reach
(assemblies, mates) and attacks the filed residue
(`profile-program-stream-is-not-length-prefixed`: `LANE` and
`verb_tag(Cusp)` are both 41 at one grammar position, argued
unreachable). One rustdoc red on the way (an intra-doc link to a
`#[cfg(test)]` const), fixed before the run of record.

## EVAL-3 implemented, reviewed, fix pass pushed (2026-09-08, PR 2160)

Prose-only; full matrix green on `e0365893c`; doc-link location set
identical at base and head. Style review MERGEABLE, no MINOR. Fix
pass by the orchestrator (`875685d22`): the two archaeology comments
the lane filed as an item, plus a third the review added, fixed in
this PR and the item closed here (the file was heavier than the fix);
the guard's site comment collapsed to one line pointing at the module
doc (S5); `emit_chamfer` cites `sweep::blend::naming` for
one-surgery-two-verbs (S3); the item's `## Closed` trimmed to a
pointer (S8). Recorded, not built: **the mechanism behind "a retired
key is never reissued" is stated nowhere on the tree** — `topo::Body`'s
arena doc is its home (TOPO's ground; (TOPO orchestrator), one
sentence at the `SlotMap` declaration), and until it is written the
kernel's sentence and the consumer's citation are both conditionals
(S1); the kernel's `naming.rs` "What consumes these rows" paragraph
describes its consumer by unlinked path (S2, BLEND's); `wire.rs:1836`
is a third consumer-cites-a-cannot-fire shape (S7).

## EVAL-2 reviews adjudicated; fix pass to the implementer (2026-09-08)

Correctness: MERGEABLE — the key dump reproduced independently over
1074 rows (corpus at f64, bumped, `Dual64`; an assembly with mates;
a resolving `Cusp` profile) at zero diff; every census went red under
a duplicated number; the 41/41 residue is unreachable for a stronger
reason than the item gave (a `Cusp` cannot end a loop — its tip is
`DirectedIncoming` — so the word after it is a verb tag, never
`LOOP_START`). Style: MERGEABLE. Converging MINORs: the contact-class
tag has a twin in `topo::ContactClass::content_tag` (exhaustive)
beside `eval`'s wildcard function; the hand-kept `ALL`/`GROUPS` and
the `winding`/`side`/`target` closures are declared but uncensused
(a swapped arm or a dropped const stays green). Fix pass sent to the
implementer with eleven items: the twin collapses onto topo's
function; projections become censused functions; a `tag_groups!`
macro projects `ALL`/`GROUPS`; the v2…v6 archaeology becomes the
bump invariant at `tag::format::VERSION`; the census rename
completes; `presence`'s wrong sentence and the borrowed fault bit;
`u64`-written tags; the residue item's argument. **Lesson (correctness
NOTE-1):** two worktrees sharing one `CARGO_TARGET_DIR` resolve to one
artifact hash and the second reads `Fresh` — a base-vs-head proof
needs separate targets or a `touch`, and the printed package PATH is
the confirmation, not the word `Compiling`.

## EVAL-3 MERGED (2026-09-08, PR 2160, merge bb30d5a3f)

Green on the merged head (the state-sync merge brought code from
main under it, so it got its own run). `emit_blend` cites the kernel
homes of its two arguments; three archaeology comments across the
emitters state their invariants; `emit_chamfer` cites one-surgery-
two-verbs. Both the unit item and the residue item are closed. Open
residues from its review are in the previous entry (the reissue
mechanism's home at `topo::Body`, TOPO's; the kernel's
consumer-describing paragraph, BLEND's).

## EVAL-2 fix pass green; state-sync pushed (2026-09-08, PR 2153)

Fix pass `812cb3b50`: every one of the eleven items landed (contact
class through `topo::ContactClass::content_tag` at every site;
`winding_tag`/`side_tag`/`target_tag`/`split_half_tag` as censused
functions with committed-number pins and a `closed_list!` that makes
an exhaustive match the row set; `tag_groups!` projects `ALL` and
`GROUPS`, unconditional; the format-version rule at its declaration;
the census rename completed and the three census modules made one;
`tag::fault`; `u64`-written sub-tags through `write_tag`; the
residue's argument rewritten on the `Cusp` tip state). Key dump
re-taken at the moved base from a `git archive` export under its own
target: 1053 rows, zero diff. Full matrix green. Meta-read by the
orchestrator of the macro, the contact-class sites and the acceptance
grep (empty). State-sync merged main (which brought code: k-lint
tests) so the merged head runs once more before the merge.

## EVAL-2 blocked on a semantic merge with LIB-G17 (2026-09-08)

PR 2153 green at `9e3b54ed6` but unmergeable: `Node::Shell` landed
(LIB-G17) with `verb_content_tag(Shell) = Some(35)`, `RoleSeg::HoleRim`
at seg tag 44, and three new payload literals (42/43/44) in
`feed_blend` renamed `feed_scalar_join` — all inside the vocabularies
EVAL-2 declared. Resolution handed back to the implementer as a
semantic merge (structure ours, meaning theirs, every literal
declared, the key dump re-taken at the new base with a shell document
added). The `keep_out`'s announced seam (LIB-G17 reaches `crates/verbs`
and, it turns out, the content key) landed while the unit was in
review; the cost is one merge round.

## EVAL-4 implemented; style review dispatched (2026-09-08, PR 2165)

Head `eca39b5ce`, full matrix green with the python suite run
(`RUN_PNCAD_PY: true` in the filter's log). `declare`/`declare_all`
return `(Applied, id)`; `pncad-py`'s second copy of the declare body
collapses onto the sugar; `refactor.rs` (FIX seam, announced on the
PR) gains one private `Recording` swap point under `rem_apply`,
`part_apply` and inline's `step`, with `SplitOutcome` carrying both
sides' maintenance and `InlineOutcome` its own — the unit went one
door further than the spec asked, and stated why. Filed from its
survey onto the owners' slates:
`work/lib/python-split-and-inline-outcomes-drop-the-maintenance.md`
((LIB orchestrator)) and
`work/docm/replay-and-load-keep-the-document-without-its-maintenance.md`
((DOCM orchestrator)).

## EVAL-5 dispatched (2026-09-08)

Prose-only (`docs/EVAL-5-SPEC.md`): one stated convention for the two
`Verb` types. **Announced seam to S-BOOL ((S-BOOL orchestrator)):** one
mirror sentence at `crates/profile/src/path/program.rs`'s `Verb` doc;
the rename (`StepVerb`) is theirs to sequence if they want it.

## EVAL-4 reviewed; fix pass pushed (2026-09-08, PR 2165, `a95add0ae`)

Style review MERGEABLE-WITH-FIXES: every claim held (the swap point
is total over every `apply` in `refactor.rs`; four field-drop
mutations each red exactly the rows that guard the field; the Python
funnel test exercises the declare doors and ran on CI). MINOR:
`Recording`'s doc claimed to keep the record it hands back as
`minted` only — sentence fixed. Headers refreshed (module,
`SplitOutcome`, `InlineOutcome`); the A10 root-list comment now says
it is not an A11 cluster act (S2); the guide's destructure says what
it drops (S5). Declined: folding the three error-wrapping closures
into `Recording::apply` (S3 — each is one refusal vocabulary's seam,
and the reviewer called it not wrong). Recorded: "in edit order" is
by construction (one extend per accepted edit) and no row orders two
acts (S4); the apply-and-keep-the-doc shape stands at five
non-mirror sites on the tree and ten tour helpers, each resting on
"no mirror today" (S1 — the class, for whoever next adds a mirror).

## EVAL-2 MERGED (2026-09-08, PR 2153, merge ba8bc0cb4)

The semantic merge with `Node::Shell` landed as a merge commit: the
shell's three role words (`Inner` 42, `Rim` 43, `HoleRim` 44) join
`seg_content_tag` over the projected `SegTag::ALL` (44 entries);
`feed_blend` became main's `feed_scalar_join` with its word declared
as `tag::scalar_join::FLOW_EXPR`; the format version keeps its rule
(a new node kind is additive, no bump); the seg census pins the newest
words. Key dump at the new base with the two shell corpus documents
added: 1071 rows, zero diff. Full matrix green. Both items closed;
`D365` is EVAL's now (DOCM's `plan.md` still lists it — DOCM's file).

## EVAL-6 dispatched (2026-09-08)

The build of ruling 2137 (`docs/EVAL-6-SPEC.md`, on main): the placers
are shape-preserving over the value. Correctness arm on. The spec's
nested-pattern layout (`j·M + i`, `Instance(j)` over `Instance(i)`) is
the orchestrator's elaboration and the lane is told to STOP if
MSOLVE-2's member numbering does not coincide with it. CHROME's
`denotes_body` and MSOLVE's two pins were announced above.

## EVAL-5 implemented; style review dispatched (2026-09-08, PR 2168)

Head `36d0cb196`, full matrix green. The convention is stated in both
crate docs (the profile side by the announced S-BOOL seam) and nine
bare `Verb` links in EVAL's own `editor-core` files now spell the
crate — one step wider than the spec's two sites, for the reason the
lane gave (the convention would otherwise be broken inside its own
fence). Reported, not fixed: LIB's `prose_census.rs` "two `Verb`
types" sentence and `py/path.rs:24`; DOCM's `node.rs:1497`. The lane
flagged that `crates/verbs/README.md` (the ratified page) carries no
clause for the convention; the review is asked whether one is owed.

## EVAL-5 reviewed; fix pass pushed (2026-09-08, PR 2168, `7c84f6b02`)

Style review MERGEABLE-WITH-FIXES. MINOR: the two convention
sentences disagreed on scope (the profile one bound the owning
crate's own bare `Verb`) — both now say one rule: outside the owning
crate, prose spells the crate and code imports at most one of the two
per file; the rule is HELD by a test in `editor-core`'s `verbs`
module (S4: an invariant nothing enforced), which walks the crate's
sources. "Verb vocabulary" was a second collision the `\bVerb\b`
sweep could not see (S2) — EVAL's files now say whose; profile's
four are S-BOOL's. The paragraph moved to its own `# The name`
section after the crate's positive statement (S7). One parenthetical
on `crates/verbs/README.md`'s "One verb vocabulary" heading (S6; a
naming clarification with no design implication). The item's stale
reader list and the spec's wrong "already qualified" claim corrected
on the item (two bare `use` imports in `viewer` and `pncad-py`, which
the convention admits). Reported to LIB: `prose_census.rs` carries
the "two `Verb` types" sentence four times, not two. Declined: the
rename (S5) — S-BOOL's; the PR body recommends `StepVerb` to them.

## EVAL-5 fix pass: the reader ledger (2026-09-08, `2ba5435c7`)

The first fix-pass push went red on `test-utils::reader_census`: every
site that reads Rust source as text is a line in
`crates/test-utils/tests/reader_census.rs`'s ledger, and the new
convention guard read source with its own directory walk. Fixed: the
guard walks through the shared `test_utils::source::rust_sources` and
reads through `code_only`, with its ledger line (`Shared`). **Lesson
for briefs:** a test that reads `.rs` text owes a ledger line and uses
the shared walker and lexer — the census's own doc says which
dispositions are honest.

## EVAL-4 MERGED (2026-09-08, PR 2165, merge 33f0f4d20)

Green on the merged head (main had moved with code under it, so it
ran once more). `declare`/`declare_all` return the accepted edit
whole; `pncad-py`'s declare doors go through the sugar; `refactor.rs`
carries one `Recording` swap point and both outcomes carry the
maintenance their edits performed. `D367` closed. The FIX seam
(`refactor.rs`) was announced on the PR; LIB's and DOCM's follow-ups
are filed on their slates (above).

## EVAL-7 dispatched (2026-09-08)

The build of ruling 2138 (`docs/EVAL-7-SPEC.md`, on main): the node's
bracket opens at the top of `eval_node`. Correctness arm on; the
re-baselines (certification keys, accounting goldens, the R2 pin) are
the point, and the PR names each. Dispatched beside EVAL-6 (a
sequencing choice: the two touch different functions of `eval/mod.rs`
and `wire.rs`, and a slot was free); whichever merges second merges
main first. **To PROPS ((PROPS orchestrator)) and M10 ((M10
orchestrator)):** announced above at the ruling.

## EVAL-6 implemented; two reviews dispatched (2026-09-08, PR 2173)

Head `829b37e21`, full matrix green, no frame moved. The STOP
condition did not fire: the mate walk's `Member` numbering coincides
with the flat layout (one `Instance` segment per level, outermost
first; maps compose). One downstream reader does not —
`check_reference` reads a `Part` above a pattern as a structural
copy — filed onto MSOLVE's slate
(`work/msolve/part-over-a-nested-pattern-reads-the-flat-index-at-check-reference.md`,
(MSOLVE orchestrator)). The lane edited one CHROME test
(`viewer/tests/combine_ops.rs`, 17 lines) to make the pattern
candidate a named exception because the ruling made its premise
false — announced to (CHROME orchestrator) on the PR with the
`denotes_body` read-through shape; CHROME's re-pin replaces it.
Residue with its own file on EVAL's slate:
`node-value-kind-answers-a-transform-by-node-kind`. Correctness and
style reviews dispatched on the frozen head.

## EVAL-5 MERGED (2026-09-08, PR 2168, merge 7b3a1ed8e)

The convention is stated at both crate docs and held by a test in
`editor-core`'s `verbs` module (through the shared source walker,
with its ledger line); "verb vocabulary" says whose in EVAL's files;
the README heading carries one parenthetical. Item closed. Five of
the seven E units are merged; EVAL-6 is in review and EVAL-7 in
implementation.

## EVAL-6 reviews adjudicated; fix pass to the implementer (2026-09-08)

Correctness: MERGEABLE-WITH-FIXES — every code claim held under the
reviewer's own probes (rotation between the two patterns composes
`M_o · T · M_i`; two layout mutations red the right rows; a 14,541-
line base/head dump at zero diff), but the PR body's "never a wrong
placement" for the `check_reference` finding was FALSE: with the
mate read at a `Part(k)` over a nested value and `k == j`, `i ≠ 0`,
the solve, the mate node and `product_named` all pass and the wrong
copy is built; only `assemble`'s gate refuses. Reachable only since
this unit. Adjudicated as a MAJOR the fix pass closes IN the PR by
announced seam to MSOLVE (`mate/member.rs`: decompose `k` through
the layout, or refuse the nested-`Part` shape typed), with the
reviewer's four-case row and the transform-between-patterns row
adopted; the MSOLVE item corrected to the measured behaviour.
Style: a four-copy place-and-restamp class (one with an unchecked
`as u32`), five spellings of one overflow refusal, the layout
arithmetic in two homes, an unguarded provenance ordinal, and four
stale "one body" sentences the symbol sweep was blind to (EVAL's
fixed; TOPO's `source.rs`, DOCM's `node.rs` and CHROME's three
reported). Fix pass sent with thirteen items.

## EVAL-7 implemented; two reviews dispatched (2026-09-08, PR 2176)

Head `9e6f5d670`, full matrix green, no frame moved. The bracket
opens as `eval_node`'s first statement; a pre-key refusal carries its
escalations; the memo hit finishes and drops the fresh frame with a
`debug_assert!` that it is a prefix of the reused log. Moved and
tabled in the PR: the R2 pin (724/75 → 799/0), `asm2a`'s 724,
`m10_6_certifying_keys.txt` (both fixtures, re-blessed), and
`m4_pr6_eps_diff.rs`'s summary populations, which DOUBLED — the
Profile node's log now holds the pre-pass validation AND the pinned
lift's op re-validating the same `Profile<f64>` (`pre[6..] == op`).
That doubling is honest under the ruling and is a design question in
its own right (skip the re-validation, or not); filed by the lane as
`profile-node-log-holds-the-f64-validation-twice-under-the-pinned-lift`.
The accounting goldens did not move (masses, not hashes). Reported
for DOCM: `PartCache::get`'s shield doc still lists the profile
pre-passes among what it catches. Correctness and style reviews
dispatched on the frozen head.

## EVAL-7 reviews adjudicated; fix pass to the implementer (2026-09-08)

Correctness: MERGEABLE-WITH-FIXES — every claim reproduced (780 rows
at f64 and Interval, 130 moved and all Profile; the re-blessed keys
reproduced byte-equal from `VerdictVector::of`); the MAJOR is a
PRE-EXISTING memo hole the hit-site argument leaned on: at the
interval scalar the content key hashes a slot's interval bits while
the pre-pass and the pinned op read its nominal f64, so one box with
two nominals hits the other's memo (reproduced on the base). Fix pass:
the comment says what is true, and the hole is filed on EVAL's slate
and scheduled as unit 9. Style: the hit-path guard was debug-only
with no row able to see it removed — it becomes a full assert with a
should-panic row; the doubled validation under the pinned lift is
decided for this PR as an honest statement (the Pinned arm says why it
re-validates; `vdiff`'s doc says the populations are doubled) with
the fix scheduled as unit 8; the pre-op row's misdescribed mechanism
corrected and the real frame-open-no-decision row adopted; the m4
golden rule's "once" corrected. Plan updated with units 8 and 9.

## EVAL-6 fix pass green; state-sync pushed (2026-09-08, PR 2173)

Fix pass `c9d4781fe`: the `check_reference` hole closed by
DECOMPOSITION in `mate/member.rs` (MSOLVE's, by announced seam):
each level of the copy chain is collected outermost-first, and a
`Part`'s index is compared to its level's copy folded through every
level below it down to the next `Part`, via the evaluator's own
`names::flat_body_index`; a `Part` over `Transform(Pattern)` is now
checked too. The reviewer's four-case row and the rotation-between-
patterns row adopted verbatim. Style items all landed: one `place`
site (the placed union's `as u32` now typed), one overflow home
(`names::output_body`), one layout function shared by the evaluator,
the emitter and the walk, `body_operand` through `placeable_operand`,
`Placeable::map`, the ordinal rule stated once at `compose_placed`
with a pairwise-distinct-stamps row, the stale "one body" sentences
fixed in EVAL's files and `docs/MIRROR-DESIGN.md`. Dump re-taken at
the new base: 14,542 lines, zero diff. Meta-read by the orchestrator
of the decomposition. State-sync merged main (code moved: a run on
the merged head before the merge).

## EVAL-6 MERGED (2026-09-08, PR 2173, merge 414b95524)

Green on the merged head. The placers are shape-preserving over the
value; the nested layout is placement-major with one layout function
shared by the evaluator, the emitter and the mate walk; the
`check_reference` hole closed by decomposition; MSOLVE's two pins
gather. Item closed; residue on EVAL's slate:
`node-value-kind-answers-a-transform-by-node-kind`. CHROME's
`denotes_body` read-through and MSOLVE's remaining index-space
account are filed on their slates. EVAL-7's state-sync merged this
in before its own run.

## EVAL-7 fix pass green; state-sync pushed (2026-09-08, PR 2176)

Fix pass `7e7bb8361`: the hit-site argument made true (a `Verdict` is
`(predicate, sign)`; the key fixes what the pre-pass decides from at
f64) with the Interval hole named and filed
(`interval-content-key-hashes-bits-the-pre-pass-does-not-read`,
scheduled as unit 9); the guard a full `assert!` with a should-panic
row; the pre-op rows told apart (the D4 door refuses before
`eval_node`; a new `Expr`-refusal row is the frame-open-no-decision
case); the doubling stated at the Pinned arm and in `vdiff`'s doc
with the fix scheduled as unit 8; the m4 golden rule corrected; the
band formula through `Band::linear`. Declined: the inner-`Result`
refactor of the nine `fail(bracket, ..)` sites (small diff kept, each
refusal finishing at its own line). State-sync merged main (with
EVAL-6) and runs once more before the merge. **Process note:** a
persisted `cd` from one Bash call put the next call's log append into
the lane's worktree as a stray commit — caught before it was pushed
(the lane's branch was reset to its pushed head; the entry re-made
here). Every orchestrator command now starts with an absolute `cd`.

## EVAL-7 MERGED (2026-09-08, PR 2176, merge 43116253b)

Green on the merged head. A node's log is every decision made on its
behalf: the bracket opens at the top of `eval_node`, a pre-key refusal
carries its escalations, the memo hit's fresh frame is asserted a
prefix of the reused log (a full assert with a should-panic row); the
certification keys and the eps-audit populations re-baselined by
ruling and tabled in the PR. **All seven E units of the opening slate
are merged.** Units 8 and 9 (EVAL-7's residue) dispatch now:
`docs/EVAL-8-SPEC.md` (validate once under the pinned lift; the
validated form's lift is `crates/profile`'s door — **announced seam to
S-BOOL ((S-BOOL orchestrator))**; M10's keys re-baseline once more)
and `docs/EVAL-9-SPEC.md` (the nominal joins the key; the profile
stream's counts ride the same format bump, closing EVAL-2's residue;
**to DOCM ((DOCM orchestrator))**: `memo.rs`'s slot hashing untouched,
the feed lives at `content_key`).

## Units 8 and 9 dispatched (2026-09-08)

Both in their own worktrees off the orchestrator branch, in parallel
(different files: unit 8 on `wire.rs`'s `Pinned` arm, `anchor.rs`,
`crates/profile` and the kstats/m4/m10_6 rows; unit 9 on
`content_key`, `mod tag` and a new interval row); whichever merges
second merges main first. Correctness arms on both.

## EVAL-9 lane relaunched (2026-09-08)

The first EVAL-9 lane ended at launch on a model-side safeguard
error before touching the branch (no commits, worktree removed);
relaunched fresh with the same brief.

## Both lanes interrupted by the session's rate limit (2026-09-08)

At 08:30Z the session hit its API limit and both implementer lanes
ended mid-turn. EVAL-8 had already pushed and opened PR 2186 (head
`4d07e6360`); its matrix came back green except `discipline
(evaluation-code)`: `check-interval-cfg-additive.py` refuses an
interval `cfg` gating a block inside `kstats_bracket_rows.rs`'s new
log-identity row (a row present in both builds must run identical
code; the interval leg becomes its own `#[test]`). EVAL-9 had three
commits on its branch and no PR. Both lanes resumed from their
transcripts once the limit reset, the EVAL-8 one with the split as
its first job. Lesson for every brief: the interval-additivity
script joins the gate list a lane runs before pushing — it is a CI
gate the local `scripts/gates/*.sh` sweep does not cover.

## EVAL-8 to review (2026-09-08, PR 2186, head `6e1cb1250`)

Full matrix green on the head (the first head's one red — the
interval-additivity gate — fixed by splitting the pinned-log row's
interval leg into its own gated row). The lane's deviations, to be
adjudicated with the reviews: (1) the lift RE-DERIVES arc carriers at
the target scalar through a factored `seg::arc_carrier` rather than
mapping the f64 carrier's bits — mapped bits are a point interval at
`Interval` where validation mints an enclosure, measured on the
rounded-rectangle fixture, so the spec's "every stored scalar maps"
premise was wrong for derived data; (2) `m10_6_certifying_keys.txt`
does not move because its rows evaluate under `Guided`; (3) claim 2
is shown by cross-scalar equality plus the `Guided` log's prefix rather
than a pre-pass re-run outside the evaluator. One open question the
lane flags: a `Dual64` derivative channel is `+0.0` from the lift and
`-0.0` from a re-validation of a reversed loop (equal, zero, read by
no predicate). Style and correctness lanes dispatched on `6e1cb1250`;
the correctness lane is asked whether any consumer reads a zero
derivative's sign.

## EVAL-9 to review (2026-09-08, PR 2190, head `f6aa77504`)

Full matrix green. The lane's main deviation is a correction to the
spec: its item 1 (a uniform nominal feed at every lane) and its claim 2
("under a forced format 6 the f64 and `Dual64` keys are identical to
the base") cannot both hold — a written word moves a hash. The lane
kept the uniform rule and proved what claim 2 was after in the form
that survives it: per pass the base-key → forced-key map is a
bijection (no split, no merge), so the nominal adds a word and no
information at f64/`Dual64`; the count words move every profile
node and its cone by design. The spec was wrong on that claim and the
lane's reading is adopted. The probe row is red on the base with the
item's exact numbers and green on the head; the f64 cost is below the
corpus noise floor. New residue on EVAL's slate:
`nominal-environment-is-rebuilt-per-node-in-wire.md` (`wire.rs`
builds the f64 environment per profile-bearing node although the
evaluation now holds one). Style and correctness lanes dispatched on
`f6aa77504`.

## EVAL-8 reviews adjudicated; fix pass dispatched (2026-09-08)

Correctness: all five claims reproduced with the reviewer's own base
build and dump (2262 rows, 261 moved, every one `Pinned`, zero
`Guided`, zero value digests); no MAJOR. Style: two MAJORs, both
upheld. (1) The door's doc argues the lift needs no re-check because
`from_f64` preserves every predicate margin — false at `Interval`,
where outward rounding makes a margin an enclosure and
`sign_within` can escalate where f64 decided definitely. **Ruling:**
the lift is right BY DESIGN, not by predicate agreement:
`ProfileLift`'s own doc makes the pinned lift the build path where
"structure must be selected once, identically for every lane" and
the guided lift the one that re-verifies and refuses; the doc is
restated on that basis, the PR body names the behaviour change (a
profile marginal at `Interval` was refused by the pinned op's
re-validation before and now carries f64's verdicts; `Guided` is
where such a margin escalates), and the lane attempts one fixture
that pins it. (2) A public generic `map` admits every map the doc
then argues away; the door closes by type —
`ValidatedProfile<f64>::lift_onto<U>(plane)` with `from_f64` inside,
retiring `with_plane` (whose one caller discarded the plane `map`
had just lifted) and the spec's `map` name; the spec's own premise
("any injective map preserves the invariants") was false — a
reflection is injective — and is recorded as such. Minor fixes:
`arc_carrier` takes the chord frame instead of recomputing it; the
value-identity row's plane column compared the lifted plane to
itself; a stale "ahead of the lane validation" sentence at the memo
hit; the eps-audit golden rule narrating three re-pins; the README
V6 sentence; one hole-before-outer fixture and per-segment
`blend_arcs`; the PR body owes the derivative-channel exclusion
(`-0.0 → +0.0` on reversed loops, no reader of a zero's sign found)
and the full historical-mention list. Orchestrator's edit:
`work/issues/profile-embed-lift-has-two-homes-anchor-and-loft.md`
narrowed — `loft::end_profile` both lifts AND re-validates an exact
lift (this unit's class one crate over, **for BLEND/S-BOOL
((S-BOOL orchestrator))**), and two test copies of the raw lift
remain.

## EVAL-9 reviews adjudicated; fix pass dispatched (2026-09-08)

Correctness: the probe's red-on-base established by a knockout of the
one nominal word (the item's exact numbers), the box arithmetic read
(`point(0.0) + [-0.25, 0.25]` and `point(0.5) + [-0.75, -0.25]` are
the same exact interval), every memo row green in both builds, the
sweep complete; no MAJOR. Two corrections it adds: the bijection
table is a determinism check (0-merged by construction, 0-split on a
box-free corpus) and the "adds no information at f64/`Dual64`" claim
rests on the environment construction in code; and "at f64 the bits
ARE the nominal" holds only under no box or a zero box — a degenerate
offset box (`Varying{lo: c, hi: c}`, accepted by `f64::axis`) shifts
the lane off the nominal, the base had that hole at f64 too, and the
uniform feed closes it there as well. Style: five MAJOR-class, upheld.
**Ruling on the shape:** the ninth `eval_node` argument defended by a
paragraph committed after the code is the rationalization shape; the
nominal environment rides `LaneEnv` beside `params` (the carrier the
lane's own residue file names), the nominal slot values come through
the same `eval_slots` door as the lane values, and a refusing nominal
fails the node typed as every other nominal reader does — the
`REFUSED` word retires and a row reaches the typed refusal under an
offset box. `program::NONE`'s retirement is recorded by a census row
(the first retirement inside a `tag_groups!` group). The rule keeps
one home at `tag::slot` with its exceptions stated there. **To DOCM
((DOCM orchestrator))**: `memo.rs`'s header defines the key without
the nominal and is now incomplete; untouched by EVAL. **For the exit
walk / Ev**: `docs/DESIGN.md`'s list of the content key's inputs
(~1071) needs the nominal added — a DESIGN revision, carried to the
`[ev]` PR.

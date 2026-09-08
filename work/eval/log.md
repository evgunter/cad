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

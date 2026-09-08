# EVAL exit walk — criteria vs evidence

**STATUS: PROPOSED — awaiting Ev's ratification** (this `[ev]` PR). EVAL =
the evaluation seat (`work/eval/plan.md`, opened 2026-09-06 from
`docs/WORK-TRACKS-2026-09.md` addendum 2; orchestrated 2026-09-08 on a
remote box). Every unit on the slate is merged: the plan's five E
openers, the two `[ev]` rulings and the two builds they became (units 6
and 7), the two units unit 7's reviews grew (8 and 9), and the two S
units the slate's own residue grew at its end (10 and 11) — eleven PRs,
no A/B rows (the band 3000–3099 was claimed for bookkeeping and stays
unused). Criteria are quoted **verbatim** from `work/eval/plan.md`.
Dispositions per the M5–M9 convention: MET /
MET-WITH-RECORDED-HONESTY / CARRIED (named owner). On ratification this
file is deleted with `work/eval/` and recorded in `docs/DOC-LEDGER.md` at
the SHA it is recoverable at; the residue table at the end names where
every open item goes.

## The walk

| # | Criterion (verbatim from plan.md) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | E1: "`affine-lift-has-a-second-home-in-anchor-embed-affine` + `D368` — one lane in `eval/anchor.rs`: `embed_affine` and its two callers retire into `SketchPlane::map`; the hand-lifted `Vec3` the row cites is the walk inside `map_affine`, whose `from_f64` instance WAS `embed_affine`, so `D368` closes by construction; whether the fallible direction wants a kernel `try_map` is put to PROPS by note, and `map_affine` stays local, parked on that answer." | MET | PR 2139 (merge `51820f3ce`). `embed_affine` gone; the plane lifts through `SketchPlane::map`, the vertex point through `Point2::map`; `map_affine` keeps the fallible direction with an identity probe pinning it to the kernel's walk. `D368` closed by construction. PROPS asked by file (`work/props/affine3-try-map-the-fallible-walk-has-no-kernel-door.md`, with the review's S3/S6/S7 folded in); `map-affine-retires-into-affine3-try-map` parked on it. Filed from the review: the profile embed lift twice (`work/issues/`), `placement.rs`'s hand lift (`work/issues/`), `m10_p_lift`'s hand-built interval plane (TCOST). |
| 2 | E2: "`node-tag-space-census-blind-to-tags-outside-sentinels` — the tag space declared once as a closed enum with `ALL`, the sites reading from it, the sentinels retired." | MET (the closed-enum shape became a declaration macro; the node-kind census kept its sentinels by ruling) | PR 2153 (merge `ba8bc0cb4`). Every tag vocabulary declared through `tag_groups!` projecting `ALL`/`GROUPS`, `seg_content_tag` over `SegTag::ALL`, the group census over the projection; `D365` moved from DOCM's slate and closed with it. Semantic merge with LIB-G17's `Node::Shell` (three role words, `feed_scalar_join`) landed as a merge commit; key dump 1071 rows, zero diff. Correctness arm ran (the key dump). |
| 3 | E3: "`emit-blend-restates-the-kernels-own-arguments` — the two re-derivations in `emit_blend.rs` replaced by citations of the kernel type that owns each; the coverage sentence made to agree with the kernel side." | MET | PR 2160 (merge `bb30d5a3f`), prose only. The two arguments cite `sweep::blend::naming`'s homes; the consumer's coverage sentence dropped; three archaeology comments across the emitters rewritten as invariants and `emitters-carry-pr-and-unit-archaeology-in-comments` closed in-PR. Reported, not edited: the reissue mechanism's home at `topo::Body` (TOPO), the kernel's consumer-describing paragraph (BLEND). |
| 4 | E4: "`D367` — `declare_all` and `rem_apply` through one accept funnel so `applied.maintenance` is returned rather than dropped (`refactor.rs` is FIX's, by announced seam)." | MET | PR 2165 (merge `33f0f4d20`). `declare`/`declare_all` return the accepted edit whole; `pncad-py`'s declare doors go back through the sugar (its duplicate body and the comment excusing it deleted); `refactor.rs` carries one `Recording` swap point and both outcomes carry their maintenance. FIX seam announced. Filed on their slates: `work/lib/python-split-and-inline-outcomes-drop-the-maintenance.md`, `work/docm/replay-and-load-keep-the-document-without-its-maintenance.md`. |
| 5 | E5: "`two-public-verb-types-verbs-and-profile` — a stated convention in both crates' module docs, or the rename on the profile side (S-BOOL's glob, announced); rides whichever unit next opens either surface, and this is the first." | MET (the convention, not the rename) | PR 2168 (merge `7b3a1ed8e`). The convention at both crate docs (the profile side by announced seam to S-BOOL, one sentence), held by a guard row in `editor-core`'s `verbs` module through the shared source walker with its `reader_census` ledger line. |
| 6 | D6: "`transform-refuses-a-patterns-instances-value` — does `Transform` (and the other single-body placers) accept `Instances`, or is the fence intended and the mate walk refuses the shape in its own voice. A small PR either way once ruled." | MET (ruled on `[ev]` PR 2137 → the EVAL-6 unit) | Ev: "1 sounds good!" — the placers are shape-preserving over the value. EVAL-6: PR 2173 (merge `414b95524`), correctness arm on: `Transform`/`Pattern` over `Instances`, the nested layout placement-major (`j·M + i`) through one layout function shared by the evaluator, the emitter and the mate walk. The correctness review found a silent wrong-copy placement (`Part(k)`, `k == j`, `i ≠ 0`) closed in-PR by decomposition in `mate/member.rs` (MSOLVE seam). Announced to CHROME (`denotes_body` reads through the placer chain — `work/chrome/body-seat-reads-through-the-placer-chain.md`) and MSOLVE (`work/msolve/part-over-a-nested-pattern-reads-the-flat-index-at-check-reference.md`). |
| 7 | D7: "`bracket-scope-is-run-op-not-the-node` — what a node's log means: the profile pre-pass and the mate solve decide before any bracket opens. A design choice made with the pre-pass's owner; the build moves every Profile node's log and the verdict-log goldens." | MET (ruled on `[ev]` PR 2138 → the EVAL-7 unit; the mate solve's document frame to PROPS) | Ev: "sure 1 seems fine" — a node's log is every decision made on its behalf. EVAL-7: PR 2176 (merge `43116253b`), correctness arm on: the bracket opens at the top of `eval_node`, a pre-key refusal carries its escalations, the memo hit's fresh frame asserted a prefix of the reused log; certification keys and the eps-audit populations re-baselined by ruling and tabled. The review found a pre-existing memo hole (row 9) and the doubled validation under the pinned lift (row 8) — both scheduled as units. The document-level frame for `mate::solve_document` is PROPS's (announced). |
| 8 | "8. `profile-node-log-holds-the-f64-validation-twice-under-the-pinned-lift` — EVAL-7's residue, scheduled: under the pinned lift at the build scalar the op re-validates the `Profile<f64>` the pre-pass already validated, so the node's log (and `vdiff`'s populations) hold each decision twice. The unit makes the Pinned arm reuse the pre-pass's validated form (the validated type's lift is `crates/profile`'s door, S-BOOL's glob, announced) and the m4 eps-audit populations return to single counts." | MET-WITH-RECORDED-HONESTY | PR 2186 (merge `51729fc1d`), correctness arm on. `ValidatedProfile<f64>::lift_onto(plane)` in `crates/profile` (S-BOOL seam: the door, `seg::ChordFrame`/`arc_carrier` factored bit-identically out of `build_seg`, the README V6 sentence, `tests/validated_map.rs`, one paragraph dropped from `resolve/vdiff.rs`); the Pinned arm lifts the pre-pass's form; `anchor::embed_profile` retired; the Profile log is the pre-pass's 75 at every scalar; the m4 populations byte-identical to their pre-EVAL-7 literal; `asm2a` 799 → 730. The spec's two premises were wrong and the lane corrected both: an arc's carrier is DERIVED data (mapping its f64 bits gives a point interval where validation mints an enclosure), and "any injective map preserves the invariants" is false (a reflection is injective). The style review's MAJOR: the door's doc argued predicate agreement, which is false at `Interval` (outward rounding can escalate where f64 decided) — ruled that the lift is right BY DESIGN (`ProfileLift`'s "structure is selected once, at f64, identically for every lane"; `Guided` re-verifies), the doc restated on that basis, the door closed by type, and the one behaviour change (a margin definite at f64 and indeterminate at `Interval` is served under `Pinned` with the f64 log and refused under `Guided`) pinned by a row. |
| 9 | "9. `interval-content-key-hashes-bits-the-pre-pass-does-not-read` — EVAL-7's review found the memo hole (pre-existing): at the interval scalar the key hashes a slot's interval bits while the pre-pass and the pinned op read its nominal f64, which is not in the key, so two documents with one box and different nominals hit each other's memo. The nominal f64 of each slot joins the key (a format bump); the review's probe is the red-first row. The file lands with EVAL-7's fix pass." | MET-WITH-RECORDED-HONESTY | PR 2190 (merge `5c9f167f9`), correctness arm on. Every slot feeds its nominal f64 beside its lane bits under `tag::slot`'s word, both lists through one door (`slots::eval_slots` at the lane and at the nominal environment, carried on `wire::LaneEnv::nominal`), a refusing nominal refuses the node typed; format 6 → 7; the profile stream's loop and step lists length-prefixed and `program::NONE` retired with its number recorded dead by the census (EVAL-2's residue `profile-program-stream-is-not-length-prefixed` closed with it). The probe red on the base with the item's exact numbers. The spec's claim 2 ("forced-format-6 keys identical to base at f64/`Dual64`") contradicted its own item 1 (a uniform feed writes a word) — the lane kept the rule and proved a per-pass bijection instead; the correctness review then showed the bijection is a determinism check, the information claim rests on the environment construction in code, and "at f64 the bits ARE the nominal" holds only under no box or a zero box (a degenerate offset box was a hole at f64 too, now closed). The style review's MAJORs — a ninth `eval_node` argument defended by a paragraph committed after the code, a third spelling of slot evaluation inside the hasher — were rebuilt as ruled. `eval/memo.rs` untouched (DOCM's; its header now defines the key without the nominal — told). |
| 10 | "10. `nominal-environment-is-rebuilt-per-node-in-wire` — EVAL-9's residue: `profile_plane_f64` and `section_of` rebuild the nominal f64 environment per node although the evaluation carries one on `LaneEnv::nominal`; threading, one evaluator site left." | MET | PR 2194 (merge `5704d502a`). `section_of` resolves at `lane.nominal`; `profile_plane_f64` takes the environment and reads the frame's slots through `slots::eval_slots` (the closure's per-axis `MissingSlot` was a backstop no frame reaches); one `frame_from_slots` shared by the `T` and f64 frame reads; the census of f64-pinned readers lives once at `LaneEnv::nominal`; `doc.param_env::<f64>()` has one evaluator site; a default-build row pins that a section resolves at the nominal under an f64 offset box; `wire.rs`'s archaeology comments rewritten as invariants. The review's Q7 residue (the frame's f64 placement re-derived per profile from values the frame's own evaluation held) and its MSOLVE finding (the solve rebuilds the environment twice per mate and twice per cluster edge) are filed: `work/eval/frame-f64-placement-is-re-evaluated-per-profile.md`, `work/msolve/mate-solve-rebuilds-the-nominal-environment-per-check.md`. |
| 11 | "11. `node-value-kind-answers-a-transform-by-node-kind` — EVAL-6's residue: the recipe-side family word reads a transform through its input, so a transform of a pattern is "instances" on both roads; the one caller in `mate/member.rs` is MSOLVE's line, announced." | MET | PR 2195 (merge `cf2f67c6d`). `node_value_kind(doc, node)` walks the transform chain then answers from the table, returns a typed `MissingInput` for a dangling input, and the value-family words have one home (`eval::family`) shared by `kind_name`, `node_value_kind` and `body_operand`. The transform-of-pattern row was red on the real base (found "body" against "instances"). MSOLVE seam: one line. Filed: `work/msolve/axis-datum-names-the-pattern-where-the-evaluation-names-the-transform.md`, `work/eval/wire-expected-phrases-spell-family-words-as-literals.md`, and to DOCM `work/docm/node-placer-field-docs-say-body-where-instances-are-accepted.md`. |
| 12 | "Standing, not units: `D360` (sweep topo refusal enums by variant name; a rule this program's lanes read first)." | MET (as a rule read; re-homed below) | Every lane brief carried a sweep "shaped differently" clause and every review re-ran the unit's sweep by another shape; the rule's subject is `topo`'s refusal enums, so it goes to TOPO's slate at the sweep. |
| 13 | "Deferred with its ratification cited: `two-verb-seats-do-not-compose` (#1345 items (2)/(3), `crates/verbs/README.md` §5), which reopens when a replay consumer arrives." | CARRIED (`work/issues/`, still deferred) | No replay consumer arrived under the program; EVAL-5 stated the two `Verb` types' convention without touching the seat question. |
| 14 | Exit shape: "The five E units land, the two rulings are answered and their builds land, the deferred row is either reopened by a consumer or still deferred; the walk convention applies." | MET-WITH-RECORDED-HONESTY | Rows 1–5 (E), 6–7 (the rulings and their builds), 8–9 (the units the rulings' builds grew), 10–11 (the residue units), 13 (still deferred). The program's ground reverts to no owner at close — the honesty row below asks whether that is the right shape. |

## Honesty rows

1. **The session's rate limit killed both running lanes once** (08:30Z,
   units 8 and 9 mid-turn). Nothing substantive was lost — EVAL-8 had
   pushed and opened its PR, EVAL-9 had three commits — and both lanes
   resumed from their transcripts once the limit reset. Recorded in the
   log; the check-in cadence carried the program across it.
2. **Three pushes went red for skipped pre-push checks**, all on
   unit 8 and all from the same lane reading a truncated summary: the
   interval-additivity gate (an interval `cfg` on a block inside a
   test), then clippy on imports used only by the interval-gated row.
   Every later brief names the additivity script and clippy in BOTH
   feature sets, and "verified" means the untruncated output was read;
   units 9–11 pushed green first time.
3. **Three specs carried a false premise the lane found**: EVAL-8's
   "every stored scalar maps" (an arc carrier is derived) and "any
   injective map preserves the invariants" (a reflection); EVAL-9's
   claim 2 against its own item 1; EVAL-11's guess at which predicate
   escalates first at a band edge. Each was corrected by the lane,
   recorded as a deviation in the PR body, and adjudicated in the
   lane's favour — the orchestrator's spec is binding on scope, not on
   a premise the tree refutes.
4. **The correctness arm earned its place**: EVAL-6's found a silent
   wrong-copy placement; EVAL-7's found a pre-existing memo hole at the
   interval scalar (row 9) and the doubled validation (row 8); EVAL-8's
   reproduced every claim with its own base build; EVAL-9's turned a
   bijection table from "evidence" into "a determinism check" and found
   the degenerate-box hole at f64. No correctness claim was accepted
   from a PR body.
5. **One behaviour change under the pinned lift at `Interval`** (row 8):
   a profile marginal at `Interval` was refused by the op's
   re-validation before and now carries f64's verdicts; `Guided` is where
   such a margin escalates. Ruled consistent with `ProfileLift`'s own
   contract, pinned by a row, stated in the PR body and the door's doc.
6. **The orchestrator's state-sync rode the harness's designated branch**
   (`claude/work-eval-readiness-au40qs`) in place of `eval/orchestrator`,
   with Ev's go ("eval/ branches are fine"); unit branches followed the
   `eval/<n>-<slug>` convention. Unit branches were cut from the
   orchestrator branch for units 1–9 (so their PRs carried the board's
   state-sync commits) and from `main` for units 10–11.
7. **One lane lost its shell to the orchestrator**: the EVAL-9 worktree
   was reclaimed while its lane was still polling CI after the merge;
   its report arrived without the last two housekeeping steps. The
   rule since: a worktree is reclaimed only after the final report.
8. **Two design records are owed by this PR**, both from row 9:
   `docs/DESIGN.md`'s Band 1 sentence listing the content key's inputs
   gains the slot nominal (the one-line revision in this PR), and
   `eval/memo.rs`'s header (DOCM's) defines the key without it — told
   to DOCM, not edited.
9. **The reviews' whole-file readings say the seat's two big files have
   grown past one reading each**: `eval/mod.rs` is the evaluation driver
   and, from about line 2900, the content-key vocabulary with its
   census (two files glued); `wire.rs` holds a ~700-line
   union-declaration routing subsystem beside the wiring it is named
   for. Neither is a defect; both are the shape a successor program on
   this ground would open with.
10. **The ground reverts to no owner.** EVAL was opened because the
    evaluation seat had been nobody's since SEAT's sweep; closing it
    returns the seat to that state, with five items on `work/issues/`
    (the residue table) as a successor's opening slate — the FILLET
    precedent. The alternative is to keep EVAL open as the seat's
    standing owner with a thin slate. **Recommendation: close**; the
    slate is small and S-sized and the `[ev]`/announce mechanics work
    without an owner, as this program's own seams showed. Ev's call at
    ratification.

## Residue re-homed before the sweep

Every open item on the slate at exit, with its new home (moved by `git mv`
with its header's program edited, ids kept):

| item | home | why |
|---|---|---|
| `D360` | `work/topo/` | a standing sweep rule over `topo`'s refusal enums — TOPO's ground |
| `map-affine-retires-into-affine3-try-map` (parked) | `work/props/` | parked on PROPS's `affine3-try-map-the-fallible-walk-has-no-kernel-door`; the door and its adopter belong together |
| `two-verb-seats-do-not-compose` (deferred) | `work/issues/` | the verb seat has no live program; deferred with its ratification cited, reopens when a replay consumer arrives |
| `frame-f64-placement-is-re-evaluated-per-profile` | `work/issues/` | a unit with a correctness arm on the evaluation seat — the successor's opening slate, beside `profile-embed-lift-has-two-homes-anchor-and-loft` and `placement-lifts-its-affine-by-hand-beside-affine3-map` already there |
| `wire-expected-phrases-spell-family-words-as-literals` | `work/issues/` | same ground, S |

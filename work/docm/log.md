# DOCM log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/docm/plan.md`. A/B band 1800–1899
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose DOCM section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `capend-top-bottom-contradicted-by-negative-extrude` from `work/issues/`
- `fused-step-slot-aliases-arrival-spec` from `work/issues/`
- `sketch-frame-from-face` from `work/issues/`
- `add-profile-mints-no-frame` from `work/issues/`
- `add-profile-placement-on-picked-face-frame` from `work/issues/`
- `split-side-and-pattern-instance-as-operand` from `work/issues/`
- `no-docedit-splices-a-deleted-node` from `work/issues/`
- `document-seam-no-in-session-change-detection` from `work/issues/`
- `layer3-recipenodeid-aliases-across-rewinds` from `work/issues/`
- `no-persistent-setplacement-session-op` from `work/issues/`
- `revolve-pole-export-interior-on-axis-vertex` from `work/issues/`
- `check-registry-gathers-product-twice` from `work/issues/`
- `save-a-copy-duplicate-id-bricks-store` from `work/lib/`
- `memo-admission-and-resolver-state` from `work/lib/`
- `instantiation-seam-drops-mate-identity` from `work/mate/`
- `no-door-mints-mate-frame-from-face` from `work/mate/`
- `certify-locally-valid-range-instead-of-sampling` from `work/m10/`
- `C6` from `work/code-quality/`
- `D365` from `work/code-quality/`
- `D366` from `work/code-quality/`
- `debug-in-prose-residue-after-finding-sink` from `work/code-quality/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## The two conversations, ratified in-chat (2026-09-04)

Both axes ran as one chat conversation with Ev and are recorded as
`docs/DOCM-REFERENCES-DESIGN.md` (DM1–DM6) and
`docs/DOCM-IDENTITY-DESIGN.md` (DI1–DI5); the PR carrying them is the
record, companion rows added to `docs/DESIGN.md`. Decisions made
unilaterally on the way: reading Ev's free-move answer as the stronger
form (DI5, release is the edit) — stated in the doc, stands unless
corrected; parking splice on `DOCM-3` rather than closing it; closing
`sketch-frame-from-face`, `memo-admission-and-resolver-state` and
`split-side-and-pattern-instance-as-operand` with pointers at their
units; converting the three E-class issues to units in place (ids
kept). Seven items re-homed to CHROME (5), VIEW (1) and LIB (1) with a
signed entry in each log. Question 1 collapsed out of the plan (already
ruled). S-MATE's closing sweep is PR 1786, which opens `mate.rs` and
`assembly.rs` here. Slate now: `DOCM-1`–`DOCM-4` (spec), the three
E-class units, and the rest as the plan lists them.

## First dispatch (2026-09-04)

`DOCM-3` dispatched on `docm/3-union` against `docs/DOCM-3-SPEC.md`
(pre-draw fields L / STRUCTURAL at the spec; block DOCM-B1's draw record
is branch-side on `docm/b1-block` until the block concludes). Chosen first
because it retires the die's chain, which every later naming-sensitive
unit would otherwise inherit. The review ordinal is claimed on main at
review dispatch, per the log's standing rule.

## Second dispatch (2026-09-04)

PR 1789 merged on Ev's sign-off ("1789 looks good"), so DI5's stronger
reading stands. `DOCM-4` dispatched on `docm/4-evaluation-identity`
against `docs/DOCM-4-SPEC.md` (pre-draw M / STRUCTURAL at the spec),
concurrently with `DOCM-3` on disjoint files; the fence is in the spec's
Constraints. Node-kind units (`DOCM-1`, `DOCM-2`) wait for `DOCM-3` to
land, so two lanes never mint content-key tags at once.

## DOCM-3 stop clause fired; DM4's key amended (2026-09-04)

The lane stopped at the measured site: the die's 21 pips are
`Transform`s of ONE revolve, a transform is pass-through under N1, so
the 21 members carry identical name tables and `FromMember(inner)`
collides (pinned executably on the lane's branch). Orchestrator ruling,
a faithful elaboration of DM4's stated intent (identity-keyed,
position-free), so recorded and merged without an `[ev]` round and
reported to Ev in chat: the segment keys on the MEMBER EDGE —
`FromMember { member: RecipeNodeId, of }` — the `Instance { i, of }`
shape with an identity where it has an index. DM5 gains its third
caller, the load validator, since a snapshot never passes an edit door
(the lane's finding). Spec amended on the lane branch; the lane resumes
as the same arm. Disclosed and accepted: `pncad-py`'s exhaustive
`EditError` tag mirrors forced three rows (the S4 shape `C6`/`D366`
carry); `SetMembers` generalized over `Loft` through `Node::list_input`.

## DOCM-4 MERGED (2026-09-04, PR 1808)

Ordinal 1800, sample #126, block DOCM-B1 slot 1. Implementer arm OPUS.
v6 dual on frozen `e8d0022d`: R1 (fable) 0/3/4 APPROVE-WITH-FIXES,
R2 (opus) 3/4/4 NOT-MERGEABLE-AS-IS. Adjudicated: two bilateral MAJORs
(the `pncad-py` pages for `placement` and `product` contradicting the
new A4; A4's "every door" universal false against six doors) gated and
were fixed; R2's third MAJOR (the `tree.rs` edit inside DOCM-3's fence,
forced by an exhaustive match, disclosed to the orchestrator but not in
the PR) adjudicated MINOR, disclosure added. Unilateral-MAJOR tally +0
(the fenced-file finding is process-class, excluded by v6 item 3b).
Fix pass: nine items, all fixed; the prose fixes re-verified by the
orchestrator on the head rather than by a re-review round. Filed at
adjudication: `pair-doors-outside-the-three-do-not-check-document-identity`.
Spec `docs/DOCM-4-SPEC.md` deleted at merge (DOC-LEDGER). Two spec
errors were the orchestrator's (three constructor sites where there
were two; the wrong Python test file).

## DOCM-3 MERGED (2026-09-04, PR 1803)

Ordinal 1801, sample #127, block DOCM-B1 slot 0. Implementer arm OPUS.
Two phases: the lane stopped at DM4's false naming key, the orchestrator
amended DM4/DM5 (#1807) and the spec, the lane resumed as the same arm.
v6 dual on frozen `a713a02e`: R1 (fable) 0/3/4 APPROVE-WITH-FIXES, R2
(opus) 1/7/5 APPROVE-WITH-FIXES. Adjudicated: one bilateral MAJOR (the
fold's refusal named fold-internal rows no table holds; R1 rated it
MINOR) fixed by collapsing refusal payloads through the union's own
rule; two silent deviations (two wildcard arms; the list floor reaching
the loft's insert and load doors) fixed and disclosed; the rest MINOR
and NOTE, all fixed. Unilateral-MAJOR tally +0. R1 disclosed an
accidental glimpse of R2's build-script PATHS through the shared session
scratchpad — no content, no findings — so the pair stays FAIR under v6
item 3e; the hazard is now a line in `memories/agent-lane-operations.md`
and every later brief names a private scratch directory. Filed at
adjudication: `n-ary-union-has-no-declaration-channel` (Ev's ruling).
Spec `docs/DOCM-3-SPEC.md` deleted at merge (DOC-LEDGER). Both node-kind
units (`DOCM-1`, `DOCM-2`) are now unblocked.

## Third dispatch (2026-09-04)

`DOCM-1` dispatched on `docm/1-face-frame` against `docs/DOCM-1-SPEC.md`
(pre-draw L / STRUCTURAL at the spec; block DOCM-B1 slot 2, the last
of the block), after `DOCM-3` merged so the new datum's content-key
tag is chosen against a tree that holds tag 31. `DOCM-2` follows on a
fresh block draw once `DOCM-1`'s tag is on main. Lane briefs now name a
private scratch directory beside the private target dir.

## DOCM-1 stop clause fired; PP6 amended on Ev's ruling (2026-09-04)

The lane stopped at PP6's fence: a derived frame has no f64 elaboration
in the document, so under the pinned lift its profile had nothing to
place with at `Interval`/`Dual`. Three ways through were put to Ev
(place at the lane value; refuse off f64; an f64 shadow of the upstream
body); Ev ruled A (in-chat). PP6 and DM1 (new DM1c) carry it; the seam
is announced in M10's log; the spec is amended on the lane branch and
the lane resumes as the same arm. The three E-class units run as
MECHANICAL units on Ev's "use your judgment" (in-chat): opus, no
review lane, no row, merged on green CI plus the orchestrator's read —
`docm/capend-ends`, `docm/spec2-slots`, `docm/revolve-pole-rule`.
## revolve-pole-export-interior-on-axis-vertex MERGED (2026-09-04, PR 1839)

Mechanical unit (opus, no review lane, no row; merged on green CI plus
the orchestrator's read of the diff). The rule as filed held on the
first run of both rows. One note for the WORK-TRACKS doc, which still
lists the question as open; that doc is a dated proposal and is not
edited.
## fused-step-slot-aliases-arrival-spec MERGED (2026-09-04, PR 1840)

Mechanical unit (opus, no review lane, no row; merged on green CI plus
the orchestrator's read of the diff). One material deviation, disclosed:
the `SetParam`/`SetExpression`/`expr_at` rows the item asked for cannot
exist, because the VQ9 insert door and the snapshot walk refuse a
program carrying an arrival `Sweep`/`ArcLen`/`Bulge` — the payload's
`expr`/`expr_mut` half is pinned instead, and the door's refusal with it.
No content-key tag was involved (`StepArg` feeds no key).
## capend-top-bottom-contradicted-by-negative-extrude MERGED (2026-09-04, PR 1851)

Mechanical unit (opus, no review lane, no row; merged on green CI plus
the orchestrator's read of the diff): `CapEnd::{End, Start}`, 76 files
of which the substance is the enum, the emitter's two loops, the
Python mirror, the corpus and seven digest pins re-taken through their
own doors, and the adopted probe. Two judgement calls accepted: the
variant declaration order kept (the derived `Ord` is every name
table's row order) and the probe moved out of the review record rather
than rewritten in place. Filed: the sweep crate's `top` field docs
(`work/issues/`). Noted for CIW's ground, not filed: the k-lint job's
byte-for-byte die-corpus step is SKIPPED on the tier this PR drew, so
hosted CI did not re-derive the regenerated corpus itself.

## DOCM-1 dual review dispatched (2026-09-04, PR 1829 frozen at 20f04189)

Implementer lane reported the PR non-draft and green at `20f04189`
(35 files, +2244/−79); the head predates the CapEnd rename now on main,
so the fix pass carries that conflict. Ordinal 1802 claimed on main at
dispatch (PR 1853); parity byte 83 ⇒ R1 = FABLE, R2 = OPUS; briefs
hashed and diff-identical modulo lane names; private build AND private
scratch directories per lane. Both lanes running concurrently on the
frozen head. The unit's log entry waits for the dual's conclusion.

## DOCM-1 MERGED (2026-09-04, PR 1829, ordinal 1802, sample #128)

Block DOCM-B1 slot 2 (FABLE), concluded; the block is concluded with
it and its record is on main. Three phases: the Phase-1 stop at PP6's
f64 sketch-plane fence (Ev ruled option A; PP6/DM1c amended, #1837),
the build under the amendment, then the v6 dual (R1 fable
NOT-MERGEABLE-AS-IS 1/2/4, R2 opus APPROVE-WITH-FIXES 2/4/4) and its
union fix pass. Adjudicated: one bilateral MAJOR (the tag-42 key-guard
row could not fail — the feed was redundant and is gone) and two
unilateral MAJORs by execution, one per slot: R1's symbolic-lane
asymmetry (a derived frame's placement does not certify on
`Sym<Interval>` under a widened upstream parameter; the fix pass
diagnosed it to the kernel's symbolic budget and stopped at its
clause — filed for M10 as
`derived-frame-placement-freezes-on-the-symbolic-lane`) and R2's A3
handedness gap (a surviving mutant, now caught by A3b). Both reviewers
falsified the PR's "any width refuses" kernel claim; the measured
floor (plain `Interval`, near ε/16) is the record now. Filed at merge:
`LIB-B-FACE-FRAME` (the Python surface), the two issues the fix pass
filed, and — from the tally read this merge required —
`ab-log-v6-stream-is-past-its-stopping-rule-unadjudicated` (Ev's).
The spec is deleted into the ledger. Next: DOCM-2 (`Node::Part`,
spec written) under a fresh block draw, DOCM-B2.

## DOCM-2 dispatched (2026-09-04, block DOCM-B2 slot 0)

Block DOCM-B2 drawn after DOCM-1's merge: pre-draw fields at the spec
(M / STRUCTURAL, committed before the draw), byte 39 ⇒ fable at slot 0,
so DOCM-2 = FABLE; slots 1–2 bank for the next kernel units once the
open questions are ruled. Record branch-side on `docm/b2-block`. Unit
branch `docm/2-part` cut from the orchestrator branch (it carries the
spec and DOCM-1's state-sync, both headed to main in PR 1856). The
implementer lane is running on it.

## DOCM-2 stopped at its clause and resumed under an amendment (2026-09-04, PR 1860 draft)

Two findings from the lane. (1) In-fence: `wire_split` stamped both
halves' section planes `minted(split, 0)` — opposed planes with one
source, a same-source-theorem violation unreachable before `Node::Part`
could take two pieces of one split; the fix (one index space across
both halves) stands and gets its own row. (2) Fenced: topo's two
same-source assertions (`merge_faces.rs`, `boolean/plane_eq.rs`) read
`eq_bits`'s `None` at a channel-less scalar (`Dual`, `Sym`) as `false`
and panic on the corpus document's union of the halves at `Dual64`.
Ruled option (a): the assertions read a channel-less scalar as no
evidence (rung 1 decides by source identity; no verdict changes), the
fence widened to exactly those two sites, the Dual-vs-f64 value-channel
row as the proof the relaxation hides no wrong merge. Spec amended on
the branch (§Amendment at the stop clause); the lane resumes as the
same arm.

## DOCM-2 dual review dispatched (2026-09-04, PR 1860 frozen at b59b2203)

The lane reported the PR non-draft and green across the whole matrix
at `b59b2203` (33 files, +1792/−91; one earlier red on a gate script's
reader, fixed and pushed). Ordinal 1803 claimed on main at dispatch;
parity byte 67 ⇒ R1 = FABLE, R2 = OPUS; briefs hashed and
diff-identical modulo lane names; private build and scratch
directories per lane. Both lanes running concurrently on the frozen
head. The unit's log entry waits for the dual's conclusion. Filed at
merge, not before: `LIB-B-PART` (the census family the unit chartered)
and the bit-identity gate reader's finding.

## DOCM-2 MERGED (2026-09-04, PR 1860, ordinal 1803, sample #129)

Block DOCM-B2 slot 0 (FABLE), concluded; slots 1–2 (OPUS, OPUS) remain
for the next kernel units. Three phases: the stop at `wire_split`'s
one-source stamping (fixed in-fence — a latent kernel defect
unreachable before a Part could take two pieces of one split) and at
topo's two channel-less same-source assertions (ruled no evidence;
fence widened to those two sites); the build under the amendment; then
the v6 dual (R1 fable NOT-MERGEABLE-AS-IS 1/2/3, R2 opus
APPROVE-WITH-FIXES 1/4/4) and its union fix pass. One MAJOR, bilateral,
reproduced by the orchestrator: the table projection refused a tie
straddling the two halves, which a pass-through tie carried into a
split produces; ruled that the projection narrows survivors by the one
rule the emitter's flush already writes by. No unilateral MAJOR — the
pair adds nothing to the tally. Filed: `LIB-B-PART`; by the fix pass,
the bit-identity gate's reader (`work/issues/`) and the two-Parts-at-
one-boolean diagnosis (DOCM's slate). The spec is deleted into the
ledger. Next: the remaining slate needs rulings before the next kernel
unit — the union declaration channel (Ev's), then the instantiation
seam, the check-registry subject and the certified range query.

## DOCM-5 dispatched (2026-09-04, block DOCM-B2 slot 1, OPUS)

The plan's ruled item 2 (the check registry's subject) cut as a unit:
`docs/DOCM-5-SPEC.md` (pre-draw M / STRUCTURAL) — `run_checks_on` over
a `Subject`, `assemble_gathered` over a gathered product with `assemble`
as a logic-free wrapper, `DocSession::land` gathering once for its
three consumers in the order fault → registry → A5 badge, a debug-only
gather counter as the witness, the registry's cost re-measured with
both terms separated and a Q6 disposition. The `product.rs` Dual arms
stay M10's (stop clause). Unit branch `docm/5-check-subject` cut from
the orchestrator branch (the spec rides PR 1863 to main). Slot 1's
pre-draw fields recorded branch-side on `docm/b2-block`.

## DOCM-5 dual review dispatched (2026-09-04, PR 1871 frozen at 4c727c88)

The lane reported the PR non-draft and green across the drawn full
matrix at `4c727c88` (13 files, +1013/−103; one earlier red on the
binding census, fixed and pushed). No stop clause fired. Its
measurement: at 161 solids / 991 faces the gather is ~30× the
registry's own resident (248 ms vs 8 ms, dev profile), so the
withdrawn sentence was right; the split is now a row of the
rebuild-latency reporting lane with its size pinned by a test. Ordinal
1804 claimed on main at dispatch; parity byte 123 ⇒ R1 = FABLE, R2 =
OPUS; briefs hashed and diff-identical modulo lane names; private
build and scratch directories per lane. Findings the lane reported for
placement at merge: `scene.rs` gathers the same landed product twice
more (VIEW's), and `docs/PERF-PLAN.md` is cited by thirty files and
absent from the tree and the ledger (the spec itself among them).

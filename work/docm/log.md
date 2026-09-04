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

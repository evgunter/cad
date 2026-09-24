# MSOLVE log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/msolve/plan.md`.

## Opened (2026-09-04)

Opened by the FIX orchestrator on Ev's steer (in-chat, PR 1731 thread:
DOCM "feels somewhat different — it's ok to open a successor unit to
S-MATE if that's what makes sense").

The five items below were measured or ruled during FIX's run and have
no owner: S-MATE closed while they were in flight, and DOCM inherited
the FILES rather than this class of question. Re-homed here by header
edit and `git mv`, ids unchanged.

Two things this program starts with that most do not:

- **A live defect with characterization rows already on main.** PR 1773
  pins the transform-blind solve as a known-wrong answer, with a header
  saying the fix DELETES the rows rather than updating them. They go red
  when item (1) is fixed; that is the signal, not a regression.
- **A ruling already made.** Ev ruled item (2) in on PR 1731, and the
  sequencing — the gate first — with it.

The territory overlap with DOCM is announced on their orchestrator PR,
not assumed. Nothing has been taken from them.

## Ruled and cut (2026-09-05)

Orchestrator's first session. Ev's rulings, in chat:

- **The gate's fix shape.** The plan's "`derived_offset` sibling that
  walks the input chain" is well-defined only for a pattern head; a
  bare transform is invisible in a mate's data (N1: no segment, so a
  mate through a transform is byte-identical to a mate on the
  instance). Three shapes weighed: (a) a transform mints a segment —
  rejected, it changes what a name IS and renames every entity
  downstream of every transform, in persisted documents too, against
  what `emit_union` and the measure door were built on; (b) the mate
  stores the node each side is read at, the measurement reference's
  shape — **ruled in**; (c) refuse a mate whose instance has a placing
  consumer until (b) lands — not needed once (b) is the unit. Ev's
  framing that decided it: a transform is represented only as a DAG
  parent of the thing it transforms; the operand IS that, provided the
  edge is A12's reading kind, not consuming (A10's roots).
- **Territory:** touch whatever, resolve conflicts as they come.
- **The first `[ev]` question** was already answered by
  `ASSEMBLY.md`'s A11 (5); withdrawn, no PR.
- The `DanglingHead` catch-all ruled in by this program (S-MATE's
  successor) as `MSOLVE-3`; AQ8's SKIP half stays Ev's, a short `[ev]`
  PR to come.

Cut: `MSOLVE-1` (spec `docs/MSOLVE-1-SPEC.md`), `MSOLVE-2` parked on
it, `MSOLVE-3` open; the three issues they answer parked on them. Two
items re-homed here by the 2026-09-04 sweep read and placed on the
slate (the memo key: a unit after 1; the lever's extent: an `[ev]`
question). Next: dispatch MSOLVE-1 on `msolve/1-mate-operand`.

## AQ8's SKIP half homed (2026-09-05)

Not an `[ev]` after all: PR 592's addendum comment carries Ev's 👍
ratifying option (b). The clause joins the weld half in `ASSEMBLY.md`;
`aq8-skip-half-is-cited-as-ratified-and-is-not` closed. Orchestrator
PR 1913 (spec and cut) merged.

## MSOLVE-1 dispatched, landed, in review (2026-09-05)

Dispatched on `msolve/1-mate-operand` against `docs/MSOLVE-1-SPEC.md`.
The lane took the two item-7 measurements first: the blindness was
never class-dependent (a residual tree edge refuses `Under` before any
pose exists, with or without the transform; only a DETERMINING fold
showed it), and it covers rotation (an x-π/6 transform tilts the placed
block, `Opposed` broken in the product, nothing refused). PR 1929
green on the full matrix at `03d812228`; the reference type is
`SitedRef` (`EntityRef` was taken by N4's arena key); one row's
expectation changed by the PR 1731 ruling (pattern-of-transform now
places). Style review and correctness arm dispatched on that head.
Findings reported outside the fence, placed at state-sync: `Node::Part
{ Instance(i) }` is a third identity-transparent node the walk refuses
(MSOLVE-2's decision); `Frame::rotate_then_translate` normalizes with a
bare `.normalize()` (filed here).

## MSOLVE-1 reviews adjudicated, fix pass dispatched (2026-09-05)

Style review: twenty findings, Q1–Q8 all exercised. Correctness arm:
APPROVE-WITH-FIXES, C1–C6 confirmed on documents of its own (non-axis
rotations, a gauge-side chain, a placed gauge frame with transforms on
both sides, the viewer end to end). One MAJOR, the suite's own: the
acceptance fixture's seat was an interpenetration (`b`'s axis on the
wrong side of the cap), so the gate refused the control too and A5's
"inconsistent pair refuses" was vacuous; the claim itself is true on a
physical seat. Ruled in the fix pass: the split door refuses TYPED, in
both directions, when a mate and its operand land on opposite sides of
a cut (the reading-edge twin of D-2's `SeveredEdge`; the kept-mate
direction was silently keeping a stale operand, the cut-mate direction
was refusing in the input's vocabulary); `DanglingHead.head` names the
node the walk stopped at, not a live instance; the product oracle pins
frame origins (rotation about the seat normal was invisible); a
non-commuting chain row; the viewer row seats a moved instance after
commit; doc rot from the rename swept. Carried to MSOLVE-2: the
`Placer`/`copy` duplication, the triple walk, splitting
`mate/member.rs`. Filed: the gate's `Vanished` on a mate read below a
pattern (here); an instance under two placing roots refusing `Naming`
at the gather, and `Transform` refusing a pattern's `Instances`
(`work/issues/`, no obvious owner).

## MSOLVE-1 MERGED (2026-09-05, PR 1929)

Fix pass green on the full matrix at `550a9f2`; merged without a
fresh run on the state-sync commit (docs and tracker only). What the
fix pass added beyond the reviews' letter: `SplitError::
OperandSeveredFromMate` runs after the cluster precondition and exempts
the interface crossing (a kept at-mint mate whose name lies wholly in
the cut re-anchors through the minted instance); a non-exhaustive
Python `split_err` match the new variant exposed. Item closed, spec
deleted into the ledger, `mate-solve-is-transform-blind` closed;
MSOLVE-2 and MSOLVE-4 un-parked. Next: dispatch MSOLVE-4 (spec on
main), write MSOLVE-2's spec against the walk as landed.

## MSOLVE-4 landed, in review (2026-09-05)

PR 1960 green on the full matrix at `b4764ea`. The spec's premise
verified on the tree: the memo has one reuse site and it matches only
`NodeResult::Ok`, so the fault's content does not feed the key. One
CHROME row's PREMISE was rewritten (it asserted the memo hazard as
its precondition and said so); the guard in `tree.rs` is gone with no
row needing it. Style review and correctness arm dispatched on that
head.

## MSOLVE-4: reviews re-dispatched on the integrated head (2026-09-06)

The two reviews dispatched on `b4764ea` died at their first tool call
on a session limit, and a day passed. Main moved 369 commits meanwhile
(among them the escalations channel beside verdicts in `eval/mod.rs`),
so the lane merged main first — one adjacency conflict beside
`verb_content_tag`, both sides kept verbatim, the memo reuse site and
the key's format tag untouched by main — and re-greened PR 1960 at
`4ea542d` on the full matrix. Style review and correctness arm
re-dispatched on that head, reviewing the tree that will merge rather
than the one that would not.

## MSOLVE-4 reviews adjudicated, fix pass dispatched (2026-09-06)

Correctness arm MERGEABLE: C1–C4 confirmed on CHROME's bench through
the viewer's own doors (blame and row agree at every step, two
different faults in succession never stale, a stale prior still reused
right by content), keys measured bit-identical on the corpus before
and after, the merge's one adjacency intact. Style: no MAJOR. Ruled
for the fix pass: the key format tag BUMPS to v6 and the prose
exception goes (the block's own purpose is an honest input-set version
for a future persistence; the spec's bit-for-bit clause is withdrawn);
one solve answer read once and fed per arm; the redundant-bit and
`None` docs corrected; the viewer row's assertion tightened; a
per-mate reuse row; the correctness probes adopted; the resolver stub
hoisted into the shared fixture. Filed: two tag functions for
`ContactClass` in the content key, and the eleven resolver-stub copies
(`work/issues/`).

## MSOLVE-4 MERGED (2026-09-06, PR 1960)

Fix pass green on the full matrix at `e1ede48`; a merge of main on
the state-sync commit re-ran the matrix green at `9885443` before the
merge. Item
closed, spec deleted into the ledger, `mate-memo-key-does-not-carry-
the-solve` closed. Next: MSOLVE-2 dispatched on `msolve/2-member-chain`
against the spec on main; MSOLVE-3 after it.

## MSOLVE-2 dispatched (2026-09-06)

`msolve/2-member-chain` cut from main in the same lane clone against
`docs/MSOLVE-2-SPEC.md`: the member chain through `Part {
Instance(i) }`, sibling distinctness at every level, one walk per
reference, `mate/member.rs` split out. (The preceding merge commit on
this branch carried the log's conflict markers; this commit resolves
them — merge-only, nothing rewritten.)

## MSOLVE-2 landed, in review (2026-09-06)

The lane opened PR 2039 and was cut off by a session limit while
polling; the one red was the Python mirror of the new
`MateFault::PartSelectsAnotherCopy`, which it finished on resumption.
Green on the full matrix at `75d975d`. What the build measured: two
inner siblings under ONE outer pattern are unbuildable (a pattern takes
one body), so A2(a) holds the outer index across two chains; an
inconsistent loop is built by naming a sibling lifted clear, because
the gate verifies geometry, not the folded frames; the gate's
`Vanished` on a mate read below a pattern reproduces one level deeper,
pinned. The viewer cannot author a `Part` node at all (no `AddPart`
op) — CHROME's, to be filed. Style review and correctness arm
dispatched on that head.

## MSOLVE-2 reviews adjudicated, fix pass dispatched (2026-09-06)

Correctness arm APPROVE-WITH-FIXES: C1–C5 confirmed on documents of
its own (two and three levels deep, transforms between and above,
circular and linear rules at non-identity indices, the viewer's real
rays), with one MAJOR the spec itself placed wrong: the `Part`-index
check lived in the offset, which runs only for a tree edge's first
mate, so a DECLARING mate whose `Part` gathers a different copy than
its name names was silently green — and the gate did not catch it,
because it looks the NAMED copy up and finds it seated while the body
the `Part` gathers floats. Ruled: the per-reference checks that need
evaluation (the `Part`'s index against the name, the index against
the count) move to the solve's own walk site, for every reference of
every mate, refusing typed there; the offset keeps the arithmetic.
Style: no MAJOR; the loop rows never folded a non-identity outer map
(fixed with the reviewer's rows), two spellings of "which mates weld"
collapsed, the copied test helpers hoisted beside the shared oracle.

## MSOLVE-2 MERGED (2026-09-06, PR 2039)

Fix pass green on the full matrix at `ef8926c`. Item closed, spec
deleted into the ledger, `nested-pattern-mate-heads-refuse` closed —
the PR 1731 ruling is fully landed. One consequence the fix pass
stated: a mate the per-reference checks refuse still welds its
cluster (the partition is structural), so an instance no pair reaches
rides its cluster's recorded frame with the mate's row red, as a
dangling head already does. Next: MSOLVE-3 on
`msolve/3-placer-refused`.

## MSOLVE-3 dispatched (2026-09-06)

MSOLVE-2 merged (PR 2039, `6dff151`). `msolve/3-placer-refused` cut
from main in the same lane clone against `docs/MSOLVE-3-SPEC.md`,
which predates MSOLVE-2's move of the count check to
`check_reference`; the lane reads the tree as it is. Slate after it:
the gate's `Vanished` below a pattern, the lever's extent (`[ev]`),
the remap-inference record correction.

## MSOLVE-3 landed, in review (2026-09-06)

PR 2081 green on the full matrix at `885d257` (one red on the way: a
`rotate_then_translate` caller in `demos/tour`, outside the first
sweep's `crates/` scope — the repo's four workspaces have no local
script that builds them all, stated in the PR). What the build
measured: `check_reference` keeps `eval_count` for its two counts
because the whole-node door would refuse a mate onto copy 0 of a
broken pattern (a change to admission, out of scope); the carried
refusal is an `Arc`-newtype over `NodeErrorKind` because the fault
types derive `Clone`+`PartialEq` and the kernel error type cannot; the
rider reaches no viewer call site at all. Style review and
correctness arm dispatched on that head.

## MSOLVE-3 reviews adjudicated, fix pass dispatched (2026-09-06)

Correctness arm APPROVE-WITH-FIXES: C1–C5 confirmed with the carried
kinds byte-identical to the placer's own on eight probes; three
MINORs — the datum road attributed the datum's slot refusal to the
pattern (ruled: `placer` names the refusing node, the datum when its
slots refuse), a blanket `From<NodeErrorKind> for EditError` that
turned any node error into "the placement axis is unusable" (the
catch-all shape reopened one door over; ruled out), and no Python row
for the carried error. Style: no MAJOR; the `need_*` slot readers
copied into `mate/`, a hand-spelled `WrongOperand` payload the one
row that could catch it did not compare, the untested explicit-rule
arm, doc rot in `topo/src/query.rs` and the role lists. Filed: the
four-workspace build hazard (CIW) and the `names/{flush,select}.rs`
`map_err(|_| ..)` discards (SEAT), `work/issues/`.

## MSOLVE-3 MERGED (2026-09-06, PR 2081)

Fix pass green on the full matrix at `8a72887` (one red on the way:
the rider row's axis-length probes assumed the default band; they come
from the run's own band now). Item closed, spec deleted into the
ledger, the catch-all finding and the placement rider closed. The
slate that remains: the gate's `Vanished` on a mate read below a
pattern, the lever's extent (`[ev]`), the remap-inference record
correction.

## Remap-inference record correction closed (2026-09-06)

`mate1-sweep-inferred-a-remap-from-a-refuted-reachability` closed as
a record correction: the finding is the record, its (b)-SKIP premise
is ratified since PR 1914, and no code moves. Slate after MSOLVE-3
merges: the gate's `Vanished` below a pattern (next unit) and the
lever's extent (`[ev]`).

## MSOLVE-5 specified (2026-09-06)

The gate's `Vanished` below a pattern is a unit: `docs/MSOLVE-5-
SPEC.md`. Ruled as a change to what the refusal says, not to what is
admitted — the master-spelling pin in `mate1_member_vocab.rs` is
ratified and keeps refusing; the gate asks the operand's own table
first, so `Vanished` means vanished and a name spelled at a non-root
refuses `ReadBelowARoot { at }`. No consumer walk. The dead `NodeGone`
arm goes with it. Minting on copy 0 instead would reopen the pin and
is not asked. Dispatches from main once PR 2081 is in.

## MSOLVE-3 merged; MSOLVE-5 dispatched (2026-09-06)

PR 2081 merged at `4568d7d` on a full green matrix. The lane clone
cut `msolve/5-read-below-a-root` from main against `docs/MSOLVE-5-
SPEC.md` (read from this branch until it lands; the lane merges main
before its first push). The lever's extent is up on `[ev]` PR 2086.

## MSOLVE-5 stop clause hit and ruled (2026-09-06)

The lane (PR 2090, draft) reached the spec's third bullet with a real
document: a mate reference naming a root instance's BODY is admitted
at the door, placed by the walk, and at the gate the operand's table
answers, `at` is a root, and the product is silent — because
`carry_names` drops body rows. Ruled: the operand's entry decides its
kind before the root question is asked — a non-face entry refuses
`NotAFace { kind }` wherever it is read (the product's own answer for
a non-face row it holds; a body never mints), a face entry at a
non-root refuses `ReadBelowARoot { at }`, and a face entry at a root
with the product silent is unreachable by construction and says so
at the arm. Inside the fence; `NotAFace`'s meaning unchanged. The
lane un-drafts and drives CI.

## MSOLVE-5 landed, in review (2026-09-06)

PR 2090 green at job level on `bc8a1ab` (the first run on that head
was cancelled by the un-draft's concurrency rule; its `gate ok` reads
failure for that reason only). What the build measured: the operand's
table is read through the same door the interrogation functions use;
`NodeGone` had no constructing site; the arm is authorable from
Python through `placed_union(transform(...))`, so the Python row is
real. Style review and correctness arm dispatched on that head.

## MSOLVE-5 reviews adjudicated, fix pass dispatched (2026-09-06)

Correctness arm APPROVE-WITH-FIXES: C1, C3, C4 confirmed (the solve
and the product bit-identical to main on the issue's document — the
one differing line is the unminted row's `why`); C2 PARTIAL, one
MAJOR: a TIED non-face entry at the operand skips the kind question
and answers `ReadBelowARoot` where the ruling says `NotAFace`
(reachable through a split part's `TieRows`). Ruled: the kind comes
from `name.kind`, which the table enforces for every candidate. Style:
no MAJOR; the operand's table read by a hand match instead of
`interrogate::value_of`; `Vanished`'s sentence not moved with its
meaning; `ReadBelowARoot`'s sentence claiming a spelling the code
never computed (the correctness arm's empty-boolean and discarded-half
probes show the product may hold no trace at all); arm 4 and the
not-live arm held by sentence, the latter with the wrong reason (the
gather's Pass 1 refuses a poisoned root, not the solve); message
substrings copied four times. Probes adopted: P1, P4, P6, P9b. Filed
(`work/issues/`): the split root whose tie spans both halves refuses
the gather (`carry_names` `DuplicateName`), and the assembly door's
"only the head is raised" follow-up that lived in a comment.

## MSOLVE-5 MERGED (2026-09-06, PR 2090)

Fix pass green on the full matrix at `ae78e6e` (one run, no cancelled
twin). Item closed, spec deleted into the ledger, the gate issue
closed. Filed on the way: the tie-before-kind asymmetry between the
product's own rows and the operand's (`work/issues/`). The slate that
remains: the lever's extent, asked on `[ev]` PR 2086.

## The lever's extent asked (2026-09-06)

`mate-lever-needs-the-parts-extent` is next in line and is a schema
question before it is a unit, so it goes to Ev as an `[ev]` PR with
three shapes weighed — the extent authored beside the datum, the
extent resolved from the mated part's own body, or E3's amendment
revised to keep the session box at the mate site — and the second
recommended: `InstantiatePart` is a leaf whose body is
placement-independent, so its reach from the part-local origin is a
function of pinned content and reaches the solve without a
wire-format change. `needs_ev: true` on the item.

## The lever ruled; MSOLVE-6 specified and dispatched (2026-09-07)

Ev ruled option B on PR 2086 after a reminder of A11's context (the
five rules; "nothing in the walk is evaluated"; A11 never spoke of
the lever's source). `docs/MSOLVE-6-SPEC.md`: the lever is
`(R_a + ‖a.origin‖) + (R_b + ‖b.origin‖) + Σ|authored lengths|`, each
`R` the part body's reach from its own origin through the measure
site's `reach_of`, read through one trait the evaluation implements
lazily over the `PartCache`; the metre and the micron floor retire;
A11 gains one sentence. Stop clauses: a corpus part with a face no
bound can be stated for; a k-lint or decision-log row moved by
evaluating a part before the first node. Rides PR 2086 with the
ruling; the lane reads the spec from this branch until it lands.

## MSOLVE-6 stopped on the edit door's maintenance (2026-09-07)

The lane built the unit (PR 2116, draft: the reach module, the lever
formula, the retired constants, the doors, the docs) and stopped on
clause (iii): `reconcile`, the keying maintenance `edit::apply` runs,
solves the PRIOR document to preserve a split cluster's gauge pose,
and the edit door holds no resolver. Two `asm_r2a` rows measure it
(a re-minted frame at z = 4 instead of the solved z = 5). A design
fork — the edit door's purity, or the maintenance's home, or a
stamped extent beside the pin — so it goes to Ev as
`reconcile-solves-with-no-resolver` on an `[ev]` PR, with the edit
door taking a reach recommended. The lane holds on its branch.

## The edit door ruled; MSOLVE-6 resumes (2026-09-08)

Ev ruled (a) with the replay refinement on PR 2118, after three
questions: how it relates to 2086 (the same B, one solve site
further in), whether a stamped extent is a cache that drifts (it is
derived from content-addressed data and checked at evaluation, but
it is still a derived geometric fact in the recipe), and which is
cleaner ignoring churn (the edit door taking the reach, with the log
carrying the maintenance's frames so replay never solves — the
orchestrator's honest answer reversed its own (h) lean). The spec
gained its amendment section; the lane resumes on PR 2116.

## Announced from LIB (2026-09-09): a derive word on `MateSide`, `AxisSense` and `MateRole`

LIB-MIRROR (PR #2271) adds `Hash` to `MateSide` (`mate.rs:84`), `AxisSense` (`mate.rs:144`) and `MateRole` (`mate/solve.rs:43`) so the Python tag mirrors match their Rust derives under Ev's (A) ruling on `[ev]` #2265; no solver behaviour and no serde spelling changes.

## MSOLVE-6 landed, in review (2026-09-12)

The lane died twice on model limits (2026-09-08, 01:45Z and ~03:20Z)
after pushing the whole unit — the lever, the reach trait and door,
the edit door taking the reach, `LoggedEdit` and the replay doors,
the retired constants, ~400 call sites — and the orchestrator's
session idled until 2026-09-12. Resumed that day: main had moved 474
merged PRs, the lane merged it (2,420 commits) into
`msolve/6-part-extent`, and PR 2116 is job-level green at `21f5a65`
(39 checks; CI grew a corrupt-input job and a viewer montage lane in
the interval). What the build measured, beyond the spec: the bracket
read lives under `EvalScalar` in the eval adapter after the
bounds-allowlist gate refused a compound bound in `mate/` (ruled: no
allowlist entry); `split` and `inline` take a resolver rather than a
reach because the maintenance of a split's own rebinds levers on the
part the split is minting; the edit refuses only where the prior
solve reached NO verdict and records `Split { frame: None }` where it
decided there is no pose (deviation 2, argued in the PR). Style review
and correctness arm dispatched on that head. Five items were routed
onto this slate by EVAL and LIB in the interval (the `FromFace` mate
frame arm under Ev's ruling (F) on PR 2256; three levered clash
margins with their arm invisible; the flat-index `Part` over a nested
pattern at `check_reference`; `axis_datum`'s seat; the per-check
nominal environment) — triaged after this unit's review.

## The routed items triaged (2026-09-12)

Five items reached this slate from EVAL, LIB and FIX while the
orchestrator idled. Triaged into three units after MSOLVE-6, in
`plan.md`: MSOLVE-7 gathers the three `member.rs` findings (the
nested-pattern flat index at `check_reference`, `axis_datum`'s seat,
the per-check nominal environment); MSOLVE-8 is the three levered
clash margins with their arm invisible, with the radians-vs-pure-number
decision the item names; MSOLVE-9 is the `FromFace` mate-frame arm
under Ev's ruling (F) on PR 2256, last, on MSOLVE-6's reach road. The
exit walk moves behind them.

## MSOLVE-6 reviews adjudicated, fix pass dispatched (2026-09-12)

Correctness arm PASS with MINORs: C1 the lever bounds the true reach
on six fixture parts (exact on all-line bodies, 24 % loose on the
cylinder by the documented rim bound); C2 by construction plus the
113 pose-pinning rows (main not rebuilt — disk); C3 typed on every
road; C4 the census; C5 replay store-free and the reach asked only
when a gauge moves (4 asks on a 3-instance chain: two pairs, two
parts). Findings: a recorded maintenance row's frame is trusted
bytes at load (a reflection loads; `SetPlacement` of it refuses);
`unsolved_because`'s document-wide fallback could attribute another
cluster's fault and, for a decided kind, PROCEED — dead in practice,
ruled fixed by dropping the fallback and refusing `fault: None` as
the typed invariant-excluded report; no interval-lane row for the
bracket read (P2b adopted). Style: no MAJOR of its own; the
`Applied.maintenance` and README premises now false; nine spellings
of "a reach from a resolver"; the lever sum formed twice; five replay
loops; two knobs for three states in `apply_maintaining`; a
`PartialEq<DocEdit>` in `src` for a test; the archaeology in
`lever_arm`'s doc (ruled: discipline §4 over the spec's "keep the
story"); `PosesOfAnotherDocument` filed as decided (ruled: refuse).
Twenty-seven items to the lane.

## MSOLVE-6 lane resumed; six routed items triaged (2026-09-19)

The implementer lane died on a session limit on 2026-09-12 with the
whole 27-item fix pass committed locally (two commits over a merge
with main) and nothing pushed; PR 2116 sat at `21f5a65ec` for a week
while main moved 3506 commits and went `dirty`. Resumed 2026-09-19:
push first, merge main again (EDIT's typed mate head, PR 2799, lands
in the same files), the full verification as a fresh head, the PR
body's fix-pass section, green at job level, then the head to this
orchestrator for the item 1/2/4/8/10/11 spot-check and the merge.

Six items reached `work/msolve/` from other programs while the
orchestrator was idle. Two closed at triage as records (the stale
`tree.rs` citation in the closed memo-key row, and MSOLVE-5's three
kind rows that EDIT's typed head made unwritable — ruled: an ordering
over one question is empty, nothing replaces it). `MatePrimitive`'s
missing `deny_unknown_fields` joins MSOLVE-7's lane; `coset.rs`'s
unit-direction witness and the `MateFault` subject row join MSOLVE-8's
(ruled: no `subject()`, a sentence on the enum naming its two
consumers and the `Band`/`PosesOfAnotherDocument` asymmetry); the
clocking row splits — the static `FrameCoincidence`+clocking refusal
at `AddMate` is MSOLVE-10, the roll convention rides MSOLVE-9's spec.
`plan.md` items 13–16.
## MSOLVE-6 MERGED (2026-09-19, PR 2116)

The lever is the mated parts' own extent, and the edit door takes the
reach with the log recording the maintenance. Reviews on `21f5a65ec`
(correctness PASS with MINORs, style no MAJOR); the twenty-seven-item
fix pass landed after the lane's session limit reset, over two merges
with main (3506 commits, then 32: EDIT's typed mate head, DM7's mated
deletes, the drafts row) and one CI round for four callers main added
and the `deny_unknown_fields` census tally (two new sites in `edit.rs`,
one in `mate/solve.rs` — a re-baseline, the sibling row passing).
Orchestrator spot-check on the diff: the row-frame walk and the
`SetPlacement` door share `Frame::admission_fault`; `unsolved_because`
reads the gauge's own fault and `None` refuses; `PosesOfAnotherDocument`
refuses; one `PartReach::with_resolver` at every resolver-only door;
`mate_coset` takes the arm; one `replay_entry` behind five replays.
Spec into the ledger at the unit head. Closes the lever item and the
reconcile fork. Next: MSOLVE-7 (member.rs residue + the `MatePrimitive`
wire hole), then MSOLVE-8, -9, -10 per `plan.md`.

## MSOLVE-7 reviews adjudicated, fix pass dispatched (2026-09-19)

PR 2885, head `6e27dac6a`. Correctness arm: C1–C4 HOLD (the offset
is the document's nominal parameter bit for bit and follows a
`SetDocParamValue`; the dangling-transform seat is the transform on
both roads; 219 mate rows unchanged at three ε; the stray key refuses
at the load door and loaded before the attribute). One MINOR: a
transform over a DATUM as a circular pattern's axis is reachable
through `apply`, and there the derivation still seats `WrongOperand`
at the pattern while the evaluation seats it at the transform — the
unit's own "live transform" row asserts one road on a fixture the
evaluation refuses. Ruled: the unit's thesis is one seat, so that
shape is seated at the transform through the one classifier, not a
copied rule, with both roads on the row. Style: no MAJOR; the
evaluator builds its nominal environment and then calls the solve,
which builds a second (the class this unit closes, open at its own
boundary — ruled: a crate-private solve entry that takes the
evaluator's environment); the eight-parameter `solve_cluster` behind
a clippy allow (ruled: a per-solve context, which the spec's "no
cache that outlives the solve" never forbade); the seated pair spelled
three ways; `node_value_kind` taking an id and its node; the A1 scan
row's vacuous-green shapes; the environment sentence restated six
times beside its new home; debug-string asserts; a stale
`PlacerRefused::placer` doc. Thirteen items to the lane; two rows to
file (the maintenance half of `solve.rs`, the `null` spelling of a
unit variant).
## MSOLVE-7 MERGED (2026-09-19, PR 2885)

The member walk's residue: one nominal environment per solve, handed
in by the evaluator; the axis operand's refusals seated where the
evaluation seats them through one classifier; the flat index's
account closed by citation; the mate wire's one `deny_unknown_fields`
hole closed. Reviews on `6e27dac6a` (correctness PASS with one MINOR,
style no MAJOR); the thirteen-item fix pass landed in one push over
one merge with main. Orchestrator spot-check on the diff: `evaluate`
→ `solve_with_env`; the `Solve` context; `node_value_kind` by id
tracking the placer; seven both-roads seat rows; the A1 pin naming
its functions. Spec into the ledger at the unit head. Closes the four
items; four rows filed (three here, one on WIRE). Next: MSOLVE-8
(`docs/MSOLVE-8-SPEC.md`, on the orchestrator branch), then -9, -10.

## MSOLVE-8 dispatched; MSOLVE-9 spec and A11 sentence to Ev (2026-09-19)

MSOLVE-8's lane launched from main after PR 2894 landed its spec.
MSOLVE-9 — Ev's (F) on PR 2256 — drafted as a design unit: the arm
resolves on MSOLVE-6's reach road (a cached part is its own product
and name table in part coordinates, so `MateReach` grows `face_pose`
and no placement pull-back is needed), the part-local name and the
`reference` rule stated, the memo key carries the part's pin. A11
rule 5's inputs sentence is the one ratified text that moves; DESIGN.md
carries no A11 sentence (measured). Both on an `[ev]` PR, waiting for
Ev's word; the unit dispatches after that merge and after MSOLVE-8.

## MSOLVE-8 reviews adjudicated, fix pass dispatched (2026-09-20)

PR 2896, head `f3896c554`. Correctness arm: C1, C3, C4 HOLD (the
three residuals re-derived from the frames bit for bit; 266 mate rows
unchanged at three ε; `Band` reaches every row and
`PosesOfAnotherDocument` none, measured). C2 PARTIAL with one MAJOR:
the planar-pair line is minted under a second decision
(`‖(n1×n2)·arm‖`) that differs from `parallel`'s (`‖n1×n2‖·arm`) by
up to two ulps, so at the escalate boundary ten door-built two-rest
documents that were UNDER on main now refuse `Indeterminate` — a
verdict move, which the spec calls a finding. Ruled: one decision —
`parallel` decides the levered cross product's length through the
normalizing constructor under its own name and returns the witness
it minted, so the planar-pair site takes the direction the predicate
decided and `mate_planar_pair_line` retires. MINOR: the aim is
decided three times per mate where main decided twice, under one
funnel name, because the lane's deviation 1 re-mints the axis under
`point_at`'s name instead of taking the frame witness — and the spec's
stop clause named that condition; the lane argued past it (recorded
as a process lapse). Ruled: the door goes in geom-core after all — a
`point_at` sibling returning the `OrthoFrame` it already builds, with
`point_at` its `to_affine`, announced on SCALAR's tracker (the row
the lane filed closes as done by this unit); `MateFrame::frame` takes
it, the copied refusal projection, the dead side-A map and the third
band build go. Style: `Contradictory`'s `clash` stored beside the
lever's halves (ruled: one closed `Clash { Structural, Length,
Levered(Lever) }`, the string compare on `MATE_MEMBER_EMPTY` with it);
`derived_direction` mapping a decided zero to the in-band escalation
(ruled: the frame ladder's own vocabulary under `MateFault::Frame`);
two raising sites bypassing the one home for a levered margin; a
second copy of the no-mate asymmetry on the enum; a hand-kept caller
list; "thirty-two" counting nothing; rows that cannot go red. The
Roll/Residual arms print different levers (the mate's own vs the
fold's) — ruled honest, each the arm its predicate was decided over,
stated at the type. Twenty-two items to the lane; one row to file
(`LeverRefusal` mirroring `ReachRefusal`).
## MSOLVE-8 MERGED (2026-09-20, PR 2896)

The typed lever and the closed `Clash`; the coset's directions as
`UnitVec3` with `parallel` deciding once and returning its witness;
`MateFrame::frame` over geom-core's new `point_at_frame` (the fence
widened by that one door, announced on SCALAR's tracker); the
`MateFault` consumer sentence. Reviews on `f3896c554` (correctness
one MAJOR — the planar-pair line's second decision refused under its own name at
the boundary — and one MINOR, the aim decided three times per mate;
style no MAJOR); the twenty-two-item fix pass landed in one push,
the lane's argued-past stop clause reversed by ruling. Orchestrator
spot-check on the diff: `parallel`'s one decision, `mate_planar_pair_
line` gone; `point_at` = `point_at_frame(..).to_affine()`; `Clash`
without the string compare; the aim-count row at two per mate. Spec
into the ledger at the unit head. Closes the three items; two rows
filed here. Next: MSOLVE-9 waits on Ev's word on PR 2895; MSOLVE-10
(the static clocking refusal at `AddMate`) specs next.
## MSOLVE-10 reviews adjudicated; fix pass to the lane (2026-09-20)

Lane's head `7534c8854` on PR 2913, CI green. Style review (nineteen
findings, six MAJOR) and a correctness arm whose probe rows the
orchestrator ran after the arm hung on its own cargo call: C1 the
door's fault equals the solve's on a bypass road to the bit (arm
`0x400b988e1409212e`), C4 a rider asks the reach once per part in
document order and every other row asks nothing, the band's edges
agree at mid-band, zero and escalate, `Rebind` onto a non-member is
admitted and the solve refuses `DanglingHead` after it (N5, as the
PR said). Rulings. (1) "Never enters the document" is retired: the
load door's snapshot walk asks only the non-finite predicate, and it
stays that way — THE DOORS DECIDE EDITS AND THE SOLVE DECIDES STATES;
a state that comes to hold a per-mate fault (a rebind-stranded head,
a shrunk pattern, a doctored snapshot) is the solve's at evaluation,
and a `save` that refused states the doors produced would be a trap.
Every sentence becomes "refused at the insert door", the A11 clause
with it (so worded it elaborates and lands with the unit), one row
pins the snapshot road, the lane's Band finding is re-premised.
(2) The pair the fold never reads — two members over one instance,
the copies pair — carries a datum the solve never decides and the
door now refuses; ruled the door's (which pairs the fold reads is a
cluster fact), not the stop clause, disclosed in the corpus row and
the claim text. A loop-closing mate IS read (measured: door equals
solve). (3) The viewer's second match over the table's static gaps
goes; one `table_gap` beside `class_admission` in `mate.rs`, read by
`mate_coset` and the tool — the fence widened by that one pub fn.
(4) `admit_mate` restated the solve's first loop and `admit_pair`'s
self-mate arm was dead at the fold — one shared per-reference prefix.
(5) Three comments called `Maintain::Never` the re-apply arm; it
refuses at the maintenance and `Recorded` re-applies — the two replay
rules stated together, the lever's answer a named type. Nine MINOR.
## MSOLVE-9 ratified by Ev (2026-09-20, PR 2895)

Ev's word on the `[ev]` PR ("lgtm"): A11 rule 5's inputs sentence —
the solve's inputs are the document plus its mated parts'
evaluations, two answers crossing one door asked lazily per pair, the
extent as the lever and a `FromFace` frame as the face's canonical
pose in part coordinates, nothing stored twice — and
`docs/MSOLVE-9-SPEC.md` are on main at `5530c0633`. The unit
dispatches from main once MSOLVE-10 merges (PR 2913 in its fix-pass
CI), since both rewrite `mate/solve.rs`. LIB's
`no-door-mints-mate-frame-from-face` follows by announcement.

## MSOLVE-10 MERGED (2026-09-20, PR 2913)

The insert door asks the solve's own per-mate admission
(`admit_mate`) and refuses `EditError::MateRefused` with the solve's
fault unaltered; the table's static gaps have one home
(`mate::table_gap`, the fence widened by that one pub fn); one
per-reference prefix (`check_references`) for the solve's first loop
and the door; the lever a named `LeverArm`; `Maintain::reach` states
the two replay rules once; `DocEdit::writes_a_mates_datum` pins that
the mate insert is the one edit writing a datum. Reviews on
`7534c8854` (style six MAJOR, the correctness arm's probes run by the
orchestrator after the arm hung: door equals solve to the bit, the
reach asked once per part, the band's edges agreeing); the fix pass
landed in one push plus the census disposition of `table_gap` and a
merge of main after PR 2895 conflicted `ASSEMBLY.md`. Orchestrator
spot-check on the diff: the five rulings as ruled; one residue (a
copied gap string as a fallback) fixed in the census push. Principle
settled and written into A11 rule 1 as an elaboration: the doors
decide edits, the solve decides states. Spec into the ledger at the
unit head. Closes `mate-clocking-has-no-gui-path` (both halves
recorded); files `mate-band-fault-unreachable-on-a-mate`. Next:
MSOLVE-9 dispatches from main; the exit walk's rows 10–11 close.


## MSOLVE-9 dispatched (2026-09-20)

Lane on `msolve/9-from-face` from main at `354ba67f6` (MSOLVE-10
in), spec as ratified on PR 2895, with the orchestrator's notes on
what MSOLVE-10 changed under it: a `FromFace` side is resolved at the
insert door through the reach the door holds before `mate_coset`
reads the frame and before the lever is formed, and DECLINED on
replay exactly as the rider's lever is (the datum alone decided, the
next solve decides the face); a face that moves or vanishes after
insert is the solve's at evaluation (the doors decide edits, the
solve decides states). Exit walk: row 11 MET on PR 2913, row 10 in
flight. The walk goes to Ev as PROPOSED when MSOLVE-9 merges.

## Announced seam from DOOR (2026-09-21) — PR 2984

**`crates/editor-core/src/mate/member.rs`, one line.** DOOR's
`the-third-datum-axis-phrase-lives-in-mate-member`: `axis_datum`'s
`expected:` was the literal `"datum axis"` and is now
`crate::eval::phrase::DATUM_AXIS`, the const composed at compile time
from the family word so the phrase and the `found:` word beside it
cannot drift. WIRE retired its two copies onto it earlier; **this was
the third and last in the tree**, re-derived as a measurement rather
than inherited from the row.

**No refusal text moves.** The const expands to exactly `"datum axis"`,
verified through `concat!(family_word!(datum), " axis")` before the
change landed, so every assertion on that sentence stays green by
construction — including your `msolve3_placer_refused` rows.

**One thing to know about your unit tests in that module.** Four of
`member.rs`'s eight rows build their expectation from the const while
the source held a literal, so they were discriminating the gap between
the two — proved by mutation: drifting the literal turned exactly those
four red. **That gap is now closed, so those four no longer discriminate
the phrase's value**; both sides are the same const. This is correct and
is the point of one home, but it means the only things in the tree still
pinning the user-visible sentence are two literals in
`crates/editor-core/tests/` (`lib_tube_node.rs`'s `expected: "datum
axis"` pattern and `msolve3_placer_refused.rs`'s `contains` check).
**Those must stay literals** — a test naming the const could never catch
a change to the const's expansion. The `phrase` module's rule is about
construction sites and does not reach test assertions.

`found:` is untouched and was already right — it comes from
`crate::eval::node_value_kind`. The row explicitly fenced off the
`node_operand`/`axis_datum` refactor, and the lane did not widen into
it; the residue is filed on WIRE's slate as
`node-operand-has-one-consumer-where-axis-datum-is-the-same-door`, where
it records that the two doors **disagree about the seat** —
`node_operand` drops what `node_value_kind` answers with, `axis_datum`
carries it per MSOLVE-7. Worth your read, since half of it is your file.

Signed (DOOR orchestrator).

## MSOLVE-9 resumed on Opus; the wire amended; review tier DUAL (2026-09-24)

The MSOLVE-9 lane stopped on a usage limit on 2026-09-20 with PR 2934
open at `24643410b` and its run red four ways, all the unit's: the
tour's `FromFace` insert through a refusing reach, an unswept
`lever_arm()` caller behind `interval`, the float-lint gate on
`mate.rs`, one ruff error. The branch also conflicts with main in
`mate/solve.rs` and the MSOLVE-10 rows. A fresh lane carries it on
Opus (Ev, 2026-09-23: every phase runs on Opus), from the pushed head
and the PR body.

**The spec's wire sentence is amended on the unit branch**
(`bc3b8c55a`): `MateFrame` is externally tagged, no reader accepts the
pre-arm bare frame, and the tracked documents that carry a mate are
regenerated. The untagged read the spec cited as precedent was retired
by Ev's ruling on PR 3123 (backward compatibility with older files is
a reason to remove code), and PR 2702's `persist-no-backtracking` gate
refuses the attribute under `editor-core`. PORT filed it here;
it rides with MSOLVE-9. The spec is this program's, not text Ev
signed: #2895 ratified A11 rule 5's sentence, and said the spec rode
along for reading.

**Review tier: DUAL** (`memories/orchestration-model.md`, protocol
`docs/DUAL-REVIEW-PROTOCOL.md`). The unit changes a public type, the
save format of every document that holds a mate, and the memo key's
format; that is broad and hard to undo.

Triage of three rows routed here while the orchestrator was idle:
the untagged-wire row rides with MSOLVE-9; PROPS's escalation row and
CHROME's `PlacerRefused` siting row become MSOLVE-11, specced after
MSOLVE-9 merges (`plan.md` items 17–18). DOOR's note on
`mate/member.rs` read: no refusal text moved, and the two literal
assertions it names stay literals.

The exit walk's draft leaves the orchestrator branch before this
state-sync merges, since a draft walk on main would carry the status
line Ev ruled out of diffs (2026-09-21). It stands at `b9e6ca0ebf8bf62eed493ab65f25584ca86486b1`
(rows 1–9 and 11 MET, row 10 in flight) and returns in the `[ev]`
PR that asks for its ratification once MSOLVE-9 and MSOLVE-11 merge.

## MSOLVE-11 specced (2026-09-24)

`docs/MSOLVE-11-SPEC.md`, written while the MSOLVE-9 lane works. Two
measurements shaped it. `k_stats::Detached` is deliberately not
`Clone`, so each solve decision gets exactly one home: the mate whose
answer it decided. The fold's additions go to the added mate, and a
pair's own verdicts go to the mate `Under` already names. And
`coset::parallel`'s hand-minted escalation is reached only by a
non-finite lever arm, since `|u × v · arm| ≤ arm` for unit witnesses.
That arm skews every levered predicate silently, so the fix is a
finite arm by construction at its one formation door, and the minted
`Indeterminate` becomes unreachable rather than re-routed. Review
tier: single, full.

---
id: a-declared-union-has-no-one-pass-authoring-path
kind: issue
title: "A union with a declaration cannot be authored in one pass: the working path inserts a duplicate union and rebinds"
status: closed
opened: 2026-09-06
closed: 2026-09-19
refs: [2028, 2028]
pr: 2809
branch: edit/sited-declarations
---


## What

DOCM-7 gives `Node::Union` a `declare` edge whose pairs name entities in
the UNION's own name space. A member-space name therefore carries the
union's own node id — and there is no order of edits that authors the
two nodes directly, for two doors that are each correct on their own:

- `crates/editor-core/src/edit.rs:1482` — `InsertNode` admits a payload
  name only when `name.node` is already live
  (`EditError::DeclareNamesMissingNode`; the ruled D3 carve-out). So the
  `Declare` cannot be inserted before the union it names.
- `crates/editor-core/src/node.rs:2429` and DM6 — no edit rewires a live
  node's inputs. So the union cannot be inserted first and given its
  `declare` edge afterwards.

The path that works, and the only one, is the fixture `declared_union`
in `crates/editor-core/tests/docm7_union_declare.rs`: insert a FIRST
union with no declaration, write the `Declare` in that union's space,
insert a SECOND union carrying the edge, `Rebind` every name from the
first union's space onto the second, then delete the first. The
intermediate document holds a duplicate union, and the rebind loop has
its own limit — a name that appears in more than one declared pair (a
chain of contacts declares the middle member's faces twice) must be
rebound ONCE, or the second `Rebind` refuses `RebindNoReferences`.

For an accumulation-entity name (a `Seam`/`Merged`/`Fragment` row of the
union itself) there is no other path even in principle: such a name has
no spelling that predates the union, since it is minted by the union's
own evaluation.

## Why it is chrome's

Nothing in `editor-core` is wrong. What is missing is a SEAT: the
operation "union these bodies and declare these contacts" is one
authoring act and reaches the document as five edits, three of which
exist only to work around the ordering. A seat that composes them (or an
edit that attaches a declaration to a live node, which is a DOCM
question about DM6) is the fix. `work/chrome/placed-union-has-no-session-op.md`
is the neighbouring gap for the same node.

## Where it stands

Open, unscheduled. `work/chrome/plan.md` has no union-seat unit; this
file is the placeholder for one. DOCM-7 ships with the two-pass shape
pinned as a measured fact rather than left implicit —
`a_declare_cannot_name_a_union_that_does_not_exist_yet` and
`a_declared_unions_document_loads_but_does_not_replay_in_order`
(`crates/editor-core/tests/docm7_union_declare.rs`) — the second of which
also records that such a document LOADS (the load door checks the mint
counter, not `order()`) while re-inserting its nodes in document order
refuses. This node is the first to rely on that asymmetry.

(At DOCM's exit sweep, `refs` names the PRs `DOCM-7` stood for: `DOCM-7` = #2028 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Re-homed to EDIT, 2026-09-15

Moved out of `work/chrome/` by the CHROME orchestrator. The row offers
two fixes — a composing seat in the viewer, or a new DM6-shaped edit —
and only the second can actually exist: **no `DocEdit` writes
`declare`.** Enumerating the vocabulary (`InsertNode`, `DeleteNode`,
`SetMembers`, `SetParam`, `SetStructuralParam`, `SetExpression`,
`SetDocParam`, `SetDocParamValue`, `Rebind`, `ReWitness`,
`ReWitnessBulk`, `SetAppearance`, `ClearAppearance`, `SetTolerance`,
`SetAppearanceMeta`) turns up nothing that attaches a declaration to a
live node, and `Node::declare_of` reads it on `Boolean` and `Union`
only. A viewer seat can only re-order edits that already exist, so the
door this needs is `crates/editor-core/src/edit.rs` — EDIT's ground
since DOCM's exit (`docs/DOC-LEDGER.md`, sweep 14).

**Citations repointed by subject**, both having rotted:
`EditError::DeclareNamesMissingNode` is still raised at the
`InsertNode` payload-name check in `crates/editor-core/src/edit.rs`
(the `:1482` band is gone); and the *"no edit rewires a live node's
inputs (DM6)"* sentence the row leans on is no longer at `node.rs:2429`
— that band is now `Node::Pattern | Node::PlacedUnion` slot lists — but
in the `declared_union` fixture's own doc comment in
`crates/editor-core/tests/docm7_union_declare.rs`. Both named pins
survive: `a_declare_cannot_name_a_union_that_does_not_exist_yet` and
`a_declared_unions_document_loads_but_does_not_replay_in_order`.

The viewer-side consequence of this gap stays on CHROME's slate as
`work/chrome/addboolean-doc-names-a-vocabulary-that-does-not-exist`.

Signed: (CHROME orchestrator)

## Question for Ev (2026-09-17, EDIT orchestrator) — on the fourth `[ev]` PR

DM6 says no edit rewires a live node's inputs. A union's `declare`
edge names entities in the union's OWN name space, so it cannot be
authored before the union exists and cannot be attached after — the
only path is the five-edit workaround this row records. The question
is whether DM6 admits the one edge that by construction cannot precede
the node it names (a narrow `SetDeclare` on a live `Union`/`Boolean`),
whether the declaration should instead become the union's own payload,
or whether the workaround stays and CHROME seats it. The
recommendation and the alternatives are on the PR; this row is parked
on Ev's answer.

## Revised on the `[ev]` PR (2026-09-17)

Ev asked whether "declare names pre-boolean entities" is possible. It
is how the pair `Boolean` already works (a bare name resolved through
the operands' tables, one pass — `kiss_carry`); it fails for the n-ary
`Union` only because DM4's members can carry identical tables (21
transforms of one ball), so a bare pre-union name cannot say which
member — which is why DOCM-7 made the declaration name the union's own
`FromMember` rows. A SITED declaration (`SitedRef { at: member, name }`
per side — the shape a mate head already has) says which member
without naming the union: one pass, no DM6 exception, and
`DeclareBothOperands` unnecessary by construction. Revised
recommendation: (B) sited declarations, a DOCM-7 shape change (the
persisted `Declare` form, two suites, five corpus documents, the
Python constructor); the narrow `SetDeclare` stays the cheap
alternative. Waiting on Ev.

## RULED (2026-09-17, Ev on `[ev]` PR #2795): sited declarations

A `Declare`'s pairs name SITED entities — `SitedRef { at, name }`,
`at` the member (a pair boolean's operand), `name` the entity in that
member's table — so a declaration names what exists before the union
and is authored in one pass; the union derives each pair's fold step
from its two sites; the site is the side, so `DeclareBothOperands`
retires; the union's published `FromMember` names are unchanged. DM6
is untouched. The DM4 clause is amended on this PR; the DM7 sentence
that cited the old cycle is re-worded with it. Kernel unit (the
resolver in `eval/wire.rs`, the persisted `Declare` form, the two
DOCM-7/8 suites and the five declaring corpus documents, the Python
declare constructors — LIB's, mechanical): v6 dual, block EDIT-B2
slot 0, spec `docs/EDIT-DECL-SPEC.md` at the next claim.


## Spec'd (2026-09-17, EDIT orchestrator) — kernel unit, v6 dual, block EDIT-B2 slot 0

`docs/EDIT-DECL-SPEC.md`; branch `edit/sited-declarations`. Eight
premises: the sited payload (`SitedRef` reused), the site is the side
(`DeclareBothOperands` retires, one new site arm), the union routes by
site and rewrites into member space before the shared resolver
(look-through unchanged), fold-minted rows stop being declaration
subjects by type (the DOCM-7 fold-row rows retire with the class,
each replaced by a cannot-be-written row), the doors follow the
payload (`payload_names` the names, `payload_read_sites` the sites),
the persisted form moves and every declaring document in the tree is
re-authored (no migration — the format is unversioned and nothing
outside the tree declares), the façades follow mechanically (LIB's,
announced), DM6 untouched. Pre-draw fields at the block record
(`edit/b2-block`): difficulty **M**, task-class **STRUCTURAL**.


## Built (2026-09-17) — `edit/sited-declarations`

**A declared pair names two SITED entities.** `Node::Declare`'s payload
is `Vec<((SitedRef, SitedRef), ContactClass)>`: `SitedRef { at, name }`,
`at` the operand the entity is read at (a member, for a union) and
`name` the entity in that node's own table. `SitedRef` is reused —
no twin was minted. A declaration therefore names only what exists
BEFORE its consumer, and `declared_union` is two edits: the `Declare`,
then the union carrying its edge. The fixture's first union, its
`Rebind` loop and its delete are gone.

**The site is the side.** `side_by_operand` maps a pair boolean's two
sites to operands (`at == a` → A, `at == b` → B, anything else
`NodeErrorKind::DeclareSiteNotAnOperand { at }`); `route_declarations`
maps a union's to the member index, and to the side that index takes
at its step. `declare_landing` reads ONE table — the one the site
picked — so `DeclareBothOperands` retires and the pair boolean now
declares between two placements of one prototype.

**The union routes by site.** Bucket `max(i, j) − 1`, the joining
member operand B and the accumulation operand A; each pair is rewritten
into the node's member space by `names::member_name` (one definition of
the member-keying rule, shared with `member_view`) before the pair
boolean's own resolver runs, so `look_through_merges` and DOCM-8's
rows are unchanged in meaning. Every sited pair has a step, so
`UnionDeclareStep` and `step_diagnosis` are gone with the class they
answered for; `DeclSite`, `declared_bucket` and `latest_member` went
with them.

**Fold-minted rows are unrepresentable.** The four DOCM-7 rows that
pinned that class retired, replaced by
`a_declared_pair_side_that_is_a_bare_name_does_not_load` (a serde
refusal on the persisted form) and by the type itself.

**The doors follow.** `payload_names` yields the two names,
`payload_read_sites` the two sites; `Rebind` rewrites a name and leaves
its site; `refactor`'s remap moves both halves. The persisted pair
codec moves with no migration (the format is unversioned and nothing
outside the tree declares); six corpus documents' persisted-text pins
moved and every name-table pin held.

**Not built, and why**: nothing the spec asked for was left. Three
premises were corrected against the tree and are argued on the PR —
`persist/pairs.rs` is the appearance-store codec and not the declare
one; `pncad::select`'s declare doors are re-exports of `editor-core`'s
and needed no change; `FlushFinding`'s pair became sited, because
`declare_node(&findings)` holds no consumer context and could not have
sited a same-operand carried finding at all.

**CI**: run `35200292463` green on the code head
`a18cf07657c8d106bb7c4a0a6e0b7cf642f8d9a7` (39 jobs: 33 success, 6
skipped, 0 failed; twelve `test (…)`, five `k-lint (gate, …)`, the python
suite, and no step in any job with a non-success conclusion). Run
`35197936603` was green on the head before it merged main. The commit
that writes this paragraph is doc-only on top of that code head and is
green on its own run, recorded on the PR.

An earlier run was red in six jobs from ONE cause, recorded on the PR:
main had moved `asm_r2a_mate_solve.rs`'s import block, and the auto-merge
of this branch with main dropped the `SitedRef` the branch had added to
it — green on the branch tip, red on the PR's merge ref. **A PR run
builds `refs/pull/N/merge`, not the branch tip**, so a branch that is
green locally can be red in CI for a conflict git resolved silently.
Main is merged in and the import restored.

## Built at the fix pass (2026-09-17) — the v6 dual review's union

Two blinded reviewers read the frozen head and converged on one MAJOR;
the orchestrator's twelve rulings are built here. What moved:

**The union's refusal against a row its own fold minted.** `sited_member`
pushed every finding through one `Option`, so a `Merged`/`Fragment`-tailed
row degraded the whole refusal to `NamingError::Emission` — the crate
blaming itself for a user's document. It is now total
(`DeclarationSubject::{Member, Merged, FoldMinted}`): a merged row's
contact is `UndeclaredContact` sited at the CONSTITUENT whose member
comes first in the list, with the whole flat set carried beside it and
named in the refusal's prose, declarable verbatim; a row no member
stands for is `NodeErrorKind::UndeclarableContact { row, diag }`
(`undeclarable_contact`). `docs/EDIT-DECL-SPEC.md`'s amendment records
that premise 4 keeps its decision and premise 7's "no case" is
withdrawn.

**One home for site → operand.** `site_operand` answers "which operand
does this site name" for both declaring doors, rung 1 first; the pair
boolean used to refuse `DeclareSiteNotAnOperand` above `NodeGone`. The
rung-1 token travels with the name instead of being paid twice
(`SidedName::{Live, Rewritten}`).

**Pins and honesty.** `Node::Declare` gains the `compile_fail` +
running-twin pair for "a bare name does not typecheck"; the serde
refusal row asserts what the message says; six stale sentences fixed
(`decl_site`, `latest_member`, two `wire_union`/`route_declarations`
paragraphs, `node.rs`'s and Python's "dropping a member re-derives the
routing" — it refuses); the mutant table re-run with eight writable
mutants, each naming the row it reds.

**Both review branches merged authorship-preserving**, every probe
re-headed to the invariant it pins and every instrument dropped.

**Filed, not built**: `work/lib/python-flush-findings-do-not-carry-their-sites.md`
and `work/edit/a-declare-orphaned-by-a-cascade-is-never-reported.md`.

**Not buildable as ruled**: the typed arm was to get a row per
fold-row kind, "`Seam` at least". A contact refusal resolves a FACE
pair, and `RoleSeg::Seam` mints only edges and vertices while
`OutputBody` names the body, so `Fragment` is the only kind reachable;
`docm8_flat_merged::a_contact_against_a_fold_minted_fragment_is_undeclarable`
pins it and states the reach.

## Closed (2026-09-19, EDIT orchestrator)

Built and merged as PR #2809 (kernel unit, v6 dual, block EDIT-B2 slot
0; sample #216, ordinals 4804/4805). Ev's ruling is the code: a
declared pair names two SITED entities (`SitedRef`, reused), the site
is the side (`DeclareBothOperands` retired; one site door,
`site_operand`, after rung 1 at both doors), the union routes each
pair to the step its two sites derive and rewrites it into member
space by `names::member_name` — the one member-keying rule — before
the shared resolver, and a declaration is authored in one pass: the
five-edit workaround is gone. Three spec premises were corrected by
the implementer before building (`persist/pairs.rs` is the
appearance-store codec; `pncad::select`'s declare doors are
re-exports; `declare_node` holds no consumer context, so the FLUSH
FINDING carries its sites). The dual converged on one MAJOR: a
union's undeclared contact against a fold-minted row degraded to an
"emission bug" — the fold-minted class Ev's ruling made unrepresentable
is exactly where the detect→declare protocol was not total. The
orchestrator ruled the refusal's shape, the decision unchanged: a
`Merged` row's contact is refused `UndeclaredContact` sited at its
first constituent (any constituent declares the same contact through
the look-through, so the choice is immaterial and said so; the flat
set rides the finding), and a row no member stands for refuses typed
`UndeclarableContact` — `Fragment` being the only such face row a
contact can reach, since `Seam` never mints a face. The spec carries
the amendment (`## Amended at the fix pass`), and **Ev is told on the
next `[ev]` PR** that a mechanism of the ruling moved. The fix pass
also pinned the site-is-the-side direction the suite lacked, both
sides of `payload_read_sites`, the compile-time twin on
`Node::Declare`, and cut the name-table-hash claim to the one corpus
union it measures; two rows filed
(`work/lib/python-flush-findings-do-not-carry-their-sites`,
`a-declare-orphaned-by-a-cascade-is-never-reported`). Territory
crossed by announcement: WIRE (`eval/{mod,wire}.rs`, `names/flush.rs`),
FIX (`refactor.rs`), LIB (`tags.rs`, `tests.rs`, `py/flush.rs`,
`py/doc.rs`), 33 TCOST/TINT suites, six corpus documents re-authored.

---
id: addboolean-doc-names-a-vocabulary-that-does-not-exist
kind: issue
title: SessionOp::AddBoolean's doc promises a declaration vocabulary that no DocEdit provides
status: closed
branch: author/declared-union
opened: 2026-09-15
priority: P0
cost: D
closed: 2026-09-30
pr: 3543
---

Found while auditing CHROME's slate against the tree on 2026-09-15,
one hop from `a-declared-union-has-no-one-pass-authoring-path` (since
re-homed to `work/edit/`).

## The finding

`SessionOp::AddBoolean`'s doc comment in
`crates/viewer/src/session/op.rs` explains why it authors `declare` as
`None`, and then says:

> A declaration is added afterwards through the vocabulary that owns
> it, never guessed at here.

**There is no such vocabulary.** No `DocEdit` writes `declare`.
Enumerating the whole vocabulary — `InsertNode`, `DeleteNode`,
`SetMembers`, `SetParam`, `SetStructuralParam`, `SetExpression`,
`SetDocParam`, `SetDocParamValue`, `Rebind`, `ReWitness`,
`ReWitnessBulk`, `SetAppearance`, `ClearAppearance`, `SetTolerance`,
`SetAppearanceMeta` — turns up nothing that attaches a declaration to a
live node, and `Node::declare_of` reads it on `Boolean` and `Union`
only. The only way a declaration reaches a document is to be present in
the `InsertNode` payload that mints the node, which is precisely what
this tool declines to do.

So the sentence does not describe a deferral. It describes a door, in
the present tense, that does not exist — and it is the *reason given*
for a design decision, which makes it load-bearing rather than
decorative. A reader deciding whether `AddBoolean`'s `None` is
acceptable is told the gap is covered elsewhere.

## Why this is not just a comment fix

`docs/prompts/reviewer-style-lane.md` Q4 names the two cases and they
need different handling:

- **the doc rotted while the code stayed right** — fix the sentence; or
- **the code drifted from something that was meant to hold** — a latent
  defect in a documentation costume, where deleting the sentence erases
  the only record of an intended invariant.

This looks like the second: the sentence reads as a plan (`Node::Declare`
exists, `declare_of` reads it, the fixture `declared_union` performs a
five-edit dance to fake the one-pass path), so the vocabulary was
evidently *intended*. Deleting the sentence would leave the viewer
authoring `None` with no record that anything was ever supposed to fill
it. The honest repair is to say what is true — that nothing can add a
declaration after the fact, and that the row tracking it is
`work/edit/a-declared-union-has-no-one-pass-authoring-path` — rather
than to strike the claim.

## Territory

`crates/viewer/src/session/op.rs` is CHROME's and VIEW's by double
claim, and VIEW is live in that file (four open VIEW rows cite it). The
fix is a doc comment and nothing else, so it should ride whichever
program next has reason to touch the file rather than open a PR of its
own.

## The class this belongs to

A comment asserting a door that does not exist is the shape
`reviewer-style-lane.md` Q5 asks about — *"what does this promise that
it doesn't do"* — and one instance is not evidence about the
population. **Nobody has swept `crates/viewer/src` for it.** The
pattern is prose in the future or elsewhere tense — "is added
afterwards", "through the vocabulary that owns it", "the door that
authors one is", "handled upstream" — checked against whether the named
door exists. That sweep has not been run and this row does not claim
its result.

Signed: (CHROME orchestrator)

## What the gap COSTS, measured (2026-09-21, AUTH-1's fix pass)

This row has said since it was filed that no `DocEdit` writes
`declare`. What it has not carried until now is an ordinary gesture
the gap blocks, driven and failing.

AUTH-1's fix pass tried to make a test row discriminate by unioning a
boss into the block it sits on — the block, a profile on the boss's
face frame, an extrude, then a union. **The union refuses**, because a
boss drawn on a face frame is FLUSH with the block at that face **by
construction**, and the kernel refuses an undeclared coincident
contact:

```
SessionOp::AddBoolean { op: Union, a: block, b: boss }
  → UndeclaredContact { finding: FlushFinding {
      pair: (block's Cap(End), boss's Cap(Start)), class: Rest, … } }
```

The declaration that would resolve it is a `Declare` node
(`Node::Boolean::declare`), and `SessionOp::AddBoolean` has no seat
for one — which is this row.

**So the gap is not only a doc promising a vocabulary that does not
exist; it is a door the GUI cannot author at all.** "Draw a boss on a
face and add it to the body" is about as ordinary as CAD gets, and it
is exactly the class this program was cut to carry (`work/author/program.md`:
what the GUI cannot author). A face frame makes the flush case the
COMMON case rather than a corner: every boss authored on a picked face
lands coincident with what it was drawn on, so AUTH-1 shipping makes
this row's sharpness worse rather than better.

Recorded here rather than as a new row, per
`docs/prompts/implementer-discipline.md` §6 — one file per item, and
this row already owns the declaration vocabulary. What is added is the
evidence and the cost, not a second finding.

**A consequence for whoever specs this**: the boss-on-a-face gesture
cannot be asserted end to end by volume through the op vocabulary
today. AUTH-1 asserts the frame's landed POSE instead (origin, normal
and sketch +x, verified by mutation), which discriminates strictly
more than the volume would have. A spec that asks for a sum-of-volumes
acceptance row on this path is asking for something unreachable until
this row closes.

## Unblocked and dispatched (2026-09-30)

The plan parked this row until EDIT's vocabulary existed. EDIT closed that dependency on 2026-09-19: `a-declared-union-has-no-one-pass-authoring-path`, #2809, Ev's ruling, "a declaration is authored in one pass". Nothing needs to attach a declaration after the fact, because the declared union is authored as `Declare` then `Boolean` in one action. So this is now a door rather than only a doc fix: the flush boss-on-face union AUTH-1 measured refusing `UndeclaredContact` becomes a union the author can declare and land. Dispatched as **AUTH-9** (`docs/AUTH-9-SPEC.md`, branch `author/declared-union`).

## Built (2026-09-30, AUTH-9; PR #3543, fix pass included)

**The flush union is a refusal with an offer, and accepting it is one
action.**

- `SessionOp::AddBoolean` carries `declare: Vec<FlushFinding>`.
- The session door stages the run: a `Node::Declare` of those findings
  (`names::declare_node`) and then the boolean naming it, or the boolean
  alone when the list is empty.
- It evaluates the staged document beside the seam
  (`evalseam::evaluate_beside`). The memo is the landed run, used only
  if it resolved through the same seam (`memo_under`, `same_resolver`).
- It records the run unless the boolean fails with an undeclared
  contact of its own that it can declare (`tree::own_error`). If it
  does, nothing is committed.

The refusal is `Refusal::Contact(RefusedBoolean)` in `session::refuse`.
It renders the kernel's sentence whole, and carries the attempt and the
generation it was judged at. Its offer (`DeclareOffer`, read by
`frame::declare_offer` beside `creation_offer`) is the same op with the
finding appended. The offer is drawn under the boolean tool's commit
row, with its wording in `Refusal::declare_question` /
`declare_pair_wording` beside `offer_wording`. It stands only at that
generation, for that op and those picks.

Two cases are deliberately not offered:
- **A pair between two faces of one operand.** The pair boolean has no
  declared step for it, so it stays the node's own refusal.
- **A pair the attempt already declared.** That is a kernel defect, and
  the refusal says so.

**The three checks.**
1. **What the refusal carries.** The pair boolean's refusal is
   `NodeErrorKind::UndeclaredContact { finding, merged, diag }` from
   `eval::wire::refusal_menu`. `finding` is
   `topo::flush::FlushFinding<(SitedRef, SitedRef)>`; the struct is
   generic, so `pub struct FlushFinding` does not grep. It carries both
   sites and the class.
2. **How many contacts one refusal reports.** One pair per refusal
   (`UndeclaredCoincidence` carries one). The offer keeps every pair
   already accepted, and nothing is committed until the union
   evaluates.
3. **Where the class comes from.** The finding: `Rest`, set by the
   coincidence ladder. The author confirms it rather than picks it.

**Rows.**
- Panel rows in `crates/viewer/src/pane/create.rs`, `declared_union`:
  - refusal → offer → Declare → one undo;
  - Decline drops the offer;
  - the offer is dropped once the picks, the op or the document move
    past it.
- Door rows in `crates/viewer/tests/combine_ops.rs`:
  - a poisoned boolean commits and offers nothing;
  - two contacts land as one action, at the exact volume;
  - a non-flush union authors no `Declare`.
- Unit rows in `session/refuse.rs`, `refused_boolean`: a re-raised pair
  is a defect, not an offer; a same-operand pair is not offered.

**Left open, filed:**
- `the-boolean-door-evaluates-its-boolean-twice`
- `a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added`
  (design)
- ZIP's `a-boss-flush-with-a-block-edge-refuses-its-declared-union`
- ZIP's `a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence`
- OFFER's `a-refusal-offers-no-action-in-the-viewer` (evidence: the
  doubled recourse)

## Closed 2026-09-30 — PR 3543 merged (`8bc49ad7`)

**A boss drawn on a face can be added to the body it sits on.** The
union's refusal offers to declare the contact the kernel found, and
accepting commits the `Declare` and the union as one action and one
undo. `SessionOp::AddBoolean`'s doc now says what is true: a contact
is declared in the same action or not at all.

Both reviews said mergeable. Correctness found no defect: the judge
reaches the seam's verdict (same options, tolerance and resolver; the
memo changes cost, not the answer), and no path loops. The fix pass
put the offer into the homes the viewer's first offer already had
(`session::refuse`, `frame`), and took every hardening the correctness
review listed.

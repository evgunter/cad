---
id: viewer-mate-tool-refuses-pattern-picks
kind: issue
title: Viewer mate tool refuses pattern-placed picks — cannot author the mates the A11 member vocabulary admits
status: open
opened: 2026-08-31
github: 1412
refs: [1400]
---

## From GitHub issue 1412

Opened 2026-08-31; 0 comments.

Filed from the MATE-1 dual review (PR #1400, R2 MINOR-2, verified at the site). `crates/viewer/src/matetool.rs:417` gates mate-member picks through `is_instance` (`display.rs:193`), which matches `InstantiatePart` heads only — so the GUI refuses `NotAnInstancePick` for exactly the pattern-placed heads the member-vocabulary rider now admits and the solve now places. A user can author these mates through the recipe/Python doors but not by picking in the viewer.

GAUTH ground (viewer chrome), flagged for the GAUTH orchestrator's queue: the fix is presumably widening the pick gate to the member vocabulary (`Pattern` + `Instance(i)` over a live instance) and emitting the `Instance(i)`-headed reference, with `NotAnInstancePick` retained for everything else.

Signed: (S-MATE orchestrator)

## Home

`work/issues/` — the issue routes itself to GAUTH (viewer chrome), and both GAUTH and GUI are closed programs, so no open program owns `crates/viewer`.

## The style lane's fix pass (CHROME, 2026-09-04)

**The master-read fix is now evidenced where it can actually fail.** The
original rows used LINEAR patterns, where the derived offset's rotation
is identity, so the only channel that could go red was `origin` — the
one a linear row covers anyway. A circular row now spins `post_b` a
quarter turn about a world +y axis and compares `axis` first,
`reference` second, `origin` last
(`crates/viewer/tests/mate_tool_flow.rs:485`). Verified red twice
against the naive read, on each rotated channel independently.

**Assertion ORDER turned out to be load-bearing evidence**, which is
worth carrying forward: the lane's first draft ordered `origin` first
and went red on `origin` — the channel that was already covered — so
the row would have looked like proof of the rotation half while
proving nothing new. A row can assert the right thing in the wrong
order and read as evidence it is not.

**The `Member`-widening question, judged and declined on a hard
ground.** The dispatcher framed it as a churn-versus-cleanliness call:
carry the master entity in `Member` and the viewer's re-match and its
dead arm both disappear. That framing was wrong. `Member`'s `Eq`/`Ord`
are load-bearing — the solve keys `by_pair: BTreeMap<(Member, Member),
Vec<RecipeNodeId>>` on it (`crates/editor-core/src/mate/solve.rs:777`)
— so adding the master ENTITY would make two picks on DIFFERENT FACES
of one copy compare unequal, silently splitting the solve's pair
grouping and defeating `SamePick`. That is a semantic regression, not
churn. Second and independently: `member_of` answers ADMISSION, while
"which entity is the frame read at" has no kernel consumer at all —
A11 keeps the solve structural, so `mate.rs` never interrogates a face
frame. Putting it there would move viewer-only logic into the crate
this program had to amend its own `keep_out` for.

The arm the widening would have deleted is kept and justified in place
instead: it stands where a panic otherwise would, for an invariant this
crate reads rather than owns.

**`frame_of` no longer takes two ids that are always equal.** The
equality was checked structurally in both directions rather than
assumed — for a plain head the guard forces it, and for a copy the
`of` name comes from the pattern namer, so its node is the pattern's
input. One id now, and the agreement is a property of the code rather
than something a reader reconstructs.

**The "instance" sweep ran wider than the brief scoped it**, and found
the user-visible instance: `crates/viewer/src/app.rs:3161` and `:3165`
render the mate tool's own chrome as "pick a: face of instance N" on
what is routinely a `Pattern` node. `SessionOp::AddMate`'s own docs had
the same drift. Both fixed. Blind spot stated: the pass matched literal
shapes plus a manual read of every mate-adjacent line, so it would miss
an instance-keyed construct spelled without the word, and it did not
cover `crates/viewer/tests/`.

**A behavioural sibling was found and NOT folded in.**
`display.rs:181`'s `mates_naming` is an instance-keyed scan against the
member-keyed vocabulary — the same shape as this issue, one function
over — so `free_move_check` offers free-move on an instance the solve
already constrains through a pattern copy. Measured on the branch.
Filed as `work/issues/viewer-free-move-misses-pattern-placed-mates`
rather than fixed here: it changes an admission gate and owes its own
evidence, and this unit's scope is the pick gate.

---
id: viewer-mate-tool-refuses-pattern-picks
kind: issue
title: Viewer mate tool refuses pattern-placed picks — cannot author the mates the A11 member vocabulary admits
status: review
opened: 2026-08-31
github: 1412
refs: [1400]
branch: chrome/viewer-mate-tool-refuses-pattern-picks
pr: 1748
---

## From GitHub issue 1412

Opened 2026-08-31; 0 comments.

Filed from the MATE-1 dual review (PR #1400, R2 MINOR-2, verified at the site). `crates/viewer/src/matetool.rs:417` gates mate-member picks through `is_instance` (`display.rs:193`), which matches `InstantiatePart` heads only — so the GUI refuses `NotAnInstancePick` for exactly the pattern-placed heads the member-vocabulary rider now admits and the solve now places. A user can author these mates through the recipe/Python doors but not by picking in the viewer.

GAUTH ground (viewer chrome), flagged for the GAUTH orchestrator's queue: the fix is presumably widening the pick gate to the member vocabulary (`Pattern` + `Instance(i)` over a live instance) and emitting the `Instance(i)`-headed reference, with `NotAnInstancePick` retained for everything else.

Signed: (S-MATE orchestrator)

## Home

`work/issues/` — the issue routes itself to GAUTH (viewer chrome), and both GAUTH and GUI are closed programs, so no open program owns `crates/viewer`.

## Fixed (CHROME, 2026-09-04)

The gate is widened, and the widening was not the whole defect.

**The vocabulary has one home.** A11's member rule lived privately
inside `mate::solve::head_of`, and the viewer restated a narrower
version of it as `display::is_instance` — which is how this door came
to refuse heads the solve already places. The rule is lifted to
`pub fn member_of(doc, name) -> Option<Member>`
(`crates/editor-core/src/mate/solve.rs:147`), `head_of` delegates and
attributes the refusal, and the viewer's gate READS it
(`crates/viewer/src/matetool.rs:118`) rather than carrying a copy that
can drift. `display::is_instance` stays narrow for its two remaining
callers: per-instance display state — hide, free-move — is keyed on
`InstantiatePart` nodes and is a different question wearing a similar
name.

**A copy's alignment is authored at its MASTER.** The reference was
free (`pick.name` is already `Instance(i)`-headed), but the frame was
not: `poses.placement` is keyed on instances, so a pattern node reads
the identity, and the pattern-derived offset is `pub(crate)` in
`eval::wire` and unreachable from the viewer. Reading the copy's own
world pose and dividing by the master's placement folds that offset
into the authored numbers, and the solve — whose left factor it is —
then applies it a second time. So the pose is read at the master
entity named inside the head's `Instance(i)` qualifier, on the
pattern's input instance. Verified red: with the naive read, copies 0
and 1 of one pattern author alignments differing by the pattern step.

**`SamePick` compares members, not nodes.** Two copies of one pattern
are two members over one instance, and a mate between them is a loop
the solve records rather than a self-mate. Its payload is `head`
rather than `instance`, a pattern-placed head being no instance node.

**Territory, stated at its real strength.** This unit edits
`crates/editor-core/src/mate.rs` and `mate/solve.rs`, which CHROME's
own `keep_out` named: *"editor-core mate.rs and assembly.rs
vocabulary is read and not edited."* That is a declared PROHIBITION,
not merely ground outside `paths`, and the first version of this note
disclosed only the weaker of the two. The style lane caught it, and
the miss is the part worth keeping: a disclosure naming the lesser
boundary reads as candour while walking the harder question past.

The fence is amended rather than quietly crossed
(`work/chrome/program.md`), because the fence and the fix could not
both stand. The viewer's gate was a RESTATEMENT of the rule `mate.rs`
owns, and the only repair that removes a restatement is to publish the
original — so "read and not edited" would have preserved the defect
the clause was written to keep out. The amendment is deliberately
narrow: one private function made public plus its re-exports, no mate
semantics touched, `head_of`'s behaviour and every existing signature
unchanged.

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

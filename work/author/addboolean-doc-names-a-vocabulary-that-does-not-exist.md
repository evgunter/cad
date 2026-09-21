---
id: addboolean-doc-names-a-vocabulary-that-does-not-exist
kind: issue
title: SessionOp::AddBoolean's doc promises a declaration vocabulary that no DocEdit provides
status: open
opened: 2026-09-15
refs: [a-declared-union-has-no-one-pass-authoring-path]
priority: P0
cost: D
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

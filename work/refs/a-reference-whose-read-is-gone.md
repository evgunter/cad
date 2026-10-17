---
id: a-reference-whose-read-is-gone
kind: ruling
title: What a reference whose read has left the document resolves to (N5 gains ReadGone); whose path the with-history lanes read
status: closed
opened: 2026-10-11
priority: P1
pr: 4674
closed: 2026-10-17
---


From INTENT DM4 unit 2 (`a-name-is-scoped-by-the-variable-holding-its-body`,
PR 4674). Both reviewers executed the case. Two designers weighed it
(fork-log row 114).

## The case

A reference is sited at `t_read`, the output of node `t`. `DeleteNode t`
runs; the name's minting node still exists. `resolve` answers
`Vanished { RecipeEdit { NodeChanged(<the minting node>) } }`, which is false.
Two more doors give false answers too:
- the node-keyed door says the minting node was deleted;
- the appearance loss cause conflates the two states.

## Q1. A fourth N5 arm (both designers: yes)

`ReadGone { name, read, edit }` is tried first. It is decided from the
document alone, and its repair is to re-site or remove the reference,
never Rebind. N5's three arms date from when a reference was a bare name.
DM4 (FORK-DM4 N1, Ev 2026-10-09) made the read a second coordinate.

## Q2. Whose path the with-history lanes read

- **The diff:** the reference's path is the nodes the name mentions plus
  the node that defines the read. FORK-DM4 N1 forces this.
- **The wider option:** one designer would also add every node on a path
  from the minting node to the read. Without it, a flip at a split between
  the mint and the read is still blamed on the minting node.

## Ruling (Ev, PR 4691)

- Q1: the four-arm text, as recommended (2026-10-12: "the recommendation
  sounds good for the first").
- Q2: the wider path (2026-10-17: "i think the wider path makes sense, but
  feel free to stage via the narrower path"). The narrower path lands with
  PR 4674; the wider one is
  `work/refs/the-reference-path-reads-every-node-from-mint-to-read.md`.

## Closed

PR 4674 builds the ruling as staged. Q1: `ResolveError::ReadGone { name,
read, edit }` is N5's first rung, ahead of `NodeGone`, with
`AppearanceLossCause::ReadGone` beside it. A live read that holds no body
is refused typed (`ResolveRefusal::ReadNotAnOutput`). Q2: the reference's
path is the nodes the name mentions plus the node defining its read. The
wider path, every node from mint to read, is
`work/refs/the-reference-path-reads-every-node-from-mint-to-read.md`.

---
id: a-reference-whose-read-is-gone
kind: ruling
title: What a reference whose read has left the document resolves to (N5 gains ReadGone); whose path the with-history lanes read
status: open
opened: 2026-10-11
priority: P1
needs_ev: true
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

The build (PR 4674) carries Q1 and the narrower Q2 meanwhile, and merges
after Ev answers.

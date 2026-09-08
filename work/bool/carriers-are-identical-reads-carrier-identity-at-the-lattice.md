---
id: carriers-are-identical-reads-carrier-identity-at-the-lattice
kind: issue
title: family.rs's carriers_are_identical (key path_carrier_identity) decides whether two carriers are the same inside the fillet family's arc extension — a lattice question the sixth-round ruling retires
status: open
opened: 2026-09-08
refs: [BOOL-10, 2135, BOOL-12]
---

Found by BOOL-10's seal measurement (PR 2135): after `arc_continue`'s
removal, the remaining chain-side readers of `Incoming.carrier` are the
fillet family's — `resolve_fillet`'s Positive-fit arm, `family::merge_of`,
and `carriers_are_identical` (the §2c dissolution amendment's
arc-extension vertex-move choice, under the key `path_carrier_identity`).
The last is exactly an "are the carriers the same" question, which the
Q1 sixth-round ruling (Ev, 2026-09-02) says the lattice never asks. It is
ruling-adjacent and outside BOOL-10's fence, so it is filed rather than
deleted: the unit that takes it decides whether the arc-extension choice
re-spells as a declared thing (the vertex moves because the author said
so) or whether the amendment's case is one the ruling did not cover, and
puts that to Ev. `verbs.rs`'s header names the readers as of BOOL-10.
Difficulty S/M. Not scheduled.

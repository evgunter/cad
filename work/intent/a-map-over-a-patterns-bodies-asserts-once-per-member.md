---
id: a-map-over-a-patterns-bodies-asserts-once-per-member
kind: issue
title: A map over a pattern's Bodies states one assertion per member, and a map over pairs of members states one per adjacent or every pair
status: open
opened: 2026-10-08
priority: P0
design: true
refs: [interference-at-rest-is-a-finding]
---

Ev, 2026-10-08, on interference between a pattern's outputs: "we should
have a map higher order function to allow declaring over all the
outputs of the pattern." D10's Assertions paragraph (`docs/DESIGN.md`)
now states the rule; nothing builds it.

Today an assertion reads one `Gap` over one opposed face pair of two
copies (stage 5's quieting rule, `docs/INTENT-STAGE5-SPEC.md` §1). A
pattern of twelve fins sunk into a base is twelve interference
findings, and quieting them takes twelve hand-written assertions that
do not follow an edit of the pattern's `Count`.

Two shapes are needed:

- **A map over a pattern's `Bodies`.** One authored assertion, a
  function of one member, defines one assertion per member; each reads
  the same bound variable, and their number follows the pattern's
  `Count`. That covers member-against-other-copy (each fin against the
  base).
- **A map over pairs of members.** Member-against-member overlap
  (neighbours in a ring of blades) needs one assertion per pair: the
  adjacent pairs (`i`, `i+1`, closing for a circular pattern) or all
  pairs.

Design open, for a designer pair: whether a map is an operation
defining a `Bodies`-indexed family of assertions or an `Assert` whose
site is a selection family; how a member's face is named inside the
map (the member's `Instance { i, of }` names, DM3's index output); how
each mapped assertion reports a verdict and quiets its own finding;
and whether the map generalises past assertions (a measure or a mate
per member). Lands with or after stage 5 B
(`interference-at-rest-is-a-finding`), which owns the quieting rule.

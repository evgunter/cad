---
id: a-family-read-at-a-body-seat-maps-implicitly
kind: issue
title: An operation that reads a family maps over its members implicitly; Ev: mapping over a list should be written, not inferred
status: open
opened: 2026-11-22
priority: P1
cost: H
refs: [a-subtract-tool-seat-takes-a-whole-family-and-cuts-once-per-member, a-split-of-a-family-has-no-halves-a-read-reaches]
---


Ev on `[ev]` 7386 (2026-11-22): "i am confused about how the issue was
possible, though. we shouldn't be implicitly mapping over lists! i
would've thought there'd be a difference in how those were written,
like `{k from 1 to N: union(A, {f(k)})}` vs `union(A, {k from 1 to N: f(k)})`
(i don't remember the actual syntax)".

Today D10 `[repetition]` has no binder. "A definition or operation whose
reads reach `k` is evaluated once per value of it": membership is
derived from reads. So `subtract(plate, holes)` silently means
`{k: subtract(plate, holes[k])}`, and the subtract-by-a-pattern trap
(`a-subtract-tool-seat-takes-a-whole-family-and-cuts-once-per-member`,
closed by making the tool a `Bodies` seat) is one instance of the
class. It came in at FORK-PAT (`docs/DESIGN-FORK-LOG.md` row 99,
2026-10-08, commit 132d3e4716). Ev had asked whether "a map
higher-order function" would suffice; the designers moved away from a
stored `map` ("restates its domain per step, kept in step by hand") to
derived membership.

P1: architecture that entrenches. Every family-reading door (the split
of a family, `a-split-of-a-family-has-no-halves-a-read-reaches`; the
seats; Python) builds on the implicit map. Designers weigh it, and the
decision goes to Ev: it changes a ratified decision.

## The design (2026-11-22)

Repetition is written. A binder `{k in D: …}` is a scope in the recipe,
and nothing repeats outside one. Derived membership (D10 `[repetition]`'s
"a definition or operation whose reads reach `k` is evaluated once per
value of it") retires.

- **The domain** `D` is a range `0..N` or a family's own keys
  (`k in holes`), so lockstep states its domain once. A binder may bind
  several indices, `{i in 0..R, j in 0..C(i): …}`. Families are flat,
  keyed by the tuple, outermost first (FORK-PAT's flat ruling kept);
  `xs[i]` of a grid is row `i`, and `xs[i, j]` is one member.
- **Reading a family.** A list seat (union, intersect, assemble, a
  subtract's tool) takes it whole. A one-body seat (split, mirror,
  fillet, shell, a subtract's kept body, a placement's source) handed a
  family is refused at the door by kind, with the recourse
  `{k in xs: op(xs[k])}`. `xs[e]` is one member anywhere.
- **"Each member" is always written**, as a binder: `{k in holes:
  subtract(plate, holes[k])}` is N plates, and `subtract(plate, holes)`
  is one plate with N holes.
- **Retires:** `within`, "a reader of two unrelated indices refuses",
  `repetition::takes_whole` and its seat exceptions, and
  `ValuePayload::Family` as a node's value. A split inside a binder
  yields two families, so the split of a family cannot arise.
- **Names** stay `Member { (i, j), of }`, keyed by the binder's index
  ids.
- **Python:** `doc.each(N, lambda k: …)`, `doc.each(holes, lambda k: …)`,
  `with doc.each(N) as k:`. The preset writers (`linear_pattern`,
  `grid`, `circular_pattern`) write a binder. **Viewer:** the pattern
  tool writes a binder around one placement. A family picked in the tree
  at a one-body seat offers "each member", which writes a binder the
  tree shows. A viewport pick on a drawn copy writes `xs[c]`.

- **The extent is derived from reads, with written yields** (Ev,
  `[ev]` 7409: "in a functional paradigm aren't these the same?"). A
  binder stores its index and the names it yields, and each yield is a
  family. Inside a binder, a step repeats exactly when it reads `k`,
  directly or through another definition. A step that reads nothing of
  `k` is the same at every member, so building it once is a cache.
- **Containment.** A definition that reads `k` may be read only by
  definitions that read `k`, or by its binder's yields. Any other reader
  is refused when written, and on load is unresolved and typed. A
  definition that reads `k` but that no yield reads is refused or
  reported, and a binder with no yield is refused.
- **A family spliced into a list** is written, `[A, …holes]`, each member
  keyed by its read (Ev, `[ev]` 7409: "... is good"). DM4's ban on
  mixing a family with single bodies retires.

Ratified text this changes once Ev rules: D10 `[repetition]` (its core,
`within`, the kind rule, "a pattern is an index, not an operation"),
`[placement.placement]`'s pattern sentence, `[assertions]` Families,
DM3's implicit-member sentences, and the sentence merged on PR 7386
("`subtract(a, holes[k])` cuts it member by member").

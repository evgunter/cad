---
id: a-clearances-rebuild-is-fixed-by-its-point-only-while-expressions-agree
kind: issue
title: A clearance served over a prior is held to the rebuild's point, not its bodies: an expression edited to another of the same enclosure and nominal rebuilds other bodies under the same keys
status: open
opened: 2026-11-23
---


Left by the member-read-key unit (PR 7512). By reading; no document
was built that shows it.

A clearance's verifier reads the `f64` rebuild beside a certifying run
(`eval::wire::Witnessing`): the document evaluated at the box's
midpoint. A prior clearance verdict that cites a pair is served only
where the rebuild's point is this run's (`eval_node`, the guard over
`clearance_point`). That fixes the rebuild's bodies as long as the
document's expressions are the ones the prior read, because the
rebuild is a function of the document and the point.

The lane's keys hash VALUES (each slot's enclosure and its nominal),
not expressions. So an edit that replaces a slot's expression by
another with the same enclosure over the box and the same nominal
leaves every lane key, and the point, where they were, while the
rebuild at the midpoint evaluates the new expression. For a box
`p ∈ [1, 3]` with nominal `p = 1`: `p` and `4 − 3/p` both enclose
`[1, 3]` and are 1 at the nominal, and are 2 and 2.5 at the midpoint.
A prior verdict over the first is then served over the second, citing
a pair found on bodies the second document does not rebuild.

PR 7512's first cut hashed the rebuild's key of each seat's operation
into the clearance's key, which closes this; the review had it taken
out because no row was red without it, and this construction was not
built into one (it needs the two enclosures to agree to the bit, which
outward rounding may not give).

It is the `f64` cousin of
`a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression`.

**Owed:** the construction as a row (red, or the reason the enclosures
never agree to the bit), and then the rebuild's seat keys beside the
point in the memo's guard.

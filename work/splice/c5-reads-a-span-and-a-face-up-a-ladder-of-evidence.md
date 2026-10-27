---
id: c5-reads-a-span-and-a-face-up-a-ladder-of-evidence
kind: issue
title: C5's curve × face clause reads a span and a face up one ladder of evidence, topology first, so twin_edge is the rule's first rung and not an exception
status: closed
opened: 2026-10-27
priority: P2
cost: E
branch: splice/ev-c5-evidence-ladder
closed: 2026-10-27
---


`topo::boolean::reduce`'s `twin_edge` (PR 5768) decides a span that is
an edge of the other operand by that operand's topology, under the
boolean's precondition that its operands are valid bodies. C5's
curve × face clause read only ends → boundary → interior, all by
geometry, so `twin_edge` sat outside its wording.

Ev (chat, 2026-10-27) asked for C5 to be rewritten more generally rather
than given an exception. The rewrite reads the pair up one ladder of
evidence:
1. topology (incidence, under the stated, assumed validity precondition);
2. the span's ends;
3. the face's boundary edges;
4. the face's interior.

Each rung reads only what the rungs before it left undecided.
`twin_edge` is then rung 1.

## Closed (2026-10-27, PR 5860)

Ev approved the rewrite on PR 5860 ("5860 looks good!", in chat). C5's
curve × face clause now reads a span and a face up one ladder of
evidence (topology, then ends, then boundary, then interior), with the
boolean's validity precondition named as assumed. `twin_edge` is its
first rung. Only the wording changed; the code is unchanged.

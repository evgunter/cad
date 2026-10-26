---
id: measure-holds-a-budget-refused-faces-bracket-at-the-round-its-sign-settled
kind: issue
title: measure()'s bracket stays at the round the sign settled on a budget-refused face, so a tighter ε reads a wider bracket
status: closed
opened: 2026-10-21
priority: P2
cost: M
closed: 2026-10-24
pr: 5628
---


## What

Split out of `check-7-refuses-the-reporting-budget-on-a-definite-sign`
("A second consequence"); measured unchanged by PR 5568.

`topo::props::SignCertificate::measure` continues a certificate to the
reporting target. A face its lane refused at the budget exit
(`geom_brep::props::quad::budget_exit`, `OpenEnd::Unreachable`) is not
resumed (`FaceRun::resumes_after`), so the bracket `measure` hands back
holds that face at the round the sign walk stopped at: round 0
wherever the sign settled there.

Measured on the release tour at PR 5568's head:

- `lily_leaf_b` at ε = 1e-12: bracket `[3.0860e-3, 3.1837e-3]`, about
  3 % wide, where the default-ε number carries a pad of 7.6e-8.
- `lily_leaf_a` at 1e-9 and 1e-12: `[1.756e-2, 5.187e-2]`.
- the three sepals at 1e-12: brackets about 2.5× wide.

So tightening ε buys a wider answer: the face's later rounds would
narrow the bracket, and nothing runs them.

## The fix's shape

The refused faces now keep their rounds open (`OpenEnd::Unreachable`
says rounds remain), so `measure` can resume them to the end of their
schedule for the bracket alone, after the continuation has refused,
without changing the refusal it reports. The cost is the refused
faces' remaining rounds, paid only by a caller that asked for the
bracket.

## Done when

`measure`'s bracket on a budget-refused face is the last round's (or
the round a stated budget allows), with a row that pins it narrower
than the round-0 bracket on one of the bodies above.


## Closed by measure door unit B (PR 5628)

The continuation (`SignCertificate::continue_to_target`) resumes every
face still short of the resolution (`FaceRun::resumes_to`) over the rest
of its schedule, to the face's own floor or work wall, and keeps each
face's narrowest round (`FaceRun::advance`). It no longer stops at the
first budget-short face. So the bracket `measure` hands back on a
budget refusal is the walls' last round's. A face a later round refuses
outright keeps the enclosure it held (`FaceStop::Refused`).

Pinned by `sweep`'s
`a_lofted_lens_blade_certifies_its_sign::a_round_tube_certifies_its_sign_while_its_number_is_refused`.
There, the gate stops each wall where the sign settles, and `measure`'s
budget-refusal bracket is strictly narrower than the gate's at all
three ε.

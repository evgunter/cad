---
id: the-shared-support-gates-annulus-beside-a-ladder-arm-has-no-fixture
kind: issue
title: The shared-support gate's annulus-beside-a-ladder arm has no fixture once a full revolve is one band
status: open
opened: 2026-11-23
priority: P3
cost: E
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
---

Found by the LATHE one-band lane (2026-11-23, PR 7414).

## What

`surgery::shared_support_gate` (`crates/sweep/src/blend/surgery.rs`,
the doc above `fn shared_support_gate`) refuses `UnsupportedChain` when
an annulus rim and a ladder rim of one request share a face. Its one
fixture was the boss of
`ring_clearance_forms::the_bosss_two_rims_carve_together_and_compose_sequentially`:
a revolve whose flat top runs in to a dome, merged, so one plane face
carries the dome rim as a ring and the top rim as its outer cycle. With
two half-bands per wall the top rim was seam-split and planned as a
ladder, which reached the arm. With one band per wall (PR 7414) both
rims plan as annuli, the call builds at the sum of the closed forms, and
no fixture reaches the annulus-beside-a-ladder refusal. The gate's doc
says so.

## Wanted

A fixture whose shared face carries a ring and a ladder rim (for
example, a plate's straight-sided outline beside a turned boss's rim on
one face), pinning the refusal in both orders with its sequential
recourse, or the arm lifted where an argument covers both walks.

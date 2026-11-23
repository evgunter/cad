---
id: carrier-on-surface-answers-unread-where-a-carrier-cannot-lie-on-the-surface
kind: issue
title: carrier_on_surface answers None (unread) for a pair that is decidably not on, such as a line on a sphere or torus, conflating cannot-lie with unread
status: open
opened: 2026-11-01
priority: P4
cost: E
refs: [the-parting-reads-no-edge-lying-in-a-face-no-record-names]
---

Found by the PR 6220 review (2026-11-01), unsure whether it bites:
`slit::carrier_on_surface` documents `None` as "nothing here reads",
yet it also answers `None` for a line on a sphere or a torus, which is
decidably not on. CLEAVE's PR 6177 has the same shape. A caller that
treats `None` as "refuse, unread" would refuse where the answer is a
plain `false`. Goes away if the parting reads edge-face records only
(`the-parting-reads-no-edge-lying-in-a-face-no-record-names`); until
then, split the two answers or say at the site which callers rely on
the conflation.

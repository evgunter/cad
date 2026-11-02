---
id: wrap-tip-rule-reads-no-joint-on-a-face-whose-rows-are-not-owed
kind: issue
title: The wrap-tip rule reads no joint on a face that stores no rows and whose derivation is not owed, so its tip goes unread
status: open
opened: 2026-10-31
---


The wrap-tip rule (`topo::pcurves::pcurve_pass`, PR 6097) reads a tip
at the joint check 8 decides there. A face that stores none of its rows
and whose derivation refuses as not owed (`not_owed`: an uncovered
class, or a fitted face at a scalar with no fitted door) is excused
before any joint is read, so a tip on it is never read. Check 8 is
silent there too, so nothing refuses a tip off the pole on such a face.

**What it needs:** read the tip's incidence on that face too, at the
tip's stored vertex by `singular_at` (the reading the joint would reset
by), or show the class cannot carry a wrap tip and say so where
`not_owed` excuses it.

Signed: (RESTCURVE implementer)

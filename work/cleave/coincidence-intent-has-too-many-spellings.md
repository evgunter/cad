---
id: coincidence-intent-has-too-many-spellings
kind: issue
title: Coincidence intent can be said in too many ways (shared key, face-pair declaration, split declaration, carried records, derived datum, shared parameters); weigh whether they reduce to fewer
status: open
opened: 2026-10-03
priority: P2
cost: M
design: true
---


Ev, on PR 3960 (2026-10-03): "i am concerned that this is evidence that
this whole system is able to say the same thing in too many ways, but i'm
not going to hold this pr back because of that".

Each of these says "these two cells are meant to coincide or touch":
- shared surface/point keys (the coincidence ladder's rung (a));
- the boolean's face-pair declarations (`FacePairDeclaration`,
  `DeclaredPair`, Rest/Continuation/Tangent);
- carried contact records (`CarriedVv` / `CarriedVf` / `ContactRecords`);
- the Split node's new on-plane declaration (PR 3960);
- the boolean's vertex-level ON-set records, which CONTACT's
  `boolean-vertex-contact-records-are-inferred-from-values` is reworking;
- derived datums (DM1);
- the detect/declare findings (`FlushFinding`).

Owed:
1. An inventory: each spelling, who writes it, who reads it, and what it
   asserts.
2. A designer pair on whether these reduce to one intent vocabulary,
   e.g. one declaration type with cell-level granularity that every verb
   node carries, before more verbs grow their own.

Weigh this before CONTACT's vertex-declaration fork lands, so that fork
does not add another spelling.

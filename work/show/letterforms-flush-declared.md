---
id: letterforms-flush-declared
kind: unit
title: silhouette, silhouette3 and az drop the 1/16 decoupling for natural proportions with declared coincident contacts
status: open
opened: 2026-10-02
priority: P3
cost: M
---

## What

`letterforms.rs` decouples the letters' coincident planes by 1/16 so
that the intersects never meet `UndeclaredCoincidence`. The declared
naive intersect now builds: the same file runs it live, tier-3′ valid,
V = 4.25. So the 1/16 is a dodge the kernel no longer needs for the
two-way case.

Re-author `silhouette`, `silhouette3` and `az` at natural proportions
(the letters a person would draw, coincident where they meet), with
the coincident contacts DECLARED, and retire the decoupling and its
narration. The 3-way `silhouette3` on declared naive operands is
UNMEASURED (it used to refuse `NonMaximalFaces`): measure it first. If
it still refuses, keep `silhouette3` on its current authoring with a
live wall probe pinning the refusal, file it on the owning program,
and re-author the other two.

## Oracle

Exact rational volumes, as today (`az`'s gate is the exact oracle
880383/327680 at the current proportions; it moves with them — derive
the new one).

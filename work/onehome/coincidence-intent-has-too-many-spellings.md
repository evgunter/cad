---
id: coincidence-intent-has-too-many-spellings
kind: issue
title: Coincidence intent can be said in too many ways (shared key, face-pair declaration, split declaration, carried records, derived datum, shared parameters); weigh whether they reduce to fewer
status: closed
opened: 2026-10-03
priority: P1
cost: M
closed: 2026-10-26
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

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the spellings it lists (FacePairDeclaration/DeclaredPair, split declarations, carried/ON-set records, FlushFinding) retire at stage 4's one recording door (D10 Coincidence). (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by stage 4 F (2026-10-11)

Stage 4 F (`declared-pairs-retire`) retired every boolean declaration: no author can declare a pair, the glue is what the margins decide, and the declare offer, the flush detector and the carried declarations are gone. What this row waited on has fired; re-read it against that tree.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Stale.** `[coincidence.door]` and the places row "A coincidence decided from values" answer it, and the declared-pair types have 0 hits. Residue: `topo::contact::DeclaredContact`/`ContactFinding`, which D10 stage 5 C retires (ATREST); the rest belongs to COINCIDE. Close it, or repoint it at those.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: one intent with several spellings: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Closed (INTENT, 2026-10-26)

INTENT owns the coincidence door (D10 stage 4), so this row's question is INTENT's, and it is answered: `[coincidence.door]` and `[one-way.places]`'s row "A coincidence decided from values" name one home. The declared-pair spellings (shared key, face-pair and split declarations) are gone from the tree, and no design conversation is owed. The residue is code, not a question:

- `topo::contact::DeclaredContact` / `ContactFinding` retire with ATREST's stage 5 C, `the-at-rest-census-is-a-check` (`work/atrest/plan.md`), which writes the census as a check over D′3's `⊔` door.
- Carried records and derived datums as coincidence evidence are COINCIDE's (`work/coincide/`).

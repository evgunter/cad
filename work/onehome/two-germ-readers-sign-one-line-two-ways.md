---
id: two-germ-readers-sign-one-line-two-ways
kind: issue
title: The v-v crossing germ is signed by sector membership and the pierce germ by its transition's codes: two readers of one question
status: open
opened: 2026-10-10
priority: P1
cost: E
---


## What

Filed by `near-tangent-pierce-poses-reach-three-classification-invariants`
at review (PR 4581, S1). Both germ readers take a section line
`±(n₁ × n₂)` and pick its ray:

- `boolean/vtxfac.rs` `pierce_germ_dir` signs it by the transition's
  own codes (`OutBound`, which bound the Out run holds), reading both
  normals as outward. Membership is a `debug_assert!` there.
- `boolean/insert.rs` `germ_dir` signs it by `sectors::within` on both
  sectors, the readings `sectors::pair_search` made the crossing record
  on, so it cannot disagree with that record. Its remaining
  both-rays case needs two grazes and was not reached in the near-tangent
  census (86 400 runs over three ε rows).

## Owed

One method and one story for both readers. That means either deriving the
crossing germ from the record's codes (`sa`/`sb` after `recl`), as the
pierce germ is, or saying why a crossing record's membership is the
better datum there.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical, plus debris.** Both readers sit on the boolean's shared `sectors.rs` layer, so this is a choice of method. The third path beside them (`tangent_locus` for a declared Tangent pair) is D10 stage 4 debris that survived F. DESIGN home: `[one-way.readings]`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: two readers sign one section line two ways: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

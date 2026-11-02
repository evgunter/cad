---
id: the-in-arrival-census-check-retires
kind: unit
title: The transitional census check inside an arrival retires once every door certifies its own result
status: parked
opened: 2026-10-19
priority: P1
cost: E
blocked_on: [boolean-door-runs-the-census-over-its-result, each-space-runs-its-census-between-arrivals]
refs: [a-census-contact-is-a-row-and-a-record]
---

D10 `[assertions]` (PR 4666): inside one arrival the census "decides nothing and reads nothing: the arrival's verdicts are its source's or its door's, carried with it". Ev, PR 4666 (2026-10-19), on a confirmation inside an arrival: "a bit weird, but it might be ok as a transitional state if explicitly scheduled to be removed when boolean etc are fixed".

`each-space-runs-its-census-between-arrivals` lands with a transitional check inside each arrival. It confirms the arrival's carried records, matches its carried findings, and reports a touch or overlap matching neither as that arrival's door's defect, typed and never a panic. It stands in for doors that don't yet certify their own results:
- the boolean door doesn't run the census (`boolean-door-runs-the-census-over-its-result`);
- Pattern, Transform and Split drop a value's records (`transform-and-pattern-drop-a-values-contact-records`);
- the carry across every placement (`a-placement-carries-its-sources-at-rest-result`).

When those land, delete the check and its finding arm, so the census reads nothing inside an arrival. Before deleting, show it has gone quiet on the corpus and tour.

Note (D′3's fix pass, 2026-10-26): every census now discards what it
finds inside an arrival a door already decided (a `⊔`'s, an instance's
copy; `product::gather_sources`). The kernel's census has no group
scope, so it still computes there and the door discards it; deleting
the computation is this unit's.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to ATREST's charter (at-rest census work; ATREST's stage 5 C makes the census a check at each `⊔`, the same door). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).

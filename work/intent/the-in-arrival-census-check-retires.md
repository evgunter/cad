---
id: the-in-arrival-census-check-retires
kind: unit
title: The transitional census check inside an arrival retires once every door certifies its own result
status: parked
opened: 2026-10-19
priority: P1
cost: E
blocked_on: [boolean-door-runs-the-census-over-its-result, transform-and-pattern-drop-a-values-contact-records, a-placement-carries-its-sources-at-rest-result, each-space-runs-its-census-between-arrivals]
refs: [a-census-contact-is-a-row-and-a-record]
---

D10 `[assertions]` (PR 4666): inside one arrival the census "decides nothing and reads nothing: the arrival's verdicts are its source's or its door's, carried with it". Ev, PR 4666 (2026-10-19), on a confirmation inside an arrival: "a bit weird, but it might be ok as a transitional state if explicitly scheduled to be removed when boolean etc are fixed".

`each-space-runs-its-census-between-arrivals` lands with a transitional check inside each arrival. It confirms the arrival's carried records, matches its carried findings, and reports a touch or overlap matching neither as that arrival's door's defect, typed and never a panic. It stands in for doors that don't yet certify their own results:
- the boolean door doesn't run the census (`boolean-door-runs-the-census-over-its-result`);
- Pattern, Transform and Split drop a value's records (`transform-and-pattern-drop-a-values-contact-records`);
- the carry across every placement (`a-placement-carries-its-sources-at-rest-result`).

When those land, delete the check and its finding arm, so the census reads nothing inside an arrival. Before deleting, show it has gone quiet on the corpus and tour.

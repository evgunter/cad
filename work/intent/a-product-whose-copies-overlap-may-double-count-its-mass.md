---
id: a-product-whose-copies-overlap-may-double-count-its-mass
kind: issue
title: Mass properties of a product whose copies overlap may count the overlap twice with only an interference finding beside them
status: open
opened: 2026-10-21
priority: P2
cost: M
---


Raised by a designer on the one-body-or-two fork (PR 5494, 2026-10-21); unverified. `mass_properties` sums over solids, so a product whose copies overlap (the corpus heat sink's fins sunk into its base) counts the overlap twice, while a union would not. Check whether `Measure` volume over such a product refuses or reports silently with only an interference finding beside it. If it is silent, that is a live wrong answer.

---
id: LIB-B-VALIDATE4
kind: unit
title: binding census family B-VALIDATE4
status: review
opened: 2026-09-03
branch: lib/b-validate4
pr: 1677
---

Queued mechanical census family (the B-READBACK/B-CHECKS shape): sweep the
family's bindings against the census contract, construct the previously
unconstructible pins where the surface now allows, and re-cut the census
rows honestly. Families share the census/tags/test files, so at most two
run concurrently, staggered.

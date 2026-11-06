---
id: python-value-bodies-drops-a-family-with-an-empty-member
kind: issue
title: Python Value.bodies() returns no body for a family of bodies with an empty member
status: open
opened: 2026-11-06
priority: P3
cost: E
---



Found by the REFS lane that closed `a-family-of-boolean-results-cannot-be-read-whole-or-by-index` (2026-11-06), from reading; not pinned by a test.

`Value.bodies()` (`crates/pncad-py/src/py/value.rs`) lists the bodies for `Instances` and returns `[]` for any other payload. A family whose members are each a body or a boolean's empty result (a per-`k` cut where one member cuts away everything) evaluates to `ValuePayload::Family`, so Python is told the family holds no body, with no error. `editor_core::NodeValue::member_bodies` is the public door that reads both payloads alike. The Python door should list each present body. Whether an absent member keeps its place, as `Value.split()` keeps an empty side as `None`, or is left out, is that door's choice to make.

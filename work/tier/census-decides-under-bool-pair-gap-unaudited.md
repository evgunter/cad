---
id: census-decides-under-bool-pair-gap-unaudited
kind: issue
title: The census decides its turned clear under K name bool_pair_gap, which neither the predicate-dimension audit nor the symbolic-tier census lists
status: open
opened: 2026-10-10
priority: P3
cost: E
---



Found in the review of PR 4550 (CONTACTHOLD, `contacthold/census-backstop`).

## What

The census backstop's arm 1 now clears a curved cross-solid pair through
`census::backstop_apart`, which calls the boolean's narrow phase
`boolean::separating::apart`. That door decides under the K names
`bool_pair_gap` (`separating::PAIR_GAP`, metres), `bool_pair_axis`
(`PAIR_AXIS`) and `bool_pair_normal` (`PAIR_NORMAL`). None of the three
appears in `docs/predicate-dimension-audit.md` or in
`work/tier/symbolic-tier-census.md`, and both list the census's own
`census_backstop_gap`. The three names were unlisted before the census
used them (PR 4122 added them for the boolean); the census is now a
second consumer.

## The shape to give

Add rows for the three names in both documents: dimension, the
comparand and its lever, and that the census reads them with
`sweep_pad` subtracted.

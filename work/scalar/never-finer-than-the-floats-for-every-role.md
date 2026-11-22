---
id: never-finer-than-the-floats-for-every-role
kind: issue
title: The told rule applies never finer than the floats at the value to angles, ratios, offers and pairs; the README says it of a computed length
status: open
opened: 2026-11-22
priority: P2
cost: M
design: true
---


`crates/geom-core/README.md`, "A number told to a person", gives the computed-length row "never finer than the spacing of adjacent floats at that value", and the computed angle or ratio row "four significant figures". `geom_core::told` (`Float::of`, its `own` figures, and `Float::figures`) applies the floats' spacing to every role and dimension: a computed angle or ratio, an offered bound, an enclosure's ends and each member of a compared pair are told as received once the decimal's step reaches the floats' gap. It bites only where floats carry fewer than four figures (the subnormal band): the least subnormal angle reads `5e-324 rad`, where four figures would read `4.941e-324 rad`, which also reads back.

## The question

Ratify the sentence the README would need, for example "no spelling is finer than the spacing of adjacent floats at the value; where the role's figures reach it, the number is told as received", or rule that angles and ratios keep four figures in the subnormal band.

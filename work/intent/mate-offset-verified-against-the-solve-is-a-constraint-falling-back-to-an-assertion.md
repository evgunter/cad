---
id: mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion
kind: issue
title: A non-root member's offset is verified against the solve (OffsetDisagrees/OffsetUnchecked): a constraint that falls back to an assertion
status: open
opened: 2026-10-03
priority: P0
cost: M
refs: [one-way-to-say-dependency-and-intent]
---


`MateFault::OffsetDisagrees` / `OffsetUnchecked`
(`crates/editor-core/src/mate.rs`) check an offset authored on a
non-root member against the pose the solve derives, the same shape as
A11 (4)'s non-tree mates that DECLARE: a placing statement that
silently becomes a checked one depending on where it falls. Ev
(2026-10-03, in chat): constraints and assertions are either one thing
or two with no cleverness; "having constraints that fall back to being
assertions seems worse than either". D10 (ruling
`one-way-to-say-dependency-and-intent`) retires both with A11 (4): a
placing statement on a pinned copy refuses as an overconstraint.
Found by the ruling's designer pair.

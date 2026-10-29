---
id: tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity
kind: issue
title: Tier 2 admits a valence-1 vertex only at a wrap edge's tip, and tier 3 refuses a wrap tip off its face's chart singularity
status: parked
opened: 2026-10-29
priority: P1
cost: M
blocked_on: [loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk]
refs: [validity-refuses-an-interior-chart-singularity, import-mints-the-wrap-tip-at-a-chart-singularity, 6014]
---

Split out of `validity-refuses-an-interior-chart-singularity` by the RESTCURVE orchestrator (2026-10-29). `[ev]` PR 6014 (fork row 215) grew that row with the wrap tip, and DESIGN `[topology.tiers]` now states two rules for it:
- tier 2: "no valence-1 vertices but a wrap edge's tip";
- tier 3: "every wrap edge's tip lies on its face's chart singularity".

The wrap tip is the one edge the kernel adds to a chart singularity that no written edge reaches, walked out and back.

## Why this is its own row

Ev approved the wrap tip "if the complexity it adds is tolerable" and asked that the shared loop walk land first (`work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`, first in the wrap-tip sequence). Until then each loop reader (props' sphere and cone loops, which check 6 reads through, the mesh walk, and the boolean sectors) reads a tip on its own. FOREIGN's probe measured a hand-written reading at 22.81 against a true 23.86, and it passed tier 3.

Admitting the tip at tier 2 before that walk exists would let such a body through. The other half of the row, "every chart singularity in a face's closure is a vertex of it", needs no tip and lands now.

## What

- **Tier 2, check 2** (`validate.rs`, the `Strut` arm near `:5301`): a valence-1 vertex is exempt only when it is the tip of a wrap edge. That is read off the stored wrap flag (`EdgeDescriptionSpec::Chart { wrap, .. }`), so it is structural and takes no ε. Every other valence-1 vertex still refuses.
- **Tier 3**: a wrap edge's tip lies on its face's chart singularity. Read it as a certified reading of the pcurve's end on the chart's singular set, not as a point-to-pole distance at a band (FOREIGN's note on the parent row). A meridian that stops short of the pole refuses; `spur.step` is the witness, and today it passes tiers 2–3′ on the tier-2 exemption alone.
- Check 6 and the other tier-3 loop readers read the tip through the shared walk, never a reading of their own.

FOREIGN's `import-mints-the-wrap-tip-at-a-chart-singularity` waits on this row: import mints the tip only once the tiers admit it.

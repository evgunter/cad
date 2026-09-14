---
id: SYM-7
kind: unit
title: the plain form outlives the leaf: a drive-scoped memo, shared across the drive's workers, with frozen re-defined as distinct-over-the-drive
status: spec
opened: 2026-09-14
branch: sym/7-plain-memo
refs: [symbolic-tier-costs-95-percent-of-the-m10-3-drive, 2581]
---



## What

D3 = (1) on `[ev]` #2581: the tier's plain-form memo, keyed by
`SymId`, outlives the leaf — one `DriveMemo` per drive, shared across
the drive's rayon workers so the receipt stays schedule-independent,
consulted on a miss and published once per leaf; the early and door
walks stay per leaf. `frozen` on the drive's receipt becomes the
number of distinct nodes frozen over the drive; the per-leaf goldens'
column is re-blessed as this unit's own move; the opaque-sequence
argument is pinned across leaves. Block SYM-B2 slot 0 (H /
STRUCTURAL, pre-draw); the full v6 dual. Spec: `docs/SYM-7-SPEC.md`.

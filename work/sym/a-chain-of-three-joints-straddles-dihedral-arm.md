---
id: a-chain-of-three-joints-straddles-dihedral-arm
kind: issue
title: a chain of three or more joints straddles dihedral_arm on a mapped edge's re-certification with an eps-independent [0, 7.34e-3] — the first refusal over the whole study, and not what bounds the certifiable box
status: open
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, measuring the certified lane on it
(`demos/tour/src/chaintol.rs`, the cell's own CI row).

One leaf over the whole declared box, `Sym<Interval>`, on the chain of
`demos/tour/src/chain.rs` at σ = 0.01 rad per joint. At THREE and at
FOUR links:

```
node 12 — the transform op refused: transform: mapped edge EdgeKey(2v1)
failed re-certification: certification: the transversality margin at
sample 1 escalated: predicate 'dihedral_arm' indeterminate: enclosure
[0e0, 7.338367397126071e-3] cannot be classified against the band
(zero = 1e-9, escalate = 1e-8)
```

**What this is, and what it is NOT.** It is the FIRST refusal over the
WHOLE study at three and four links — which is a fact about evaluation
order, not about which predicate the certifiable box runs into.
MEASURED just above the wall (`1.02×` and `1.10×` of each link count's
certifiable fraction, default ε and `1e-6`), the first refusal is a
different predicate: `dihedral_wedge` with a POISONED margin, at two,
three and four links alike. **That** is what bounds
`chain::CERTIFIABLE_FRACTION`, and it is filed at
`a-chain-of-two-or-more-joints-poisons-its-transversality-margin`.
This row said the opposite for one review cycle; it was a causal story
nobody had executed.

**The enclosure does not move with ε; only the band does.** At
ε = 1e-6 the same leaf reports the same bracket to every digit —
`[0e0, 7.338367397126071e-3]` — against `zero = 1e-6, escalate = 1e-5`
instead of `1e-9 / 1e-8`. That is this row's own finding and it stands:
the bracket's WIDTH at a given box is a property of the arithmetic and
not of the run's tolerance. (The certifiable box DOES move with ε,
from `0.1110` to `0.1083` — which is the wedge's doing, not this
bracket's, and is why the two rows are two rows.)

The enclosure's LOWER end is exactly `0e0`, which is the shape of a
dependency-widened margin around a structurally-zero quantity rather
than of a real sign flip: a bar's cap-to-side dihedral is a right
angle at every joint angle, and the transversality arm at that rim
does not actually approach zero as the joints move.
The plate's own ceiling is the same class, and its cell's header
carries the worked version of this argument: there the margin was
affine, its true range was positive everywhere, and the real flip
first entered the box well beyond the width the enclosure stopped at.
(That row itself is gone with its program; `demos/tour/src/tolerance.rs`
is where the argument survives.)

## What answers it

The true range of `dihedral_arm` at that rim over the whole study,
measured — if it is positive everywhere, this is dependency widening
and the fix is the tier's (a tighter form for the arm, or a registered
identity for the right-angle rim the extrude's own builder guarantees).
If it genuinely reaches zero, the straddle is honest and the row closes
saying so.

Note what answering it would and would not buy: retiring this straddle
would remove the first refusal the whole study reports at three and
four links, and `chain::CERTIFIABLE_FRACTION` would not move, because
the wedge is already refusing below it. The two rows have to be
answered in that order to see any width.

## Home

SYM — the tier's reach on a construction Ev asked for.

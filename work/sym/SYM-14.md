---
id: SYM-14
kind: unit
title: the chain: four links joined with an angular error at each joint, its dispersion drawn like the plate's, and the certified lane measured on it
status: closed
opened: 2026-09-22
priority: P1
cost: H
branch: sym/14-chain-demo
refs: [a-widened-rotation-angle-is-unmeasured-on-the-certified-lane]
closed: 2026-09-22
---


## What

**Requested by Ev, in chat, 2026-09-22 — P1 specifically requested:**
"a demo of error propagation like with the two holed plate but it's a
chain of 4ish elements joined with some error in the angle at each
join and so it will be visibly be more and more dispersed going down
the chain"; "if it is possible, or will be possible after some of your
planned work lands, take it on as a unit". It is possible today on the
advisory lane (the `f64` Monte-Carlo replays that draw the plate's
density sheet) through public doors — `Node::Transform` with an
`Angle`-dimensioned parameter carrying a `Distribution`, the built
bodies read back, the tour drawing the SVG itself — so the unit builds
the chain document once (`demos/tour/src/chain.rs`), draws its
dispersion sheet beside the plate's, and measures the certified lane
(`Interval`, `Sym<Interval>`, the drive) on the same document, filing
every wall at P1 with Ev's request carried: a widened rotation angle
has never been measured on the certified lane, and the derived-frame
family's walls are live. No kernel change. Protocol v7 OUT (a demo and
a measurement): OPUS implementer, one OPUS style review with a
correctness arm, no draw, no ordinal, no row. Spec:
`docs/SYM-14-SPEC.md`.

## Closed (2026-09-22)
Merged as #3073 (head `aea96608e`, run 35741468541). The single style review
with a correctness arm (OPUS): MERGEABLE-AFTER-FIXES 3/4/5 — the code and
the picture reproduced to the byte and survived every plant; three causal
claims were enshrined in P1 rows and headers without execution (what
bounds the box is `dihedral_wedge`'s poisoned margin, not `dihedral_arm`;
the invariant across link counts is HALF THE PIN RADIUS, a property of
this document and not of the tier; the three-link chain carries the
two-link poison); the fix pass A–O corrected them by execution, added the
drawn-values check, one `summarize` on the façade and the `SHEETS` guard;
the delta NOT MERGEABLE on four surviving copies of the retracted
"straddling" sentence (one load-bearing, one printed); fix pass 2 grepped
the claim (seven sites), carried the bisection's monotonicity as an
assumption with its ladder check run, and left the ε question open on the
wedge row; delta 2 NOT MERGEABLE on the last copy of the
retracted sentence, in `demos/README.md`, taken in the state-sync commit
with its source's wording. The spec's sentences that did not survive:
"the tip end-face centre" as the measured quantity (the tip PIN's centre
against a fixed target pin — cylinder × cylinder is the only closed form
that reaches a centre exactly; a planar end face gives the chain's REACH,
second order); "what is NOT reachable today is the CERTIFIED version" —
it was: the symbolic lane certifies the four-link chain whole at 0.111 of
the study and the enclosure per joint is drawn beside the cloud, while
the plain interval lane refuses at every link count. Spec deleted with
its note (`docs/doc-ledger/sym-14-spec.md`). Rows left: the wedge wall
(with the ε question), the arm straddle, the plain-interval refusal, the
façade row on LIB, and `the-chain-demo-detects-no-self-intersection`
(Ev's follow-up, P1).

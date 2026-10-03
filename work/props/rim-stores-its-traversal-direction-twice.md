---
id: rim-stores-its-traversal-direction-twice
kind: issue
title: Rim stores the traversal direction twice (d_u: T and d_u_sign: Sign) and du_of_rims compares the exact one through the tolerance funnel
status: closed
closed: 2026-09-16
pr: 2741
branch: props/sphere-pole-side
opened: 2026-09-11
refs: [877, S40]
---


## Carved out of `S40` (2026-09-11, by the WIRE orchestrator)

`S40` ("Residue and editing artifacts", accepted by Ev 2026-08-18)
was one file carrying four unrelated residues on three programs'
territory. WIRE inherited it whole in the cut of 2026-09-11 and owns
only one of the four; the rest are filed where the code lives, per
`work/README.md` ("when the owning program is clear, file the item
straight onto that program's slate"). `S40` keeps its id and its
`WitnessSlot` row; this is the third of its four bullets, verbatim
below. **Nothing about it was re-judged in the move** — the citations
are `S40`'s, written against a tree `#877` has since moved, and the row
was last read on 2026-08-18.

## Finding (`S40` bullet 3, verbatim)

- **Confidence**: sure

`Rim` stores the same traversal direction twice (`d_u: T` and
`d_u_sign: Sign`), and the exact one is compared through the tolerance
funnel — subtracting two exactly-±1 values and banding a result that is
always 0 or ±2 (`props/curved.rs`, `Rim`'s two fields and `du_of_rims`'
`props_rim_dir_group` decide — cited by target name per **S176(a)**; the
line numbers were written against a tree #877 moved). **STILL OPEN** —
which of the two representations is authoritative is a design call. #877
did not touch it: the margin is now levered at `RimArms::azimuth` rather
than a bare `arm`, which is the same comparison at a named lever.

## Why it is PROPS's

`crates/geom-brep/src/props/*` is PROPS's glob and no other open program
claims it, and "which representation is authoritative" is a call about
this program's own data, not a ruling to escalate. Worth reading beside
its sibling `rim-level-rule-manufactures-its-error-by-feeding-nan-into-classify`
— both are `props/curved.rs` and both came off `S40`.

## Answered (PROPS sphere-pole-side, 2026-09-15)

**`d_u_sign` is authoritative; the `T` copy is gone.** The design call
the row left open is settled by what the direction IS: a definite
`Sign`, minted by `rim_dir` from a definite `props_circle_axis_class`
outcome and the stored traversal bool, both discrete at origin. There
is nothing for a scalar image to say that the sign does not, and the
one consumer that used it — `du_of_rims`' group key — was banding a
difference of two values that are `±1` by construction, so it compares
`rim.d_u_sign == g.1` now. `props_rim_dir_group` forms no margin, reaches
no funnel site, and is retired from `docs/predicate-dimension-audit.md`
(its note **N2** with it).

The unit that could retire it is this one because
`props_rim_interior_side` reads the same field for σ: the direction had
to be settled before a second predicate was built on it.

Moved with it: `voided_rod`'s recorded verdict multiset loses
`("props_rim_dir_group Positive", 2)`, and the three
`shell-census-digest/eps-*.txt` rows move on that body's channel alone,
`verdicts n=40 h=c40da6c25f7ffe61` → `verdicts n=38 h=30974de0c3cd1571`,
identically at every ε. Every reading is bitwise unchanged.

## Closed

Landed on PR #2741. `Rim` carried the traversal direction twice, as a
scalar `d_u: T` beside the discrete `d_u_sign: Sign`, and `du_of_rims`
compared the exact one through the tolerance funnel. The scalar is
gone, with no reader left anywhere, and the grouping compares the
discrete sign directly, which retires `props_rim_dir_group`.

The retirement is strictly stricter rather than equivalent, and the
unit says so: the old margin was `±2·arms.azimuth`, which lands in the
zero band whenever `2·azimuth < K·ε`, so on a gasket-scale torus the
old code merged OPPOSITELY traversed rims where the sign compare never
does. That is the direction a premise may move without re-deciding what
it admits. Two recorded-verdict populations lost that name — the census
digests' `voided_rod` (40 verdicts to 38) and `bulged_extrusion` (19 to
18), identically at all three ε, with every reading bitwise unchanged.

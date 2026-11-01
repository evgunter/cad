# Design fork: the boolean's cone reader (ONEREAD)

Row: `work/oneread/the-booleans-cone-reader-is-a-second-link.md`.
Protocol: `docs/DESIGN-FORK-PROTOCOL.md` at the commit last touching it on main.

Draw (2026-11-01, two `/dev/urandom` bytes): 33, 250.
- 33 < 86: three designers (two Opus, one Fable).
- 250 (no redraw) mod 3 = 1: Fable is **B**; Opus are **A** and **C**.

Dispatched concurrently as subagents of the ONEREAD orchestrator session, one brief.

## First reports (as delivered, 2026-11-01)

| | answer | argument | conf. | framing rejected | ratified text | tokens |
|---|---|---|---|---|---|---|
| A | one reader in `topo::link`, split into a Cone (parity over tangent sectors) and a Link (cone + pieces + ball); the boolean gathers one cone of the other operand per vertex across partners, held face and held edge, and reads its edges; cone_side family, shortcut, met/pointed and vtxfac layering deleted; metering: lean to the boolean's levers inside a widened door; alternative (unsure): naming reads the link itself | the fuzz measures the boolean deciding 891/998 vs link 730/998 at 1e-9, 970 vs 589 at 1e-6, both never wrong; the shortcut is a [one-way.readings] fallback and refuses bore rims (NoBisector) | likely | partly (fuzz does not keep them in step; real duplicate includes layering) | none binds; bends ATREST's unratified door rule | 191664 |
| B | one reader; the boolean builds the other operand's `Link` from its BoolSectors (sign-free `Link::of_sectors`) and calls `Link::reads` per edge; several partners and pierced face as one link, layering deleted; Bound -> On sharpening; census_touch_* -> link_*; cone_fuzz moves to link/ | same question over same sectors; the cone reader is the weaker one, under no door; status quo pin is one-way | likely | no (corrections: cone reads first order, shortcut not a D3 stage) | none | 214083 |
| C | one reader in `topo::link`: a cone type (planar sectors, parity, metric door) under Link (at-scale claims); a germ builder makes the other operand's tangent cone across all its keys; the boolean reads own edges through it; reader and layering retired | one question only once the cone is built (germ vs ball); the problem is one level up, per-partner reading; shortcut is a [one-way.readings] fallback | likely | partly | none | 176669 |

Agreement: on the core final state, yes. Open between them: metering (A levers; B, C the link's door), and the Cone/Link type split (A, C yes; B builds a Link). A's naming-reads-the-link alternative is A's alone.

## Round 1 (2026-11-01)

Each designer (A, B, C, resumed with its context) was shown a labels-only summary of all three first reports (`round1-others`, kept off-repo) and asked, on metering, the type split and A's naming alternative, what moved it and whether its earlier argument is answered; A was asked whether its measured decided-count gap comes from the metering or from something fixable inside the door.

Round 1 outcome:
- A moved on metering, to the link's door, on its own instrumentation (the gap is Bound refusals ~70% and in-band-outside-sector ~25%, not levers). Held the type split. Held naming as an alternative, not recommended.
- B kept the door with its own per-cause measurement (same split; wide parity + Bound->On close the gap to noise; `own` decides 86 in band). Conceded the type split (a Link from BoolSectors would answer judge/offset on non-face points). Naming: not taken.
- C kept the door; measured Bound->On closing ~60-70% of the gap; `own` unsound wholesale. Held the split. Naming: against for now.

Converged on every point. Decided by the ONEREAD orchestrator without Ev (no ratified text changes; no [ev] PR, so no fork-log row), recorded in the row's `## Decided`.
Tokens (harness, cumulative): A 210776, B 268750, C 198587.

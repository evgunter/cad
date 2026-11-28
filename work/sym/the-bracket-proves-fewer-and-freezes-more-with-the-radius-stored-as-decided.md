---
id: the-bracket-proves-fewer-and-freezes-more-with-the-radius-stored-as-decided
kind: issue
title: The filleted bracket proves 19 fewer theorems and freezes 321 more forms with arcs' radii stored as decided, and the kept-atom ladder recovers none of them
status: open
opened: 2026-11-28
---


Found by BOXFINISH's signed-radius unit (PR 7686), re-cutting
`editor-core`'s `Sym<Interval>` pins on its merge of main `bbe2b18f6e`
(2026-11-28), at ε `1e-9` and `1e-6` alike.

`r2_filleted_bracket`, replayed past its ceiling
(`sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling`),
`symbolic / numeric / frozen`:

| | symbolic | frozen |
|---|---|---|
| main | 1329 | 53 |
| the unit's head | 1310 | 374 |

At its nominal (`sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`)
main reads `[1323, 1544]` and the ladder retries 6 of them
(`[1329, 1538]`); the head reads `[1304, 1565]` and the ladder retries
none. The other four measured documents hold or gain on the head (the
plate `951`, the annulus `511`, the link `801 → 805` with its frozen
count `1493 → 656`, the pad `1455 → 1471` with `127 → 59`).

**What it is not**, measured with three local switches on the head
(none committed): the gap's spelling (`(n − r)·s/s` against the
squares' difference over `s`: the same counts, the tier reads one
polynomial), the line×circle margin's spelling (the same counts; two
more forms frozen on the bracket with the hand spelling), and the
fillet offsets' `abs` (the same counts).

**Not measured:** which predicates lose the nineteen, which forms
freeze, and whether the cause is the fillet's arcs storing the carrier
radius as written, the recorded radials' registered facts
(`Arc2::register_radials`), or the two meeting main's direction mint
(before that mint landed, the same unit read the bracket eight
theorems above main).

`sym11`'s floor and frozen ceiling for the bracket are re-cut to the
head's `1310` and `374` with this row named at the pin; its doc says a
form that newly freezes reds there, and this is one.

**The tests that say it is closed.**
- The bracket's `sym11` floor is back at or above `1329` and its
  frozen ceiling at or under `53`, or each lost theorem is shown to be
  a reading no longer taken.
- The ladder row's bracket retries are accounted for.

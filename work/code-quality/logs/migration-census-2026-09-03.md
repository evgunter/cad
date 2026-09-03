# Findings census — tracker migration, 2026-09-03

Every `## S<n>` heading of `docs/SMELL-SCAN-2026-08.md` at the migration
(the tree this PR branches from), with where its substance lives now.
A finding cited by a live Track K–X row is carried by that row's file
(the number is in the row's `refs:` or its text); a finding cited only
by a decision for Ev is carried inside the ruling file; a live finding
no row cites is its own `kind: issue` file with `## Was: unrowed`. No
heading was found whose own record says closed with no live row — the
document had already deleted those — so nothing is dropped.

| finding | disposition | carried by |
|---|---|---|
| S1 | cited by a live row | Track M — H5 |
| S2 | cited by a live row | Track M — H5 |
| S3 | cited by a live row | Track M — H5 and S90-impl |
| S4 | cited by a live row | Track Q — D288; Track V — C6; Track W — D382 |
| S5 | live and unrowed → issue | `S5.md` |
| S11 | live and unrowed → issue | `S11.md` |
| S13 | live and unrowed → issue | `S13.md` |
| S14 | ruling | `S14.md` (kind ruling; carries the finding section) |
| S16 | cited by a live row | Track Q — D280 |
| S18 | live and unrowed → issue | `S18.md` (members D30 on Track R and D31 on Track N are rows that do not cite the number) |
| S19 | live and unrowed → issue | `S19.md` (members D36 on Track Q and D366 on Track V are rows that do not cite the number) |
| S22 | cited by a live row; row 1 is a ruling | Track Q — D283; `S22-row-1.md` (closed ruling, full text) |
| S26 | cited by a live row | Track R — S26 |
| S27 | cited by a live row | Track R — C3 |
| S28 | cited by a live row | Track R — S28 |
| S29 | live and unrowed → issue | `S29.md` (its residue is Track R — C23) |
| S32 | cited by a live row | Track N — C24 |
| S35 | live and unrowed → issue | `S35.md`; `L3` names it |
| S36 | live and unrowed → issue | `S36.md`; `L1` names it |
| S37 | live and unrowed → issue | `S37.md` (rule 6: C2/H17, in no track) |
| S38 | live and unrowed → issue | `S38.md`; `L2` names it |
| S39 | cited by a live row | Track Q — D281 |
| S40 | live and unrowed → issue | `S40.md` |
| S41 | live and unrowed → issue | `S41.md` |
| S43 | live and unrowed → issue | `S43.md` (verdict SETTLED as the D2 addendum; the `mesh` `MissingEntity` residue and the graft-sentence class it states are open) |
| S44 | cited by a live row | Track M — H5 |
| S45–S48 | reserved — never allocated; no item | `plan.md` (numbering) |
| S49 | live and unrowed → issue | `S49.md` |
| S52 | live and unrowed → issue | `S52.md` (#672 residue) |
| S55 | cited by a live row | Track M — H5 |
| S57 | live and unrowed → issue | `S57.md` |
| S58 | live and unrowed → issue | `S58.md` (its §D row C11 is not live) |
| S65 | ruling | `S65.md` (kind ruling; carries the finding section with the three-way table and measured prices) |
| S66 | live and unrowed → issue (parked on #862) | `S66.md` |
| S69 | cited by a live row | Track P — S69 |
| S70 | ruling | `S70.md` (kind ruling; carries the finding section) |
| S73 | cited by a live row | Track K — C15 and D201 |
| S79 | live and unrowed → issue (parked on #757 #758 #759) | `S79.md` |
| S82 | ruling | `S82.md` (kind ruling; carries the finding section) |
| S83 | cited by a live row | Track Q — S83 |
| S87 | cited by a live row (title verbatim; number uncited) | Track V — G4 |
| S88 | cited by a live row | Track R — D305 |
| S89 | cited by a live row | Track N — D244; Track Q — D289; Track W — D384 |
| S90 | cited by a live row | Track M — S90-impl |
| S93 | cited by a live row | Track P — S93 |
| S94 | live and unrowed → issue | `S94.md` (Track P prose folds it into whichever lane opens `validate.rs`; no row) |
| S95 | cited by a live row | Track Q — G9 |
| S96 | cited by a live row | Track Q — G9 |
| S190 | cited by a live row | Track V — S190 / #855 |
| S193 | cited by a live row | Track V — D360 |
| S195 | live and unrowed → issue | `S195.md` |
| S107 | ruling | `S107.md` (kind ruling; carries the finding section) |
| S111 | cited by a live row | Track T — D322 and D323 |
| S112 | cited by a live row | Track T — D324 |
| S113 | live and unrowed → issue | `S113.md` |
| S114 | live and unrowed → issue | `S114.md` |
| S115 | live and unrowed → issue | `S115.md` |
| S116 | cited by a live row; member (p) is a ruling | Track M — S290 (member b); `S116p.md` |
| S117 | cited by a live row | Track K — D205 and D208; Track Q — D287 |
| S126 | cited by a live row | Track W — D70 |
| S120 | cited by a live row | Track K — D64 (member (a)'s second clause is `L4`) |
| S161 | cited by a live row | Track P — D107 |
| S130 | cited by a live row | Track X — D79 |
| S133 | live and unrowed → issue | `S133.md` |
| S128 | cited by a live row | Track W — D72 |
| S135 | cited by a live row | Track W — D113 |
| S136 | cited by a live row | Track W — D380 |
| S176 | live and unrowed → issue | `S176.md` (its cite-by-name rule is also in `plan.md`) |
| S177 | carried by a Last-deliberately row | `L5.md` (the walk); `plan.md` (the strike rule) |
| S172 | cited by a live row | Track Q — D287 |
| S173 | cited by a live row | Track Q — S173 |
| S121 | cited by a live row | Track R — D300; Track W — D383 |
| S122 | cited by a live row | Track Q — D66 |
| S124 | cited by a live row | Track K — D68 |
| S158 | cited by a live row | Track K — D102 |
| S159 | cited by a live row | Track K — D103 |
| S163 | cited by a live row | Track K — D109 and D208 |
| S168 | cited by a live row | Track K — D114 |
| S214 | cited by a live row | Track W — H12 |
| S210 | live and unrowed → issue | `S210.md` |
| S234 | cited by a live row | Track Q — S234 |
| S235 | cited by a live row | Track N — S235 |
| S212 | cited by a live row | Track Q — H11 |
| S216 | cited by a live row | Track W — S216 |
| S230 | cited by a live row | Track W — S230 and D383 |
| S236 | cited by a live row | Track R — S236 |
| S237 | cited by a live row | Track R — S237 |
| S391 | live and unrowed → issue | `S391.md` |
| S392 | live and unrowed → issue | `S392.md` |
| S393 | live and unrowed → issue | `S393.md` |
| S394 | live and unrowed → issue | `S394.md` |
| S411 | cited by a live row | Track U — D343 |
| S414 | live and unrowed → issue | `S414.md` |
| S415 | live and unrowed → issue | `S415.md` |

## Totals

- headings: 94
- cited by a live Track K–X row: 54
- live and unrowed → `kind: issue` file: 33
- cited only by a ruling (carried in the ruling file): 5
- reserved ids, no item: 1
- carried by a Last-deliberately unit: 1
- marked closed in their own record and dropped: 0

Two rows are counted once under their carrier and also have a ruling
member: `S22` (row 1, `S22-row-1`) and `S116` (member p, `S116p`).

## Read alongside

- `S87` is carried by Track V's `G4`, whose text is the finding's title
  verbatim without the number; a reader grepping `S87` finds nothing.
- `S18`, `S19` and `S29` have members carried by rows that do not cite
  the number (`D30`/`D31`, `D36`/`D366`, `C23`); their issue files name
  those rows in `refs:` so the halves stay linked.
- `S94` is named only in Track P's prose (folded into whichever lane
  opens `validate.rs`); it is an issue file with no track so it is on
  the board rather than in a sentence.
- `S43`'s verdict is SETTLED (the D2 addendum), but its record states an
  open residue (`MissingEntity` in `mesh`; the three-copy graft sentence
  that is `S14`/`S70`'s), so it is an issue rather than dropped.
- `S177`'s rule half is `plan.md`; its walk is `L5`; its one enumerated
  instance is dispositioned (`D124`) and is not restated.

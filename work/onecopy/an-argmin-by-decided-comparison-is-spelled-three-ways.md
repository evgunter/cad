---
id: an-argmin-by-decided-comparison-is-spelled-three-ways
kind: issue
title: The nearest of several candidates by decided comparisons is spelled three ways (join::keep_nearer, offset_derive::nearest, link's turn fold), and none says what an in-band comparison leaves of the minimum
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by QML tranche 2 (`work/onecopy/question-mark-ladders-the-gate-registered.md`,
`onecopy/refusal-standing`), reading `offset_derive::nearest`, which the
row listed "for its argument, not as a clear port".

## What

Three folds keep the nearer of an incumbent and a candidate by a
decided comparison:

- `topo::boolean::join::keep_nearer` (≈`join.rs:2267`): the incumbent
  keeps a tie (D9), and a comparison in band refuses by `?`. It is typed
  on `PartnerDistance` and `BooleanError`, so it serves join alone
  (`find_match`, the walk order, its rows).
- `topo::offset_derive::nearest` (≈`offset_derive.rs:601`): a decided
  `Zero` refuses `Unsupported` ("two branches of the section lie equally
  near the edge"), and an in-band comparison escalates by `?`, the site
  `escalation-question-marks.counts` registers.
- `topo::link` (≈`link.rs:1122`, the piece-end turn fold): the incumbent
  keeps anything not `Less`, and the order's escalation returns by `?`.

The three differ in what a tie is (kept, refused, kept), so they are
three answers to one question, spelled apart.

## The argument the register listed it for

`climb`'s rule ("a reading in band is kept, and refuses only where no
later reading answers") does not transfer to an argmin as it stands. An
in-band comparison between the incumbent and a candidate leaves the
minimum open: either may be the nearer. A later candidate decided
nearer than **both** closes it, since the in-band pair is then not the
minimum whichever is nearer. One decided nearer than only the incumbent
does not. So an argmin's kept reading is a *set* of candidates still in
contention, not one rung, and the rule is "refuse only where no later
candidate is decided nearer than every one in contention".

## What closing it takes

One home: an argmin over decided comparisons that carries the
in-contention set and names its tie rule (keep the incumbent, or
refuse). Port the three folds onto it. Then decide, per site, whether
the set rule above is the one it wants. That is a design question, for
the design-fork procedure before anyone builds it. Until then
`offset_derive::nearest`'s `?` stays registered with this row cited.

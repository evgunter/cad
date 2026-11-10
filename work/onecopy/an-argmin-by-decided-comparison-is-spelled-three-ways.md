---
id: an-argmin-by-decided-comparison-is-spelled-three-ways
kind: issue
title: The decided argmin (geom_brep::ladder::argmin_decided) refuses at its first in-band comparison; what an in-band comparison leaves of the minimum, and each site's tie rule, is open
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

## Narrowed by `onecopy/argmin-and-leftovers` (2026-11-10)

The one home exists: `geom_brep::ladder::argmin_decided`, one step of
an argmin over a decided order (`Nearer::{Candidate, Incumbent, Tie}`),
whose `Step` each site resolves by a named tie rule
(`Step::keep_incumbent`, `Step::refuse_tie`). The three folds read it
with every outcome unchanged:

- `join::keep_nearer` is its adapter for `PartnerDistance`: keep the
  incumbent.
- `link`'s piece-end turn fold: keep the incumbent (`Equal` is a tie).
- `offset_derive::nearest`: refuse a tie, as `Unsupported`.

Each step still refuses at its first comparison that does not decide.
That is today's semantics, which the door keeps.

**What remains** is the design question above: what an argmin's kept
reading is (the in-contention set and its rule), and which tie rule
each site wants. It is at the designers. Its ruling lands as a change
to `argmin_decided` alone. Until then the steps' `?`s stay registered
in `escalation-question-marks.counts`, citing this row.

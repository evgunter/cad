---
id: the-value-edit-numbering-check-costs-a-replay-per-swept-profile
kind: issue
title: The value-edit numbering check replays every swept profile that reads the edited value, twice
status: closed
opened: 2026-09-24
priority: P2
cost: D
closed: 2026-09-25
pr: 3223
---


`reanchor_report` (`crates/editor-core/src/edit.rs`, PR #3180) replays
every swept profile the edited value reaches, on both sides of the edit,
to compare the numberings. Profiles that nothing sweeps are skipped before
any replay. Neither side is validated: `eval::anchor::readable_naming` is
the replay-only reading, which equals the validated anchor wherever the
loops validate.

These are debug-build timings per `SetDocParamValue`, for profiles of a
square plus 20 circles all reading one parameter. The scratch bench is the
reviewer's `rv_cost`, extended with an extrude per profile.

| profiles | swept | without the check | with it |
|---|---|---|---|
| 200 | no | 7.9 ms | 7.9 ms |
| 200 | yes | 10.4 ms | 28.8 ms |
| 20 | yes | 1.0 ms | 2.8 ms |

For comparison, #3180's first push measured 285 ms on the reviewer's
200-profile no-sweep bench, and 312 ms with sweeps. That push validated
both sides and did not skip unswept profiles.

The remaining ~2.8× is the two replays per swept profile. Option A of
`a-value-edits-last-published-numbering-is-not-recipe-state` removes the
old-side replay, because the published numbering would be read off the
document. A dependency filter finer than "the program references the
param" would remove more.

## Ruled (2026-09-25)

Under the minted-id rule (#3193), the value-edit numbering check is
deleted, not made cheaper. Parked on
`profile-pieces-are-named-by-minted-step-ids`, which closes it.

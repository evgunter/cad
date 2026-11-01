---
id: one-material-leaves-its-in-band-arm-and-one-sector-reason-unpinned
kind: issue
title: rest::one_material's two-sector in-band arm is unpinned and its one-sector rule hides the tie loop's blind spot
status: open
opened: 2026-10-26
priority: P4
cost: E
refs: [tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid]
---



Found by PR 5704's second verifier (VERIFIED-WITH-NITS at `7c65beeb4a`).

## What

- **An unpinned arm.** In `rest::one_material`'s crossing guard, the
  `through >= 2 → in_band` arm turns "one sector confirmed near the
  ball, another only far out" into an in-band refusal. A mutant dropping
  it survived the 337 census and line-touch rows. Every case the
  verifier sketched also went in band through `crosses`' own re-check
  near the ball, so the arm may be redundant; it is neither argued nor
  pinned. This is the same class as the `wrap` arm the first verifier
  found unpinned.
- **A comment that gives the wrong reason.** The one-sector rule ("One
  sheet alone") argues from geometry. It exists because the tie loop
  reads a lone plane `InBand`: with the rule removed, a lone disc and a
  crack tip refuse in band. The comment should say so, so the tie
  loop's blind spot is visible.
- **One defect, two refusals.** A face or surface key that does not
  resolve reads `Corrupt`, but an edge key that does not resolve still
  reads `Unreadable`, reconciled only in a parenthetical in
  `TouchUnread::Unreadable`'s doc.
- **Duplicate candidates.** The guard reads every ray two wedges share
  twice, once as each wedge's end.
- **Unreachable today:** a loop winding twice round the tip reads Rest
  through the one-sector rule (`face_piece` assumes a simple loop). The
  census refuses such a loop's own crossing as `EdgeEdgeCross`, and a
  one-sector link cannot reach the census from a body that passes tier 2.

## Owed

Pin the arm with a row, or argue it redundant in the doc and remove it.
Re-word the one-sector comment. Have an unresolved edge key read
`Corrupt` like a face key. Read each shared ray once.

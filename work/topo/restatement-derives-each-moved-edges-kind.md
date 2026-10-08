---
id: restatement-derives-each-moved-edges-kind
kind: issue
title: One restatement function derives each moved edge's kind by tier 3's check-4 predicate; its details and D2's prefer-intrinsic authority rule wait on the intent refactor
status: open
opened: 2026-10-06
priority: P3
cost: M
blocked_on: [intent-stage4-is-built]
---


## What

Choice 3 of PR 3970, ruled in outline on 2026-10-06; see `kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`, "Ruled". Ev: "the overall idea of the final state sounds good, but the details here (incliding the ones the designers differ on) seem like they'll be changed by the `intent` refactor."

The outline:
- One restatement function, called by the describing twins, `set_face_surfaces_describing`, the shell, the offset doors, the boolean's describe pass and the split's finish.
- It derives each moved edge's kind from the surfaces its faces wear after the move, through one shared predicate, tier 3 check 4's reading. Today that reading is spelled three times: check 4 inline, `must_carry_over_edge`, and `describe_edges`.
- It re-certifies what it derives.
- An in-band dihedral refuses typed.

The round-5 reports are on `analysis/design-fork/topo-kef-kfmrh-r5-{A,B}` and in the PR 3970 thread.

Open, for the intent refactor:
- A tangent, second-order-determined edge stored as a chart: turn it into a `TangentIntersection`, or keep the chart.
- A declared chart image moved onto a transverse pair: refuse, or drop the declaration.
- D2's prefer-intrinsic authority rule. Ev's 2026-07-19 text names no authority exemption. The agent-written clause from `99cc678bfd` exempts declared descriptions. Tier 3 check 4 exempts derived ones. See also `offset-held-neighbour-image-keeps-a-declared-chart-on-a-transverse-section`.

## What waits on it

The boolean's four seam kills onto kept faces (`zip_seam`'s retiring
kills, `zip_folded`'s two in `boolean/rest.rs`) cannot move to
`kef_describing` with a key swap: the seam edge's stored description
names a surface other than the survivor's, so its re-description has
to be derived, kind included. Nor can the coplanar merge's kill: the
rest of a shared chain lands with both halves on the kept face, where
the key-swapped `Intersection(kept, kept)` does not certify and a
chart image is owed. Until this row's restater exists they
stay on `kef_minting`, and `kef_minting` cannot be absorbed
(`kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`,
"Built, and what waits"). The restater plugs into the twins at
`Body::vouch_described_move` (`crates/topo/src/attach.rs`), where a
listed description is certified today.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: Ev held its details for the refactor; they turn on declared chart images, canonical carrier forms and the rest.rs zip kills, which stage 4 settles. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

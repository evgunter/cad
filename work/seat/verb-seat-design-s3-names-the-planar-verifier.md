---
id: verb-seat-design-s3-names-the-planar-verifier
kind: issue
title: VERB-SEAT-DESIGN S3 names the planar verifier the detector no longer runs (Ev-gated)
status: open
opened: 2026-09-05
needs_ev: true
---


Disclosed by SEAT-FW; **Ev-gated**, which is why the unit stopped
rather than editing.

`docs/VERB-SEAT-DESIGN.md` is ratified (#1388) and its §1 S3 describes
the flush detector as "the C4 verifier run in candidate-generation
mode — the verifier (`oriented_plane_eq`,
`topo/src/boolean/plane_eq.rs`) already lives there"
(`docs/VERB-SEAT-DESIGN.md:113-119`). The clause's ARGUMENT is
untouched by SEAT-FW — the detector is still the verifier in
candidate-generation mode, and the anti-twin rule still holds by
construction — but the function it names is no longer the one the
detector runs: `topo::flush::pair_finding` asks
`carrier_pair_relation`, whose `(Plane, Plane)` arm delegates to that
same verdict function. A reader of the charter today is told the
detector's arm is the planar one.

The unit's spec named SELECT-DESIGN §3's scope paragraph as the prose
it may amend, and a ratified-doc change beyond that is Ev's call, so
the charter sentence stands unedited.

Two lines, whoever takes it:

- S3's parenthetical: name `carrier_pair_relation` (the door
  verify-at-use calls) instead of `oriented_plane_eq`, and say that
  the detector's scope is the `Rest` ladder's rather than the plane
  ladder's.
- The acceptance bullet at `docs/VERB-SEAT-DESIGN.md:369` reads
  "twopeg's nine and lily's six declarations collapse to detector +
  declare". **The lily's count is wrong and always was**: the corm
  carries ONE bore wall face and the foot three, so the socket is
  THREE declarations, not six — measured in
  `demos/tour/src/lily.rs`'s
  `the_curved_rungs_declare_the_socket_and_leave_the_stem_glue_alone`.
  (The scene's own prose said six too, and SEAT-FW deleted that
  sentence with the hand assembly it described.)

One line citation in the same clause has drifted too and was left with
it: S3 cites `editor-core/src/names/flush.rs:187` for the name-level
detector, which is `:182` after SEAT-FW's module-doc edit.

## Decision for Ev (2026-09-05; the `[ev]` PR carries it)

**Ratify two factual corrections to the ratified charter's §1 S3** —
its argument is untouched by SEAT-FW; two of its facts are now false:

1. The parenthetical names `oriented_plane_eq` as the verifier the
   detector runs. Since SEAT-FW (PR 1974) the detector asks
   `carrier_pair_relation` — the door verify-at-use itself calls — in
   candidate-generation posture; `oriented_plane_eq` is that door's
   `(Plane, Plane)` arm. Proposed text names the door and says the
   detector's scope is the `Rest` ladder's. (The anti-twin rule the
   clause states is STRONGER after the change — one function, two
   postures — which is why this is a correction, not a re-litigation.)
2. The acceptance bullet says "lily's six declarations". The socket is
   measurably THREE (one corm bore wall × three foot faces —
   `demos/tour/src/lily.rs`'s measurement row); the scene's own prose
   said six and was deleted with the hand assembly it described.

Both edits are made in place on the `[ev]` PR as the proposed
revision; the stale `flush.rs:187` line citation in the same clause is
replaced by the door's name. Nothing else in the doc moves. If you
would rather the charter keep its 2026-08-31 wording with a dated
note, say so and the edit becomes a footnote instead.

---
id: a-leaned-zero-reads-germs-far-apart-in-direction-as-one-line
kind: issue
title: A leaned zero reads germs far apart in direction as one line, and the order then rests on the second-order tie
status: open
opened: 2026-11-13
priority: P2
cost: M
---

Found by PR 6871's FULL review (`topo::star` stage 3), finding 1. The orchestrator ruled to keep the lean doctrine as it stands and file this row.

## What

The star's one germ order (`star/order.rs`, `order_sine`) reads every side and turn of two germs levered by their smaller lean (`star::leaned_lever`). A face line's lean is `|n_a × n_b|`. Where it is tiny, the leaned reading decides zero for germs that lie far apart as computed, so the two read as one line. The order then rests on the caller's tie, one order down (`geom_brep::tied_germ_turn` along the first germ's tangent, or `vtxfac::tie_breaks_after`), at directions that do not share a tangent.

- **The reviewer's probe set.** Germs at 0°, ±60° and 180°, each a face line leaning `1e-7` or `1e-6`, at arm `1e-3`, called on the door directly.
  - Every leaned sine reads zero.
  - The ±60° germs read along the first germ and opposite each other.
  - On main's code the split mapped that to an `unreachable!` (`OrderFault::Opposite`), a reachable panic.
  - PR 6871 folds `Opposite` into `OrderFault::Coincident`, a typed refusal, pinned by `star::order::tests::germs_far_apart_on_steep_face_lines_refuse_as_coincident`.
  - With no third germ to expose the contradiction, two such germs still read as one line and go to the tie.
- **Why the doctrine reads them so.** By `leaned_lever`'s derivation, a face line turns by up to `δ / sin φ` under a face tilt `δ ≤ zero / L`. A leaned zero means `|sin θ|·L·min(s) < zero`, so the bare reading `|sin θ|·L` is below `zero / min(s)`, which is inside the bare noise bound `zero/s₀ + zero/s₁`. Every leaned zero is consistent with "along within the faces' tolerance". Germs 60° apart at lean `1e-7` are, by the doctrine, within tolerance of along.
- **A remedy tried and reverted.** PR 6871's fix pass tried taking a leaned zero as one line only where the same sine read at the bare arm is not definite. It moved 216 of 4 560 readings of the doctrine's own pin, `boolean::insert::tests::face_lines_tilted_within_the_zero_decide_as_upright`, for example "edge φ 1 L 0.01 θ 0 axis 0 t 0.2: Some(true) → None": a face line tilted within the zero, which the doctrine decides as upright, refused instead. A form of the cosine fails as well: `1 − cos θ` levered at `L·s` also reads zero at 60° with lean `1e-7`.

## Owed

Before the doctrine is revisited, measure whether any real body reaches a far-apart leaned pair: two germs that read zero leaned while their bare sine is definite, at the split's `section_chords`, the pierce ring's `vtxfac::germ_order` or the vertex–vertex corner order (`insert::strut_order`, `walks_after`). Where one does, record what the second-order tie decides there, and whether the build it gives is right against an independent oracle. Then decide whether the doctrine needs a far-apart arm, and what it would read.


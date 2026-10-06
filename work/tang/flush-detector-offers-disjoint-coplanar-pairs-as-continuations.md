---
id: flush-detector-offers-disjoint-coplanar-pairs-as-continuations
kind: issue
title: find_flush_candidates offers disjoint coplanar face pairs — as Continuation, and opposite-facing as Rest — though neither abuts nor overlaps
status: open
opened: 2026-10-02
---


## What

`pncad::topo::flush::find_flush_candidates` reports a cross-body face
pair as a coincidence whenever the two faces share a carrier, in either
sense, even when the faces' extents are disjoint. Such a pair neither
abuts nor overlaps, so by the contract it is no coincidence:

- `crates/topo/README.md`, C4's **Continuation** paragraph: "one
  surface carried on, whether the two abut along a boundary curve or
  overlap on a patch";
- `BooleanCoincidence::Continuation`'s doc in
  `crates/topo/src/contact.rs` (:126): "one surface carried on,
  abutting or overlapping".

The candidate check is `carrier_pair_relation` /
`carrier_pair_verdict` in `crates/topo/src/boolean/rest.rs` (:922),
which decides carrier identity and sense over the pair's extent and
never whether the two faces' regions meet.

## Measured

- **The tour's `projectbox`** (`demos/tour/src/projectbox.rs`, four
  round bosses standing on the floor, each unioned through
  `crate::booleans::try_union_declared`). The k-th boss union (k = 0..3)
  is offered its cap-on-floor `Contact(Rest)` plus k `Continuation`s:
  its top (z = 0.875) against the tops of the bosses already standing,
  which are disjoint discs. That is 4 + (0+1+2+3) = 10 findings where 4
  are real; `booleans::consumer_census` pins the 10. Each union,
  measured three ways (PR 3899 review): undeclared, it refuses only on
  the Rest pair; with the Rest pair alone declared, it builds with
  volumes and face counts bit-identical to declaring everything; with
  everything declared, the same. So the six continuations are unused.
- **Two disjoint unit boxes** (tour test probe, since removed):
  A = [0,1]^3 against B = [2,3]x[0,1]x[1,2] (B's bottom on A's top
  plane, 1 apart in x, OPPOSITE senses) gives
  `[Contact(Rest), Continuation, Continuation]`. So a disjoint
  opposite-facing pair is offered as a **Rest** too, and the y = 0 and
  y = 1 sides come back as continuations. A against
  C = [2,3]x[0,1]x[0,1] (same-facing tops, bottoms and y sides) gives
  four `Continuation`s.

## Why it matters

`declare_all` over the findings is the natural spelling, so every
caller of it (the tour's `flush_declarations`, Python's
`Doc.declare_all`) declares contacts that do not exist. The op accepts
them today, but a declaration is the author's stated intent, and the
detector is putting words in it. A census or a record that reads the
declarations (C4's "declared contact as data") reads pairs that never
touch.

## Do

Make the candidate generation require the pair's regions to meet
(abut along a boundary curve or overlap on a patch) before offering a
`Continuation` or a contact. When it lands, `projectbox`'s census in
`demos/tour/src/booleans.rs` (`consumer_census`) drops from
4 Rest + 6 Continuation to 4 Rest; re-pin it there.

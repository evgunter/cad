---
id: identity-data-reads-its-ladder-from-geom-brep
kind: issue
title: carrier_eq::identity_data carries its own copy of the identity-ladder rule geom_brep::ladder::read_identity now owns
status: open
opened: 2026-10-14
priority: P1
cost: E
---


## What

The identity ladder's rule is "any definite datum answers; an in-band
one is kept, and refuses only where no later datum decides". It now has
a home in geom-brep: `geom_brep::ladder::read_identity`
(`crates/geom-brep/src/ladder.rs`). That helper is generic over the
per-datum reader, owns the detached / splice / splice-superseded log
handling, and returns `Apart{at, sign}`, `Zero(Option<MarginDiag>)` or
`Undecided(Kept<E>)` (non-empty, in data order). `pcurve_cache`'s spiric and Villarceau ladders read
it (PCERT,
`work/pcert/an-in-band-gate-masks-a-definite-one-in-the-spiric-and-villarceau-ladders.md`).

`topo::boolean::carrier_eq::identity_data`
(`crates/topo/src/boolean/carrier_eq.rs:1098`, PR 4918) spells the same
rule out by hand, so there are two copies of one rule.

## Fix

Read `identity_data` through `read_identity`:

- the reader is `CoincidenceMeasure::decide`;
- `Apart` maps to `IdentityData::Distinct` and `Zero(m)` to
  `IdentityData::One(m)`;
- `Undecided(kept)` is ranked by `ray_walk::Evidence` (unreadable ahead
  of in band) as today, over `kept.into_vec()`.

One difference to check: the helper splices runs in data order. It
holds the Zero runs read after the first kept one beside the kept ones,
where `identity_data` splices each Zero at once and the kept ones at the
end. The verdict and escalation channels keep their orders either way,
so only the probe sample stream's interleaving can move. Re-baseline any
golden that reads it, and say what moved.

## Two more to retire with it (PR 5089's reviews)

- **Two cause rankers.** `ladder::Kept::refusing_cause` puts the first
  poisoned reading ahead of the first in-band one, and topo's
  `ray_walk::Evidence::ranked` ranks blocked ahead of in-band. They are
  the same ranking twice. The port should keep one: `identity_data`
  ranks its `Kept` through `Evidence`, or `Evidence` reads
  `refusing_cause`'s order.
- **`topo::boolean::rim_wedge::shared_rim`**
  (`crates/topo/src/boolean/rim_wedge.rs:138`) is a near-parallel of
  `read_identity` with the polarity reversed. It takes the first rim
  pair decided one, keeps undecided pairs with their logs held, and
  splices them superseded once a pair answers. It is the same rule and
  the same recording, so it can read the ladder too: `read_identity`
  extended to a pass predicate, as
  `work/pcert/gate-loops-return-on-an-in-band-gate-before-a-definite-one.md`
  asks, covers the reversed polarity.

Ownership: `carrier_eq.rs` is claimed by CLEAVE, HONE and TANG (`work.py
territory`). This stays on CLEAVE, whose slate holds
`rim-identity-decides-centre-radius-and-axis-one-at-a-time`.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical** (cluster D). `geom_brep::ladder::read_identity` stays. Repoint `carrier_eq::identity_data`, and likely `ray_walk::Evidence::ranked`. DESIGN home: `[one-way.readings]` (geom-brep). Structural note: `carrier_eq` is a pair reading hosted in topo against the crate rule (umbrella `work/honeone/carrier-coincidence-is-decided-by-seven-readings`). **Cluster D, carrier readings re-spelled outside their named home.** DESIGN names the home. Ev (2026-10-26): the first fix is making the canonical reading the path of least resistance (the API consumers need), with a gate only as a backstop. 

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P4 → P1: `identity_data` spells out `read_identity`'s rule by hand: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

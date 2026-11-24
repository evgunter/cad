---
id: main-red-the-registry-buys-68-decisions-on-the-link-against-a-pin-of-60
kind: issue
title: main red: editor-core m10_9_pins_interval::m10_9_what_the_registry_buys reads 68 decisions bought on r2_link against a pin of 60 (slow set)
status: open
opened: 2026-11-24
priority: P1
cost: E
---



Found by the BOXFINISH one-plane lane (`boxfinish/one-plane`,
2026-11-24) running the symbolic pins its change could have moved. No
open row held it: `work/shell/main-red-three-m10-interval-pins-moved-at-5728.md`
holds an older move of the same row (the link's door-shut refusal).

**Measured** on main `e0ec409b2` (the lane's tree after merging main,
with main's `geom-brep/src/plane_eq.rs` and `carrier_eq.rs` restored),
default ε, the gate's profile:
`cargo nextest run -p editor-core --test all --ignore-default-filter m10_9_what_the_registry_buys`
fails at `crates/editor-core/tests/m10_9_pins_interval.rs`'s closing
assert:

> at eps=1e-9, what the registry buys moved: r2_link: the door
> discharges Some(68) decisions, measured 60: SymCounts { symbolic: 801,
> registrations_refused: 0, numeric: 929, retried: 0, frozen: 1493 } vs
> SymCounts { symbolic: 733, registrations_refused: 0, numeric: 997,
> retried: 0, frozen: 1493 }

The same counts, to the decision, with the lane's change applied, so
the one-plane reading does not reach it. The other documents of the
row read their pins. Not run on the lane's first base `dc21cb117`, and
not bisected: which merge moved the eight decisions is not known.
`sym11_exact_channel_rows`, `m10_9_the_pad_at_both_rule_f_dials`,
`m10_sym_profile_interval` and `sym_9_retry_interval` pass on the same
tree.

**Done when** the row is green: the pin re-cut with the merge that
moved it named, or the move shown to be a defect and held by its own
row.

Signed: (BOXFINISH one-plane lane)

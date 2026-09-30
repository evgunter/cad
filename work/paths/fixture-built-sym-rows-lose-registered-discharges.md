---
id: fixture-built-sym-rows-lose-registered-discharges
kind: issue
title: Fixture-built Sym rows lose the discharges the sweep used to register
status: open
opened: 2026-09-30
---


Found in 5a (`retire-the-stored-bulge`). Under #3453 a table-built arc
(fixture, copy, embedding) claims nothing: only the construction that
proves an arc's endpoint facts registers them (`profile::lower_arc`
through `Arc2::register_endpoints`), and the sweep registers only
rigidity (`sweep::swept::register_rigidity`,
`register_placed_landing`). A `Sym` row whose profile is written
through the fixture door therefore has no 2-D rim or landing to chain
the placed carrier through, and the discharges the sweep used to
register unconditionally are gone.

**The instance.** `crates/sweep/tests/sym11_far_placement_rows.rs`,
the certified lane's stadium (`stadium_extrude`, built with
`test_support::bulge_loop`): built → `MappedSource` at `(1e-6, 1e9)`
and `(1e-9, 1e6)`, and `Surface1Residual` → `EndpointStart` at the five
cells that refused already. Every one is a typed refusal; no cell
contradicts and none answers wrongly. The row asserts the refusals.

**Why neither obvious repair is in 5a.**
- The fixture door cannot register: `test_support::bulge_loop` holds
  no `Tol`, and `scripts/gates/witness-not-ambient.sh` refuses a
  `Tol::witness()` in `src/test_support.rs`, so it lowers with
  `lower_chain(chain, None)`.
- Re-authoring the stadium through the path algebra
  (`arc_to(Bulge)`) refuses at `Sym<Interval>` before it is built:
  `path_junction_turn`, enclosure `[−0.875, 0.875]`, already at
  `d = 0`, over the row's `r` box of ±ε/64. The fixture door makes no
  junction decision; the lattice does.

**Options.**
- A registering fixture door that takes the caller's `Tol`
  (`bulge_loop_registered(chain, tol)`, or a `tol` on `bulge_loop`,
  whose ~714 call sites argue for the former).
- Authoring the stadium without a junction decision the interval lane
  cannot make: a lattice spelling whose junctions are declared, or a
  `r` that is not a parameter.

Other `Sym` rows built from fixtures: `sweep/tests/revolve_washer.rs`
still passes (its registrations are the revolve's latitude carriers,
unchanged). Every editor-core `Sym` document is program-authored.

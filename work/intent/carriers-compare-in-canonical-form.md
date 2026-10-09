---
id: carriers-compare-in-canonical-form
kind: issue
title: D10 stage 4 PR C (with D): the door replays the document at Sym and proves a coincidence by two rungs: the kernel's carrier-pair verdict on the cells' symbolic carriers, then the deciding margin's identity
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [coincidences-are-recorded-at-one-door, a-computed-value-re-enters-as-a-constant]
---

INTENT stage 4, PR C, which absorbs PR D
(`the-door-s-third-rung-is-the-symbolic-tier`). Ev approved the design
in PR 4322 (fork log row 93, FORK-S4F); `docs/INTENT-STAGE4-SPEC.md`
§4 and §5 predate it, and where they disagree this row governs.

**No construction describes its outputs.** The door replays the
document in the `Sym` lane with every variable a symbol (definitions
bound, exact constants kept exact) and reads each recorded cell's
carrier off that replay at the cell's name. Nothing hand-writes a
carrier's form: there is no per-verb `CarrierFlow`, no `LinForm` or
`PoseForm` builder and no f64 corpus witness. A per-construction table
of output forms is only ever a cache generated from the symbolic
evaluation and checked against it.

**Two rungs**, after B's rung 1 (the same construction read twice):

1. **The kernel's own carrier-pair verdict** (`oriented_plane_eq`,
   `carrier_eq`; `crates/topo/src/boolean/plane_eq.rs:200`,
   `crates/topo/src/boolean/carrier_eq.rs:331`) run on the two cells'
   symbolic carriers. The verdicts already read each kind modulo its own
   symmetry, so that reduction is written once and the door adds none.
2. **The identity of the deciding margin**: the same decision's margin,
   read in the replay, is zero as a polynomial in the symbols with exact
   rational coefficients. This is what was PR D: `var_env_symbolic`
   binds every continuous free variable as `Sym::param` whatever its
   tolerance, rows are matched across lanes by (node, site, ordinal
   among that site's rows, cell names), and `Ratio::eval` is checked to
   be exact at `Sym`. It measures every box mitre (Ev, 2026-10-06) and
   closes `unproven-coincidence-lint-binds-every-variable-as-a-symbol`
   and `isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box`.

Either rung's zero is a theorem for every value at which the
construction takes the decisions it took here (E12). A replay that
refuses or escalates proves nothing, and the row stays unproven. The
row's discharge kind (B) says which rung proved it.

**No computed value re-enters as a constant.** A numerical routine's
output enters the replay as one opaque symbol per call. The guard is one
re-valuation test: re-value each corpus document's variables and
compare the carriers derived from the symbolic replay with the f64
build, so a laundered quantity (`T::from_f64` of a computed value) shows
wherever the re-valued variables move it. The test and the audit of
today's sites are `a-computed-value-re-enters-as-a-constant`; until that
lands, a laundered site can make either rung report a false proof.

**The recourse** (Q3) names the residual and the edit that would make
the two cells one construction: a residual `v − w` over two free
variables of one kind and equal value recourses "make the slot reading
`v` read `w`" (the GUI's offer, `typing-a-value-mints-or-offers-a-variable`);
a residual in two placed poses recourses "place one relative to the
other".

**Needs nothing from stage 2**: slots and placement steps are `VarId`s
since stage 1, and the replay reads them as they are. Needs B (the
records and the door). **Until stage 3, a solved pose is one opaque
symbol per solve.** H (`placed-carriers-compare-through-their-frames`)
becomes a measurement after stage 3 (does a mate-placed contact
discharge as a theorem?) and a build only if it does not.

**The discharge kind.** B's `topo::Coincidence` carries
`discharge: Discharge`, which has only `Numeric` (orchestrator ruling on
S4-B, 2026-10-08). This unit adds `Theorem(kind)` (the symbolic tier's
`SymbolicZero`, `SignGated`, `Registered`) and threads it from
`Decided` when it replays at `Sym`; `geom_core::Decided` carries no
discharge today (`Sym::sign_within` computes `how` and drops it).

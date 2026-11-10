---
id: a-tier-3-premise-a-dual-operand-skips
kind: issue
title: A reader that leans on a premise tier 3 holds is reached by a dual's operand, a Replay that tier 3 never read
status: open
opened: 2026-11-09
priority: P2
refs: [6689]
---


Filed by `onecopy/normal-fault-kind` (PR 6689), from its review.

**The class.** A reader states, as the reason an arm cannot be
reached or a datum can be taken unchecked, that "tier 3 refuses it
first" or "the operand is a finished body". That holds at a
certifying scalar (f64, Probe, Interval, Sym), whose finished body
passed tier 3. At a dual, `AtRestPolicy::Finished` is a `Replay`
(`crates/topo/src/props.rs`, `impl<T> AtRestPolicy for
geom_core::Dual<T>`: "tier 2 alone"). So a Boolean or split operand at
a dual can carry what tier 3 would refuse: a torus outside the ring
convention, an edge off its carrier, a ring with `R ≤ r`. The arm the
premise rules out is then reached, and it says the wrong thing,
usually a kernel invariant for what is an invalid input.

6689 fixed one instance: seam_germ_side's region normal at
`crates/topo/src/boolean/join.rs`, where the premise was stated, now
answers by the abort's row. The census does not have the problem,
because it does not run at a dual: `census_at_rest` and
`flush_at_rest` are empty for `Dual`.

**Sites to decide** (each: is it reached at a dual, and if so, what it
says there):

- `crates/topo/src/boolean/torus_region.rs:163`: a circle or spiric
  edge is "taken as a plane section of this torus, unchecked: every
  such edge of a torus face is one at rest (tier 3 places each edge on
  both its faces' carriers)".
- `crates/topo/src/boolean/ruled_region.rs:295`: the same premise for
  a cylinder or cone face's edges.
- `crates/topo/src/boolean/solid_contain.rs:779`: "Agreement across
  the group is a tier-3 obligation, not re-checked here."
- `crates/topo/src/boolean/solid_contain.rs:828`: "`R > r` by the
  ring-torus convention tier-3 check 1 enforces at rest".
- `crates/topo/src/boolean/contain.rs:1211`: "`q` is a vertex of the
  body, which tier 3's check 3 holds to the face's carrier", so the
  cut reading "never answers `OffCarrier`".
- `crates/topo/src/boolean/contact_verify.rs:134`: "Every caller reads
  a validated body, and tier 3 refuses such a torus first". Its
  callers today are the census only, which a dual never runs, but
  `rest_pair_verdict` is `pub`.
- `crates/editor-core/src/product.rs:1346`: `flush_at_rest` over the
  product's aggregate of placed copies. The aggregate as a whole is
  not a body tier 3 read.
- `crates/topo/src/validate.rs` `corner_normal`: it folds every
  normal refusal but an escalation into `None`, and
  `crates/topo/src/splitting/section_cones.rs` `fusion_at` refuses that
  `None` as `ConeCornerUnread` on a split operand. Is a torus outside
  the convention reached there at a dual?

**The sweep obligation.** Grep `crates/` for `tier 3 refuses`,
`tier-3 obligation`, `tier 3 places`, `tier 3's check`, `at rest
(tier`, `finished bod` and `validated body`. For every hit on a path a
dual reaches (the Boolean, the split, the shell, the product), either
answer by the error's own row with no premise, or state why a dual
cannot reach it, the way `crates/topo/src/census.rs` `at_rest_fault`
does. Then list the hits and their dispositions in the PR.

## Closed (2026-11-10, `onecopy/dual-tier3`): the premise is asserted at the evaluation service

On the orchestrator's ruling (B), the premise is held where DUAL-DESIGN DL3 already places it, now by type.
- **`editor_core::evaluate`** is bounded on `OwnRecord`, which no dual meets.
- **`evaluate_recorded`** is a dual's one door. It takes the base-scalar evaluation of record and refuses every node that record refused (`NodeErrorKind::RecordRefused`), before the node runs. A record of another document refuses every node (`RecordMispaired`).

So no topo reading a dual reaches through the service leans on a gate the record failed. The premise sites listed above cite one anchor, `topo::Replay`'s "A tier-3 premise at a dual". 6689's site at `join.rs` cites it too.

**Moved:** three msolve14 rows, two in the default set and `c5` in the slow set. The dual now reports `RecordRefused` where the record refused the mate bundle, and the rows read the record's own refusal there. No measured value's report changed.

**Left:** topo's own doors at a dual, called directly with no record, stay contract-bound: `topos-doors-at-a-dual-take-no-tier-3-evidence-by-type`.

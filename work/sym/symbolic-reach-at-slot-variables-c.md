---
id: symbolic-reach-at-slot-variables-c
kind: issue
title: The symbolic tier's reach across INTENT-LITERALS PR C (slots that hold variables), measured
status: closed
opened: 2026-10-06
closed: 2026-10-06
---


INTENT-LITERALS PR C makes every slot hold the id of a variable: a value
typed at a slot is an anonymous free variable, and a formula is an
anonymous defined one. Spec Q3 asked what that costs the symbolic tier.
It is measured here, under the rule Ev ruled at C (spec §11, VARIABLES-DESIGN
VR8): only a toleranced variable is an analysis axis, and an untoleranced
one binds in the Sym lane as its exact nominal.

**The first measurement, with every variable bound as a symbol, was
unaffordable.** That was Q3's original ruling. A datum frame's nine
components became nine symbols, and the Gram–Schmidt discharge over them
ran for minutes per box. 57 interval tests ran past their 120 s budget,
and `docm9_range::a_branch_change` ran past 50 minutes. gdb sampling put
all of it in `geom_core::sym::algebra::reduce` / `poly_subst_square`.
That cost is why the rule changed.

**Under the ruled rule, the tier's reach does not drop on any pinned
document.** The only column that moves is `symbolic_zero`, and it rises.
A formula at a slot is now a definition. The environment binds that
definition through the non-finite door (`Doc::bind_definitions` →
`expr::eval`), and the slot reads the bound variable through the door
once more, so each written formula adds a theorem per environment:

- `sym_9_retry_interval` (`crates/editor-core/tests/sym_9_retry_interval.rs`),
  theorems with and without the ladder alike: the plate 955 → 956, the
  annulus 440 → 442, the boss 459 → 463, the bracket 1259 → 1262, the
  link 689 → 691 (with the ladder 691 → 693). `sign_gated`,
  `registered`, `numeric` and `retried` do not move.
- `m10_sym_profile_interval`
  (`crates/editor-core/tests/m10_sym_profile_interval.rs`): one more
  `Plain/Decision` call on the slab (980 → 981) and on the plate
  (1143 → 1144), and those lines' digests. Every other line holds.
- `m10_bulge_interval` on the boss: `expr_non_finite` 29 → 33
  theorems, and nothing else.

**The untoleranced named variables now bind as constants.** Before C they
were symbols. That is the half of the rule that could have moved a
decision, and on the pinned documents it moved none: every column but the
theorem count is unchanged.

Open: none for the tier. Stage 4's structural rung must not inherit this
lane's constants; that is filed as
`work/intent/unproven-coincidence-lint-binds-every-variable-as-a-symbol.md`.

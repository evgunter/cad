---
id: derived-frame-placement-freezes-on-the-symbolic-lane
kind: issue
title: A profile placed on a derived frame does not certify on the symbolic lane under a widened upstream parameter: the kernel's symbolic budget freezes re-normalised stored unit vectors
status: open
opened: 2026-09-04
---


## What

Found by DOCM-1's dual review (PR 1829, R1's two red probes on
`docm/1-review-r1` @3435cf61:
`r1_c7_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin`,
`r1_c7_the_prs_transform_lifted_shape_with_an_extrude_above_it`; both
reproduced by the orchestrator and by the fix pass at ε default, 1e-6
and 1e-12). On `Sym<Interval>` an extrude of a profile on a DERIVED
frame (`Datum::FaceFrame`, DM1c) whose body carries a widened
parameter refuses certification (`carrier_endpoint_*` /
`carrier_on_surface_1` indeterminate, enclosure 6–12× the width) at
every width from ε/8 to 0.05, under both `ProfileLift`s; the
AUTHORED-frame twin with the same widened placement certifies under
both lifts through the same `frame_plane_lane` door. On plain
`Interval` the two frame kinds agree at every width.

## Diagnosis (the fix pass, instrumented)

`SymCounts` per evaluation: derived/Pinned `frozen = 16`,
derived/Guided `37`, authored `0`. Every freeze is a budget refusal in
`geom_core::sym::form_in` (`crates/geom-core/src/sym.rs:1478–1484`,
`combine(...).filter(within(budget))` → a frozen indeterminate), never
a missing node; the frozen ops are `Powi 2` / `Mul` / `Add` on kid
forms already at rational degree 65–128. Raising `max_degree` to 4096
(`max_terms` 65536) still leaves seven freezes, the survivors at
degree 433 and 670. Mechanism: a `Sqrt` is an indeterminate keyed by
its argument's form (`sym.rs:1397`), so each normalisation
`v / sqrt(v·v)` adds a denominator and each square doubles the degree;
the `Decide for Sym` rescue (`sym.rs:1871`) fires only on an
identically-zero form, and a frozen subtree cancels nothing. An
authored frame's axes are literals normalised once; a derived frame's
axes are the kernel's ALREADY-normalised stored vectors (the cap
normal and `u_ref`, themselves rational forms from the extrude's
`w.normalize()` and the profile plane's), which the boss extrude
normalises again and squares in certification. Emitting the
already-unit `u`, `n × u` without editor-core's re-normalisation
(a local experiment) still refuses (`frozen` 9/31): editor-core's
levels are not decisive, the degree comes from the stored vectors.

## Where the fix lives

The kernel's symbolic lane — `geom_core::sym` (a `Sqrt` of a
value-exact norm minted as a degree-resetting atom, or normalisation
simplified before squaring) or `topo::UnitVec3` / the extrude's
certification — outside DOCM's fence and inside M10's (E12, the
symbolic identity lane; the program stays open "until certification
is parameter-aware", and this is a case where it is not). DOCM-1
merged with the f64 and plain-`Interval` behaviour pinned and this
limitation disclosed in its PR body; R1's two rows are the pin, red
until this is answered, and become unit rows when it is.

## Home

M10. Filed by DOCM at DOCM-1's merge (2026-09-04).

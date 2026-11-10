---
id: skin-renders-a-knot-api-recourse-to-the-loft-user
kind: issue
title: SkinError::KnotAlgebra renders KnotAlgebraError's caller-facing recourse to the loft's user
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6684
---


Filed by the ONECOPY `onecopy/unbuilt-recourse` lane (PR 6622). That lane
found the same shape at `topo::SectionUnbuilt::Seat` and fixed it there.

## What

`sweep::skin::SkinError::KnotAlgebra` (`crates/sweep/src/skin.rs`) writes
its `KnotAlgebraError` whole (`Self::KnotAlgebra(e) => write!(f, "{e}")`).
The skin raises it from two operations the kernel chooses, on the user's
sections: `elevate_degree(raise)` and `NurbsCurve3::refine_to_union`.

`KnotAlgebraError`'s `Display` (`geom_core::spline::algebra`) ends on a
recourse addressed to whoever called the knot API, for example "Recourse: ask
for at most {left} more copies of it, or raise the degree" or "Recourse:
ask for a value the knot vector carries". Here that caller is the skin, not the user. So a loft refused here
tells its user to change a knot request they never made.

## What closing it takes

Decide what a refused elevation or union refinement of valid sections
means for the loft: a kernel defect, or a section shape the loft cannot
yet carry. Render it with that ending, as the payload's own recourse.
Do not pass the knot-API sentence through.

`geom_core::spline::KnotAlgebraError::fact` renders the refusal without
its caller-facing recourse (PR 6622 added it for `SectionUnbuilt::Seat`),
so a holder can keep the diagnostics and end on its own recourse.

## Closed

Closed by ONECOPY's kernel-built-recourse lane (PR 6684), on the ONECOPY orchestrator's ruling, option (a). `SkinError::KnotAlgebra` holds `geom_core::spline::Built<KnotAlgebraError>` and decides its ending on the payload:
- a knot or weight that is no point ends in "evaluate the loft at a point scalar";
- every other arm ends in `KERNEL_DEFECT_ENDING`.

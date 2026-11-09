---
id: skin-renders-a-knot-api-recourse-to-the-loft-user
kind: issue
title: SkinError::KnotAlgebra renders KnotAlgebraError's caller-facing recourse to the loft's user
status: open
opened: 2026-11-09
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

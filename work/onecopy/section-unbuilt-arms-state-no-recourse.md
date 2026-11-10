---
id: section-unbuilt-arms-state-no-recourse
kind: issue
title: SectionUnbuilt's Winds, TubeUnread and PadShort arms render no recourse, so GermSectionUnbuilt ends without one
status: closed
priority: P3
opened: 2026-11-06
closed: 2026-11-09
pr: 6622
---


Found in the review of PR 6548 (`onecopy/spline-walks`). That PR gave `SectionUnbuilt::Unwalked` the walk's own recourse (`geom::PieceWalkRefusal::recourse`).

Three of its sibling arms in `topo::boolean::SectionUnbuilt::write` (`crates/topo/src/boolean/mod.rs`) end on a description with no recourse:
- `Winds`: "a marched loop's turns about an operand's axis or core could not be read ({why:?})". This also renders a `Debug` payload.
- `TubeUnread`: "a marched loop's uniqueness tube could not be read …".
- `PadShort`: "a marched loop's carrier lies {hull_sup} m from the surfaces, past the {pad} m …".

`BooleanError::GermSectionUnbuilt` renders `why` and adds no ending of its own, so a Boolean refused on any of these tells the user nothing to do. Each needs its recourse decided: a kernel limit (`KERNEL_LIMIT_RECOURSE`), not yet (`NOT_YET_ENDING`), or a geometry lever.

Check `refusal_concision` too. It renders node refusals through the driver, so it should have caught these unless no row builds them.

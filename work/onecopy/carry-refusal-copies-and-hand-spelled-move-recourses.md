---
id: carry-refusal-copies-and-hand-spelled-move-recourses
kind: issue
title: SiteRefusal is a subset of CarryError rebuilt by a map, and NoCarrier spells its move recourse by hand
status: open
opened: 2026-11-09
---


Filed from PR 6667's FULL review (S2, S6) by the ONECOPY
`onecopy/carry-notes` lane.

## S2: is `SiteRefusal` just `CarryError`?

`geom_brep::pair_section::SiteRefusal` (`crates/geom-brep/src/pair_section.rs`)
is three arms:
- `Escalated(Indeterminate)`
- `Desync(&'static str)`
- `Unwalked(PieceWalkRefusal)`

Each of them is already a `CarryError`: `Escalated`, `Desync`, and
`Refused(CarryRefusal::Unwalked(_))`. The site reader's caller rebuilds
one from the other in a three-arm map (near line 2837,
`SiteRefusal::Escalated(diag) => CarryError::Escalated(diag)`, …). That
map is a second spelling of the correspondence.

**The question**: can a site read return `CarryError` (or a narrowed
type that converts by `From`) and the map go? The answer turns on
whether a site's refusal has a reader that must not see the other
`CarryRefusal` arms. `SiteRefusal`'s own `From<CarrierRefusal>` is the
one place it adds behaviour.

## S6: `NoCarrier`'s move recourse is spelled by hand

`NoCarrier::recourse` writes "Recourse: move one face so the surfaces
clearly cross or clearly part" and "… clearly part" as literals. The
kernel has a home for its geometry lever (`geom_core::COINCIDENCE_RECOURSE`,
`lever_recourse`, `coincidence_move_arm!`), and 216 lines spell a
"move …" recourse inline (`grep -rn 'Recourse: move' crates/*/src`). This
belongs with that move-recourse class: decide the class's one spelling,
then route `NoCarrier`'s two through it.

Main has since gained `geom_core::MOVE_RECOURSE`, `COINCIDENCE_RECOURSE`
labelled as a whole ending (`move_recourse!`). That is the likely one
spelling. `scripts/gates/recourse-ending-by-hand.sh` holds only pairs
named `X_RECOURSE` / `X_ENDING`, so it does not hold
`COINCIDENCE_RECOURSE` / `MOVE_RECOURSE`. About a dozen production sites
write `Recourse: {COINCIDENCE_RECOURSE}`, several of them continuing it
("; {note}", ", or …"). Deciding the class means deciding which of those
are the whole ending, and then extending the gate's pairs.

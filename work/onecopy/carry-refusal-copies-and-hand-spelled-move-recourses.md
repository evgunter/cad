---
id: carry-refusal-copies-and-hand-spelled-move-recourses
kind: issue
title: SiteRefusal is a subset of CarryError rebuilt by a map, and NoCarrier spells its move recourse by hand
status: closed
opened: 2026-11-09
closed: 2026-11-10
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

## Closed (ONECOPY, `onecopy/carry-copies`, 2026-11-10)

**S2.** A site read keeps returning `SiteRefusal`, because it has readers that must not see the other `CarryRefusal` arms. `topo`'s `chord_join` (`From<(FaceKey, SiteRefusal)> for SplitJoinError`), `boolean::join::site_refusal` and `boolean::section_table`'s `FootRefusal::Site` each map exactly its three arms into their own sorts; handed `CarryError`, each would have to answer a march, lane, pad, tube or seat refusal that no site makes. The type's doc now says it is a narrowing of `CarryError` and names those readers. The correspondence is spelled once, as `From<SiteRefusal> for CarryError`, and `CarriedComponent::holds` reads through it with `?`. The three-arm map is gone.

**S6, and the class.** The move recourse's home gains one arm. `geom_core::move_recourse!("so …")` is `Recourse: move the geometry so …`: the labelled move arm with the site's own condition after it. That is the composition `editor-core`'s naming levers and `topo`'s `chord_join` and `merge_faces` already write unlabelled (`concat!(coincidence_move_arm!(), " so …")`). `NoCarrier`'s two recourses go through it. Their text moves from "move one face so the surfaces …" to "move the geometry so the surfaces …"; no test or fixture pins either spelling.

The class's one spelling, then:
- `MOVE_RECOURSE` is the whole ending, continued or not.
- `COINCIDENCE_RECOURSE` is the fragment for a sentence of the site's own.
- `move_recourse!("so …")` is a move named more closely.

`scripts/gates/recourse-ending-by-hand.sh` now holds `COINCIDENCE_RECOURSE` / `MOVE_RECOURSE` as a named pair. It fails loud if either constant leaves geom-core, and its self-test fires on that case and on the pair's hand label. The 12 production sites that wrote `Recourse: {COINCIDENCE_RECOURSE}` or `Recourse: {}", …COINCIDENCE_RECOURSE` (intersect, frame, extrude ×3, revolve ×4, skin, ray_walk, replace_face) write `MOVE_RECOURSE`, with byte-identical text, including the ones that continue it ("; {note}"). Sites that compose the fragment mid-sentence after words of their own are the fragment's intended use and pass. Literal "Recourse: move …" spellings are the gate's stated blind spot, filed with their counts as `work/onecopy/move-the-geometry-is-spelled-as-a-literal-at-a-hundred-sites.md`.

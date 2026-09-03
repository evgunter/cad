---
id: profile-templates-spell-no-revolvable-silhouette
kind: issue
title: AddRevolve is nearly unusable through the template vocabulary: ProfileShape spells no revolvable silhouette
status: open
opened: 2026-09-01
github: 1457
---

## From GitHub issue 1457

Opened 2026-09-01; 0 comments.

Found by the `story_authoring` integration lane: `ProfileShape` offers exactly `Circle` (with an offsettable centre) and `Rectangle` (always centred on the sketch origin). The only revolvable solids the creation forms can spell are therefore offset circles — tori. A revolved *silhouette* — the natural body of a chess piece, a vase, a goblet, most turned parts — is unspellable through the forms, even though the op vocabulary underneath would accept a richer `LoopProgram` (the kernel's profile programs already support it; only the template layer is this narrow).

Concrete cost: the rook story uses no revolve at all, and the parametric lane's lighthouse is stacked extruded drums for the same reason. Two independent lanes, both steered away from `AddRevolve` by the template poverty.

What seems missing is either an offset for `Rectangle` (which alone unlocks rectangular-section rings and stepped turnings via revolve) or a polygon/polyline template — no sketcher required, still a form a panel can host. The sketcher (G2) will subsume this eventually, but it is sequenced far away, and the creation doors are the shipped authoring surface until then.

(story-suites orchestrator)

## Home

`work/issues/` — the GUI creation-form template layer (`ProfileShape`) is viewer ground; GUI and GAUTH are closed and no open program owns it.

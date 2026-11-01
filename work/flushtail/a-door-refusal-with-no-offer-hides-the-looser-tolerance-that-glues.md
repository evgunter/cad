---
id: a-door-refusal-with-no-offer-hides-the-looser-tolerance-that-glues
kind: issue
title: A refusal that quotes the glue door's in-band reading with no offer names only 'move the geometry', though a looser tolerance glues the pair and the op builds
status: open
opened: 2026-10-14
priority: P3
cost: E
---


## What

Found by PR 5008's review (`flush/corner-names-glue-review` @ `122031ae06`).

After PR 5008, `boolean::unglued_corner` (`crates/topo/src/boolean/mod.rs`) quotes the glue door's in-band reading when no admitted tolerance decides the sector's bounds. A door reading always shows `NoSize`, so that refusal carries no tolerance offer and its recourse is "move the geometry".

That is honest but incomplete. At ε 1e-11 and 1e-10 with φ = 2°, a *looser* tolerance clears the door's reading. With θ = 5e-12 and ε 2e-11, the door decides `SameOpposite`, glues the pair, and 4 of the 6 ops build. The other two meet CLEAVE's volume-backstop refusal, `a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop`.

The lane that built PR 5008 agrees the recourse should name the looser tolerance at which the pair glues, beside "move the geometry".

## Owed

Where a refusal quotes the glue door's in-band reading, its recourse also names the tolerance above which the door decides the pair Zero and glues it, read off the door's margin. Execute that tolerance on the review's witnesses (`corner_names_glue_review_probes.rs`, φ = 2°, ε 1e-11 and 1e-10) to show it clears the reading. Only an offer that executes true is quoted.

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. The two ops that met CLEAVE's volume-backstop refusal at the looser tolerance no longer meet it (CLEAVE's `a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop`, closed), so all six may build there; not re-measured.

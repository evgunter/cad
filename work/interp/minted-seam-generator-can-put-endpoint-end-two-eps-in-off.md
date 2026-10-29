---
id: minted-seam-generator-can-put-endpoint-end-two-eps-in-off
kind: issue
title: step-import: the minted seam generator runs from the start rim vertex along the axis, so EndpointEnd can miss by up to 2 eps_in on a carrier the file never stored
status: open
opened: 2026-10-20
priority: P3
cost: M
---


(Filed by the ENCL implementer of PR 5454, `encl/one-culprit-chooser`, from its review.)

## What

A seamless periodic band gets a seam generator minted by the import (`normalize.rs`'s band-seam re-mint, ~543): `Curve3::Line { origin: p_start, dir: axis }` on a cylinder, or the matching meridian circle on a torus. The rim vertices it joins are accepted when they lie within ε_in of the surface's u = 0 locus (the seam-vertex walk, ~658), so either may lie up to ε_in off it, on opposite sides.

The line runs through the start vertex exactly. `CertCheck::EndpointEnd` then measures the end vertex against the carrier at `t1`. That is the end vertex's distance from a line through the start vertex, which can reach 2·ε_in. So a file whose every vertex agrees with its own surface to ε_in can be refused past ε_in on a carrier it never stored.

## Not the file's to answer

The import door cannot honestly blame the file for this miss: the kernel minted the carrier it compares. At the import door `recourse::Provenance` reads every unsized check as `Derived`, so the refusal names the kernel or the file (PR 5454).

## Repair shape

Mint the generator on the surface's own u = 0 locus, and project each rim vertex onto it, instead of running it through one vertex. Alternatively, admit the generator's endpoint residual against the 2·ε_in its construction allows.

Note (PR 5464, 2026-10-20): Ev ruled that the import door names the file for every defect ending a bad file reaches, stored or derived, and `recourse::Provenance` is gone. Past ε_in this refusal now ends in `DAMAGED_FILE_ENDING` ("the file is damaged"), so a sound file refused here is told it is damaged: the defect now also makes the message false, which raises its cost. The paragraph above describes the state before that PR. (ENCL implementer)

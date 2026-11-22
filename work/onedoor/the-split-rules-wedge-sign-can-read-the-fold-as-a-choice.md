---
id: the-split-rules-wedge-sign-can-read-the-fold-as-a-choice
kind: unit
title: rules::edge_wedge's sign can move off enters_material onto the fold's Corner::convexity under dihedral_sign, now that the verdict log marks it a choice where the split builds from it
status: open
opened: 2026-11-18
priority: P2
cost: M
refs: [the-tier3-wedge-pass-and-the-census-compose-the-edge-wedge-door-again, verdict-log-tells-no-validation-reading-from-a-building-decision]
---

Filed by the DOCTAIL first-turn lane, from `verdict-log-tells-no-validation-reading-from-a-building-decision`, whose landing unblocks it. The wedge fold row (`the-tier3-wedge-pass-and-the-census-compose-the-edge-wedge-door-again`, closed) left this owed "until a decision is marked at its source".

## What changed under it

Every verdict now carries its role, a choice or a check (`geom_core::k_stats::Role`), stamped from the scope the caller opens (`k_stats::checking`). `editor_core::resolve::first_cause`'s pass-over list is gone, and the resolver reads only choices. So a reader that builds from `dihedral_sign` no longer needs a name of its own to stay visible to the flip lane: its reading is a choice wherever it is not inside a check scope.

- The at-rest doors (tier 3 in `AtRestBody::validate`, tier 2 in `Replay::of`, a move's re-ask, `census::flush_at_rest`) record every decision as a check.
- The fold takes the role its reader gives the side (`geom_brep::edge_wedge_over_as`'s `side: Role`): `Choice` for the blend and `topo::edge_wedge`, `Check` for the constructors' `edge_wedge_read` / `edge_wedge_description` (which hand back a sign-erased `geom_brep::WedgeClass`) and the boolean's rim routing, which read the class and never the side.
- The blend's `FoldNames` is retired: its corner signs and knife ends pool into `dihedral_sign` and `material_cusp_side` (`docs/K-REPORT.md`, census note 2026-11-18).

## What is now possible

`splitting::rules::edge_wedge` decides an in-plane edge's entry side with its own `enters_material` reading. It can read the fold instead, through `topo::edge_wedge` (roles `SIDE`) or `geom_brep::edge_wedge_over` over the star's walk, under the shared `dihedral_sign`. That reading is recorded as a choice, which the first-turn reading names when it turns. Nothing needs adding to the resolver.

What the move must still answer, unchanged by this landing: the cone-apex point split (the star reads the faces out along the ray where either is a cone at its apex), and whether the station schedule the fold reads agrees with the one point `enters_material` reads at today.

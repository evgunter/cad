---
id: cover-readers-moved-the-plates-sym-ledger-undisclosed
kind: issue
title: PR 6588's cover readers grew the plate's Plain/Decision walk by 2214 forms (frozen 36 → 62, largest form 28 → 38) with no decision moving; its body said the row passed
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Filed by `onecopy/plate-ledger-pin`, which re-pinned the plate.

Bisected first-parent over main: merge `6bbebd7e31` (PR 6588, `boxes/cover-app-readers`) moved `editor-core` `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned` from `Plain/Decision calls 1203 forms 51423 frozen 36` to `forms 53637 frozen 62`, and the largest form built from 28 to 38 terms. Every other walk line is byte-identical across the merge, and the plate's decisions are too (`Definite` 109 / 715, `Symbolic` 1203, no theorem disputed); the session's frozen forms rise 229 → 255 and its atoms 1338 → 1513.

The op profile (`sym_profile_plate_nominal`, before → after): `Max` +71, `Min` +10, `Sqrt` +90, `Sin` +12, `Cos` +14, `Powi` +244 (degree up to 110), `Add` up to 38 terms. That is a reach read off a face cover, which is the PR's subject (mate and trace reaches read topo's face cover). Reverting `geom/src/cover.rs` alone at the merge leaves the ledger moved, so the line site's re-posed arc range is not the cause; the readers are.

The PR's body says the row passes at its final head. Main at its merge fails it, so the move reached main red and undisclosed.

What is owed: say whether the plain walk's 26 newly frozen forms and the 2214 forms are the price of the cover reading, or whether the reach can be read without building them (the decisions do not need them, since none moved).

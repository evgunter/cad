---
id: the-retired-min-clearance-refusal-names-two-seats-it-does-not-carry
kind: issue
title: measure: the retired min_clearance refusal says to re-author over 'the same two seats' and carries neither
status: open
opened: 2026-11-22
priority: P4
cost: E
---


**Found by OFFER's sweep** (`a-check-offers-no-way-to-add-the-assertion-it-asks-for`, 2026-11-22), over every recourse sentence in `crates/*/src` that advises an assertion.

`RetiredMinClearance` (`crates/editor-core/src/measure.rs`) is a unit struct, and its sentence says "Recourse: re-author it as a clearance assertion over the same two seats". DESIGN `[one-way.places]`, "A refusal's recourse", says a failure carries as data what its recourse determines (the cells, and the edit where its layer can name one). The file that held the measure names the two seats, and the refusal drops them at the load door, so the sentence points at seats the reader has to find in the file by hand, and no door can offer the re-authoring.

The fix: the refusal carries the two seats the retired measure read, and the sentence names them. Whether it can also carry the `Node::clearance` edit depends on whether a bound can be stated; the retired measure had none, so the bound stays the person's, as an interference's is (`InterferenceFinding::quieting` carries the face pair and no bound).

Low traffic: only a file from before the retirement reaches it.

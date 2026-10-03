---
id: a-carried-rows-route-says-its-first-instance-by-tag
kind: unit
title: A carried row's route says this document's instance by tag where the assembly frame holds the document
status: review
opened: 2026-10-02
priority: P3
pr: 3841
cost: E
parent: node-labels-are-document-data
---


Found by `selection-door-refusals-speak-the-node`'s sweep. The rule is DESIGN.md Band 1, "Node labels".

`Route`'s `Display` (`assembly.rs`) reads "document {of} through instance {through} → instance {via}…". `through` is an instance node of THIS document, and `via` are nodes of the documents below. `AssemblyError::spoken` says every carried row by its tags (`CarriedMintRefusal`'s rows, `Attribution::Carried`), because the row's own ids are the part's. Only the first hop of the route is this document's, so a frame holding the document could say it as `InstantiatePart "left bracket" (…)` and keep the rest by tag.

The same holds wherever a `Route` prints at a frame that holds the outer document: `CarriedRefusal`, `CarriedUnplaced` (also in `ExportError::UnplacedBelow`), and `Attribution::Carried`.

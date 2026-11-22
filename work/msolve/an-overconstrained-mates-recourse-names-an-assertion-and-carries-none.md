---
id: an-overconstrained-mates-recourse-names-an-assertion-and-carries-none
kind: issue
title: mate: the overconstrained recourse says 'an assertion says so' and carries nothing that says which
status: open
opened: 2026-11-22
priority: P3
cost: M
---


**Found by OFFER's sweep** (`a-check-offers-no-way-to-add-the-assertion-it-asks-for`, 2026-11-22), over every recourse sentence in `crates/*/src` that advises an assertion.

`OVERCONSTRAINED_RECOURSE` (`crates/editor-core/src/mate.rs`) ends "where the two copies also meet, an assertion says so". DESIGN `[one-way.places]`, "A refusal's recourse", says a failure carries as data what its recourse determines, and its words are rendered from that. `MateFault::Overconstrained` names its constraints, and nothing in it says which assertion would say that the copies meet: no measure, no cells. A person reading it has to work out the `Gap` and its two faces alone, and the viewer's offers function (`a-refusal-offers-no-action-in-the-viewer`) has nothing to build a control from.

The coincidence finding now carries that value (`editor_core::Quieting`: the measure, its two references, `= 0`, and the edits that author it), minted by the rule that reads the authored assertion back. The fix here is the same shape: where the dropped mate held a contact the rest of the bundle leaves, the fault carries the claim that says it, and the sentence is rendered from it; where the kernel cannot state one, the clause goes.

Not checked: whether the mate layer can name the two cells at the point it refuses (the mate reads poses, and the contact's cells are the faces those poses are read off).

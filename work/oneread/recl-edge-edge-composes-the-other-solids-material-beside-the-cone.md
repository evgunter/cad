---
id: recl-edge-edge-composes-the-other-solids-material-beside-the-cone
kind: issue
title: recl::resolve_edge_edge composes the other solid's material at a vertex from its wedge and the faces holding the line, a second gathered reading beside the boolean's cone
status: open
opened: 2026-11-01
---

Filed by `the-boolean-reads-the-other-operand-through-one-cone`'s sweep
(branch `oneread/boolean-one-cone`), which made the boolean's published
vertex-vertex rows read each own edge through one cone of the other
operand gathered at the point (`topo::link::Cone::gathered`,
`boolean::vtxfac::cone_classes`).

## What

`crates/topo/src/boolean/recl.rs` `resolve_edge_edge` decides, for two
edges of the two solids lying on one line through a shared vertex,
whether a flanker's representative direction lies in the other solid's
material (`membership`): it composes that solid's wedge about the line
(`in_wedge_of`, and `wedge_is_reflex` where the two half-planes
disagree) with each of its faces holding the line beside the wedge, by
the side of the face the wedge's bounds read (`link_of`, through
`vtxfac::PierceDatum::side`). That is the material of the other solid at
the point gathered across keys (its vertex's sectors and the faces
through the point), read by convexity and composition rather than by
the cone's crossing parity: the same fact the gathered cone answers.

It is not this unit's: the spec keeps the boolean's construction data
(sector chains, reclassification), and this reading decides which germ
a reclassified record carries, not a published row. It is also one of
the consumers of TANGRIM's bound-side reading (`link_of` reads a
partner's bounds against a face through `PierceDatum::side`).

## Owed

Weigh whether `membership` can ask the gathered cone for the
representative's material (the wedge's faces and the held faces as one
`Cone`), keeping `link_of`'s construction checks; or say why its
composition reads something the cone does not.

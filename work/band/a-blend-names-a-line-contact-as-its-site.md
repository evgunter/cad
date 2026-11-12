---
id: a-blend-names-a-line-contact-as-its-site
kind: unit
title: A blend names a line contact as its site and fills its void crescents
status: open
opened: 2026-11-12
priority: P1
cost: H
parent: a-fillet-leaves-the-kiss-of-two-touching-solids-as-it-is-with-no-refusal
blocked_on: [a-finished-body-holds-its-contact-records-at-every-door]
---

Builds the rule Ev approved on PR 6807 (`a-fillet-leaves-the-kiss-of-two-touching-solids-as-it-is-with-no-refusal.md`, "The rule").

**The unit (sweep, blend).**
- One key enum for where two faces meet, `Edge(EdgeKey) | Contact(..)`, carried by `BlendRequest` (today `edges: Vec<EdgeKey>`, `crates/sweep/src/blend/battery.rs` ~254), `battery::Link` (today `edge: EdgeKey` plus `he_plus`/`start`/`end` from it, ~269) and the refusal site `BlendSite::Link { edge }` (`crates/sweep/src/blend/mod.rs` ~790). A contact lowers to the same `Link`: its two supports are the contact's two sides' faces, its direction and ends derived from the record's cells. The arms, predicates and surgery keep reading `Link`; no parallel contact path.
- A tangent touch's crescents go through the knife arm (`Branch::Knife`, `fillet3_knife_room`, PR 6510); a transverse touch's notches through the rim arms. Every void crescent at the contact fills.
- A contact is one link with its own ends, joined to no other link, until something needs a chain of them.
- The surgery merges the band's caps across the two pieces; the piece sort at finish decides the solid count. The record is consumed through the blend's lineage.
- A chamfer at a contact refuses its plane–plane arm, as at every knife.
- An edge request is unchanged (a requested smooth edge on a contact stays the identity).

**Consumers and pins:** two unit tubes kissing (ΔV = 2 × the slit's `valley_fill(r).area` per unit height, one solid, the record consumed); the same tubes bridged elsewhere (one solid losing its self-contact); a rod on a plate, seam on and off the line (edge–face and face–face records, the same result); the `NoRoom` refusal past the room.

**Needs:** the blend door seeing its operand's contact records (`a-finished-body-holds-its-contact-records-at-every-door`, HELDRECORDS).

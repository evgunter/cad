---
id: a-member-read-is-no-continuation-and-no-cell
kind: issue
title: Deleting fillet(xs[2]) strands its readers, and the at-rest rows and coincidence proofs read no member read
status: open
opened: 2026-11-10
priority: P3
cost: M
---

Found by the review of PR 6555 (F11). A selection of one member, `Select { body: xs[i] }`, is passed over at six sites that read a selection's body as a variable:

- `crates/editor-core/src/continuation.rs` `continues` (`Node::Fillet` / `Chamfer` / `Shell`): `None` for a member read, so deleting `fillet(xs[2])` splices nothing in its place and its readers are stranded, where a fillet of a plain read hands them its body.
- `crates/editor-core/src/checks/at_rest.rs` `Saying::reference` (the `selected` closure): a member read's one name is no row cell, so a measure or assertion over it quiets no at-rest row.
- `crates/editor-core/src/checks/at_rest.rs` `gap_assertions` (`face_of`): a member read names no copy the product gathers, so a gap assertion over it quiets nothing.
- `crates/editor-core/src/coincide.rs` (the traced-face construction in the profile's sweep, and `plane`'s `PoseDef::Plane` arm) and `crates/editor-core/src/coincide/stated.rs` (`entity`): a member read proves no construction, so a coincidence it would prove stays unproven.

Each is loud, never wrong: a stranded reader refuses, an unquieted row reports, an unproven coincidence stays a finding. One fix covers the five read sites: read the member's head as the family spells it (`crate::mate::member::spelled_whole`, written counts) and walk from the family. `continues` needs a member read to stand for, which a variable cannot: the continuation is `xs[i]` itself, which the splice must write as a read. Done when each reads a member read as its plain counterpart reads the body, or says why it cannot.

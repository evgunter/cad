---
id: a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence
kind: issue
title: Every declared-REST zip refusal ends on 'declare the coincidence', a lane reached only after a declaration
status: open
opened: 2026-09-30
priority: P2
cost: E
refs: [a-boss-flush-with-a-block-edge-refuses-its-declared-union, a-refusal-offers-no-action-in-the-viewer]
---


Found by AUTH-9 (`author/declared-union`) and its review, from the
refusal in `a-boss-flush-with-a-block-edge-refuses-its-declared-union`.

## The finding

`BooleanError::RestZipUnsupported`'s `Display`
(`crates/topo/src/boolean/mod.rs`, the `RestZipUnsupported { what }`
arm) ends every refusal it renders on `COINCIDENCE_RECOURSE`,
"declare the coincidence, move the geometry, or lower the tolerance".
All 21 `unsupported(` sites in `crates/topo/src/boolean/rest.rs` reach
it. The declared-REST lane is entered only once the contact IS
declared, so the first arm of that recourse is never true of a refusal
from it. Since AUTH-9 the viewer's boolean tool declares the contact
in one click, and the author reads this sentence directly afterwards
on the union's row.

`rest_zip_unsupported_carries_the_shared_recourse_once`
(`crates/topo/src/boolean/mod.rs`, tests) pins the defect: it asserts
the recourse appears exactly once.

## The class, a little wider

The review's through-cylinder scene (recorded on the seam-chord row)
fails past its declaration in a different lane, containment. It ends
on "Recourse: declare the coincidence" too (`BooleanError::Containment`,
"the solids do not cross, and the Boolean …"). So the class is "a
refusal reached only after a declaration still tells the author to
declare". The zip lane is its largest instance, and the containment
arm is worth checking in the same pass.

## Territory

`rest.rs` is claimed by TANG and ZIP. `boolean/mod.rs` is claimed by
no open program (`work.py territory`). The lane is ZIP's, so the row
is too.

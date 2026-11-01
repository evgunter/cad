---
id: pncad-py-has-no-door-that-mints-a-revolves-role-names
kind: issue
title: pncad-py has no door that mints a revolve's role name, so a Python author hand-writes the JSON
status: closed
opened: 2026-09-09
closed: 2026-09-09
refs: [no-facade-door-mints-a-revolves-role-names, LIB-NAMES]
---


The Rust façade now mints a revolve's role names in one call —
`pncad::select::{band, band_pi, band_rim, meridian_vertex, carried}`
(LIB-NAMES, closing `no-facade-door-mints-a-revolves-role-names`).
Python speaks names as TEXT (`crates/pncad-py/src/py/doc.rs:602`
`name_text`, `:619` `name_from_text`), and it authors exactly the two
selections the gap was opened for: `Node.fillet`'s frozen selection
(`doc.rs:1969`) and `Node.shell`'s open list (`doc.rs:2041`), both
taking names as text. A Python caller with no evaluation to select
against therefore hand-writes the serialized `StableName` JSON, field
by field, which is the same defect one alphabet over — and a worse
one, because the JSON's shape is file data rather than a type the
compiler checks.

What would close it: the same five doors on the Python side, each
answering the name TEXT the fillet and shell doors already take, so
that authoring a selection is one call there too.

Filed by LIB-NAMES rather than done by it: its brief scopes Python
out ("no Python doors"), and the binding's own curation and stub
rows are a separate lane's work.


## Closed

Closed at LIB-PYNAMES: `pncad.band`, `band_pi`, `band_rim`,
`meridian_vertex` and `carried` are module-level doors answering the
same name TEXT `Node.fillet` and `Node.shell` take, byte for byte
with the kernel's own emitter (`crates/pncad-py/tests/test_role_names.py`
pins each). The census family `B-NAME-BUILDERS` closed with it.

What it does not reach is the outer loop limit it inherits from the
Rust builders, which stays open as
`the-role-name-builders-reach-only-the-outer-profile-loop`.

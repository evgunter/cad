---
id: lib
kind: program
title: LIB — usable as a library
status: open
opened: 2026-08-06
area: api
prefix: lib/
ab_band: 300-399
paths: [crates/pncad/*, crates/pncad-py/*, docs/LIBRARY-DESIGN.md, docs/RECIPE-DOORS-DESIGN.md, docs/GUIDE.md]
keep_out: [kernel crates are VERBS and SEAT ground (LIB carries recipe doors and bindings only), the viewer is GUI-era ground, the analysis lane is M10's, evaluate's signature and the resolver door are design conversations before they are units]
---

Makes the kernel usable as a library under `docs/LIBRARY-DESIGN.md` (the
contract, ratified #229; there is no separate plan): the `pncad` façade and
prelude, the `pncad-py` bindings with their census and audit gates, the guide,
curation of carried payloads, and the recipe doors of
`docs/RECIPE-DOORS-DESIGN.md` (chamfer landed as G16; tube and shell remain).
Mechanical units run outside the model A/B by the 2026-08-29 ruling;
substantive units take the full protocol. Live narrative: `log.md`'s tail.

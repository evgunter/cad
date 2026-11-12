---
id: at-rest-constant-pairing-reads-two-non-parallel-planes-as-opposed
kind: issue
title: at_rest's constant_pairing accepts any two planes with no parallel check, so non-parallel plates can read opposed
status: open
opened: 2026-11-12
priority: P1
---

Filed by the ONEDOOR orchestrator from the face-pair designers (2026-11-12). Found by reading; not executed.

`crates/editor-core/src/checks/at_rest.rs`'s `constant_pairing` matches two planar faces and returns a constant pairing with no check that the planes are parallel. So two non-parallel plates can read "opposed" and quiet an interference assertion. One designer judged this likely screened by the `Gap` refusal that precedes it, without tracing every caller.

Wanted: a probe of two non-parallel plates through the quieting rule. The designed fix (`face-pair-contact-pairing-has-four-spellings`, "Designed") retires `constant_pairing` for the facing the `Gap` reading decides.

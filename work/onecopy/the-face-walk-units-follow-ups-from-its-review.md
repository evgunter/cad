---
id: the-face-walk-units-follow-ups-from-its-review
kind: issue
title: PR 6362 follow-ups: the join's germ faces panic in one file and refuse typed in the next; no pin for a stale proven face key
status: closed
opened: 2026-11-02
priority: P1
cost: E
refs: [6362]
branch: onecopy/face-loops
closed: 2026-11-03
pr: 6460
---

From the delta review of PR 6362 (`boolean-walks-face-to-surface-inline-at-two-dozen-sites`, `slit-reads-a-faces-edges-through-its-own-walk`), recorded at merge.

1. **The one rule (a key the kernel enumerated or recorded panics through `Body::proven_face_surface`; only input stays typed) is still split across the join's germ faces.** `SectionTable::key` (with `held`, `carried` and `germ_foot` through it) now panics on a stale germ face. `join::face_surface` reads the same germ and segment faces and refuses typed `JoinDesync`, and several join sites read a stale key as an honest no (`.is_ok_and`, `.ok()`, `let (Ok…, Ok…)` around the join's section and segment reads). Bring the join to the rule, or say at `join::face_surface` why its faces are input.
2. **No pin covers a stale *face key* reaching `proven`** (the conversion behind ~37 sites); a row asserting `unproven`'s message ("which this call resolved or read out of a record") covers the class. `proven_face_edges_panics_on_a_torn_mate` matches a substring every `dangling_link` panic shares; match the mate field (`claim.mate_field()`) so it pins the hop it names.
3. **Dead hops:** `side_reader::loop_aim` keeps `get_face(face)?` and `get_half_edge(first.half)?` after `proven_face_edges`, and their `None` can no longer fire.
4. **Docs:** `SolidFaces`' `# Panics` should say a selection minted for another body now panics in `point_in_solid_faces` (formerly `CorruptFace`), a foreign-key misuse of the Rust API.

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.

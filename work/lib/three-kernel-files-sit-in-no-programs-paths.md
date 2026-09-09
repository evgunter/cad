---
id: three-kernel-files-sit-in-no-programs-paths
kind: issue
title: three live kernel files are in no program's paths: editor-core mc.rs, names/interrogate.rs, topo flush.rs
status: open
opened: 2026-09-09
---


Found by LIB-MIRROR (PR #2271) while announcing its kernel derive
touches on the owning programs' logs: `work/*/program.md`'s `paths:`
globs cover none of `crates/editor-core/src/mc.rs` (the advisory
Monte Carlo lane), `crates/editor-core/src/names/interrogate.rs`
(`Denotation`, `face_frame`, the interrogation doors) or
`crates/topo/src/flush.rs` (the flush findings). A change there has
no tracker to announce on, so the announce-rather-than-avoid rule
has nowhere to land, and a program closing its walk cannot know the
file is drifting under it. Not LIB's to fix — territory is each
program's `program.md` — but LIB is the program that touched all
three this run (LIB-MC, LIB-DISCRIMINANTS, LIB-MIRROR), so it is
filed here with the three paths for whoever draws the next
territory map (`work/README.md` says how paths are claimed).

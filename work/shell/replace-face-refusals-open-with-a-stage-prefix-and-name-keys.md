---
id: replace-face-refusals-open-with-a-stage-prefix-and-name-keys
kind: issue
title: topo: every ReplaceFaceError arm opens with replace_face_offset: and names its face by arena key, and the shell op shows it whole
status: open
opened: 2026-09-23
---


## Finding

`ReplaceFaceError`'s `Display` (`crates/topo/src/replace_face.rs:502-695`)
opens nearly every arm with the function's name — `replace_face_offset:`
or `replace_faces_offset:` (`:506`, `:510`, `:513`, `:517`, `:521`,
`:526`, `:532`, `:536`, …) — and names its face, edge or partner face
through `Debug` (`{face:?}`, `{edge:?}`, `{other:?}`), which renders an
arena key (`FaceKey(3v1)`).

The shell op shows it whole: `ShellError::Face` and `ShellError::Lift`
(`crates/topo/src/shell.rs:614-620`) forward `{error}`, so the feature
tree's fault line reads "node 5 failed: the shell op refused: offsetting
a face inward refused: replace_face_offset: FaceKey(…) …". The two
wrappers no longer name a key themselves (CHROME concision fix pass,
PR #3108); the prefix and keys below them are this row.

`crates/topo/src/replace_face.rs` sits in open PR #2861
(`curved/spiric-1b`), so the rewrite was filed rather than made. The
feature tree's guard
(`crates/editor-core/tests/refusal_concision_chains.rs`, `FILED`)
admits exactly the `replace_face_offset` label on exactly the
`Shell/Face` and `Shell/Lift` rows, rendered over the one arm that
names no key (`Corrupt`); remove both entries, and render an arm that
names a face, when this lands.

## The standard

`work/chrome/error-and-check-text-overflows-its-region.md`, "The
standard a refusal is rewritten to": no stage prefix, no arena key
outside a kernel-bug arm, 75 words on the rendered chain.

---
id: a-label-may-be-only-invisible-or-direction-override-characters
kind: issue
title: Label::new admits a label of only zero-width characters, and bidi overrides
status: open
opened: 2026-10-01
priority: P3
cost: E
refs: [node-labels-are-document-data]
---

`Label::new` (`crates/editor-core/src/label.rs`) refuses blank text by `char::is_whitespace`, line breaks and `char::is_control`. Zero-width characters (U+200B ZERO WIDTH SPACE, U+200D, U+FEFF, U+2060) are neither, so a label of only those passes and reads as nothing on every surface; and the bidi overrides and isolates (U+202A–U+202E, U+2066–U+2069) are format characters, not controls, so a label can carry one and reorder the text a sentence prints after it (`Extrude "…" (tag)` — the tag could render before the label). Decide the rule: refuse format characters (`Cf`) outright, or refuse a label with no visible character and the bidi overrides only.

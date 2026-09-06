---
id: stl-header-refuses-plausible-names
kind: issue
title: A plausible part name is a hard panic in both demos — StlOptions::header refuses solid-block and anything over 80 bytes
status: open
opened: 2026-08-20
github: 743
refs: [732]
---

## From GitHub issue 743

Opened 2026-08-20; 0 comments.

Surfaced by the demos, found by #732's style review, and deliberately **not** fixed in #732 — per `memories/demo-purpose.md` the awkwardness is the finding and hiding it would delete the evidence.

## What happens

Both demos pipe an arbitrary body label straight into the new `StlOptions::header` and unwrap:

- `demos/tour/src/main.rs` — `header: label.clone()`, then `.unwrap_or_else(|e| panic!(…))`
- `demos/wild/src/main.rs` — `header: name.to_owned()`, same shape

Two ordinary part names take the whole demo down:

1. **`solid-block`, `solid_shaft`, `SolidWorks-import`** — refused with `StlError::HeaderSniffsAscii`. #732 widened that check to cover whitespace-skipping and case-folding sniffers, which is right for the format and makes this **more** likely, not less: `Solid Block` now refuses too.
2. **Any label over 80 bytes** — refused with `StlError::HeaderTooLong`.

Neither is a bug in the writer. Both refusals are correct: an 81-byte header does not fit binary STL's header field, and a header that reads as `solid` makes the file sniff as ASCII STL in some readers. The problem is at the **door**: a file-format quirk has become a constraint on a user's part name, with no fixup, no escape hatch, and nothing at the call site saying so.

## The behaviour question, which is why this is not a patch

What *should* a caller do with a part named `solid-block`? The candidates are all defensible and all different:

- **Nothing — the caller handles it.** Today's answer. Fail-loud, and it makes every consumer write the same three lines.
- **An escape at the door**, e.g. a constructor that takes a desired header and returns the refusal typed, so a caller can fall back deliberately.
- **A documented sanitizer** the caller opts into (prefix, truncate-with-ellipsis) — which is a silent fixup and this project refuses those by default, so it would need an argument.
- **Nothing in the library; fix the demos** to carry a fallback, which keeps the API honest and makes the demos say what a real consumer must do.

The last is probably right and is still a decision, not a cleanup — and the demos are the evidence that the door has this shape, so whatever is chosen should keep showing it rather than papering over it.

## Not to do

Do not narrow the sniff check to make `solid-block` pass. It was widened on purpose in #732 (`crates/stl/src/lib.rs`, `sniffs_as_ascii`), and the wide version is the one that matches the sentence the writer promises.

## Home

LIB: the LIB register fold placed this issue in its **category A** (the F1 curation-gap class) alongside #742 and #741 as the export option surface — a demo-surfaced library-API finding, which is the program's charter under `memories/demo-purpose.md`.

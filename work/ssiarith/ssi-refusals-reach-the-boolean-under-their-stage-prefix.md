---
id: ssi-refusals-reach-the-boolean-under-their-stage-prefix
kind: issue
title: SsiError's sentences open with an ssi: stage prefix inside the Boolean's GermSectionUnbuilt refusal
status: open
opened: 2026-11-09
---


Filed by the ONECOPY `onecopy/unbuilt-recourse` lane, whose gate
`topo::boolean::section_unbuilt_rows::every_section_unbuilt_arm_ends_on_one_recourse`
admits this by name in its `FILED` list.

## What

About 70 of `SsiError`'s `Display` sentences (`crates/geom-brep/src/ssi.rs`)
open with `ssi: `. For example, `InvalidMarchTol` renders "ssi: the marcher's step tolerance …
is not a usable length". `topo::SectionUnbuilt::March` renders the march's
refusal through `read_at` inside `BooleanError::GermSectionUnbuilt`, so the
Boolean's sentence reads "… meets the second operand's cylinder face: ssi:
the marcher's …". `test_utils::refusal::stage_prefixes` reads that as a
pipeline stage's label.

There are already payload-only helpers that drop the prefix: the two `sentence` readers
in `ssi.rs` documented "without the `ssi:` prefix". So the
prefix is a second spelling of what a holder that names the stage
itself already says.

## What closing it takes

Drop the prefix from the sentences, or have every holder that names its
own stage render the prefix-free sentence. Then delete the `March` entry
from `section_unbuilt_rows`' `FILED` list.

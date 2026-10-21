---
id: the-frame-seam-census-rows-guard-a-constructor-no-minted-frame-reads
kind: issue
title: The frame-seam census rows assert a seam count over minted planes whose frames no longer come from orthonormal_basis
status: open
opened: 2026-10-21
---


`crates/editor-core/tests/onb_wall_normal_census.rs`
(`no_corpus_face_sits_on_the_frame_seam`) and
`crates/step-export/tests/onb_wall_normal_census.rs`
(`no_fixture_face_sits_on_the_frame_seam`) assert that no planar face of
their corpora has a normal on `Vec3::orthonormal_basis`'s seam, because
that constructor once minted every Newell plane's `u_ref`. Since
`a-minted-reference-direction-follows-the-computing-axes`, a sweep's
caps, walls and split sections take their reference from their inputs,
so no frame these corpora store reads the seam and the two assertions
hold whatever the kernel does. Their `#[ignore]`d instruments still say
the stored frame is the constructor's
(`the_direction_records_every_stored_frame_is_written_to` counts
"frames the constructor mints").

`crates/step-import/tests/onb_wild_normal_census.rs` is different:
`step-import/src/recognize.rs` still charts an imported plane with
`orthonormal_basis`, so its census still measures stored frames.

Decide whether the two rows retire, or are re-aimed at what still reads
the constructor (the import, and the scratch homes the gate lists), and
trim `test_utils::seam_census`'s docs ("three corpus instruments") to
match.

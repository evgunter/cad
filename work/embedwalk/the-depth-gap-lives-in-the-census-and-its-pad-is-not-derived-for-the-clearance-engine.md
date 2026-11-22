---
id: the-depth-gap-lives-in-the-census-and-its-pad-is-not-derived-for-the-clearance-engine
kind: issue
title: The depth gap the clearance engine reads lives in the census's module, and the pad it takes off a gap is the Boolean sweep's, asserted and not derived for a reader that grants on it
status: open
opened: 2026-11-18
priority: P2
cost: M
---


From the review of PR 7119 (EMBED, 2026-11-18), filed by lane
`embed/nested-tori`.

## Evidence

`topo::clearance` reads `census::depth_gap::gaps_in_depth` at a face
pair's root (`parted_in_depth`). Two things about where that reading
sits:

- **Its home is one of its two readers'.** `crates/topo/src/census/depth_gap.rs`
  holds the gap, the face read (`face_read`) and two readings that are
  the census's alone (`toward`, `across`: the directions arm 1's
  turned-direction test reads), and its K names are
  `census_backstop_*`. PR 7119 names both readers in its header and in
  `boxes::sweep_pad`'s docs, and left the file where it is: moving the
  gap out takes `face_read` and the pad with it, splits the file and
  its fuzz (`depth_gap_fuzz`), and touches the census's README
  paragraphs and audit rows, on ground ATREST and HELDRECORDS own.
- **Its pad is asserted for this reader, not derived.** A gap is less
  `2·sweep_pad(band)`. `sweep_pad` (`escalate + 2·zero`) is derived
  for the Boolean sweep's event points, where a larger pad only admits
  more candidates. The depth gap takes it as "a placement misread
  within the band, and the boundary's vertices certified within ε of
  the face", and here a SMALLER pad is the unsafe direction: it parts
  faces a larger one would not. Nothing derives that those two terms
  bound what a face's cover does not hold of the face, for the census
  or for the engine. `clearance::depth_rows`'s unit row restates the
  pad by hand, so a change to it goes red there.

## What is needed

- A home for the gap and the face read that is neither reader's (a
  `topo` module beside `cover`), with `toward` and `across` left in
  the census.
- The pad derived at the gap: what a face cover may miss of its face
  (a trim decided within the band, a vertex within ε of its surface),
  term by term, or a different pad if those are not the terms.

`the-census-and-the-certificate-may-read-one-face-pair-twice` is the
wider question of the two readers; this row is the one function they
already share.

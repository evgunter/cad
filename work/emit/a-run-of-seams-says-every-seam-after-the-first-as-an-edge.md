---
id: a-run-of-seams-says-every-seam-after-the-first-as-an-edge
kind: issue
title: The words for an EdgeCrossing run of seams say every seam after the first as 'an edge runs through', so two seam-run vertices can read alike
status: closed
opened: 2026-10-13
closed: 2026-10-14
priority: P3
cost: E
branch: emit/union-readings
---


## What

The designer pair on fork-log row 130 found this; it is flagged and not yet measured.

`words.rs`'s `EdgeCrossing` arm says the first seam in a run by its faces. Every seam after the first it says only as "an edge runs through". So two vertices whose runs share a first seam and differ in the rest can read alike to a person.

The converged recommendation keeps exactly such vertices, for example the three-seam run where `c` cuts the glued rim.

**Owed:** words that say each seam in a run by its faces, and a `name_words_corpus` row that would catch two runs reading alike.

## Closed (branch `emit/union-readings`, 2026-10-14)

Measured on the three-block glued rim (`c`'s wall cutting `b`'s rim
glued on `a`): two vertices are runs of three seams. Within the
evaluation each read "the touch where the seam edge of a face and (b's
wall) of Union … runs through, an edge runs through and an edge runs
through of Union …". They read apart, by the first seam only.

`words.rs`'s `EdgeCrossing` arm now says a seam of the vertex's own
node by its two faces, each a citation of the run's own: "the seam of
X and Y runs through". Within the evaluation the same vertex reads
"… the seam of a face and (b's wall) runs through, the seam of a face
and a face runs through and the seam of a face and a face runs
through …"; in full, every face of every seam is said. How much of
each face is said is still the speaker's detail.

Rows in `name_words_corpus.rs`:
- `a_run_of_seams_says_each_seam_by_its_faces`: in full and within
  the evaluation, each of the two runs says "the seam of" three times
  and "an edge" nowhere, and the two read apart;
- the glued rim joins `documents_outside_the_corpus_read_apart_too`, so
  any two names of its tables said alike in any form fail.

No corpus name holds a seam run: `SAID_DIGEST` and `NAME_WORDS` did not
move.

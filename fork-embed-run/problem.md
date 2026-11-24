# Design question `embed-run`: what a face's "run" is, and what the embedding question promises beside a shared edge

**Read `docs/prompts/designer.md` in full before you start; it binds you.** You are a designer lane of the EMBED program in `evgunter/cad` (a B-rep CAD kernel in Rust). This is the statement of a problem, not of its solutions: the options are yours to find. Your report is at most 150 lines, in the two sections the prompt sets (`## For Ev`, `## For the orchestrator`). Do not name yourself or your model. Do not edit any file; read only.

## The ratified sentence

`docs/DESIGN.md`, `[topology.tiers]`, in the paragraph beginning "The question is asked inside the gate once tier 3 has passed":

> Where faces do meet, or a face is read against itself, the question reads from the verdict what tier 3 decided there (an edge's class and turn, an end, a pole or apex vertex) and decides none of it. Over each cell it reads the rate at which the two faces part, the faces' rates summed, levered at how far the narrower of them runs from what they share, measured across it ([tolerance.epsilon]), never at a cell of the question's own subdivision, and that passes by more than the band. Where tier 3 holds no fact, that reading is the only one. A cell holds none of a face only where it lies off the face. A reading inside the band refuses as ambiguous, never as Holds or Violated.

It was ratified by the owner (Ev) on PR 7456. Read that PR's body (`gh` is not available; the text is quoted in `work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`, sections `## Ruling` and `## Spec`, on `origin/embed/verdict-read`). Of the lever in that sentence ("the narrower of them") the owner wrote: "2 is also probably good, though i'm less sure that there won't be some issue later. we can proceed with it though". The check on provenance the prompt asks of you (`git log --all -S…`) applies: say what you find.

## What was built

PR 7666 (`origin/embed/verdict-read`, head `273f765de5`; not merged) builds the sentence. The orchestrator, elaborating the ruling for the implementer, defined a face's run from a shared edge as **the greatest distance at which the face stands from the shared locus's whole carrier** (for a line edge, the edge's whole line; for a circular edge, the whole circle), taken over the face's boundary stations and read from below; the lever of a pair is **the lesser of the two faces' runs**; the reading is the two faces' sines off the separator, **summed**, times that lever, against K·ε. Where to look: `crates/topo/src/rest_facts.rs` (`Tier3Facts`), `crates/topo/src/validate.rs` (where tier 3 keeps each run), `crates/geom-brep/src/extent.rs` (`Locus::distance_below`), `crates/topo/src/injective.rs` (`Facts::of`, `extent_of`, `parts`, `departure`), all on that branch. Both readers of the PR say the code is a faithful implementation of that definition. `[tolerance.epsilon]` in DESIGN says where a lever arm comes from.

## What was measured under it

Each of these is on the PR's head; the readers' own words are in `reviews/reader-1.md` and `reviews/reader-2.md` beside this file.

1. **The ruling's own flat strip**: a face 1e-7 m wide whose plane leaves its neighbour's at a sine of 5e-5, so it stands 5e-12 m from the neighbour everywhere across the shared edge. Run 1e-7; the reading is inside the band; refused. This is the case the ruling was made for.
2. **The same strip with one more boundary vertex a metre off the edge's line**, in the strip's plane, beyond the edge's end (an "ear"). Run 1.0; **certified**. It is a hand-built boundary, not a finished body (a body with such a strip has closing faces thinner than ε).
3. **A sliver 10 m long and 1 mm wide** whose two faces share its 1 mm tip edge and part at 5e-8 rad: within ε of each other for about 20 mm from the edge, 500ε apart at the far end. Both faces run 9.6 m from the tip edge's line; certified; tier 3's own check classes that edge a convex crease. A 1 m sliver at the same angle is certified too; one 20ε apart at its far end refuses on budget; slivers that are inside the band at the far end are refused by the Boolean before the question is asked.
4. **A strip 0.6·K·ε wide meeting its neighbour at 2.6 rad** is certified, though no point of it is more than 0.6·K·ε from the other face: the two faces' rates summed read the chord between their far sides (2·sin(θ/2) times the run), where one face's distance from the other is at most sin θ times the run.
5. An L-shaped face, a face with a hole beside the edge, and a face that touches the edge's line again far away all read their far arm, by inspection of the definition (no body was built).
6. A shared edge that is a spline is measured against the edge's own arc, so a face's run ALONG such an edge counts toward its run from it; for a line edge that reading was retired in this PR as an over-read. Unmeasured.
7. Tier 3's own check of an edge's dihedral (`check 4`) levers at the FARTHER face's run today (rows on another program's slate: `work/restfront/check-4-levers-a-wedge-at-the-farther-faces-run-so-a-narrow-face-inside-the-band-reads-a-crease.md` and its sibling), so the question can refuse a body tier 3 passed.

## The two readers

One reader holds that the definition certifies what the ruling meant to refuse: "R (the supremum over the whole face of the distance from the whole carrier line) is not 'how far the face runs from what they share, measured across it' wherever the face is wider somewhere else along the line: an L-shaped face, an ear, a hole beside the edge." The other holds that for a planar face the far arm is the right lever, "since tilting the plane about the edge moves the far arm", and found no defect. Both agree the sliver of case 3 is certified by the sentence as written, and that "the '20 mm within ε' stretch exists beside every crease near its edge". Neither reader's position is the orchestrator's to settle: it is a question about what the sentence means, and the owner stated a doubt about exactly this lever.

## What needs deciding

What the embedding question's lever beside a shared edge IS, such that the sentence's promise ("passes by more than the band"; "a reading inside the band refuses as ambiguous") says something definite about each of the cases above, for a face of any shape and any surface kind, and what the question therefore promises about a body it certifies. Say what each case should read under your answer and why, including cases 2 and 3; say whether the built definition is your answer, a special case of it, or wrong; say what your answer needs tier 3 to keep; and say whether the ratified sentence already says it or would change, and how. Whether tier 3's check 4 should move to the same lever is another program's open row: say what your answer implies for it and no more.

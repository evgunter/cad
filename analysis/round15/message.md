Round 15 on the turn-lever fork: the unfolded recount is in for the SWEEP suite. Read it below, answer two questions, and return your final text.

A. THE UNFOLDED RECOUNT, sweep, ε 1e-9, all 3,825 tests (84 minutes). "raw" is the covers' own far distance from the line, nothing folded in. Each pair is (past 1e-12 relative / past ε).

| class | reads | raw<below | raw<truth | raw<vertices | Zeros served at such reads |
|---|---|---|---|---|---|
| plane, straight | 1,768,879 | 0 / 0 | 0 / 0 | 0 / 0 | 0 |
| plane, straight (inexact) | 202,978 | 0 / 0 | – | 0 / 0 | 0 |
| plane, arcs | 554,854 | 0 / 0 | 0 / 0 | 0 / 0 | 0 |
| plane, arcs (inexact) | 53,102 | 15 / 0 | – | 0 / 0 | 0 |
| cylinder/cone wall | 539,124 | 5,476 / 5,246 | 5,476 / 5,246 | 526 / 342 | 0 |
| wall (inexact) | 85,060 | 117 / 82 | – | 4 / 4 | 0 |
| other | 257,976 (250,748 poison) | 69 / 67 | – | 1 / 1 | 0 |
| other (inexact) | 11,712 (246 poison) | 6 / 0 | – | 0 / 0 | 0 |

BY DOOR, the reads past ε (sampled details, at most 5 per door and class per test; every one of the 179 tests with such reads is represented):
- `euler::certify_rebased_run` (a rebased run before its move lands): 494 samples; booleans (cone apices, rulings, parallel cylinders, face-face touches).
- `euler::certify_edge_spec`: 98 samples. BLEND SURGERY SHOWS UP HERE and not as a door of its own: it reaches the kernel through Euler operations (the blend, chamfer, fillet and arms suites, the contact-edge corpus), with an import seed and a sphere carve probe.
- `attach::set_face_surfaces_certifying`: 83 samples, all offset and shell tests; the boundary stands exactly the offset distance (0.02 or 0.05) off the newly set surface: the surface is swapped before the boundary moves.
- the boolean's graft (`boolean/combine.rs`, reading its working source body): 20 samples, in two tests. Settled by a per-face probe: the vertex 0.539 from the line belongs to a face whose boundary is 0.16 to 0.56 off its own radius-0.2 cylinder (a mid-boolean working face; its cover reads that cylinder at 0.40); the partner face, on its surface, reads vertices 0.283 and cover 0.283. The finished result passes tier 3.
AT-REST doors (tier 3 `validate.rs`, `boolean/ops.rs`, `attach.rs`'s at-rest read) appear only relative-only, never past ε: the largest shortfall is 5e-10 and equals the boundary's own off-surface distance there (a boundary within tolerance of its surface).
WALLS: of 725 sampled wall reads past ε (87 core, 638 sweep), none has every wall face's boundary on its surface; each has a wall face 0.02 to 2 off its surface.
VERDICTS at inverted reads, keyed per read: `tangent_second_order` 560, all served Positive, at the graft door; `classify_dihedral` 4, served Transverse, at `certify_rebased_run`. In none did a Zero stand; each is a definite sign from below, which stands under the design's rule. No door serves a Zero, or a verdict needing the bound from above, at an inverted read.
NOT IN: the design's rule run on the sweep (running now, about 85 minutes; on core it changed 0 of 4,799 outcomes); the recount at 1e-12 and 1e-6; the symbolic lanes' cost; the interior-maximiser check of the ruled truth.

B. Two facts from the lane since round 14: (1) the 66 verdicts the design run did not make on core are fuzz inputs (five tests in two suites that draw a seed per run); "0 of 4,799" stands. (2) THE MIDDLE SITE'S DOOR FOLDS TOO: `wedge_reach.rs` there builds `above` as `reads.fold(far(own).max(below), …)`, so "about 282 M paired decisions where the two bounds agree" at the middle site says how often the folded bounds agree on a verdict and nothing about a cover. A text that cites that figure as evidence that choosing a bound changes no verdict must drop it or say what it counts; the premise stands on the arithmetic (a reading multiplied by its lever grows with the lever).

QUESTIONS, a line or two each:
(1) Is this enough to ask Ev now, the design run on the sweep stated as running? If not, name the result you are waiting for and what it would change.
(2) Does anything in (A), (B) or (C) change? The classes found off their surface are four doors' mid-construction states (a rebased run, an edge certified onto a seed surface, a surface set before its boundary moves, a boolean's working face): is "the typed missing bound from above, entered from what the door knows" still the build's answer for all four, and does any of them belong in the text?

THEN your complete, final `## For Ev` section, brought up to date with both suites (cite the recount's tables; nothing as evidence that holds by construction), and any change to `## For the orchestrator`. Final message only; write no file. At most 150 lines.

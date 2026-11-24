Excerpts of another reader's review of PR 7666 at `273f765de5` (attribution stripped): its finding on the run's definition, on the summed rate at wide wedges, on the sector sliver, and its account of the run as coded.

**M4. Definition R certifies the ruling's own in-band strip once the strip's face has any station far from the edge's line.** DEMONSTRATED at fixture level. Contract; this is the dispute with the ruled definition.
- Probe `rv_probe_an_eared_strip_in_band`: the flat strip of `a_flat_strip_inside_the_band_of_its_plate_refuses` (1e-7 wide, sine 5e-5, 5e-12 from its plate everywhere across the shared edge) reads extent 1e-7 and `Split`.
- The same strip with one more boundary vertex a metre off the edge's line, in the strip's plane, beyond the edge's end, reads extent 1.0 and **`Certified`**.
- The code is a faithful lower bound of R. R (the supremum over the whole face of the distance from the whole carrier line) is not "how far the face runs from what they share, measured across it" wherever the face is wider somewhere else along the line: an L-shaped face, an ear, a hole beside the edge.
- I could not build it as a finished body (its closing faces are below ε), so it is listed as a shape, not a body. PR body's "a face cannot be nearer than the band to the other at the far side of the narrower one" is false for this shape.

**m2. The summed rate over-states the parting at wide wedges, and the two arms then disagree.** DEMONSTRATED at fixture level. Contract and doc, claim 4.
- Probe `rv_probe_a_wide_wedge_under_the_two_arms`: a strip 0.8·Kε wide at a right angle is `Split` under the carrier arm alone and `Certified` once the plane arm runs. A strip 0.6·Kε wide at 2.6 rad is `Certified`, though no point of it is more than 0.6·Kε from the other face.
- The sum is `2·sin(θ/2)` against the carrier's `sin θ`. `parts`' doc (`injective.rs:1133`, "differ by the wedge's square over eight") holds for small wedges only; the over-read approaches a factor of 2 and touches only faces narrower than the band.
- `a_crease_under_the_carrier_and_under_the_bisecting_plane_decides_alike` tests only `sin φ` near the band at extent 1.

**N2. The sector sliver (claim 3): I agree with the lane and the orchestrator.**
- Across a 1 mm edge the faces run 10 m; the ruling's test is the far side of the narrower face, 500ε, past the band. Every crease is within ε of itself near its edge; the clause's last sentence reaches a pair only when that stretch is the whole run.
- Tier 3 agrees: `rv_probe_the_sector_slivers_neighbours` reads all nine edges `Crease(Convex)`, verdict `Embedded`, receipt `(15, 24, 0, 0, 0, 9, 13)`.
- Neighbours: 100 mm long with 500ε at the far end holds, same receipt. 10 m with 30ε holds. A sliver 1 mm across with a 10 m tip holds at 500ε and at 12ε (1.2 bands).
- The in-band neighbours (100 mm at the same angle, 5ε; 10 m at 2ε and at 12ε) are refused by the `∩` door before the question is asked, so the question's own reading of an in-band sliver body was not reached.

**N4. Claim 2, the run as coded.**
- Stations are each certified boundary edge's ends and middle and each lone vertex.
- Line: the station's distance from the whole line. Circle: from the whole circle. Ellipse and spiric: the nearest of 64 covers of the whole turn. Spline: the nearest of 64 covers of the edge's own span (the disclosed over-read). Vertex: the farthest station, kept only where more than three half-edges start.
- A pair near a vertex with no kept run has no extent; only a `Clear` reading can certify it, and otherwise it splits to its budget.

ADDENDUM 2 (PR 7096, 48b6eed279): nothing in it changes my verdict, which stays APPROVE-WITH-FIXES. Nothing in it is wrong by my reading. I read the diff only and ran nothing.
- `cert::FaceVerdict`: the extraction is equivalent in all three lanes. `read` folds the verdict and `worse`; `refusal` keeps the old order (Exceeded, then RoundingUnresolved, each with the worst bound).
  - In the CDT emit, `worst` used to be updated after the budget deviation block. Now it is updated at `read`. Nothing between those two points (826552a0e9 trimmed.rs:1650–1720) returns, continues, or reads `measure.worst`, so the move is harmless.
  - The refine gate now reads `measure.verdict.worst()`, the same value.
- The apex read once: the route reads the vertex through `topo::readback::vertex_point` and decides through `at_a_pole`, which calls `at_a_singular_point`. The lane's `is_singular` makes the same call on the same band. The comment at the second `unreachable!` states this correctly.
- `may_make` soundness at the site: this answers my A4 note.
  - The algebra is right: a passed triangle lies within δ(1+A)+a; "decided greater" puts the true distance past δ(1+A)+Kε−r; so the exclusion is sound while Kε > a + r.
  - The residual (ε_min on a body at the box's edge, left to the census) is stated honestly.
- Not checked by reading: the new ball-1.3 pins, the three D-prism pins, and the filed row's text against measurements.

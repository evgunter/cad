# REVIEW — PR #4074 (frozen head a1e90e6c)

IN PROGRESS. Established so far (executed):
- chart probes: three cones, reflex/thin wedges (down to 1e-4°), hull pinch, near-adjacent corners all mesh watertight; crossed 3-cone refuses.
- a non-crossing chart with ids (2,2,8) at one point refuses PinchWedge (valid shape refused).
- a ring touching the outer loop at a pinch refuses PinchWedge.
- mutants: first/last/other-side wedge choice and trimmed-off go red; dropping the one-id-twice refusal and the unbounded-rotation refusal goes red nowhere.
Pending: main-vs-head mesh suite, tour sweep, body volume / fine-δ probes.

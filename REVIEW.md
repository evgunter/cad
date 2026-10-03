# IN PROGRESS — JOIN-2 review, lane r2

PR #3880 at frozen head `17c254c99d` (base main `66bbdaa6bd`).

First findings (being verified against main):
- The zip's ring-first chord order refuses `RestZipUnsupported{ChordBetweenIsolatedPierces}`
  on a channel-under-plate pose where an order exists (probe `join2_r2_ring_order_through_the_zip`).
- The filed residue (like far-end touches) is reached by `join2_r2_like_far_end_touches`; it refuses
  `Join(UnpairedLooseEnds)`, no wrong body.

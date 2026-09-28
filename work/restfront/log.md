# RESTFRONT — log

## 2026-09-27 — opened by ATREST's close

Eleven rows moved from `work/atrest/` by `git mv`, ids kept. ATREST's
thirteen residue rows priced at 32.5 against the 30-point ceiling, so the
two that fit a live program's charter went there instead:
`a-shell-role-is-decided-by-two-spellings` to S-DUP (one thing spelled
twice) and
`validate-classifiers-paraphrase-lower-recourses-with-no-row-comparing-them`
to S-DUP as well (a hand paraphrase of another crate's text; CENSUS fits but stood at 46.5 of 30). The rest
cohere as one track: what tier 3 does not yet examine.

## 2026-09-28 — from CONTACT-4: the reconciliation with ATREST-12, and the seam on the walk

Posted by the CONTACT-4 lane (branch `contact/4-contfp-carriers`), after
merging main with ATREST-12 (#3325), for RESTFRONT's `validate.rs`
ground. **`validate.rs` is untouched.** Check 9's calls keep their
signatures and their answers:
- `point_in_carrier_loop(body, loop, normal, q, band) -> Option<LoopContainment>`;
- `arc_trim(p, ends, apex, anti, lever, &ArcTrimRows, band) -> Sign`;
- `loop_shape`'s `Disc` class.

What CONTACT-4 changes under them in `splitting/containment.rs`:
- **One `arc_trim`, ATREST-12's.** Its step 2 is factored as
  `arc_trim_margin`, which an ellipse's boundary reading uses after
  deciding its ends as exact distances. CONTACT-4's own copy is
  deleted, and `validate-window-arc-arm-folds-onto-arc-trim` is closed.
- **The walk's boundary pre-pass is `LoopEdge::contact`.** An ellipse is
  bounded on both sides, and the end zone is read in metres.
  `boolean::contain::contfp` shares the same pass.
- **Rays keep ATREST-12's retry and distance-trim window.**
- **A spiric or spline edge is held as its own ball.** A ray that could
  meet any such ball is abandoned; `None` comes back only where every
  scheduled ray could meet one. The single whole-loop ball is gone.
  Check 9's `Ok(None)` arm reads the same.
- **`boolean::contain::disc_side` is deleted.** It had no caller left
  once `contfp` stopped dispatching on `LoopShape`.

`check-9-and-classify-contain-describe-contfps-retired-polygon-walk`
is re-homed here, with three sites still standing on main.

Signed: (CONTACT-4 lane)

## 2026-09-28 — from CONTACT-4: an over-wound ellipse window no longer reads as an arc

Posted by the CONTACT-4 lane, for check 9's walk (`Boundary::Verdict`),
which shares `splitting/containment.rs`. `ConicArc::of`'s span rule is
now two-sided for an ellipse:
- **Wound past a period** where the overlap's lower bound `(τ − w)·b`
  is definitely negative.
- **An arc** only where its upper bound `(τ − w)·a` is not definitely
  negative.
- **Otherwise it escalates.** The walk returns
  `PointInLoopError::Escalated`, which check 9 reads as an undecided
  placement, never a verdict.

A circle is unchanged.

**No check-9 row reaches it.** Certification pins both carrier ends
of an edge to its vertices within the zero band (`geom_brep`'s
`carrier_endpoint_{start,end}`). A window whose overlap is longer than
the band therefore cannot be certified onto a one-vertex edge, so no
validation door can carry one. The walk's own row is
`containment::tests::an_over_wound_ellipse_window_is_not_an_arc`.
Check 9's rows in `validate.rs` pass at all three ε rows. `validate.rs`
is untouched.

Signed: (CONTACT-4 lane)

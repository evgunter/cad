# CONTACT-11: the torus chart-box check compares areas

**Binds one implementer lane.** Deleted at merge;
`work/contact/CONTACT-11.md` survives. Read
`docs/prompts/implementer-discipline.md` in full first.

Branch `contact/11-torus-chart-l`, from `main`. Read the row in full:
`work/contact/torus-chart-box-check-passes-an-l-shaped-face`.

## The defect, as filed

`torus_chart_windows` (`crates/topo/src/boolean/solid_contain.rs`,
~:1901) decides `bool_torus_chart_box` on `variation − 2·span` per
chart channel. Its docs say that rejects an L, but an L is monotone in
both channels, so it passes. The window then over-covers the notch, and
the torus trim serves the L its bounding box. The cone took the fix
already: `chord_join::chart_box_defect`, an area comparison, decided as
`bool_cone_chart_box`.

The row is arithmetic, not a measurement.

## The work, in order

1. **Make it fail first.** Build a torus face whose chart image is an L,
   using Euler ops on a torus sheet along parallels and meridians, the
   way the cone's row `section_cert_rows.rs`
   `an_l_shaped_cone_face_refuses_rather_than_trim_by_its_hull` does.
   Drive it through the public door the cone row uses, plus one boolean
   or containment query whose answer depends on the notch. Show a wrong
   answer at base against ground truth: a point in the notch, or a
   volume. If the over-cover is caught somewhere downstream and no wrong
   answer reaches the user, say what catches it; the fix still lands, as
   the check's docs are false.
2. **The fix.** Share the cone's check. Use `chart_box_defect`, or the
   one decision it feeds, rather than respelling it. Decide it with its
   own predicate name, and pin a U and an L as refusals and a rectangle
   as accepted.
3. **Sweep the class.** Any other chart-box or hull check that decides
   "the face is its box" by variation or by extents alone: fix it or
   file it.

Review tier: single full review.

## Discipline

- Work in your own clone, with `CARGO_TARGET_DIR=/home/user/contact-11-target`,
  `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_DEV_DEBUG=line-tables-only`.
- Wrap every cargo command in `with-build-slot.sh`, and run `df` first.
- No process listings, and kill only PIDs you recorded.
- Never run git in `/home/user/cad`.
- Push the branch only; no PR.

Before hand-back, run all of the following:
- `topo` + `sweep` at three eps;
- ALL of `editor-core`;
- `test-utils`;
- the Python suite (maturin wheel);
- clippy, `cargo fmt --all --check`, gates, lint.

Write `docs/CONTACT-11-PR.md`, the PR body, and hand back.

## Territory

`crates/topo/src/chord_join.rs` is REACH's and TANG's territory. Share
`chart_box_defect` by calling it. If it has to move or change, leave a
one-line seam note in `work/reach/log.md` and in `work/tang/log.md`.

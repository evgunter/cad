# CONTACT-13: the census reads every declared meeting

**Binds one implementer lane.** Deleted at merge;
`work/contact/CONTACT-13.md` survives. Read
`docs/prompts/implementer-discipline.md` in full first.

Branch `contact/13-meeting-ledger`, from `main`.

Read first:
- The row, `work/contact/declared-only-meetings-clear-at-the-census-gate-unread`,
  in full, including both built wrong clears and Ev's ruling.
- PR 3422, whose body carries both designers' reports: the
  recommendation this unit builds, in their words.
- The updated exclusion-step paragraph in `crates/topo/README.md`, as
  merged by PR 3422.
- CONTACT-7's closing note in `work/contact/CONTACT-7.md`. The touch
  analysis this unit extends is CONTACT-7's metric one: `mod metric`,
  `Distance`, and the visibility pieces.

## The ruling, as the final state

1. **A declaration licenses a coincidence, never a side.** Every meeting
   between two solids is read by the one touch analysis, and the pair
   clears only when each meeting reads **Rest**. That holds whether a
   sweep found the meeting undeclared, a record names it, or a declared
   face pair backs it. A record licenses only what its own certificate
   checked.
2. **One meeting ledger.** Each sweep pushes every coincidence it
   decides as a meeting, with its backing: undeclared, a direct record,
   or face-pair backed.
   - `UndeclaredContact` is raised for the unbacked meetings only, as
     today.
   - Arm 2 reads every ledger meeting of a pair through
     `TouchSite::verdict`.
   - These go: `records_on_their_word`, arm 2's re-derivation of sites
     from `Declared`, the `recorded`/`meets_found`/`declared_only` split,
     and the blanket `DeclaredFacePair` refusal.
   - An overlap reported once per pair (`EdgeFaceOverlap`,
     `EdgeEdgeOverlap`) is read at every one of its cells. Check what
     CONTACT-12 does to the cells; see Interaction.
3. **C1: curved star faces are read through their reach box.**
   - A star face the analysis cannot trace (a curved carrier, or a
     planar face with a curved boundary, such as the boss's cap) enters
     as the corners of its certified reach box (`face_reach`). Each
     corner's signed distance from the candidate plane passes through
     `Distance`.
   - A box within its side certifies the piece. A box that crosses the
     plane makes the star certify-or-refuse: it never decides Crossing,
     and "no candidate held, a coarse piece present" maps to
     `TouchUnreadable`.
   - Arm 1's skip of a pair that a v-on-f or face record names ("the
     confirm pass's pair") is replaced by this reading. Check also the
     curve-record skip one designer flagged: a jet schedule along one
     witness curve, with two curved faces possibly crossing elsewhere.
     Probe it. Fix it if it is the same class; otherwise file it.
   - If the box reading does not certify the M9-2 boss in practice
     (`m5_pr9_boss_union::a_touching_curved_assembly_validates_declared_and_refuses_undeclared`),
     stop and report. The fallback is C2 (refuse, and re-baseline that
     row), and it is Ev's call, not the lane's.

## Rows

- **The three built wrong clears, now refused.** Their poses are in the
  row:
  - the I-profile through a slab, declared as four patches;
  - the same, declared as sixteen v-on-f;
  - the log dipping into a wall, declared with four v-on-f.
- **Their true-rest controls, certifying:**
  - the flat-bottomed log;
  - an I-profile seated without passing through.
- **Every ratified acceptance row keeps its answer:**
  - the nineteen declared planar seats (among them
    `m9_c1_rest_face_rung::the_flush_seat_certifies_in_both_argument_orders`
    and `mate9_crossing_rung::the_declared_crossing_seat_certifies_both_ways`);
  - the M9-2 boss.

  Name any row that moves, and why.
- **A saddle star at a declared site with no face on the plane** now
  refuses `TouchUnanalysed` where it cleared before. Pin that, and name
  any ratified row it touches.

## Interaction

CONTACT-12 (`contact/12-ef-crossing-cuts`) is changing
`ef_overlap_cells` and the grandfathered `ef_bound_backed` in the same
file, in parallel.
- Keep to arm 1's skip, arm 2, the ledger, and the touch analysis's
  star.
- Merge `origin/main` often.
- When CONTACT-12 lands (it lands first), merge it and reconcile the
  overlap cells with the ledger.

## Discipline

- Work in your own clone, with `CARGO_TARGET_DIR=/home/user/contact-13-target`,
  `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_DEV_DEBUG=line-tables-only`.
- Wrap every cargo command in `with-build-slot.sh`, one job at a time
  (the slot is shared), and run `df` first.
- No process listings, and kill only PIDs you recorded.
- Never run git in `/home/user/cad`.
- Commit and push often.
- Push the branch only; no PR.

Before hand-back, run all of the following:
- `topo` + `sweep` at three eps;
- ALL of `editor-core`;
- `test-utils`;
- step-import and verbs;
- the Python suite (maturin wheel);
- clippy, `cargo fmt --all --check`, gates, lint.

Write `docs/CONTACT-13-PR.md`, the PR body.

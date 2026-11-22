(Reviewer, PR 7096) ADDENDUM on 48b6eed279, by reading `git diff 826552a0e9 48b6eed279`. Nothing built or run on this commit.

**Verdict unchanged: APPROVE-WITH-FIXES. I found nothing wrong in it.**

- **`cert::FaceVerdict` is a faithful extraction on all three lanes.** `read` does exactly what each site did: the verdict, the exceeded flag, the largest allowance, then `cert::worse` on the bound. `refusal` returns `CertificateExceeded` before `RoundingUnresolved` with the same fields, as each site did.
  - Rectangle lane (curved.rs ~:438-520) and `walk_face`: line-for-line replacements.
  - CDT emit: the one place the order moved. `worst` used to be updated about 80 lines after the verdict, past the deviation-sampling block. I read the old text between them at 826552a0e9: it has no `continue`, `break` or return and does not read `measure.worst`, so updating earlier changes nothing. The `worst().is_nan()` gate on refinement and `worst_cert` read the same value.
  - So "no byte moved" is what the code says. I did not re-take digests.
- **The apex read once.** `walkable` now reads the vertex through `topo::readback::vertex_point` and decides through `chart_table::at_a_pole` at the run's band. That answers the reservation in my last addendum as far as reading goes. I did not trace `singular_vertices` to the same function at this commit, and I did not run the 81-gap row.
- **`may_make`'s premise** is now stated at the site with its residual. That closes my MINOR on it.
- **The second bore (R 1.3) pinned per cell** closes my MINOR that the over-cut was unpinned off the PR's own thickness, for that body. R 3 stays unpinned; I accept that.
- **The two stale citations** I named are edited. The unit file's "Not measured" section is not in this diff, so "nobody has built it" still stands unless a tracker commit takes it.

Still owed from my final, none of it in this commit's reach: a green hosted run on the final head, and the description's account of the D-prism cells (the unsplit body meshes on both trees).

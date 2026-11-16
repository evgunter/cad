---
id: the-thread-count-goldens-sym-validate-lines-and-arc-lofts-volume-are-pinned-unexplained
kind: issue
title: The thread-count golden's two Sym validate_geometric lines (every eps) and arc_loft_1e9eps' volume (1e-12, one ulp) moved unexplained and are pinned at the serial reading
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Filed by GREEN's `green/goldens` lane (PR 6956). It takes over what is still open of `work/green/main-red-the-four-thread-walk-misses-tilted-cut-uppers-serial-golden.md`, which is closed: `tilted_cut_upper` was judged and re-blessed there.

`crates/sweep/tests/thread-count-digest/eps-*.txt` is read by `mass_props_are_thread_count_invariant`'s one-thread and four-thread rows. These lines moved on main with no merge named and nobody saying the readings are right. They are pinned at what the serial walk reads, and the file's `expected` doc names this row:

- `sym_arc_loft validate_geometric`, every ε row. At 1e-9: `decisions=639 sym=440 num=199 chk=136/141 frozen=643 shapes=639`, where the table held `610 / 432 / 178 / 99/104 / 462 / 610`. At 1e-6 the same reading (the table held `611 / 432 / 179 / 100/105 / 462`); at 1e-12 `chk=134/141 frozen=642` (the table held `97/104`, `462`).
- `sym_thin_strip validate_geometric`, every ε row. At 1e-9: `decisions=643 sym=392 num=251 chk=128/138 frozen=536 shapes=643`, where the table held `614 / 400 / 214 / 92/102 / 348 / 614`; at 1e-12 `chk=129/139`.
- `arc_loft_1e9eps` at 1e-12: `v=3e43a1a4c860cf44`, where the table held `…cf43`. One ulp of a certified centre; the pads are unchanged.

`disp=0` on every line, and every receipt clears `dispute_coverage`, so no dispute is lost. What is unexplained is which decisions were added (29 more on each body) and why `frozen` rose.

What is known of when: on 2026-11-10 at main `57161586` the two `Sym` lines read `611 / 179 / 100/105` and `615 / 215` at 1e-9, one decision over the table. PR 6820 (`3d3fe2311`) and PR 6869 (`3b59315dc`) both moved `sym11_exact_channel_rows`' counts on 2026-11-12 and are candidates, unmeasured here.

**Owed.** Bisect first-parent on the two `Sym` lines and on `arc_loft_1e9eps`' volume, file each on the program whose merge it is, and say whether the new counts are right. The doc paragraphs that explained the old counts (rule H's `chk=97/104` at 1e-12; PR 5889's `frozen` 4 -> 462 and 60 -> 348) were removed from `expected` with the pin, because the table no longer reads those numbers; whoever accounts for the move restores what still holds.

Reproduce: `cargo nextest run -p sweep -E 'test(the_walk_matches_the_serial_golden_at_one_thread)'`, and with `CAD_TOLERANCE_EPS=1e-6` and `1e-12`.

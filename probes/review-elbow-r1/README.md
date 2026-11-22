# Review probes, PR 7432 (R1)

`mutate.py crates/topo/src/boolean/mod.rs` makes `route_rim`'s arms
selectable at run time by `R1_MUT` (`T_GLUE`, `SEAM_UNROUTED`,
`SAME_GLUE`, `NO_ROUTE`); unset, the code is the PR head's. Not applied
on this branch. Run the rim pins and the probes under each:

    R1_MUT=<m> cargo nextest run -p sweep -p editor-core --test all \
      -E 'test(review_elbow_r1) | test(/a_rod_unions_with_what_ends_on_its_rim/) | test(/mate7a_torus_rest::/) | test(/a_pipe_unions_with_its_elbow::/)'

Only `SEAM_UNROUTED` turns a pin red (`a_rod_unions_with_a_half_ball_on_its_cap`).

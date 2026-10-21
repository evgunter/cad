#!/bin/bash
for t in head base; do for e in 1e-9 1e-12; do cd /home/user/wt-$t && CAD_TOLERANCE_EPS=$e CARGO_TARGET_DIR=/home/user/tgt-$t cargo nextest run --release --profile ci --ignore-default-filter -p topo -p sweep -p editor-core --no-capture -E 'test(=a_vertex_read_again_classes_every_edge::every_edge_a_vertex_read_again_reads_is_classed_against_the_germ)' 2>&1 | grep -E "^a (fin|dented)|PASS|FAIL" | sed -E 's/\[[^]]*\]//' > /home/user/logs/tally2-$t-$e.txt; done; done
echo done > /home/user/logs/tally2.done

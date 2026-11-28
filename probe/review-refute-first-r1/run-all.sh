#!/bin/bash
P="-p topo -p sweep -p profile -p geom-brep -p editor-core"
K='package(topo)|package(sweep)|package(profile)|package(geom-brep)'
for t in head base; do
 ./run-suite.sh $t kfast - -j 3 $P -E "$K"
 ./run-suite.sh $t kslow - -j 3 $P --ignore-default-filter -E "($K) & test(/an_aimed_ray|boxed_revolved|side_reader|volume_premises|sym11_far_placement|radial_reader|a_boxed_rectangle_validates|computed_values_enter/)"
 ./run-suite.sh $t e6 1e-6 -j 3 $P -E 'package(editor-core)'
 ./run-suite.sh $t e12 1e-12 -j 3 $P -E 'package(editor-core)'
 ./run-suite.sh $t eslow - -j 2 $P --ignore-default-filter -E 'package(editor-core) & test(/contradiction_corpus|sym_9_retry|m10_9_pins_interval|sym_drive_memo|m10_sym_profile/)'
done
echo ALLDONE > all.done

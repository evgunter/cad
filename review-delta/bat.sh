# usage: bat.sh BIN OUTDIR WORKDIR
B=$1; O=$2; cd $3/crates/sweep; mkdir -p $O
run(){ name=$1; shift; env "$@" $B --ignored --exact --nocapture --test-threads 1 $T > $O/$name.log 2>&1; echo "EXIT $?" >> $O/$name.log; }
T=join3_review_r1::j3r1_mixed_pockets run r1mix7171 J3_SEED=7171 J3_N=100
T=join3_review_r1::j3r1_mixed_pockets run r1mix31415 J3_SEED=31415 J3_N=100
T=join3_review_r1::j3r1_tilted_through run r1tilt7171 J3_SEED=7171 J3_N=100
T=join3_r2_probes::j3r2_pocket_battery run r2pocket X=1
T=join3_r2_probes::j3r2_tilted_battery run r2tilt X=1
T=join3_r2_probes::j3r2_rand run r2rand R2_SEED=5150 R2_N=400
echo ALLDONE > $O/done

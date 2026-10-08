#!/bin/bash
# evalmut.sh NAME : unit rows + in-tree fuzz + review fuzz families for /home/user/bin-NAME
n=$1; B=/home/user/bin-$n
fails=$($B boolean::sectors::tests boolean::vtxfac::tests boolean::sectors::cone_fuzz --test-threads 2 2>&1 | grep -E "^test .* FAILED$|^test result" | sed 's/^test boolean:://; s/ \.\.\. FAILED//' | tr '\n' ' ')
fz=""
for s in crisp crisp3 g1 g2 bnd; do
  r=$(./run.sh $B $n $s 1e-9 | grep -o "'cs_WRONG': [0-9]*" | grep -o "[0-9]*$"); fz="$fz $s:${r:-0}"
done
echo "$n | $fails | review-fuzz cs WRONG:$fz"

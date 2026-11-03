import sys
f='/home/user/wt-head/crates/topo/src/bound_side.rs'
M={
'M1_drop': ("        self.witnessed(leading, arm)\n", "        let _ = arm;\n        Ok(leading.map_or(Descent::Unresolved(Unresolved::Gap), Descent::Leaves))\n"),
'M2_flip': ("            Sign::Positive => Side::Above,\n            Sign::Negative => Side::Below,\n        };\n        match leading {", "            Sign::Positive => Side::Below,\n            Sign::Negative => Side::Above,\n        };\n        match leading {"),
'M3_zero': (".witness(self.dir, arm)", ".witness(self.dir, T::zero())"),
'M6_gap_leaves': ("            Err(Refusal::InBand(diag)) if in_one_gap(diag) => {\n                return Ok(Descent::Unresolved(Unresolved::Gap));\n            }", "            Err(Refusal::InBand(diag)) if in_one_gap(diag) => {\n                return Ok(leading.map_or(Descent::Unresolved(Unresolved::Gap), Descent::Leaves));\n            }"),
'M7_zero_leaves': ("            Sign::Zero => return Ok(Descent::Unresolved(Unresolved::Returned)),", "            Sign::Zero => return Ok(leading.map_or(Descent::Unresolved(Unresolved::Returned), Descent::Leaves)),"),
}
name=sys.argv[1]
s=open(f).read()
if name=='restore':
    open(f,'w').write(open(f+'.orig').read()); sys.exit()
old,new=M[name]
assert s.count(old)==1, (name, s.count(old))
open(f,'w').write(s.replace(old,new))

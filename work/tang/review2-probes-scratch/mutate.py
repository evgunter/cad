import sys
f='/home/user/cad/crates/topo/src/boolean/vtxfac.rs'
s=open(f).read()
m=sys.argv[1]
R={
 'M0_baseline':[],
 'M1_parity_too_few':[("match (on, held % 2, base)","match (on, held.saturating_sub(1) % 2, base)")],
 'M2_nested_as_apart':[("match (on, held % 2, base)","match (on, held.min(1), base)")],
 'M3_ignore_on_depth2':[("        (1, _, _) => Some(SideCode::On),","        (1, 0, _) => Some(SideCode::On),\n        (1, _, SideCode::Out) => Some(SideCode::In),\n        (1, _, SideCode::In) => Some(SideCode::Out),\n        (1, _, _) => Some(SideCode::On),")],
 'M4_pair_intersection':[(".filter_map(|&(he, _)| Some((he, layered(SideCode::Out, he, pairs.iter())?)))",
   ".filter_map(|&(he, _)| { let has = |c| pairs.iter().all(|p| p.wedge.rows.iter().any(|&(h, k)| h == he && k == c)); let any_on = pairs.iter().any(|p| p.wedge.rows.iter().any(|&(h, k)| h == he && k == SideCode::On)); Some((he, if has(SideCode::In) { SideCode::In } else if any_on { SideCode::On } else { SideCode::Out })) })")],
 'M5_ignore_met':[("(SideCode::In, true) | (SideCode::Out, false) => held += 1,","(SideCode::In, _) => held += 1,")],
 'M6_first_partner_only':[("let side = pairs.iter().filter(|p| p.side == Some(own));","let side = pairs.iter().filter(|p| p.side == Some(own)).take(1);")],
 'M7_ignore_on':[("(SideCode::On, _) => on += 1,","(SideCode::On, _) => {}")],
 'M8_either_side':[("let side = pairs.iter().filter(|p| p.side == Some(own));","let side = pairs.iter().filter(|p| p.side.is_some() || own == own);")],
 'M9_pairs_declined':[("            if pairs\n                .iter()\n                .all(","            if false && pairs\n                .iter()\n                .all(")],
}
for a,b in R[m]:
    assert s.count(a)==1,(m,a)
    s=s.replace(a,b)
open(f,'w').write(s)

import sys
src=open(sys.argv[1]).read()
m=sys.argv[2]
R={
 'M1_any_relation': ("p.a == fa && p.b == fb && p.relation == CarrierRelation::SameOpposite","p.a == fa && p.b == fb"),
 'M2_all_to_any': ("&& reaching.all(|r| match x_is {","&& reaching.any(|r| match x_is {"),
 'M3_no_peek': ("if reaching.peek().is_some()\n                            && reaching.all(","if true\n                            && reaching.all("),
 'M4_swap_operands': ("Operand::A => touches_only(settled, r.face, yf),\n                                Operand::B => touches_only(settled, yf, r.face),","Operand::A => touches_only(settled, yf, r.face),\n                                Operand::B => touches_only(settled, r.face, yf),"),
 'M5_fa_only': ("p.a == fa && p.b == fb && p.relation","p.a == fa && p.relation"),
 'M6_no_bbox': ("r.key == fd.surface && r.bbox.overlaps(&y_row.bbox)","r.key == fd.surface"),
 'M7_sphere_skip_off': ("if reaching.peek().is_some()\n","if false && reaching.peek().is_some()\n"),
 'M8_section_skip_off': ("|fa, fb| touches_only(settled, fa, fb),","|_, _| false,"),
}
a,b=R[m]
assert src.count(a)==1,(m,src.count(a))
open(sys.argv[1],'w').write(src.replace(a,b))

p='/home/user/cad-mut/crates/topo/src/boolean/circle_roots.rs'
s=open(p).read()
old="""        (T::zero() - self.cos_part).select_le_zero(
            (base - t).reduce_periodic_centred(tau),
            (opposite - t).reduce_periodic_centred(tau),
        )"""
assert s.count(old)==1
s=s.replace(old,"""        ((T::zero() - self.cos_part).select_le_zero(base, opposite) - t).reduce_periodic_centred(tau)""")
open(p,'w').write(s)

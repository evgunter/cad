import itertools
import os; exec(open(os.path.join(os.path.dirname(__file__),'held_cut_case_table.py')).read().split('diff={}')[0])
cuts=[(e,r) for e in range(3) for r in range(4)]
ins=lambda o: o!=False
d=0;t=0
for lo,hi,ilo,ihi in itertools.product(cuts,repeat=4):
    if bfix(lo,hi)!=True or bfix(ilo,ihi)!=True: continue
    if bfix(lo,ilo) is None and bfix(ihi,hi) is None: continue
    t+=1
    old=ins(bfix(lo,ilo)) and ins(bfix(ihi,hi))
    new=all(False not in on_arc(bfix,lo,hi,x) for x in (ilo,ihi))
    if old!=new: d+=1; print(lo,hi,ilo,ihi,old,new)
print(t,d)

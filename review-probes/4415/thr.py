import sys; sys.path.insert(0,'probe')
from lump import *
a,b=3.04e-8,3.05e-8
f=lambda d: (lambda r: r[0]/r[1])(lump([(2,0),(4,0),(4,2),(2,1)],1.0,[2.0,1.0,1.0],pose([2.0,1.0,0.0],3,d)))-mp.mpf('1e-8')
for i in range(45):
    c=(a+b)/2
    if c in (a,b): break
    if f(c)<0: a=c
    else: b=c
print(repr(a),repr(b), mp.nstr(f(a),5), mp.nstr(f(b),5))

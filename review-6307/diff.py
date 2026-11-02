import sys,collections
def load(p):
    d={}
    for l in open(p):
        k=l.rsplit('|',1); d[k[0]]=k[1].strip()
    return d
b=load('/home/user/dump-base.txt'); h=load('/home/user/dump-head.txt')
moves=collections.Counter(); ex=collections.defaultdict(list)
wrong_b=wrong_h=0
for k in b:
    vb,vh=b[k],h.get(k)
    truth=k.split('|')[4].split()[1]
    def definite(v): return v in ('Some(true)','Some(false)')
    if definite(vb) and vb!=f'Some({truth})': wrong_b+=1
    if definite(vh) and vh!=f'Some({truth})':
        wrong_h+=1; ex['WRONGHEAD'].append(k+' '+vh)
    if vb!=vh:
        key=(k.split('|')[0],vb,vh); moves[key]+=1
        if len(ex[key])<6: ex[key].append(k)
print('wrong base',wrong_b,'wrong head',wrong_h)
for k,v in sorted(moves.items(),key=lambda x:-x[1]): print(v,k)
for k,v in ex.items():
    print('==',k)
    for x in v[:6]: print('  ',x)

from lib import *
from i2 import pt, arrowhead, arrow, poly_reg
from i3 import star4
from i4 import G,B,face
from i5 import arc_arrow
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
@icon
def edit():
    ic=Icon('edit','Edit')
    T=(11,53); d=(0.7071,-0.7071); n=(0.7071,0.7071); E=(52,12)
    def at(base,a,b): return (base[0]+d[0]*a+n[0]*b, base[1]+d[1]*a+n[1]*b)
    S=(T[0]+d[0]*11,T[1]+d[1]*11)
    body=poly([at(T,11,5.5),at(E,0,5.5),at(E,0,-5.5),at(T,11,-5.5)])
    ic.fill(body,P1)
    ic.fill(poly([at(E,-7,5.5),at(E,0,5.5),at(E,0,-5.5),at(E,-7,-5.5)]),A2)
    ic.fill(poly([T,at(T,11,5.5),at(T,11,-5.5)]),A1)
    ic.stroke(poly([T,at(T,11,5.5),at(E,0,5.5),at(E,0,-5.5),at(T,11,-5.5)]),INK,OW)
    ic.stroke(poly([at(T,11,5.5),at(T,11,-5.5)],False),INK,OW)
    ic.stroke(poly([at(E,-7,5.5),at(E,-7,-5.5)],False),INK,OW)
    return ic
@icon
def rename():
    ic=Icon('rename','Rename')
    fr=rrect(6,20,58,46,5); ic.fill(fr,P1); ic.stroke(fr,INK,OW)
    ic.stroke('M15 33L33 33',P3,AW); ic.stroke('M43 14L43 52M38 14L48 14M38 52L48 52',A3,AW*0.8)
    return ic
@icon
def list_():
    ic=Icon('list','List')
    for y in (16,32,48):
        ic.dot(12,y,4,A3); ic.stroke(poly([(23,y),(54,y)],False),P3,AW)
    return ic
@icon
def inspect():
    ic=Icon('inspect','Inspect')
    ic.fill(ell(27,27,17),A1)
    ic.stroke('M40 40L55 55',INK,NW)
    ic.stroke(ell(27,27,17),INK,AW*0.8)
    ic.stroke('M19 24A9 9 0 0 1 26 18',P1,AW*0.8) if False else ic.stroke('M18 25A10 10 0 0 1 25 18',A3,AW*0.8)
    return ic
@icon
def copy():
    ic=Icon('copy','Copy')
    a=rrect(8,8,38,44,4); b=rrect(24,20,56,58,4)
    ic.fill(a,P2); ic.stroke(a,INK,OW); ic.fill(b,P1); ic.stroke(b,INK,OW)
    ic.stroke('M32 32L48 32M32 40L48 40M32 48L42 48',A3,AW*0.8)
    return ic
@icon
def paste():
    ic=Icon('paste','Paste')
    b=rrect(10,12,54,58,5); ic.fill(b,P2); ic.stroke(b,INK,OW)
    ic.stroke(rrect(24,6,40,16,3),INK,OW,fill=P3)
    s=rrect(20,24,50,54,3); ic.fill(s,P1); ic.stroke(s,INK,OW)
    ic.stroke('M26 33L44 33M26 41L44 41M26 49L36 49',A3,AW*0.8)
    return ic
@icon
def activate():
    ic=Icon('activate','Activate')
    ic.fill(ell(32,32,22),A1); ic.stroke(ell(32,32,22),A3,AW); ic.stroke('M21 33L29 41L44 24',A3,AW)
    return ic
@icon
def play():
    ic=Icon('play','Play')
    ic.stroke(poly([(20,10),(54,32),(20,54)]),INK,OW,fill=A2); return ic
@icon
def explode():
    ic=Icon('explode','Explode'); I=Iso().fit([(0,0,0),(36,36,0),(0,0,44),(36,36,44),(0,36,0),(36,0,0)],56,58)
    ic.stroke(poly([I.p(18,18,-6),I.p(18,18,50)],False),GUIDE,OW) if False else None
    I.box(ic,0,0,0,36,36,6,G); I.box(ic,0,0,19,36,36,25,B); I.box(ic,0,0,38,36,36,44,G)
    return ic
@icon
def library():
    ic=Icon('library','Library')
    for (x0,x1,y0,c) in ((7,20,22,P2),(22,35,10,A2),(37,50,18,P1)):
        d=poly([(x0,y0),(x1,y0),(x1,56),(x0,56)]); ic.fill(d,c); ic.stroke(d,INK,OW)
        ic.stroke(poly([(x0,y0+8),(x1,y0+8)],False),INK,OW) if False else None
        ic.stroke(poly([(x0+3.5,y0+7),(x1-3.5,y0+7)],False),INK,OW)
    t=poly([(51,24),(61,22),(66,56) if False else (60,56),(50,56)]) if False else None
    return ic
@icon
def restore():
    ic=Icon('restore','Restore')
    ic.fill(ell(32,33,14),P1); ic.stroke(ell(32,33,14),INK,OW)
    ic.stroke('M32 25L32 33L38 37',A3,AW*0.8)
    arc_arrow(ic,32,33,23,30,-200,A3,AW*0.9,8,9)
    return ic
@icon
def checkpoint():
    ic=Icon('checkpoint','Checkpoint')
    ic.stroke('M16 8L16 58',INK,AW)
    d=poly([(16,12),(54,12),(46,23),(54,34),(16,34)]); ic.fill(d,A2); ic.stroke(d,INK,OW)
    return ic
@icon
def suppress():
    ic=Icon('suppress','Suppress')
    d='M4 32C16 14 48 14 60 32C48 50 16 50 4 32Z'
    ic.fill(d,P1); ic.stroke(d,GUIDE,OW); ic.fill(ell(32,32,9),P2); ic.stroke(ell(32,32,9),GUIDE,OW)
    ic.stroke('M10 54L54 10',INK,AW)
    return ic
@icon
def rollback():
    ic=Icon('rollback','Rollback')
    ic.stroke('M8 46L36 46',A3,AW); ic.stroke('M36 46L56 46',P3,AW)
    ic.stroke('M36 8L36 40',A3,AW); ic.fill(poly([(36,52),(28,40),(44,40)]),A3) if False else None
    ic.fill(poly([(36,56),(27,44),(45,44)]),A3) if False else None
    ic.dot(36,46,5.5,A3)
    ic.fill(poly([(36,8),(52,13),(36,20)]),A2); ic.stroke(poly([(36,8),(52,13),(36,20)]),INK,OW*0.8)
    return ic
@icon
def reorder():
    ic=Icon('reorder','Reorder')
    for y in (14,32,50): ic.stroke(poly([(28,y),(56,y)],False),P3,AW)
    ic.stroke('M12 22L12 42',A3,AW); ic.fill(arrowhead(12,8,0,-1,9,10),A3) if False else None
    ic.fill(arrowhead(12,10,0,-1,9,11),A3); ic.fill(arrowhead(12,54,0,1,9,11),A3)
    return ic
@icon
def graph():
    ic=Icon('graph','Dependency graph')
    ic.stroke('M15 17L49 32M15 47L49 32',INK,OW)
    ic.ring(14,16,7,P3,AW,P1); ic.ring(14,48,7,P3,AW,P1); ic.ring(50,32,7.5,A3,AW,A1)
    return ic
def star5(cx,cy,R,r):
    ps=[]
    for i in range(10):
        a=math.radians(-90+36*i); rr=R if i%2==0 else r; ps.append((cx+rr*math.cos(a),cy+rr*math.sin(a)))
    return poly(ps)
@icon
def star():
    ic=Icon('star','Favorite'); ic.stroke(star5(32,34,25,11),INK,OW,fill=A2); return ic
@icon
def render_image():
    ic=Icon('render_image','Render image')
    fr=rrect(6,12,58,52,5); ic.fill(fr,P1)
    ic.fill('M7 46L7 44L22 28L33 40L40 33L57 49L57 47A5 5 0 0 1 52 52L12 52A5 5 0 0 1 7 46Z',A2)
    ic.stroke(fr,INK,OW); ic.fill(star4(46,24,10,0.12),A3)
    return ic
@icon
def render_scene():
    ic=Icon('render_scene','Render scene'); I=Iso(1,28,42); I.box(ic,-13,-13,-13,13,13,13,G)
    ic.fill(star4(48,16,12,0.12),A3); return ic

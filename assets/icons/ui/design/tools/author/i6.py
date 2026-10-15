from lib import *
from i2 import pt, arrowhead, arrow, poly_reg, dashes
from i4 import G,B,face,cyl3,bez,surf_patch
from i5 import arc_arrow, triad
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
def smooth(t): return t*t*(3-2*t)
@icon
def boundary():
    ic=Icon('boundary','Boundary surface'); I=Iso().fit([(0,0,0),(40,0,0),(40,40,0),(0,40,0),(0,0,12),(40,40,12)],58,52)
    surf_patch(ic,I,n=2,fillc=A1,line=A2,lw=OW*0.8,out=A3,ow=AW); return ic
@icon
def blend_surface():
    ic=Icon('blend_surface','Blend surface'); I=Iso().fit([(0,0,0),(52,0,0),(52,34,0),(0,34,0),(0,0,8),(52,34,8)],58,48)
    def tile(x0,x1,z):
        face(ic,I,[(x0,0,z),(x1,0,z),(x1,34,z),(x0,34,z)],P1)
    tile(0,16,0)
    face(ic,I,[(36,0,9),(52,0,9),(52,34,9),(36,34,9)],P1)
    xs=[16+20*i/10 for i in range(11)]
    top=[I.p(x,0,9*smooth((x-16)/20)) for x in xs]; bot=[I.p(x,34,9*smooth((x-16)/20)) for x in xs]
    band=poly(top+bot[::-1]); ic.fill(band,A1); ic.stroke(band,A3,AW*0.75)
    return ic
@icon
def edge_continuity():
    ic=Icon('edge_continuity','Continuity'); I=Iso().fit([(0,0,0),(52,0,0),(52,40,0),(0,40,0)],58,52)
    face(ic,I,[(0,0,0),(26,0,0),(26,40,0),(0,40,0)],P1); face(ic,I,[(26,0,0),(52,0,0),(52,40,0),(26,40,0)],P2)
    ic.stroke(poly([I.p(26,0,0),I.p(26,40,0)],False),A3,AW)
    for i,y in enumerate((5,13,21,29,37)):
        L=[5,9,12,9,5][i]
        a=I.p(26,y,0); b=I.p(26-L,y,0); c=I.p(26+L,y,0)
        ic.stroke(poly([b,c],False),A3,OW)
    return ic
@icon
def perforate():
    ic=Icon('perforate','Perforate'); I=Iso().fit([(0,0,0),(52,0,0),(52,40,0),(0,40,0),(0,0,5)],58,50)
    I.box(ic,0,0,0,52,40,5,G)
    for x in (10,26,42):
        for y in (10,30):
            X,Y=I.p(x,y,5); rx,ry=I.ellipse_r(4.2)
            ic.fill(ell(X,Y,rx,ry),A3); ic.stroke(ell(X,Y,rx,ry),INK,OW*0.8)
    return ic
@icon
def pattern_path():
    ic=Icon('pattern_path','Path pattern')
    P=((6,50),(20,20),(44,60),(58,16))
    ic.stroke('M6 50C20 20 44 60 58 16',GUIDE,OW)
    for i,t in enumerate((0.08,0.36,0.64,0.92)):
        x,y=bez(*P,t); I=Iso(1.2,x,y+2); cols=G if i==0 else B
        I.box(ic,-4.5,-4.5,-4.5,4.5,4.5,4.5,cols,OW*0.9)
    return ic
@icon
def offset_surface():
    ic=Icon('offset_surface','Offset surface'); I=Iso(0.8,32,38)
    face(ic,I,[(-26,-26,-9),(26,-26,-9),(26,26,-9),(-26,26,-9)],P1)
    ic.stroke(I.poly((-26,-26,18),(26,-26,18),(26,26,18),(-26,26,18)),A3,AW*0.8,fill=A1)
    return ic

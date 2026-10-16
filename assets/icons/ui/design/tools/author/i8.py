from lib import *
from i2 import pt, arrowhead, arrow, poly_reg
from i3 import star4
from i4 import G,B,face
from i5 import arc_arrow
from i4 import cyl3
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
    return ic.recenter()

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
    ic.stroke(poly([(17,9),(52,32),(17,55)]),INK,OW,fill=A2); return ic
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
    return ic.recenter()

@icon
def restore():
    ic=Icon('restore','Restore')
    ic.fill(ell(32,33,14),P1); ic.stroke(ell(32,33,14),INK,OW)
    ic.stroke('M32 25L32 33L38 37',A3,AW*0.8)
    arc_arrow(ic,32,33,23,30,-200,A3,AW*0.9,8,9)
    return ic.recenter()

@icon
def checkpoint():
    ic=Icon('checkpoint','Checkpoint')
    ic.stroke('M16 8L16 58',INK,AW)
    d=poly([(16,12),(54,12),(46,23),(54,34),(16,34)]); ic.fill(d,A2); ic.stroke(d,INK,OW)
    return ic.recenter()

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
    return ic.recenter()

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
    ic=Icon('render_scene','Render scene'); I=Iso(0.62,26,38); I.box(ic,-13,-13,-13,13,13,13,G)
    ic.fill(star4(47,17,10,0.12),A3); return ic.recenter()
def render_scene():
    ic=Icon('render_scene','Render scene'); I=Iso(1,28,42); I.box(ic,-13,-13,-13,13,13,13,G)
    ic.fill(star4(48,16,12,0.12),A3); return ic

@icon
def component():
    ic=Icon('component','Component'); I=Iso().fit([(0,-16,0),(16,16,0),(0,-16,32),(16,16,32),(0,16,0),(16,-16,0)],58,58); c=16
    I.box(ic,0,-c,0,c,0,c,B); I.box(ic,0,-c,c,c,0,2*c,G)     # blue cube behind and to the right, grey cube on top of it
    I.box(ic,0,0,0,c,c,c,G)                                   # grey cube in front
    return ic
def component():
    ic=Icon('component','Component'); I=Iso().fit([(0,0,0),(32,32,0),(0,0,32),(32,32,32),(0,32,0),(32,0,0)],58,58); c=16
    I.box(ic,0,0,0,c,c,c,B); I.box(ic,0,0,c,c,c,2*c,G)
    I.box(ic,c,0,c,2*c,c,2*c,G); I.box(ic,0,c,c,c,2*c,2*c,G)
    return ic
@icon
def interference():
    ic=Icon('interference','Interference'); I=Iso().fit([(0,-14,0),(40,28,0),(0,-14,24),(40,28,24)],58,56)
    I.box(ic,0,0,0,24,24,24,G); I.box(ic,16,-14,0,40,10,24,G)
    sq=I.poly((16,0,24),(24,0,24),(24,10,24),(16,10,24)); ic.fill(sq,A2); ic.stroke(sq,INK,OW)   # the overlap, marked on the up-facing face
    return ic
def interference():
    ic=Icon('interference','Interference'); I=Iso().fit([(0,-14,0),(40,28,0),(0,-14,24),(40,28,24)],58,56)
    I.box(ic,0,0,0,24,24,24,G); I.box(ic,16,-14,0,40,10,24,G)
    I.box(ic,16,0,0,24,10,24,B)
    return ic
@icon
def point():
    ic=Icon('point','Point')
    ic.stroke('M32 8L32 56M8 32L56 32',P3,AW)
    ic.fill(ell(32,32,10.5),GAP); ic.dot(32,32,6,A3)
    return ic
def point():
    ic=Icon('point','Point')
    ic.stroke('M32 8L32 56M8 32L56 32',A3,AW)
    ic.fill(ell(32,32,10.5),GAP); ic.dot(32,32,6,A3)
    return ic
@icon
def import_():
    ic=Icon('import','Import'); I=Iso(0.9886,32,41)
    I.box(ic,0,0,0,20,20,20,G)
    sh='M32 5L32 22'
    ic.stroke(sh,GAP,NW+6); ic.stroke(poly([(32,33),(23,21),(41,21)]),GAP,6,fill=GAP)
    ic.stroke(sh,A3,NW); ic.stroke(poly([(32,33),(23,21),(41,21)]),A3,2,fill=A3)
    return ic
@icon
def fastener():
    ic=Icon('fastener','Fastener')
    xl,xr=25.5,38.5
    shaft='M%s 25L%s 54L32 61L%s 54L%s 25Z'%(f(xl),f(xl),f(xr),f(xr))
    ic.fill(shaft,P2); ic.fill('M33.5 25L33.5 57.2L32 61L%s 54L%s 25Z'%(f(xr),f(xr)),P3)
    ic.stroke(shaft,INK,OW)
    for y in (35,43,51):
        ic.stroke('M%s %sQ32 %s %s %s'%(f(xl),f(y+3),f(y+8),f(xr),f(y-2)),INK,OW)
    cylinder(ic,32,13.5,16,10,G,OW)
    ic.stroke('M26 13.5L38 13.5M32 9L32 18',A3,AW*0.95)
    return ic
def fastener():
    ic=Icon('fastener','Fastener')
    xl,xr=25.5,38.5
    shaft='M%s 22L%s 52L32 60L%s 52L%s 22Z'%(f(xl),f(xl),f(xr),f(xr))
    ic.fill(shaft,P2); ic.fill('M34 22L34 55.2L32 60L%s 52L%s 22Z'%(f(xr),f(xr)),P3) if False else ic.fill('M33.5 22L33.5 56.3L32 60L%s 52L%s 22Z'%(f(xr),f(xr)),P3)
    ic.stroke(shaft,INK,OW)
    for y in (33,41,49):
        ic.stroke('M%s %sQ32 %s %s %s'%(f(xl),f(y+3),f(y+8),f(xr),f(y-2)),INK,OW)
    cylinder(ic,32,10,16,10,G,OW)
    ic.stroke('M26 10L38 10M32 5.5L32 14.5',A3,AW*0.95)
    return ic

@icon
def offset_face():
    ic=Icon('offset_face','Offset face')
    src=open(os.path.join(OUT,'extrude.svg')).read()
    body=[l.strip() for l in src.split('\n') if l.strip().startswith('<path') and 'M32 3L39.54' not in l]
    for x in (21,43):                                          # arrows start behind the cube
        ic.stroke(poly([(x,44),(x,12)],False),A3,5)
        ic.fill(poly([(x,2.5),(x-6.2,12.5),(x+6.2,12.5)]),A3)
    ic.stroke('M9.36 30.35L32 17.15L54.63 30.35L54.63 46.38L32 59.58L9.36 46.38Z',GAP,10,fill=GAP)   # very wide punch-out around the cube
    ic.els+=body
    return ic
def offset_face():
    ic=Icon('offset_face','Offset face')
    src=open(os.path.join(OUT,'extrude.svg')).read()
    body=[l.strip() for l in src.split('\n') if l.strip().startswith('<path') and 'M32 3L39.54' not in l]
    ic.els=body
    for dx in (-10.5,10.5):
        pts=[(32,3),(39.54,10.55),(34.82,10.55),(34.82,13.38),(29.17,13.38),(29.17,10.55),(24.45,10.55)]
        ic.fill(poly([(x*0.82+dx+5.76*0 + (32-32*0.82),y) for x,y in pts]),A3)
    return ic

@icon
def accessibility():
    ic=Icon('accessibility','Accessibility'); I=Iso().fit([(0,0,0),(32,32,0),(0,32,0),(32,0,0),(22,10,50)],58,58)
    I.box(ic,0,0,0,32,32,14,(P1,A2,A3))                            # short cube base: vertical faces accented, top grey
    cyl3(ic,I,16,16,9,14,24,(P1,A2,A3))                            # short cylinder on top
    bx,by=22,10                                                    # lower and to the right
    X,Y=I.p(bx,by,50); X0,Y0=I.p(bx,by,25); rx,ry=I.ellipse_r(2.3)
    body='M%s %sL%s %s%sL%s %sZ'%(f(X-rx),f(Y),f(X-rx),f(Y0),arcto(rx,ry,X+rx,Y0,0,0),f(X+rx),f(Y))
    tip=poly([(X-rx,Y0),(X,Y0+8),(X+rx,Y0)])
    ic.fill(body,A3); ic.stroke(tip,INK,OW,fill=P2); ic.stroke(body,INK,OW)               # the drill bit
    ic.stroke(ell(X,Y,rx,ry),INK,OW,fill=P1)
    return ic.recenter()

@icon
def folder():
    ic=Icon('folder','Folder')
    back='M5 17Q5 12 10 12L24 12L30 18L54 18Q59 18 59 23L59 50Q59 55 54 55L10 55Q5 55 5 50Z'
    ic.stroke(back,INK,OW,fill=P3)
    front='M5 31Q5 26 10 26L54 26Q59 26 59 31L59 50Q59 55 54 55L10 55Q5 55 5 50Z'
    ic.stroke(front,INK,OW,fill=P1)
    return ic

// Stand-in icons for commands the handoff set does not cover yet, drawn on a 24 grid in the
// handoff palette: grey passive geometry, blue changed/result regions, ink outlines.
const cube = (x, y, w, s) => {
  const h = w / 2;
  return {
    t: `M${x} ${y}L${x + w} ${y + h}L${x} ${y + 2 * h}L${x - w} ${y + h}Z`,
    l: `M${x - w} ${y + h}L${x} ${y + 2 * h}V${y + 2 * h + s}L${x - w} ${y + h + s}Z`,
    r: `M${x + w} ${y + h}L${x} ${y + 2 * h}V${y + 2 * h + s}L${x + w} ${y + h + s}Z`,
    o: `M${x} ${y}L${x + w} ${y + h}V${y + h + s}L${x} ${y + 2 * h + s}L${x - w} ${y + h + s}V${y + h}ZM${x - w} ${y + h}L${x} ${y + 2 * h}L${x + w} ${y + h}M${x} ${y + 2 * h}V${y + 2 * h + s}`
  };
};
const join = (cs, k) => cs.map((c) => c[k]).join('');
const P = (...cs) => ({ gt: join(cs, 't'), gl: join(cs, 'l'), gr: join(cs, 'r'), o: join(cs, 'o') });
const A = (...cs) => ({ bt: join(cs, 't'), bl: join(cs, 'l'), br: join(cs, 'r'), o: join(cs, 'o') });
const mix = (...parts) => {
  const out = {};
  for (const p of parts) for (const k of Object.keys(p)) out[k] = (out[k] || '') + p[k];
  return out;
};
const dot = (x, y, r = 1.15) => `M${x - r} ${y}a${r} ${r} 0 1 0 ${2 * r} 0a${r} ${r} 0 1 0 ${-2 * r} 0z`;
const cross = 'M12 2l3.2 3.2h-2.1V11h5.8V8.9L22 12l-3.1 3.1V13h-5.8v5.8h2.1L12 22l-3.2-3.2h2.1V13H5.1v2.1L2 12l3.1-3.1V11h5.8V5.2H8.8z';
const big = cube(12, 3, 8, 9), slab = cube(12, 9, 9, 4), plate = cube(12, 7, 9, 5), low = cube(12, 13, 8.5, 3.5);
const cylT = 'M5 7a7 2.6 0 1 0 14 0a7 2.6 0 1 0-14 0z', cylS = 'M5 7v10a7 2.6 0 0 0 14 0V7a7 2.6 0 0 1-14 0z', cylO = 'M5 7a7 2.6 0 1 0 14 0a7 2.6 0 1 0-14 0zM5 7v10a7 2.6 0 0 0 14 0V7';
const F = {
  box: A(big),
  cylinder: { bt: cylT, bl: cylS, o: cylO },
  sphere: { bl: 'M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0z', bt: 'M6.5 9.5a6.5 4.5 0 0 1 9-5a8 8 0 0 0-9 5z', o: 'M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0zM3 12a9 3 0 0 0 18 0' },
  coil: { a: 'M6 4h12L6 8h12L6 12h12L6 16h12L6 20h12' },
  loft: { bt: 'M9 5a3 1.2 0 1 0 6 0a3 1.2 0 1 0-6 0z', bl: 'M9 5L4.5 17a7.5 2.6 0 0 0 15 0L15 5a3 1.2 0 0 1-6 0z', o: 'M9 5a3 1.2 0 1 0 6 0a3 1.2 0 1 0-6 0zM9 5L4.5 17a7.5 2.6 0 0 0 15 0L15 5' },
  rib: { gl: 'M4 21V8h3.5v9.5H20V21z', bl: 'M7.5 17.5l7-7v7z', o: 'M4 21V8h3.5v9.5H20V21zM7.5 17.5l7-7v7' },
  hole: mix(P(plate), { br: 'M8.5 11.5a3.5 1.6 0 1 0 7 0a3.5 1.6 0 1 0-7 0z', o: 'M8.5 11.5a3.5 1.6 0 1 0 7 0a3.5 1.6 0 1 0-7 0z' }),
  thread: { gt: 'M8 5a4 1.5 0 1 0 8 0a4 1.5 0 1 0-8 0z', gl: 'M8 5v14a4 1.5 0 0 0 8 0V5a4 1.5 0 0 1-8 0z', o: 'M8 5a4 1.5 0 1 0 8 0a4 1.5 0 1 0-8 0zM8 5v14a4 1.5 0 0 0 8 0V5', a: 'M8 9l8-2M8 12.5l8-2M8 16l8-2' },
  draft: { gt: 'M12 4L16 6 12 8 8 6Z', gl: 'M8 6L12 8V21L4 17Z', bl: 'M16 6L12 8V21L20 17Z', o: 'M12 4L16 6 20 17 12 21 4 17 8 6ZM8 6L12 8 16 6M12 8V21' },
  presspull: mix(P(slab), { a: 'M12 1.5v6.5M9.8 3.7L12 1.5l2.2 2.2M9.8 5.8L12 8l2.2-2.2' }),
  offsetface: mix(P(slab), { bt: 'M3 9.5L12 5l9 4.5L12 14z', o: 'M3 9.5L12 5l9 4.5L12 14z' }),
  shell: mix(P(big), { bl: 'M12 5L16.5 7.2 12 9.4 7.5 7.2z', o: 'M12 5L16.5 7.2 12 9.4 7.5 7.2z' }),
  split: mix(P(big), { a: 'M2.5 15.5L21.5 6.5' }),
  move: { bl: cross },
  align: mix(P(cube(8, 5, 4.5, 5)), A(cube(17, 5, 4.5, 5)), { a: 'M2 5h20' }),
  mirror: { gl: 'M10 5L3 19h7z', bl: 'M14 5l7 14h-7z', o: 'M10 5L3 19h7zM14 5l7 14h-7z', a: 'M12 2v20' },
  pattern: mix(P(cube(7.5, 2.5, 4.5, 4)), A(cube(16.5, 2.5, 4.5, 4), cube(7.5, 12, 4.5, 4), cube(16.5, 12, 4.5, 4))),
  axis: mix(P(low), { a: 'M12 1.5v18', bd: dot(12, 2.5, 1.4) }),
  point: mix(P(low), { o: 'M12 6v8.5', bd: dot(12, 4.5, 2.2) }),
  sheet: { bl: 'M2.5 18.5h9.5l5-10h4.5v2.5h-3l-5 10H2.5z', bt: 'M2.5 18.5l2-2h7L16 7.5h5.5l-4.5 1L12 18.5z', o: 'M2.5 18.5h9.5l5-10h4.5v2.5h-3l-5 10H2.5zM2.5 18.5l2-2h7L16 7.5h5.5' },
  plastic: mix(P(cube(12, 11, 9, 3.5)), { bt: 'M9 8.5a3 1.2 0 1 0 6 0a3 1.2 0 1 0-6 0z', bl: 'M9 8.5v5a3 1.2 0 0 0 6 0v-5a3 1.2 0 0 1-6 0z', o: 'M9 8.5a3 1.2 0 1 0 6 0a3 1.2 0 1 0-6 0zM9 8.5v5M15 8.5v5' }),
  appearance: { bl: 'M12 3c4 5 6.5 8 6.5 11a6.5 6.5 0 0 1-13 0c0-3 2.5-6 6.5-11z', bt: 'M9 14a3 3 0 0 0 3 3', o: 'M12 3c4 5 6.5 8 6.5 11a6.5 6.5 0 0 1-13 0c0-3 2.5-6 6.5-11z' },
  surface: { bt: 'M3 13c4-6 8 2 18-6v4c-10 8-14 0-18 6z', bl: 'M3 17c4-6 8 2 18-6v2c-10 8-14 0-18 6z', o: 'M3 13c4-6 8 2 18-6v6c-10 8-14 0-18 6z' },
  select: { s: 'M5 3l14 8-6 2-3 7z' },
  chain: { o: 'M10 14a4 4 0 0 0 6 0l3-3a4 4 0 0 0-6-6l-1 1M14 10a4 4 0 0 0-6 0l-3 3a4 4 0 0 0 6 6l1-1' },
  dots: { s: dot(5, 12, 1.6) + dot(12, 12, 1.6) + dot(19, 12, 1.6) },
  eye: { o: 'M2 12s4-7 10-7 10 7 10 7-4 7-10 7S2 12 2 12z', s: 'M9 12a3 3 0 1 0 6 0a3 3 0 1 0-6 0z' },
  filter: { gt: 'M3 5h18l-7 8v6l-4 2v-8z', o: 'M3 5h18l-7 8v6l-4 2v-8z' },
  origin: { a: 'M5 19V5M5 19h14M5 19l9-6', bd: dot(5, 19, 1.6) },
  // sketch tools: blue geometry, blue vertices, pale fill for closed shapes
  line: { a: 'M6 18L18 6', bd: dot(6, 18) + dot(18, 6) },
  rect: { bt: 'M4 6h16v12H4z', a: 'M4 6h16v12H4z', bd: dot(4, 6) + dot(20, 18) },
  circle: { bt: 'M4 12a8 8 0 1 0 16 0a8 8 0 1 0-16 0z', a: 'M4 12a8 8 0 1 0 16 0a8 8 0 1 0-16 0z', bd: dot(12, 12) },
  arc: { a: 'M4 18a8 8 0 0 1 16 0', bd: dot(4, 18) + dot(20, 18) + dot(12, 18) },
  polygon: { bt: 'M12 3l8 4.5v9L12 21l-8-4.5v-9z', a: 'M12 3l8 4.5v9L12 21l-8-4.5v-9z', bd: dot(12, 12) },
  slot: { bt: 'M8 7h8a5 5 0 0 1 0 10H8A5 5 0 0 1 8 7z', a: 'M8 7h8a5 5 0 0 1 0 10H8A5 5 0 0 1 8 7z', bd: dot(8, 12) + dot(16, 12) },
  spline: { a: 'M3 18c4-14 9 6 18-12', bd: dot(3, 18) + dot(12, 12) + dot(21, 6) },
  ellipse: { bt: 'M3 12a9 5 0 1 0 18 0a9 5 0 1 0-18 0z', a: 'M3 12a9 5 0 1 0 18 0a9 5 0 1 0-18 0z', bd: dot(12, 12) },
  text: { s: 'M5 4h14v4h-1.6l-.6-2h-3.8v12.5h2V20H9v-1.5h2V6H7.2l-.6 2H5z' },
  trim: { o: 'M7 3v18M17 3v18', a: 'M2.5 12H7M17 12h4.5' },
  extend: { o: 'M19 3v18', a: 'M3 12h11', bd: 'M14 8.5l5 3.5-5 3.5z' },
  offset: { a: 'M4 19c3-9 13-9 16 0', o: 'M4 13c3-9 13-9 16 0' },
  breakk: { a: 'M3 12h6.5M14.5 12H21', bd: dot(9.5, 12) + dot(14.5, 12) },
  skfillet: { o: 'M4 21v-9M12 4h8', a: 'M4 12a8 8 0 0 1 8-8' },
  circpat: { o: 'M5 12a7 7 0 1 0 14 0a7 7 0 1 0-14 0z', bd: dot(12, 4, 2) + dot(20, 12, 2) + dot(12, 20, 2) + dot(4, 12, 2) },
  rectpat: { bt: 'M3 3h7v7H3z', a: 'M3 3h7v7H3zM14 3h7v7h-7zM3 14h7v7H3zM14 14h7v7h-7z' },
  skmirror: { a: 'M3 18l6-12v12zM21 18l-6-12v12z', o: 'M12 2v20' },
  dimension: { o: 'M4 5v14M20 5v14', a: 'M8 12h8', s: 'M4 12l4.5-3v6zM20 12l-4.5-3v6z' },
  auto: { bl: 'M12 2.5l2.2 5.3 5.3 2.2-5.3 2.2L12 17.5l-2.2-5.3L4.5 10l5.3-2.2z', bt: 'M18.5 15.5l1 2.5 2.5 1-2.5 1-1 2.5-1-2.5-2.5-1 2.5-1z', o: 'M18.5 15.5l1 2.5 2.5 1-2.5 1-1 2.5-1-2.5-2.5-1 2.5-1z' },
  construction: { a: 'M4 20l2.5-2.5M9 15l2.5-2.5M14 10l2.5-2.5M19 5l1-1' },
  project: mix(P(low), { a: 'M12 2.5v9.5', bd: dot(12, 3, 1.6) }),
  intersect: { gt: 'M3 12a6 6 0 1 0 12 0a6 6 0 1 0-12 0z', bt: 'M9 12a6 6 0 1 0 12 0a6 6 0 1 0-12 0z', o: 'M3 12a6 6 0 1 0 12 0a6 6 0 1 0-12 0zM9 12a6 6 0 1 0 12 0a6 6 0 1 0-12 0z' },
  include: mix(P(cube(12, 5, 6.5, 7)), { a: 'M12 0.8v3.4' }),
  flange: mix(P(cube(12, 10, 9, 1.5)), { bl: 'M3 14.5L12 10V2.5L3 7z', o: 'M3 14.5L12 10V2.5L3 7z' }),
  hem: { gl: 'M3 14h13v2.5H3z', bl: 'M16 14h2.5a3.75 3.75 0 0 1 0 7.5H11V19h7.5a1.25 1.25 0 0 0 0-2.5H16z', o: 'M3 14h15.5a3.75 3.75 0 0 1 0 7.5H11V19h7.5a1.25 1.25 0 0 0 0-2.5H3z' },
  fold: { gl: 'M3 15h9v2.5H3z', bl: 'M12 15l6-6 1.8 1.8-6.4 6.7H12z', o: 'M3 15h9l6-6 1.8 1.8-6.4 6.7H3z', a: 'M17 16.5a5 5 0 0 1-1.5 3.5' },
  unfold: { bl: 'M3 17h18v2.5H3z', o: 'M3 17h18v2.5H3zM3 6v8h9', a: 'M9 4.5c5 0 8.5 3 9.5 9M16.2 11.8l2.3 1.7 1.4-2.6' },
  refold: { bl: 'M3 17h12V6.5h2.5V19.5H3z', o: 'M3 17h12V6.5h2.5V19.5H3z', a: 'M5 13c0-4 2.5-7 7-7.5M9.6 3.8l2.4 1.7-1.6 2.4' },
  flatpattern: { bt: 'M8 3h8v5h5v8h-5v5H8v-5H3V8h5z', a: 'M8 3h8v5h5v8h-5v5H8v-5H3V8h5z', o: 'M8 8h8v8H8z' },
  bendtable: { gt: 'M3 4h18v16H3z', o: 'M3 4h18v16H3zM3 9h18M3 14.5h18M9 4v16', a: 'M12.5 17.5h3.5l3-3' },
  smrules: { gt: 'M6 3h9l4 4v14H6z', o: 'M6 3h9l4 4v14H6zM15 3v4h4', bl: 'M8.5 17h5l3-4.5 1.6 1-3.8 5.5H8.5z' },
  exportdxf: { gt: 'M6 3h9l4 4v14H6z', o: 'M6 3h9l4 4v14H6zM15 3v4h4', a: 'M12.5 17.5V10M9.5 13l3-3 3 3' },
  boss: mix(P(cube(12, 11, 9, 3.5)), { bt: 'M9 6.5a3 1.2 0 1 0 6 0a3 1.2 0 1 0-6 0z', bl: 'M9 6.5v7a3 1.2 0 0 0 6 0v-7a3 1.2 0 0 1-6 0z', o: 'M9 6.5a3 1.2 0 1 0 6 0a3 1.2 0 1 0-6 0zM9 6.5v7M15 6.5v7' }),
  snapfit: { gl: 'M4 20V5h4v15z', bl: 'M8 7h7l-3 4h-4z', o: 'M4 20V5h4v15zM8 7h7l-3 4h-4M3 20h18' },
  image: { gt: 'M3 5h18v14H3z', bl: 'M3 16l5-5 4 4 3-3 6 6v1H3z', o: 'M3 5h18v14H3z', s: 'M15 7.5a1.5 1.5 0 1 0 0 3 1.5 1.5 0 1 0 0-3z' }
};
const ORDER = [['gt', 'fill', '#e2e8ef'], ['gl', 'fill', '#b8c3cf'], ['gr', 'fill', '#8e9faf'], ['bt', 'fill', '#b9ddf8'], ['bl', 'fill', '#328bd3'], ['br', 'fill', '#236ead'], ['s', 'fill', '#34465b'], ['o', 'stroke', '#34465b'], ['a', 'stroke', '#236ead'], ['bd', 'fill', '#236ead']];
const fs = require('fs');
const out = process.argv[2];
for (const [name, p] of Object.entries(F)) {
  let body = '';
  for (const [k, kind, col] of ORDER) {
    if (!p[k]) continue;
    body += kind === 'fill'
      ? `<path d="${p[k]}" fill="${col}"${k === 's' ? ' fill-rule="evenodd"' : ''}/>`
      : `<path d="${p[k]}" fill="none" stroke="${col}" stroke-width="${k === 'a' ? 1.45 : 0.96}" stroke-linejoin="round" stroke-linecap="round"/>`;
  }
  fs.writeFileSync(`${out}/f-${name}.svg`, `<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 64 64"><g transform="scale(2.6667)">${body}</g></svg>`);
}
console.log(Object.keys(F).length);

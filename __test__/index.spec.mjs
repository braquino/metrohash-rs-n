import test from 'ava';

import { metrohash128, metrohash64 } from '../index.js';

test('metrohash128 returns expected hashs', (t) => {
  t.is(metrohash128('hello world', 1), '3fc809f2c95a8dfe14ceede9e2c7f4cc');
  t.is(metrohash128('@foo&bar$go?', 5694), '39db3a43155c2aec39bea91b8696ab96');
});

test('metrohash64 returns expected hashs', (t) => {
  t.is(metrohash64('hello world', 1), '3f0ea0e83e2e8389');
  t.is(metrohash64('@foo&bar$go?', 5694), 'b0c3569ee5906288');
});

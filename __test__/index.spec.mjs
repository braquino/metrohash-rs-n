import test from 'ava'

import { metrohash128 } from '../index.js'

test('sum from native', (t) => {
  let hash = metrohash128("hello world", 1);
  t.is(hash, "3fc809f2c95a8dfe14ceede9e2c7f4cc");
})

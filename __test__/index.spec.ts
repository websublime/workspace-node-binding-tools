import test from 'ava'

import { getDefinedAgent } from '../index'

test('sync function from native code', (t) => {
  t.is(getDefinedAgent(), 'pnpm');
});

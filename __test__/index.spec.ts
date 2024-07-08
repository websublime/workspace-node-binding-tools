import test from 'ava'

import { getDefinedAgent } from '../index'

test('get defined package manager', (t) => {
  t.is(getDefinedAgent(), 'pnpm');
});

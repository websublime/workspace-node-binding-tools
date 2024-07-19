import test from 'ava';

import {
  getDefinedPackageManager,
  PackageManager,
  getProjectRootPath
} from '../index';

test('get defined package manager', (t) => {
  t.is(getDefinedPackageManager(), PackageManager.Pnpm);
});

test('get project root path', (t) => {
  t.is(getProjectRootPath(process.cwd()), process.cwd());
});
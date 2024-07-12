# `@websublime/workspace-tools`

![https://github.com/websublime/workspace-node-binding-tools/actions](https://github.com/websublime/workspace-node-binding-tools/workflows/CI/badge.svg)

> Tools to use on github actions for bumping version, changelogs on a monorepo.

## Install this package

```
pnpm add @websublime/workspace-tools
```

## Usage

This package offer a set of functions to retrieve information about the monorepo and the packages that contain. It support all package managers including Bun (WIP).

## API

### `getProjectRootPath()`

Get the root path of the project.

### `getDefinedAgent()`

Get the package manager defined in the project.

### `getMonorepoPackages()`

Get the list of packages in the monorepo.

### `getMonorepoChangedPackages(sha: string)`

Get the list of packages that have changed since the given sha ('main').

### `gitFetchAll(cwd?: string)`

Execute a `fetch` command to get the latest changes from the remote repository.

### `gitFetchAllTags(cwd?: string)`

Execute a `fetch` command to get the latest tags from the remote repository.

### `gitCommit(message: string, body?: string, footer?: string cwd?: string)`

Commit the changes to the repository.

### `gitTag(tag: string, message?: string, cwd?: string)`

Tag the repository with the given tag.

### `gitPush(cwd?: string)`

Push the changes to the remote repository.

### `gitCurrentSha(cwd?: string)`

Get the current sha of the repository.

### `gitCommitBranchName(sha: string, cwd?: string)`

Get the branch name for the commit id.

### `isWorkdirUnclean(cwd?: string)`

Check if the workdir is unclean (uncommited changes).

### `getDivergedCommit(sha: string, cwd?: string)`

Get the diverged commit from the given sha (main).

### `getCommitsSince(cwd?: string, since?: string, relative?: string)`

Get the commits since the given sha (main) for a particular package.

### `getAllFilesChangedSinceSha(sha: string, cwd?: string)`

Get all the files changed since the given sha (main).

### `getAllFilesChangedSinceTagInfos(package_info: Array<PackageInfo>, tag_info: Array<PublishTagInfo>, cwd?: string)`

Get all the files changed since the given tag infos.

### `getAllFilesChangedSinceBranch(package_info: Array<PackageInfo>, branch: string, cwd?: string)`

Get all the files changed since the given branch.

### `getRemoteOrLocalTags(cwd?: string, local?: boolean)`

Get all the tags in the remote or local repository.

### `getLastKnownPublishTagInfoForPackage(package_info: PackageInfo, cwd?: string)`

Get the last known publish tag info for a particular package.

### `getLastKnownPublishTagInfoForAllPackages(package_info: Array<PackageInfo>, cwd?: string)`

Get the last known publish tag info for all packages.

### `getConventionalForPackage(package_info: PackageInfo, no_fetch_all?: boolean cwd?: string, conventional_options?: ConventionalPackageOptions)`

Get the conventional commits for a particular package.

### `validateMonorepoPackagesJson()`

Validates the minimum config of package.json for the monorepo packages.

## Develop requirements

- Install the latest `Rust`
- Install `Node.js@16+` which fully supported `Node-API`
- Run `corepack enable`

## Test in local

- pnpm
- pnpm build
- pnpm test

And you will see:

```bash
$ ava --verbose

  ✔ get defined package manager
  ─

  2 tests passed
✨  Done in 1.12s.
```

## Release package

Ensure you have set your **NPM_TOKEN** in the `GitHub` project setting.

In `Settings -> Secrets`, add **NPM_TOKEN** into it.

When you want to release the package:

```
npm run build
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.

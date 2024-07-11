/* auto-generated by NAPI-RS */
/* eslint-disable */
export const enum Agent {
  Npm = 'Npm',
  Yarn = 'Yarn',
  Pnpm = 'Pnpm',
  Bun = 'Bun'
}

export interface Commit {
  hash: string
  authorName: string
  authorEmail: string
  authorDate: string
  message: string
}

export interface ConventionalPackage {
  packageInfo: PackageInfo
  conventionalConfig: any
  conventionalCommits: any
  changelogOutput: string
}

export interface ConventionalPackageOptions {
  version?: string
  title?: string
}

export declare function executeFetchAll(cwd?: string | undefined | null): boolean

export declare function executeFetchAllTags(cwd?: string | undefined | null): boolean

export declare function getAllFilesChangedSinceBranch(packageInfo: Array<PackageInfo>, branch: string, cwd?: string | undefined | null): Array<string>

export declare function getAllFilesChangedSinceSha(sha: string, cwd?: string | undefined | null): Array<string>

export declare function getAllFilesChangedSinceTagInfos(packageInfo: Array<PackageInfo>, tagInfo: Array<PublishTagInfo>, cwd?: string | undefined | null): Array<string>

export declare function getCommitsSince(cwd?: string | undefined | null, since?: string | undefined | null, relative?: string | undefined | null): Array<Commit>

export declare function getConventionalForPackage(packageInfo: PackageInfo, noFetchAll?: boolean | undefined | null, cwd?: string | undefined | null, conventionalOptions?: ConventionalPackageOptions | undefined | null): ConventionalPackage

export declare function getCurrentSha(cwd?: string | undefined | null): string

export declare function getDefinedAgent(): string | null

export declare function getDivergedCommit(refer: string, cwd?: string | undefined | null): string | null

export declare function getLastKnownPublishTagInfoForAllPackages(packageInfo: Array<PackageInfo>, cwd?: string | undefined | null): Array<PublishTagInfo | undefined | null>

export declare function getLastKnownPublishTagInfoForPackage(packageInfo: PackageInfo, cwd?: string | undefined | null): PublishTagInfo | null

export declare function getMonorepoChangedPackages(sha?: string | undefined | null): Array<PackageInfo>

export declare function getMonorepoPackageRepositoryInfo(url: string): PackageRepositoryInfo

export declare function getMonorepoPackages(): Array<PackageInfo>

export declare function getProjectRootPath(): string | null

export declare function getRemoteOrLocalTags(cwd?: string | undefined | null, local?: boolean | undefined | null): Array<RemoteTags>

export declare function gitPush(cwd?: string | undefined | null): boolean

export declare function isWorkdirUnclean(cwd?: string | undefined | null): boolean

export interface PackageInfo {
  name: string
  private: boolean
  packageJsonPath: string
  packagePath: string
  packageRelativePath: string
  pkgJson: any
  root: boolean
  version: string
  url: string
  repositoryInfo?: PackageRepositoryInfo
}

export interface PackageRepositoryInfo {
  domain: string
  orga: string
  project: string
}

export interface PublishTagInfo {
  hash: string
  tag: string
  package: string
}

export interface RemoteTags {
  hash: string
  tag: string
}

export declare function setCommit(message: string, body?: string | undefined | null, footer?: string | undefined | null, cwd?: string | undefined | null): boolean

export declare function setTag(tag: string, message?: string | undefined | null, cwd?: string | undefined | null): boolean

export declare function validateMonorepoPackagesJson(): boolean


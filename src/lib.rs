#![allow(clippy::all)]

use std::path::{Path, PathBuf};

use workspace_node_tools::bumps::{get_bumps, BumpOptions, BumpPackage};
use workspace_node_tools::changes::{
  add_change, change_exist, changes_file_exist, get_change, get_changes, init_changes,
  remove_change, Change, Changes, ChangesFileData, ChangesOptions,
};
use workspace_node_tools::conventional::{
  get_conventional_for_package, ConventionalPackage, ConventionalPackageOptions,
};
use workspace_node_tools::git::{
  get_all_files_changed_since_branch, get_commits_since, get_diverged_commit,
  get_last_known_publish_tag_info_for_all_packages, get_last_known_publish_tag_info_for_package,
  get_remote_or_local_tags, git_all_files_changed_since_sha, git_branch_from_commit, git_commit,
  git_current_branch, git_current_sha, git_fetch_all, git_first_sha, git_previous_sha, git_push,
  git_tag, git_workdir_unclean, Commit, PublishTagInfo, RemoteTags,
};
use workspace_node_tools::manager::{detect_package_manager, PackageManager};
use workspace_node_tools::packages::{
  get_changed_packages, get_monorepo_package_manager, get_packages, PackageInfo,
};
use workspace_node_tools::paths::get_project_root_path;

#[macro_use]
extern crate napi_derive;

/// Get the project root path
///
/// # Examples
///
/// ```
/// const { getProjectRootPath } = require('workspace-node-tools');
/// const project_root = getProjectRootPath(process.cwd());
/// ```
///
/// @param root - The root path to start searching from
#[napi(js_name = "getProjectRootPath")]
pub fn js_project_root_path(root: Option<String>) -> Option<String> {
  match root {
    Some(root) => get_project_root_path(Some(PathBuf::from(root))),
    None => get_project_root_path(None),
  }
}

/// Get the defined package manager
///
/// # Examples
///
/// ```
/// const { getDefinedPackageManager } = require('workspace-node-tools');
/// const packageManager = getDefinedPackageManager(process.cwd());
/// ```
///
/// @param root - The root path to start searching from
#[napi(js_name = "getDefinedPackageManager")]
pub fn js_define_package_manager(root: Option<String>) -> Option<PackageManager> {
  get_monorepo_package_manager(root)
}

/// Detect the package manager
///
/// # Examples
///
/// ```
/// const { detectPackageManager } = require('workspace-node-tools');
/// const packageManager = detectPackageManager(process.cwd());
/// ```
///
/// @param root - The root path to start searching from
#[napi(js_name = "detectPackageManager")]
pub fn js_detect_package_manager(root: String) -> Option<PackageManager> {
  detect_package_manager(&Path::new(&root))
}

/// Get packages available in the monorepo
///
/// # Examples
///
/// ```
/// const { getPackages } = require('workspace-node-tools');
/// const packages = getPackages(process.cwd());
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "getPackages")]
pub fn js_get_packages(cwd: Option<String>) -> Vec<PackageInfo> {
  get_packages(cwd)
}

/// Get changed packages
///
/// # Examples
///
/// ```
/// const { getChangedPackages } = require('workspace-node-tools');
/// const changedPackages = getChangedPackages("main", process.cwd());
/// ```
///
/// @param sha - The commit sha to compare against (normally main branch)
/// @param cwd - The root path to start searching from
#[napi(js_name = "getChangedPackages")]
pub fn js_get_changed_packages(sha: Option<String>, cwd: Option<String>) -> Vec<PackageInfo> {
  get_changed_packages(sha, cwd)
}

/// Fetch all git changes
///
/// # Examples
///
/// ```
/// const { gitFetchAll } = require('workspace-node-tools');
/// gitFetchAll(process.cwd(), true);
/// ```
///
/// @param cwd - The root path to start searching from
/// @param fetch_tags - Fetch tags from remote
#[napi(js_name = "gitFetchAll")]
pub fn js_git_fetch_all(cwd: Option<String>, fetch_tags: Option<bool>) -> bool {
  git_fetch_all(cwd, fetch_tags).is_ok()
}

/// Commit changes to git
///
/// # Examples
///
/// ```
/// const { gitCommit } = require('workspace-node-tools');
/// gitCommit("feat: add new feature", "body", "footer", process.cwd());
/// ```
///
/// @param message - The commit message
/// @param body - The commit body
/// @param footer - The commit footer
/// @param cwd - The root path to start searching from
#[napi(js_name = "gitCommit")]
pub fn js_git_commit(
  message: String,
  body: Option<String>,
  footer: Option<String>,
  cwd: Option<String>,
) -> bool {
  git_commit(message, body, footer, cwd).is_ok()
}

/// Tag a git commit
///
/// # Examples
///
/// ```
/// const { gitTag } = require('workspace-node-tools');
/// gitTag("v1.0.0", "message", process.cwd());
/// ```
///
/// @param tag - The tag to apply
/// @param message - The tag message
/// @param cwd - The root path to start searching from
#[napi(js_name = "gitTag")]
pub fn js_git_tag(tag: String, message: Option<String>, cwd: Option<String>) -> bool {
  git_tag(tag, message, cwd).is_ok()
}

/// Push changes to git
///
/// # Examples
///
/// ```
/// const { gitPush } = require('workspace-node-tools');
/// gitPush(process.cwd(), true);
/// ```
///
/// @param cwd - The root path to start searching from
/// @param follow_tags - Follow tags
#[napi(js_name = "gitPush")]
pub fn js_git_push(cwd: Option<String>, follow_tags: Option<bool>) -> bool {
  git_push(cwd, follow_tags).is_ok()
}

/// Get the current branch
///
/// # Examples
///
/// ```
/// const { gitCurrentBranch } = require('workspace-node-tools');
/// const branch = gitCurrentBranch(process.cwd());
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "gitCurrentBranch")]
pub fn js_git_current_branch(cwd: Option<String>) -> Option<String> {
  git_current_branch(cwd)
}

/// Get the current commit id
///
/// # Examples
///
/// ```
/// const { gitCurrentSha } = require('workspace-node-tools');
/// const sha = gitCurrentSha(process.cwd());
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "gitCurrentSha")]
pub fn js_git_current_sha(cwd: Option<String>) -> String {
  git_current_sha(cwd)
}

/// Get the previous commit id
///
/// # Examples
///
/// ```
/// const { gitPreviousSha } = require('workspace-node-tools');
/// const sha = gitPreviousSha(process.cwd());
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "gitPreviousSha")]
pub fn js_git_previous_sha(cwd: Option<String>) -> Option<String> {
  Some(git_previous_sha(cwd))
}

/// Get the first commit id
///
/// # Examples
///
/// ```
/// const { gitFirstSha } = require('workspace-node-tools');
/// const sha = gitFirstSha(process.cwd());
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "gitFirstSha")]
pub fn js_git_first_sha(cwd: Option<String>, branch: Option<String>) -> Option<String> {
  Some(git_first_sha(cwd, branch))
}

/// Check if the workdir is unclean
///
/// # Examples
///
/// ```
/// const { isWorkdirUnclean } = require('workspace-node-tools');
/// const unclean = isWorkdirUnclean(process.cwd());
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "isWorkdirUnclean")]
pub fn js_git_workdir_unclean(cwd: Option<String>) -> bool {
  git_workdir_unclean(cwd)
}

/// Get the branch name from a commit id
///
/// # Examples
///
/// ```
/// const { gitCommitBranchName } = require('workspace-node-tools');
/// const branch = gitCommitBranchName("sha", process.cwd());
/// ```
///
/// @param sha - The commit id
/// @param cwd - The root path to start searching from
#[napi(js_name = "gitCommitBranchName")]
pub fn js_git_branch_from_commit(sha: String, cwd: Option<String>) -> Option<String> {
  git_branch_from_commit(sha, cwd)
}

/// Get all files changed since a commit id
///
/// # Examples
///
/// ```
/// const { gitAllFilesChangedSinceSha } = require('workspace-node-tools');
/// const files = gitAllFilesChangedSinceSha("sha", process.cwd());
/// ```
///
/// @param sha - The commit id (accepts branch, tag)
/// @param cwd - The root path to start searching from
#[napi(js_name = "gitAllFilesChangedSinceSha")]
pub fn js_git_all_files_changed_since_sha(sha: String, cwd: Option<String>) -> Vec<String> {
  git_all_files_changed_since_sha(sha, cwd)
}

/// Get the diverged commit
///
/// # Examples
///
/// ```
/// const { getDivergedCommit } = require('workspace-node-tools');
/// const diverged = getDivergedCommit("main", process.cwd());
/// ```
///
/// @param refer - The branch to compare against
/// @param cwd - The root path to start searching from
#[napi(js_name = "getDivergedCommit")]
pub fn js_get_diverged_commit(refer: String, cwd: Option<String>) -> Option<String> {
  get_diverged_commit(refer, cwd)
}

/// Get commits since a commit id
///
/// # Examples
///
/// ```
/// const { getCommitsSince } = require('workspace-node-tools');
/// const commits = getCommitsSince(process.cwd(), "main", "packages/package-a");
/// ```
///
/// @param cwd - The root path to start searching from
/// @param since - The commit id (accepts branch, tag)
/// @param relative - The relative path to search from
#[napi(js_name = "getCommitsSince")]
pub fn js_get_commits_since(
  cwd: Option<String>,
  since: Option<String>,
  relative: Option<String>,
) -> Vec<Commit> {
  get_commits_since(cwd, since, relative)
}

/// Get all files changed since a branch
///
/// # Examples
///
/// ```
/// const { getAllFilesChangedSinceBranch } = require('workspace-node-tools');
/// const files = getAllFilesChangedSinceBranch([PackageInfo{}], "main", process.cwd());
/// ```
///
/// @param package_info - The list of package info
/// @param branch - The branch to compare against
/// @param cwd - The root path to start searching from
#[napi(js_name = "getAllFilesChangedSinceBranch")]
pub fn js_get_all_files_changed_since_branch(
  package_info: Vec<PackageInfo>,
  branch: String,
  cwd: Option<String>,
) -> Vec<String> {
  get_all_files_changed_since_branch(&package_info, &branch, cwd)
}

/// Get the last known publish tag info for a package
///
/// # Examples
///
/// ```
/// const { getLastKnownPublishTagInfoForPackage } = require('workspace-node-tools');
/// const tagInfo = getLastKnownPublishTagInfoForPackage(PackageInfo{}, process.cwd());
/// ```
///
/// @param package_info - The package info
/// @param cwd - The root path to start searching from
#[napi(js_name = "getLastKnownPublishTagInfoForPackage")]
pub fn js_get_last_known_publish_tag_info_for_package(
  package_info: PackageInfo,
  cwd: Option<String>,
) -> Option<PublishTagInfo> {
  get_last_known_publish_tag_info_for_package(&package_info, cwd)
}

/// Get the last known publish tag info for all packages
///
/// # Examples
///
/// ```
/// const { getLastKnownPublishTagInfoForAllPackages } = require('workspace-node-tools');
/// const tagInfo = getLastKnownPublishTagInfoForAllPackages([PackageInfo{}], process.cwd());
/// ```
///
/// @param package_info - The list of package info
/// @param cwd - The root path to start searching from
#[napi(js_name = "getLastKnownPublishTagInfoForAllPackages")]
pub fn js_get_last_known_publish_tag_info_for_all_packages(
  package_info: Vec<PackageInfo>,
  cwd: Option<String>,
) -> Vec<Option<PublishTagInfo>> {
  get_last_known_publish_tag_info_for_all_packages(&package_info, cwd)
}

/// Get remote or local tags
///
/// # Examples
///
/// ```
/// const { getRemoteOrLocalTags } = require('workspace-node-tools');
/// const tags = getRemoteOrLocalTags(process.cwd(), true);
/// ```
///
/// @param cwd - The root path to start searching from
/// @param local - Fetch local tags
#[napi(js_name = "getRemoteOrLocalTags")]
pub fn js_get_remote_or_local_tags(cwd: Option<String>, local: Option<bool>) -> Vec<RemoteTags> {
  get_remote_or_local_tags(cwd, local)
}

/// Get the conventional for a package
///
/// # Examples
///
/// ```
/// const { getConventionalForPackage } = require('workspace-node-tools');
/// const conventional = getConventionalForPackage(PackageInfo{}, false, process.cwd(), ConventionalPackageOptions{});
/// ```
///
/// @param package_info - The package info
/// @param no_fetch_all - Do not fetch all commits
/// @param cwd - The root path to start searching from
/// @param conventional_options - The conventional options
#[napi(js_name = "getConventionalForPackage")]
pub fn js_get_conventional_for_package(
  package_info: PackageInfo,
  no_fetch_all: Option<bool>,
  cwd: Option<String>,
  conventional_options: Option<ConventionalPackageOptions>,
) -> ConventionalPackage {
  get_conventional_for_package(&package_info, no_fetch_all, cwd, &conventional_options)
}

/// Get bumps
///
/// # Examples
///
/// ```
/// const { getBumps } = require('workspace-node-tools');
/// const bumps = getBumps(BumpOptions{});
/// ```
///
/// @param options - The bump options
#[napi(js_name = "getBumps")]
pub fn js_get_bumps(options: BumpOptions) -> Vec<BumpPackage> {
  get_bumps(options)
}

/// Init changes
///
/// # Examples
///
/// ```
/// const { initChanges } = require('workspace-node-tools');
/// const changes = initChanges(process.cwd(), ChangesOptions{});
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "initChanges")]
pub fn js_init_changes(
  cwd: Option<String>,
  change_options: Option<ChangesOptions>,
) -> ChangesFileData {
  init_changes(cwd, &change_options)
}

/// Add change
///
/// # Examples
///
/// ```
/// const { addChange } = require('workspace-node-tools');
/// addChange(Change{}, process.cwd());
/// ```
///
/// @param change - The change to add
/// @param cwd - The root path to start searching from
#[napi(js_name = "addChange")]
pub fn js_add_change(change: Change, cwd: Option<String>) -> bool {
  add_change(&change, cwd)
}

/// Remove change
///
/// # Examples
///
/// ```
/// const { removeChange } = require('workspace-node-tools');
/// removeChange("branch-name", process.cwd());
/// ```
///
/// @param branch_name - The branch name to remove
/// @param cwd - The root path to start searching from
#[napi(js_name = "removeChange")]
pub fn js_remove_change(branch_name: String, cwd: Option<String>) -> bool {
  remove_change(branch_name, cwd)
}

/// Change exist
///
/// # Examples
///
/// ```
/// const { changeExist } = require('workspace-node-tools');
/// const exist = changeExist("branch-name", process.cwd());
/// ```
///
/// @param branch_name - The branch name to check
/// @param cwd - The root path to start searching from
#[napi(js_name = "changeExist")]
pub fn js_change_exist(branch_name: String, cwd: Option<String>) -> bool {
  change_exist(branch_name, cwd)
}

/// Get change
///
/// # Examples
///
/// ```
/// const { getChange } = require('workspace-node-tools');
/// const changes = getChange("branch-name", process.cwd());
/// ```
///
/// @param branch_name - The branch name to get
/// @param cwd - The root path to start searching from
#[napi(js_name = "getChange")]
pub fn js_get_change(branch_name: String, cwd: Option<String>) -> Vec<Change> {
  get_change(branch_name, cwd)
}

/// Get changes
///
/// # Examples
///
/// ```
/// const { getChanges } = require('workspace-node-tools');
/// const changes = getChanges(process.cwd());
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "getChanges")]
pub fn js_get_changes(cwd: Option<String>) -> Changes {
  get_changes(cwd)
}

/// Changes file exist
///
/// # Examples
///
/// ```
/// const { changesFileExist } = require('workspace-node-tools');
/// const exist = changesFileExist(process.cwd());
/// ```
///
/// @param cwd - The root path to start searching from
#[napi(js_name = "changesFileExist")]
pub fn js_changes_file_exist(cwd: Option<String>) -> bool {
  changes_file_exist(cwd)
}

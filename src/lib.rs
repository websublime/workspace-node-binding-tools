#![allow(clippy::all)]

use std::path::{Path, PathBuf};

use workspace_node_tools::bumps::{get_bumps, BumpOptions, BumpPackage};
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

#[napi(js_name = "getProjectRootPath")]
pub fn js_project_root_path(root: Option<String>) -> Option<String> {
  match root {
    Some(root) => get_project_root_path(Some(PathBuf::from(root))),
    None => get_project_root_path(None),
  }
}

#[napi(js_name = "getDefinedPackageManager")]
pub fn js_define_package_manager(root: Option<String>) -> Option<PackageManager> {
  get_monorepo_package_manager(root)
}

#[napi(js_name = "detectPackageManager")]
pub fn js_detect_package_manager(root: String) -> Option<PackageManager> {
  detect_package_manager(&Path::new(&root))
}

#[napi(js_name = "getPackages")]
pub fn js_get_packages(cwd: Option<String>) -> Vec<PackageInfo> {
  get_packages(cwd)
}

#[napi(js_name = "getChangedPackages")]
pub fn js_get_changed_packages(sha: Option<String>, cwd: Option<String>) -> Vec<PackageInfo> {
  get_changed_packages(sha, cwd)
}

#[napi(js_name = "gitFetchAll")]
pub fn js_git_fetch_all(cwd: Option<String>, fetch_tags: Option<bool>) -> bool {
  git_fetch_all(cwd, fetch_tags).is_ok()
}

#[napi(js_name = "gitCommit")]
pub fn js_git_commit(
  message: String,
  body: Option<String>,
  footer: Option<String>,
  cwd: Option<String>,
) -> bool {
  git_commit(message, body, footer, cwd).is_ok()
}

#[napi(js_name = "gitTag")]
pub fn js_git_tag(tag: String, message: Option<String>, cwd: Option<String>) -> bool {
  git_tag(tag, message, cwd).is_ok()
}

#[napi(js_name = "gitPush")]
pub fn js_git_push(cwd: Option<String>, follow_tags: Option<bool>) -> bool {
  git_push(cwd, follow_tags).is_ok()
}

#[napi(js_name = "gitCurrentBranch")]
pub fn js_git_current_branch(cwd: Option<String>) -> Option<String> {
  git_current_branch(cwd)
}

#[napi(js_name = "gitCurrentSha")]
pub fn js_git_current_sha(cwd: Option<String>) -> String {
  git_current_sha(cwd)
}

#[napi(js_name = "gitPreviousSha")]
pub fn js_git_previous_sha(cwd: Option<String>) -> Option<String> {
  Some(git_previous_sha(cwd))
}

#[napi(js_name = "gitFirstSha")]
pub fn js_git_first_sha(cwd: Option<String>, branch: Option<String>) -> Option<String> {
  Some(git_first_sha(cwd, branch))
}

#[napi(js_name = "isWorkdirUnclean")]
pub fn js_git_workdir_unclean(cwd: Option<String>) -> bool {
  git_workdir_unclean(cwd)
}

#[napi(js_name = "gitCommitBranchName")]
pub fn js_git_branch_from_commit(sha: String, cwd: Option<String>) -> Option<String> {
  git_branch_from_commit(sha, cwd)
}

#[napi(js_name = "gitAllFilesChangedSinceSha")]
pub fn js_git_all_files_changed_since_sha(sha: String, cwd: Option<String>) -> Vec<String> {
  git_all_files_changed_since_sha(sha, cwd)
}

#[napi(js_name = "getDivergedCommit")]
pub fn js_get_diverged_commit(refer: String, cwd: Option<String>) -> Option<String> {
  get_diverged_commit(refer, cwd)
}

#[napi(js_name = "getCommitsSince")]
pub fn js_get_commits_since(
  cwd: Option<String>,
  since: Option<String>,
  relative: Option<String>,
) -> Vec<Commit> {
  get_commits_since(cwd, since, relative)
}

#[napi(js_name = "getAllFilesChangedSinceBranch")]
pub fn js_get_all_files_changed_since_branch(
  package_info: Vec<PackageInfo>,
  branch: String,
  cwd: Option<String>,
) -> Vec<String> {
  get_all_files_changed_since_branch(&package_info, &branch, cwd)
}

#[napi(js_name = "getLastKnownPublishTagInfoForPackage")]
pub fn js_get_last_known_publish_tag_info_for_package(
  package_info: PackageInfo,
  cwd: Option<String>,
) -> Option<PublishTagInfo> {
  get_last_known_publish_tag_info_for_package(&package_info, cwd)
}

#[napi(js_name = "getLastKnownPublishTagInfoForAllPackages")]
pub fn js_get_last_known_publish_tag_info_for_all_packages(
  package_info: Vec<PackageInfo>,
  cwd: Option<String>,
) -> Vec<Option<PublishTagInfo>> {
  get_last_known_publish_tag_info_for_all_packages(&package_info, cwd)
}

#[napi(js_name = "getRemoteOrLocalTags")]
pub fn js_get_remote_or_local_tags(cwd: Option<String>, local: Option<bool>) -> Vec<RemoteTags> {
  get_remote_or_local_tags(cwd, local)
}

#[napi(js_name = "getConventionalForPackage")]
pub fn js_get_conventional_for_package(
  package_info: PackageInfo,
  no_fetch_all: Option<bool>,
  cwd: Option<String>,
  conventional_options: Option<ConventionalPackageOptions>,
) -> ConventionalPackage {
  get_conventional_for_package(&package_info, no_fetch_all, cwd, &conventional_options)
}

#[napi(js_name = "getBumps")]
pub fn js_get_bumps(options: BumpOptions) -> Vec<BumpPackage> {
  get_bumps(options)
}

#![deny(clippy::all)]

use workspace_node_tools::git::commands::{Commit, Git, PublishTagInfo, RemoteTags};
use workspace_node_tools::git::conventional::{ConventionalPackage, ConventionalPackageOptions};
use workspace_node_tools::monorepo::packages::{Monorepo, PackageInfo, PackageRepositoryInfo};

#[macro_use]
extern crate napi_derive;

#[napi(js_name = "getProjectRootPath")]
pub fn project_root_path() -> Option<String> {
  Monorepo::get_project_root_path()
}

#[napi(js_name = "getDefinedAgent")]
pub fn defined_agent() -> Option<String> {
  Some(Monorepo::get_agent().unwrap().to_string())
}

#[napi(js_name = "getMonorepoPackages")]
pub fn monorepo_packages() -> Vec<PackageInfo> {
  Monorepo::get_packages()
}

#[napi(js_name = "getMonorepoPackageRepositoryInfo")]
pub fn package_repository_info(url: String) -> PackageRepositoryInfo {
  Monorepo::get_package_repository_info(url)
}

#[napi(js_name = "validateMonorepoPackagesJson")]
pub fn validate_packages_json() -> bool {
  Monorepo::validate_packages_json()
}

#[napi(js_name = "getMonorepoChangedPackages")]
pub fn monorepo_changed_packages(sha: Option<String>) -> Vec<PackageInfo> {
  Monorepo::get_changed_packages(sha)
}

#[napi(js_name = "executeFetchAll")]
pub fn git_fetch_all(cwd: Option<String>) -> bool {
  Git::fetch_all(cwd).is_ok()
}

#[napi(js_name = "executeFetchAllTags")]
pub fn git_fetch_all_tags(cwd: Option<String>) -> bool {
  Git::fetch_all_tags(cwd).is_ok()
}

#[napi(js_name = "setCommit")]
pub fn commit(
  message: String,
  body: Option<String>,
  footer: Option<String>,
  cwd: Option<String>,
) -> bool {
  Git::git_commit(message, body, footer, cwd).is_ok()
}

#[napi(js_name = "setTag")]
pub fn tag(tag: String, message: Option<String>, cwd: Option<String>) -> bool {
  Git::git_tag(tag, message, cwd).is_ok()
}

#[napi(js_name = "gitPush")]
pub fn push(cwd: Option<String>) -> bool {
  Git::git_push(cwd).is_ok()
}

#[napi(js_name = "getCurrentSha")]
pub fn current_sha(cwd: Option<String>) -> String {
  Git::git_current_sha(cwd)
}

#[napi(js_name = "isWorkdirUnclean")]
pub fn workdir_unclean(cwd: Option<String>) -> bool {
  Git::git_workdir_unclean(cwd)
}

#[napi(js_name = "getDivergedCommit")]
pub fn diverged_commit(refer: String, cwd: Option<String>) -> Option<String> {
  Git::get_diverged_commit(refer, cwd)
}

#[napi(js_name = "getCommitsSince")]
pub fn commits_since(
  cwd: Option<String>,
  since: Option<String>,
  relative: Option<String>,
) -> Vec<Commit> {
  Git::get_commits_since(cwd, since, relative)
}

#[napi(js_name = "getAllFilesChangedSinceSha")]
pub fn all_files_changed_since_sha(sha: String, cwd: Option<String>) -> Vec<String> {
  Git::git_all_files_changed_since_sha(sha, cwd)
}

#[napi(js_name = "getAllFilesChangedSinceTagInfos")]
pub fn all_files_changed_since_tag_infos(
  package_info: Vec<PackageInfo>,
  tag_info: Vec<PublishTagInfo>,
  cwd: Option<String>,
) -> Vec<String> {
  Git::get_all_files_changed_since_tag_infos(package_info, tag_info, cwd)
}

#[napi(js_name = "getAllFilesChangedSinceBranch")]
pub fn all_files_changed_since_branch(
  package_info: Vec<PackageInfo>,
  branch: String,
  cwd: Option<String>,
) -> Vec<String> {
  Git::get_all_files_changed_since_branch(package_info, branch, cwd)
}

#[napi(js_name = "getRemoteOrLocalTags")]
pub fn remote_tags(cwd: Option<String>, local: Option<bool>) -> Vec<RemoteTags> {
  Git::get_remote_or_local_tags(cwd, local)
}

#[napi(js_name = "getLastKnownPublishTagInfoForPackage")]
pub fn last_known_publish_tag_info_for_package(
  package_info: PackageInfo,
  cwd: Option<String>,
) -> Option<PublishTagInfo> {
  Git::get_last_known_publish_tag_info_for_package(package_info, cwd)
}

#[napi(js_name = "getLastKnownPublishTagInfoForAllPackages")]
pub fn last_known_publish_tag_info_for_all_packages(
  package_info: Vec<PackageInfo>,
  cwd: Option<String>,
) -> Vec<Option<PublishTagInfo>> {
  Git::get_last_known_publish_tag_info_for_all_packages(package_info, cwd)
}

#[napi(js_name = "getConventionalForPackage")]
pub fn conventional_for_package(
  package_info: PackageInfo,
  no_fetch_all: Option<bool>,
  cwd: Option<String>,
  conventional_options: Option<ConventionalPackageOptions>,
) -> ConventionalPackage {
  Git::get_conventional_for_package(package_info, no_fetch_all, cwd, conventional_options)
}

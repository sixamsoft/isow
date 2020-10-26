# Change Log

<!-- ### 0.3.0

0.3 marks a culmination of all of 0.2's updates. Where 0.2 focused on features and improvements in displaying the time, this release cycle will focus on time keeping. Although 0.3 brings no new user features, a lot of dependencies have been upgraded. -->

### 0.2.16

- A lot of architectural changes with potential fixes.
- Removed "feature not supported" if patcher isn't supported

### 0.2.15

- If Git isn't present, show only the version without the hash and dirty/clean suffixes. This is meant for Crates.io builds.

### 0.2.13-14

- Fixed a nasty oversight where the ``--utc`` flag returned the local time.
- If the updater flag is not enabled, ISOW will return "feature not supported".

### 0.2.11-12

- Version now includes the git commit hash. This should help improve security, as described on the [Reproducible Builds](https://reproducible-builds.org/) website, and provide a good means of debugging production builds. Versions prior to 0.2.11 should be considered untrusted going forward.

### 0.2.10

- Time has been added shown default along with the rest of the date.  It can be optionally added with the ``-t`` or ``--time`` flag in combo with any existing ones.

### 0.2.6-7

- Maintenance releases. Internal changes.

## 0.2.5

- Removed arbitrary flag conflicts and allowed support for all possible combos, including the default. The reason they were there in the first place was due to my misunderstanding of how Rust interrupts match patterns.
- Big version bump to 0.2.5, due to how usefulness factor going way up.
- Changed the match pattern order to better reflect the actual iso week format.
- Renamed output artifact
- Updated dependencies

## 0.2.1

- Renamed "status" flag to "list" in the update subcommand

## 0.2.0

- Relatively stable features
- Initial release with self-updating support.

## 0.1.0

This was the internal development version that focused on getting it to a relatively stable release.
---
name: prepare-release
description: Prepare a new release for reboot-arch-btw. Use when the user wants to cut a release, bump the version, update the CHANGELOG, or create a release branch/PR.
---

# Prepare a New Release for reboot-arch-btw

This repository is a Rust CLI tool (`reboot-arch-btw`). Releases follow [Semantic Versioning](https://semver.org/) and use a [Keep a Changelog](https://keepachangelog.com/) format.

## Step 1 — Determine the version

1. Ask the user which version to release (or propose one based on unreleased changes).
2. Inspect the `## [Unreleased]` section of `CHANGELOG.md` to judge the bump (patch / minor / major).

## Step 2 — Create a release branch

Create and check out a branch with one of the historical naming patterns:

```
prepare-X.Y.Z-release
```

or (for larger releases that also include dependency updates):

```
release/vX.Y.Z
```

## Step 3 — Update dependencies

Run a full dependency update and commit it first:

```
cargo update
```

Commit this separately:

```
Update dependencies
```

This is a **mandatory** step for every release, not just minor/major ones.

## Step 4 — Update the CHANGELOG

In `CHANGELOG.md`:

1. Replace `## [Unreleased]` with:
   ```markdown
   ## [vX.Y.Z] - YYYY-MM-DD
   ```
   Use today's date.
2. Ensure all unreleased changes are listed under the new version header.
3. At the bottom of the file, update the comparison links:
   - Change `[Unreleased]: https://github.com/rnestler/reboot-arch-btw/compare/vPREVIOUS...master` to start from the **new** version.
   - Add a new link:
     ```markdown
     [vX.Y.Z]: https://github.com/rnestler/reboot-arch-btw/compare/vPREVIOUS..vX.Y.Z
     ```

## Step 5 — Bump the version in Cargo files

1. Edit `Cargo.toml` and update `version = "X.Y.Z"`.
2. Run `cargo update --workspace` to refresh the workspace version in `Cargo.lock`.

## Step 6 — Optional updates

- **improvement-ideas.md:** If any listed ideas were implemented in this release, tick them off (replace `* ` with ` ✅ `).

## Step 7 — Commit the changes

Standard commit order (may all be separate commits or combined into one, but dependency update should **always happen first** if separate):

1. `Update dependencies` (if run separately)
2. `Bump version to X.Y.Z`  
   Files: `Cargo.toml`, `Cargo.lock`
3. `Prepare CHANGELOG for X.Y.Z release`  
   File: `CHANGELOG.md`

For very small patch releases with no dependency changes, all three can be squashed into a single commit:
```
Prepare CHANGELOG and bump version for X.Y.Z release
```

## Step 8 — Open a pull request

Push the branch and open a PR. The PR title should match the branch intent, e.g.:

```
Prepare X.Y.Z release
```

or

```
Release vX.Y.Z
```

Mention the notable changes and link to the `CHANGELOG.md` diff.

## Step 9 — Post-merge tagging (human step)

After the PR is merged, a **signed tag** must be created on the merge commit:

```bash
git tag -s vX.Y.Z -m "Version X.Y.Z

 * <first change>
 * <second change>"
```

The tag message should list the user-visible changes from the release. Push with `git push origin vX.Y.Z`.

## Quick checklist

- [ ] Branch created from latest `master`
- [ ] Dependencies updated (`cargo update`) and committed
- [ ] `CHANGELOG.md` has new version header with today's date
- [ ] `CHANGELOG.md` comparison links updated
- [ ] `Cargo.toml` version bumped
- [ ] `Cargo.lock` refreshed
- [ ] Committed (single or multi-commit pattern)
- [ ] PR opened and merged
- [ ] Signed tag `vX.Y.Z` created on merge commit

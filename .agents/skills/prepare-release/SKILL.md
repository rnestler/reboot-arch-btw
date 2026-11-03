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

## Step 3 — Update the CHANGELOG

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

## Step 4 — Bump the version in Cargo files

1. Edit `Cargo.toml` and update `version = "X.Y.Z"`.
2. Run a Cargo command to update `Cargo.lock`:
   ```
   cargo update --workspace
   ```
   or simply `cargo build`.

## Step 5 — Optional updates

- **Dependencies:** For minor/major releases it is common to run `cargo update` first in a separate commit (e.g. "Update dependencies").
- **improvement-ideas.md:** If any listed ideas were implemented in this release, tick them off (replace `* ` with ` ✅ `).

## Step 6 — Commit the changes

Two patterns have been used historically. Pick one and apply it consistently:

### Pattern A — Single commit (quick patch releases)
```
Prepare CHANGELOG and bump version for X.Y.Z release
```
Files touched: `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`

### Pattern B — Two commits (common for minor/major releases)
1. `Bump version to X.Y.Z`  
   Files: `Cargo.toml`, `Cargo.lock`
2. `Prepare CHANGELOG for X.Y.Z release`  
   File: `CHANGELOG.md`

If you also updated dependencies or ticked off improvement ideas, keep those as separate preceding commits.

## Step 7 — Open a pull request

Push the branch and open a PR. The PR title should match the branch intent, e.g.:

```
Prepare X.Y.Z release
```

or

```
Release vX.Y.Z
```

Mention the notable changes and link to the `CHANGELOG.md` diff.

## Step 8 — Post-merge tagging (human step)

After the PR is merged, a **signed tag** must be created on the merge commit:

```bash
git tag -s vX.Y.Z -m "Version X.Y.Z

 * <first change>
 * <second change>"
```

The tag message should list the user-visible changes from the release. Push with `git push origin vX.Y.Z`.

## Quick checklist

- [ ] Branch created from latest `master`
- [ ] `CHANGELOG.md` has new version header with today's date
- [ ] `CHANGELOG.md` comparison links updated
- [ ] `Cargo.toml` version bumped
- [ ] `Cargo.lock` refreshed
- [ ] Committed (single or two-commit pattern)
- [ ] PR opened and merged
- [ ] Signed tag `vX.Y.Z` created on merge commit

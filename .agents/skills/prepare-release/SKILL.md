---
name: prepare-release
description: Prepare a new release for reboot-arch-btw. Use when cutting a release, bumping versions, updating CHANGELOG, or creating a release branch/PR.
---

# Prepare a New Release for reboot-arch-btw

## 1. Determine version

Propose a version based on `CHANGELOG.md` `[Unreleased]` changes (patch/minor/major).

## 2. Create branch

```
git checkout -b release/vX.Y.Z
```

## 3. Update dependencies

```
cargo update
git add Cargo.lock
git commit -m "Update dependencies"
```

**Always do this first.**

## 4. Update CHANGELOG

In `CHANGELOG.md`:

- Replace `## [Unreleased]` with `## [vX.Y.Z] - YYYY-MM-DD`
- Update links at the bottom:
  ```markdown
  [Unreleased]: https://github.com/rnestler/reboot-arch-btw/compare/vX.Y.Z...master
  [vX.Y.Z]: https://github.com/rnestler/reboot-arch-btw/compare/vPREVIOUS..vX.Y.Z
  ```

## 5. Bump version

In `Cargo.toml`: `version = "X.Y.Z"`

```
cargo update --workspace
```

## 6. Optional: tick off ideas

If any `improvement-ideas.md` items were implemented, mark them `✅`.

## 7. Commit

**Multi-commit (preferred):**
```
git add Cargo.toml Cargo.lock && git commit -m "Bump version to X.Y.Z"
git add CHANGELOG.md && git commit -m "Prepare CHANGELOG for X.Y.Z release"
```

**Single commit (tiny patch releases only):**
```
git add CHANGELOG.md Cargo.toml Cargo.lock
git commit -m "Prepare CHANGELOG and bump version for X.Y.Z release"
```

## 8. Open PR

Push and open PR titled `Release vX.Y.Z` or `Prepare X.Y.Z release`.

## 9. Tag after merge

```bash
git tag -s vX.Y.Z -m "Version X.Y.Z

 * <change 1>
 * <change 2>"
git push origin vX.Y.Z
```

## Checklist

- [ ] Branch from latest `master`
- [ ] `cargo update` committed
- [ ] `CHANGELOG.md`: version header + date + links
- [ ] `Cargo.toml`: version bumped
- [ ] `Cargo.lock`: refreshed
- [ ] Committed
- [ ] PR opened and merged
- [ ] Signed tag `vX.Y.Z` pushed

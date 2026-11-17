---
name: prepare-release
description: Prepare a new release for reboot-arch-btw. Use when cutting a release, bumping versions, updating CHANGELOG, or creating a release branch/PR.
---

# Prepare a New Release for reboot-arch-btw

## 1. Check git history

```
git log $(git describe --tags --abbrev=0)..HEAD --oneline --no-merges
```

Add any missing user-facing changes to `## [Unreleased]` in `CHANGELOG.md`:
```
 * Description (#<pr>)
```

## 2. Determine version

Based on the now-complete `## [Unreleased]` section, propose patch/minor/major.

## 3. Create branch

```
git checkout -b prepare-X.Y.Z-release
```

## 4. Update dependencies

```
cargo update
git add Cargo.lock
git commit -m "Update dependencies"
```

## 5. Update CHANGELOG

- Replace `## [Unreleased]` with `## [vX.Y.Z] - YYYY-MM-DD`
- Update links at the bottom:
  ```markdown
  [Unreleased]: https://github.com/rnestler/reboot-arch-btw/compare/vX.Y.Z...master
  [vX.Y.Z]: https://github.com/rnestler/reboot-arch-btw/compare/vPREVIOUS..vX.Y.Z
  ```

```
git add CHANGELOG.md && git commit -m "Prepare CHANGELOG for X.Y.Z release"
```

## 6. Bump version

In `Cargo.toml`: `version = "X.Y.Z"`

```
cargo update --workspace
git add Cargo.toml Cargo.lock && git commit -m "Bump version to X.Y.Z"
```

## 7. Optional: tick off ideas

If any `improvement-ideas.md` items were implemented, mark them `✅`.

## 8. Open PR (by user)

Ask user to push and open PR titled `Release vX.Y.Z` or `Prepare X.Y.Z release`.

## 9. Tag after merge (by user)

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

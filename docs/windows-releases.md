# Windows Releases

This repository publishes downloadable Windows builds through GitHub Releases.

## What gets published

- `rusty-breakout-windows-x64.zip`
  - Portable build
  - Unzip anywhere and run `breakout.exe`
- `rusty-breakout-setup-x64.exe`
  - Installer build created with Inno Setup
  - Adds Start Menu shortcuts, optional desktop shortcut, and uninstall support

The packaged release assets include only the game executable plus static project docs such as `README.md` and `LICENSE`.
Runtime save data under `settings/` is created by the game after launch and is not bundled into release assets.

## Release Workflow

GitHub Actions release automation lives in [`.github/workflows/release.yml`](/E:/Code/rusty-breakout/.github/workflows/release.yml).

- Push/PR validation remains in `ci.yml`
- Windows packaging runs only for:
  - version tags like `v0.1.0`
  - manual `workflow_dispatch` runs
- Tagged builds publish assets directly to GitHub Releases
- Manual runs produce workflow artifacts for dry runs without publishing a release

## How To Publish

1. Make sure `main` is in the state you want to ship.
2. Create and push a version tag such as `v0.1.0`.
3. Wait for the `Release` workflow to finish on GitHub Actions.
4. Verify the GitHub Release contains:
   - the portable zip
   - the installer
5. Test both assets on a clean Windows machine before announcing the release.

Example:

```powershell
git tag v0.1.0
git push origin v0.1.0
```

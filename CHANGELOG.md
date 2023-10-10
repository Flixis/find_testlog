# Changelog

All notable changes to this project will be documented in this file.

## [2.0.1]


### Fix
- Reverted back to old algorithm
- Updated versioning

## [2.0.0]

### Feat

- Initial version of GUI with Tauri
- Changed logo to something fresh
- V2.0.0


### Add

- All new GUI using Tauri framework
- CLI integrations backwards compatible
- All new searching algorithm

### Bug Fixes

- Changed order of items to align everywhere
- CLI parsed items are now stripped of garbage
- Use appconfig::defaults_values() instead of creating a new struct
- Program now exits correctly, fixed exit codes
- Cleanup in tauri.conf

### Refactor

- V2.1.0 -> New search algorithm #13 #14
- Entire codebase refactored
- Changing name conventions to something more readable

<!-- generated by git-cliff -->
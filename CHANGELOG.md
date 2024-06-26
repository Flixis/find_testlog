# Changelog

All notable changes to this project will be documented in this file.

## [V4.3.1] - 2024-05-20

### Reverted
- 'ABORT' status is now the displayed with a '⚠️' icon next to 'testtype'.
- 'SERVICE' mode is now displayed as 'ORANGE' again 

### Changed
- 'Partial' test are now displayed with a '🔧' next to 'testtype'. #74 (Partial test should have an indicator).
- Time to results now shows "🔥" when search takes less than 3 second.

### Refactor
- Rewrite 'create_header_hashmap_from_headers_string'.

### Fix
- #76 (SN/PN field are case sensitive)
- #75 (automatic hypening isn't working in 4.3.0)


## [V4.3.0] - 2024-04-22

#### 4.3.0 vs 4.2.1 Wow, so much faster!
![speedup](/4_3_0vs4_2_1.png)

### Change
- 'ABORT' status is now the orange colour, additionally it shows a '⚠️' next to 'testtype'.
- 'SERVICE' mode is now displayed as '🔧' next to 'testtype'.
- Time to results now shows "🔥" when search takes less than 1 second.

### Refactor
- Rewrite 'extract_info_from_log'.
- main.js

### Chore
- Dependency, whoami -> fallible.
- Removed unreachable code.

### Fix
- #68 (Fallback should report on which file it was used on)
- #69 (ABORT status not implemented)

## [V4.2.2] - 2024-02-17

### Fix
- #66 (Standalone does not work)

## [V4.2.1] - 2024-02-17

### Add
- `pn` and `sn` inputs now automatically add `-` to the string. 
- Added tutorial button for barcode scanning.

### Fix
- #61 (package jquery with app)
- #62 (automatically add "-" when typing sn or pn)
- #63 (Reduce app size)

## [V4.2.0] - 2024-01-20

### Fix
- App logs no longer overwritten if exsiting.

### Change
- `extract_info_from_log` no longer reads last line equal to `text_keep_amount` instead its fixed to 4 lines.
- Updated readme image to reflect changes.

### Removed
- `.dotenv` Removed as dependency.

## [V4.1.0] - 2023-12-17

### Add
- GUI colours have been adjusted.
- Legenda for colours.
- GUI new error dialog.
- Version checking has been updated.
- DOC strings have been updated.

### Refactor
- More code refactors in the background.

### Misc.
- Dependencies bump.

## [V4.0.1] - 2023-12-09

### Fix
- Openlog button not working in windows.
- logger not working as intended.

## [V4.0.0] - 2023-12-09

### New GUI features
- GUI now has colour for PASS or FAIL status.
- GUI now has "Time to results:" at top of page.
- GUI now trims leading and trailing spaces.
- Datepicker can now be cleared using backspace/delete.

### NEW App logging
- Added crate 'simplelog'.
- Application now logs messages to logfile.

### Refactor
- Code is now split into multiple files with distinguishable names.
- Rewrote 'parse_frontend_search_data'.
- Rewrote 'extract_info_from_log'.
- Unwrap calls removed in favor of error handling.
- Windows dependencies now only build with Windows.


## [V3.0.2] - 2023-10-31

### Fix again...
- #40

## [V3.0.0] - 2023-10-29


### Feat GUI

- GUI now has colums for `Revision` & `Test-id`. 
- Can now search on `test type`
- Sorting indicator

### IMPORTANT NOTE
CLI IS NO LONGER SUPPORTED FEATURES AVAILABLE IN V3.0.0 ARE NOT IN CLI!.
- Added warning for CLI about deprecated feature.

## [V2.4.0] - 2023-10-27

### Add

- Update checker implemented
- Github actions

### Change

- SN -> CLNT
- testenv -> testype
- get `get_test_env_string` now pulls info from file

## [V2.3.2] - 2023-10-20

### Add

- Tables are now sortable

### Change

- Flipped option for testenv

## [V2.3.1] - 2023-10-15

### Remove

- Removed -O from command line arguments

## [V2.3.0] - 2023-10-15

### Feat

- Clap implementation done
- You can now pick from list when using CLI commands

### add

- New icon 
- removed threadsleep before opening GUI

## [V2.2.1] - 2023-10-12

### Fix
- Loading bar not working when clicking seach button
- Loading bar finishing before search completion.
- Updated versioning

## [V2.2.0] - 2023-10-12

### Fix
- [#20](https://github.com/Flixis/find_testlog/issues/20) Fixed
- Loading bar fixed
- Updated versioning


## [2.0.1]


verted back to old algorithm
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
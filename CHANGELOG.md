# 0.6.0

- Database and configuration now lives in separate files, which means you need to move all your notes manually.
- Configuration file now uses TOML.
- Database file now uses JSON.
- `copy` and `insert` commands removed.
- `record` command renamed to `add` 
- Use file name as note name when adding file to database with `add` command.
- `edit` command renamed to `open`.
- Added `template` command for changing note name template.
- Reorder commands in `help`.
- Fixed incorrect display of icons.
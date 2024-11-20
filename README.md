# wiedzieliscie-backend
## Rules
- Commit names have to be lowercase
- One branch per one feature
## Env
- WIEDZIELISCIE_BACKEND_RESET_DB - if set to "false" or "1" it resets the database on startup
- ROCKET_CLI_COLORS - if set to "off" or "0" it disables colors and emoji in rocket's logs
## Testing
```
WIEDZIELISCIE_BACKEND_RESET_DB=1 ROCKET_CLI_COLORS=0 cargo test
```

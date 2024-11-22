# wiedzieliscie-backend
## Rules
- Commit names have to be lowercase
- One branch per one feature
## Env
- WIEDZIELISCIE_BACKEND_RESET_DB - if set to "true" or "1" it resets the database on startup
- ROCKET_CLI_COLORS - if set to "off" or "0" it disables colors and emoji in rocket's logs
- WIEDZIELISCIE_BACKEND_FROM_MAIL - the email addres we are sending from (mandatory)
- RESEND_API_KEY - resend api key
- WIEDZIELISCIE_BACKEND_URL - duh
## Testing
```
ROCKET_CLI_COLORS=0 
RESEND_API_KEY='<api-key>'
WIEDZIELISCIE_BACKEND_FROM_MAIL='<email>'
WIEDZIELISCIE_BACKEND_URL='localhost:8000'
cargo test -- --test-threads=1
```

# wiedzieliscie-backend
## Rules
- Commit names have to be lowercase
- One branch per one feature
## .env
- WIEDZIELISCIE_BACKEND_RESET_DB - if set to "true" or "1" it resets the database on startup
- ROCKET_CLI_COLORS - if set to "off" or "0" it disables colors and emoji in rocket's logs
- WIEDZIELISCIE_BACKEND_FROM_MAIL - the email addres we are sending from (mandatory)
- RESEND_API_KEY - resend api key
- WIEDZIELISCIE_BACKEND_URL - duh
- WIEDZIELISCIE_BACKEND_SECRET - jwt secret in base64
- WIEDZIELISCIE_BACKEND_KEIN_MAIL - set to 1 to remove email spam from testing
## Testing
```
python tests/main.py
```

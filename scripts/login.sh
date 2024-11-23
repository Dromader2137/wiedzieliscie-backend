#!/bin/bash

path="$1/auth/login"

curl -s --location --request POST "$path" \
--header 'Content-Type: application/json' \
--data-raw '{
	"email": "'"$2"'",
	"plaintext_password": "'"$3"'"
}'

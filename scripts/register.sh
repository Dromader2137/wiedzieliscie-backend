#!/bin/bash

path="$1/auth/register"

curl -s --location --request POST "$path" \
--header 'Content-Type: application/json' \
--data-raw '{
	"email": "'"$2"'",
	"plaintext_password": "'"$3"'",
	"first_name": "'"$4"'",
	"last_name": "'"$5"'",
	"gender": "'"m"'"
}'

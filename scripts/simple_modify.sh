#!/bin/bash

path="$1/user/modify/$2"

curl -s --location --request POST "$path" \
--header 'Content-Type: application/json' \
--data-raw '{
	"jwt": "'"$3"'",
	"new_value": "'"$4"'",
	"account_id": "'"$5"'"
}'

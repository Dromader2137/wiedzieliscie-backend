#!/bin/bash

path="$1/auth/retrieve_user"

curl -s --location --request POST "$path" \
--header 'Content-Type: application/json' \
--data-raw '{
	"jwt": "'"$2"'"
}'

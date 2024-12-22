#!/bin/bash

path="$1/admin/characters/get"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
	"jwt": "'"$2"'"
}'

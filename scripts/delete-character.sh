#!/bin/bash

path="$1/admin/characters/delete"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
	"jwt": "'"$2"'",
  "character_id": '$3'
}'

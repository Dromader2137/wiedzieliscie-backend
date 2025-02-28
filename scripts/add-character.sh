#!/bin/bash

path="$1/admin/characters/add"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
	"jwt": "'"$2"'",
	"name": "'"$3"'",
	"short_description": "'"$4"'",
	"full_description": "'"$5"'",
	"image": "'"$6"'"
}'

#!/bin/bash

path="$1/admin/dialogues/get/unused"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
	"jwt": "'"$2"'"
}'

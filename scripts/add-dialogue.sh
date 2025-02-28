#!/bin/bash

path="$1/admin/dialogues/add"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
	"jwt": "'"$2"'",
	"name": "'"$3"'",
  "is_skippable": '$4',
  "parts": [
  ['$5', "'"$6"'"]
  ]
}'

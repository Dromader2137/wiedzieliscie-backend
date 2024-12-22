#!/bin/bash

path="$1/admin/dialogues/delete"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
"jwt": "'"$2"'",
"dialogue_id": '$3'
}'

#!/bin/bash

path="$1/admin/tasks/text_answer/add"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
	"jwt": "'"$2"'",
	"name": "'"$3"'",
  "question": "'"$4"'",
  "correct_answers": [
    '$5'
  ]
}'

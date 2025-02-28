#!/bin/bash

path="$1/admin/tasks/multiple_choice/add"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
	"jwt": "'"$2"'",
	"name": "'"$3"'",
  "question": "'"$4"'",
  "answers": [
    '$5'
  ],
  "correct_answers": [
    '$6'
  ]
}'

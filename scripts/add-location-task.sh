#!/bin/bash

path="$1/admin/tasks/location/add"

curl -s --location --request POST "$path" \
  --header 'Content-Type: application/json' \
  --data-raw '{
	"jwt": "'"$2"'",
	"name": "'"$3"'",
  "min_radius": '$4',
  "max_radius": '$5'
}'

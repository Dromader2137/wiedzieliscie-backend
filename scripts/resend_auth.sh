#!/bin/bash

path="$1/auth/resend_verification/$2"

curl -s --location --request POST "$path" \
--header 'Content-Type: application/json' \
--data-raw '{}'

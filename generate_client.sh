#! /bin/sh

openapi-generator generate -i spec.yml -g rust -o ./plaid-generated-openapi -t custom_template \
--additional-properties=packageVersion=0.0.1

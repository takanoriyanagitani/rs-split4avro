#!/bin/sh

input=./sample.d/input.avro

genavro(){
	echo generating input...

	export ENV_SCHEMA_FILENAME=./sample.d/input.avsc

	cat sample.d/input.jsonl |
		json2avrows |
		cat > "${input}"
}

test -f "${input}" || genavro

export ENV_SPLIT_COUNT=2
export ENV_OUTPUT_DIR_NAME=./sample.d/output.d

mkdir -p "${ENV_OUTPUT_DIR_NAME}"

cat "${input}" | ./rs-split4avro

ls -l "${input}"
ls -l ./sample.d/output.d/*.avro

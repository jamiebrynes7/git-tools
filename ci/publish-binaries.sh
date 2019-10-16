#!/bin/bash

# Ensure that the GITHUB_TOKEN secret is included
if [[ -z "$GITHUB_TOKEN" ]]; then
  echo "Set the GITHUB_TOKEN env variable."
  exit 1
fi

# Ensure that there is a pattern specified.
if [[ -z "$1" ]]; then
    echo "Missing file (pattern) to upload."
    exit 1
fi

# Only upload to non-draft releases
IS_DRAFT=$(jq --raw-output '.release.draft' "$GITHUB_EVENT_PATH")
if [ "$IS_DRAFT" = true ]; then
    echo "**************************************************"
    echo "  This is a draft-release, so we will do nothing"
    echo "**************************************************"
    exit 0
fi

# Prepare the headers for our curl-command.
AUTH_HEADER="Authorization: token ${GITHUB_TOKEN}"

# Create the correct Upload URL.
RELEASE_ID=$(jq --raw-output '.release.id' "$GITHUB_EVENT_PATH")

# For each matching file..
for file in $*; do

    echo "Processing file ${file}"

    if [ ! -e "$file" ]; then
        echo "*****************************"
        echo " file not present - skipping."
        echo "*****************************"
        continue
    fi

    if [ ! -s "$file" ]; then
        echo "**************************"
        echo " file is empty - skipping."
        echo "**************************"
        continue
    fi


    FILENAME=$(basename "${file}")

    UPLOAD_URL="https://uploads.github.com/repos/${GITHUB_REPOSITORY}/releases/${RELEASE_ID}/assets?name=${FILENAME}"
    echo "Upload URL is ${UPLOAD_URL}"

    # Generate a temporary file.
    tmp=$(mktemp)

    # Upload the artifact - capturing HTTP response-code in our output file.
    response=$(curl \
        -sSL \
        -XPOST \
        -H "${AUTH_HEADER}" \
        --upload-file "${file}" \
        --header "Content-Type:application/octet-stream" \
        --write-out "%{http_code}" \
        --output $tmp \
        "${UPLOAD_URL}")

    # If the curl-command returned a non-zero response we must abort
    if [ "$?" -ne 0 ]; then
        echo "**********************************"
        echo " curl command did not return zero."
        echo " Aborting"
        echo "**********************************"
        cat $tmp
        rm $tmp
        exit 1
    fi

    # If upload is not successful, we must abort
    if [ $response -ge 400 ]; then
        echo "***************************"
        echo " upload was not successful."
        echo " Aborting"
        echo " HTTP status is $response"
        echo "**********************************"
        cat $tmp
        rm $tmp
        exit 1
    fi

    # Show pretty output, since we already have jq
    cat $tmp | jq .
    rm $tmp

done
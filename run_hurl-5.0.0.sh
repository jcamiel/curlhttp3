#!/usr/bin/env bash
set -Eeuo pipefail

# Check if the correct number of arguments is provided
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <url> <number_of_times> <sleep_interval_in_seconds>"
    exit 1
fi

HURL=hurl/target/release/hurl

$HURL --version
ldd $HURL

# Assign arguments to variables
URL=$1
N=$2
SLEEP_INTERVAL=$3


# Loop N times
for (( i=1; i<=N; i++ ))
do
    echo "Request #$i:"

    # Perform the curl request and display the HTTP status code and time
    echo "HEAD $URL" | $HURL --max-time 20 --http3 --very-verbose
    echo "Pinto ret=$?"

    # Sleep between requests if not the last request
    if [ "$i" -lt "$N" ]; then
        sleep "$SLEEP_INTERVAL"
    fi
done

echo "Completed $N requests to $URL."
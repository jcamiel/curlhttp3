#!/usr/bin/env bash
set -Eeuo pipefail

# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <number_of_times> <sleep_interval_in_seconds>"
    exit 1
fi

# Assign arguments to variables
N=$1
SLEEP_INTERVAL=$2


# Loop N times
for (( i=1; i<=N; i++ ))
do
    echo "Request #$i:"

    ./main

    # Sleep between requests if not the last request
    if [ "$i" -lt "$N" ]; then
        sleep "$SLEEP_INTERVAL"
    fi
done

echo "Completed $N requests."
#!/usr/bin/env bash
set -Eeuo pipefail

# Check if the correct number of arguments is provided
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <url> <number_of_times> <sleep_interval_in_seconds>"
    exit 1
fi

curl --version
ldd /usr/bin/curl

# Assign arguments to variables
URL=$1
N=$2
SLEEP_INTERVAL=$3


# Loop N times
for (( i=1; i<=N; i++ ))
do
    echo "Request #$i:"

    # Perform the curl request and display the HTTP status code and time
    curl --verbose --max-time 20 --head --http3 -s -o /dev/null -w "HTTP Status: %{http_code}, Time: %{time_total} s\n" "$URL"

    # Sleep between requests if not the last request
    if [ "$i" -lt "$N" ]; then
        sleep "$SLEEP_INTERVAL"
    fi
done

echo "Completed $N requests to $URL."
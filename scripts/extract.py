#!/usr/bin/env python3

import sys
import json
import pytz
from datetime import datetime, timezone

def convert_timestamp(unix_timestamp, from_tz, to_tz):
    """
    Convert a UNIX timestamp from one timezone to another.
    """
    # Convert UNIX timestamp to a datetime object in the 'from' timezone
    dt = datetime.fromtimestamp(int(unix_timestamp), from_tz)

    # Convert the datetime object to the 'to' timezone
    dt_to = dt.astimezone(to_tz)

    # Return the UNIX timestamp of the converted datetime
    return int(dt_to.timestamp())

def main():
    if len(sys.argv) < 2:
        print("Usage: extract.py <EVENT_ID> <OUTCOME_ID>")
        sys.exit(1)

    event_id = sys.argv[1]
    outcome_id = sys.argv[2]
    output_filename = f"./generated/{event_id}-{outcome_id}.tsv"

    # Define timezones (adjust as needed)
    from_timezone = pytz.UTC  # Assuming the original timestamps are in UTC
    to_timezone = pytz.timezone('America/New_York')  # Example: converting to Eastern Time

    with open(f"output-{event_id}.tsv", "r") as infile, open(output_filename, "w") as outfile:
        # Write the header to the CSV
        outfile.write("timestamp\timplied_probability\tamerican\thandicap\n")

        # Iterate through each line of the input
        for line in infile:
            parts = line.strip().split('\t')
            if "PriceEvent" in line and outcome_id in line:
                try:
                    # Convert timestamp from one timezone to another
                    converted_timestamp = convert_timestamp(parts[1], from_timezone, to_timezone)

                    # Try to extract the decimal price from the JSON string
                    event = json.loads(parts[3])

                    # Default handicap value
                    handicap = 0

                    # Update handicap if it exists in the data
                    if "handicap" in event["price"]:
                        handicap = float(event["price"]["handicap"])

                    decimal_price = float(event["price"]["decimal"])
                    american_price = float(event["price"]["american"])
                    implied_probability = round(1 / decimal_price, 2)

                    # Write the processed data to the output file
                    outfile.write(f"{converted_timestamp}\t{implied_probability}\t{american_price}\t{handicap}\n")
                except (IndexError, ValueError, KeyError, TypeError) as e:
                    # Handling potential JSON parsing errors, or missing keys
                    continue

if __name__ == "__main__":
    main()

#!/usr/bin/python3

import sys
import json

def main():
    if len(sys.argv) < 2:
        print("Usage: extract.py <EVENT_ID> <OUTCOME_ID>")
        sys.exit(1)

    event_id = sys.argv[1]
    outcome_id = sys.argv[2]
    output_filename = f"./generated/{event_id}-{outcome_id}.tsv"

    with open(f"output-{event_id}.tsv", "r") as infile, open(output_filename, "w") as outfile:
        # Write the header to the CSV
        outfile.write("timestamp\timplied_probability\thandicap\n")

        # Iterate through each line of the input
        for line in infile:
            parts = line.strip().split('\t')
            # Check if the current line matches our conditions
            if "PriceEvent" in line and outcome_id in line:
                try:
                    timestamp = parts[1]
                    # Try to extract the decimal price from the JSON string
                    event = json.loads(parts[3])

                    if "handicap" in event["price"]:
                        decimal_price = float(event["price"]["decimal"])
                        handicap = float(event["price"]["handicap"])
                        implied_probability = round(1 / decimal_price, 2)
                        outfile.write(f"{timestamp}\t{implied_probability}\t{handicap}\n")
                    else:
                        decimal_price = float(event["price"]["decimal"])
                        implied_probability = round(1 / decimal_price, 2)
                        outfile.write(f"{timestamp}\t{implied_probability}\t0\n")
                except (IndexError, ValueError, KeyError, TypeError) as e:
                    # Handling potential JSON parsing errors, or missing keys
                    continue

if __name__ == "__main__":
    main()

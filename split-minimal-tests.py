#!/usr/bin/env python3

"""split-minimal-tests
This script splits HTML minimal-tests, produced by a software called
`json-minimal-tests`, into distinct directories depending on metric differences.

Usage:

./split-minimal-tests.py -i INPUT_DIR -o OUTPUT_DIR [-t MT_THRESHOLD]

NOTE: OUTPUT_DIR is the path to the output directory to be created.
This directory could contain either a series of directories, called as
the metrics that presents differences, or be empty if no metric differences
are found.
MT_THRESHOLD determines the maximum number of considered minimal tests
for a metric.
"""

import argparse
import pathlib
import re
import shutil
import typing as T

# List of metrics
# TODO: Implement a command into rust-code-analysis-cli that returns all
# computed metrics https://github.com/mozilla/rust-code-analysis/issues/478
METRICS = [
    "cognitive",
    "sloc",
    "ploc",
    "lloc",
    "cloc",
    "blank",
    "cyclomatic",
    "halstead",
    "nom",
    "nexits",
    "nargs",
]


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="split-minimal-tests",
        description="This tool splits HTML minimal-tests, produced by "
        "a software called `json-minimal-tests`, into distinct directories "
        "depending on metric differences.",
        epilog="The source code of this program can be found on "
        "GitHub at https://github.com/mozilla/rust-code-analysis",
    )

    # Arguments
    parser.add_argument(
        "--input",
        "-i",
        type=lambda value: pathlib.Path(value),
        required=True,
        help="Input directory containing HTML minimal tests.",
    )

    parser.add_argument(
        "--output",
        "-o",
        type=lambda value: pathlib.Path(value),
        required=True,
        help="Path to the output directory.",
    )

    # Optional arguments
    parser.add_argument(
        "--threshold",
        "-t",
        type=int,
        help="Maximum number of considered minimal tests for a metric.",
    )

    # Parse arguments
    args = parser.parse_args()

    # Create output directory
    args.output.mkdir(parents=True, exist_ok=True)

    # Save files associated to each metric
    metrics_saver: T.Dict[str, T.List] = {metric_name: [] for metric_name in METRICS}

    # Iterate over the files contained in the input directory
    for path in args.input.glob("*.html"):
        # Open a file
        with open(path) as f:
            # Read a file
            file_str = f.read()

            # Remove all code inside <pre></pre> tags
            file_no_pre = re.sub(r"<pre>(.|\n)*?<\/pre>", "", file_str)

            # Iterate over metrics
            for metric_name, metric_files in metrics_saver.items():
                # Check if there is a metric difference in a file
                m = re.search(f"(\.{metric_name})", file_no_pre)

                # If some errors occurred, skip to the next metric
                if m is None:
                    continue

                # Save path if there is a metric difference in a file
                if m.group(1):
                    metric_files.append(path)

    # Iterate over metrics to print them
    for metric_name, metric_files in metrics_saver.items():
        # Create path for metric directory
        metric_path = args.output / metric_name

        if metric_files:
            # Create metric directory
            metric_path.mkdir(parents=True, exist_ok=True)

            # Save the number of files specified in the threshold
            output_paths = (
                metric_files[: args.threshold] if args.threshold else metric_files
            )

            for path in output_paths:
                # Copy files in the directory
                shutil.copy(path, metric_path)


if __name__ == "__main__":
    main()

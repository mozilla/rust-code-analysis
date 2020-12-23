#!/usr/bin/env python3

"""check-submodule
This script checks whether breaking changes could be introduced in
rust-code-analysis code after the update of a tree-sitter-language submodule.
To do so, it compares the differences between the metrics, computed on a
chosen repository, before and after a tree-sitter-language update.


To compute metrics:

./check-submodule.py compute-metrics -u REPO_URL -p LOCAL_DIR -l TREE_SITTER_LANGUAGE

NOTE: The compute-metrics subcommand MUST be run on a clean master branch!

To compute metrics on a continuous integration system:

./check-submodule.py compute-ci-metrics -p LOCAL_DIR -l TREE_SITTER_LANGUAGE


To compare metrics and retrieve minimal tests:

1. Install deepdiff: pip install deepdiff

./check-submodule.py compare-metrics -l TREE_SITTER_LANGUAGE
"""

import argparse
import asyncio
import json
import math
import pathlib
import re
import subprocess
import sys
import typing as T

import deepdiff

# The /tmp directory will be used as workdir
WORKDIR = pathlib.Path("/tmp")
# Suffix for the directory containing the old metrics
OLD_SUFFIX = "-old"
# Suffix for the directory containing the new metrics
NEW_SUFFIX = "-new"

# Extensions parsed by each tree-sitter-language
EXTENSIONS = {
    "tree-sitter-mozjs": ["*.js", "*.js2", "*.jsm"],
    "tree-sitter-javascript": ["*.js", "*.js2"],
    "tree-sitter-tsx": ["*.tsx"],
    "tree-sitter-typescript": ["*.ts", "*.jsw", "*.jsmw"],
    "tree-sitter-java": ["*.java"],
    "tree-sitter-rust": ["*.rs"],
    "tree-sitter-cpp": [
        "*.cpp",
        "*.cx",
        "*.cxx",
        "*.cc",
        "*.hxx",
        "*.hpp",
        "*.c",
        "*.h",
        "*.hh",
        "*.inc",
        "*.mm",
        "*.m",
    ],
    "tree-sitter-python": ["*.py"],
}


class JsonDiff:
    def __init__(
        self,
        old_metrics: T.List[pathlib.Path],
        new_metrics: T.List[pathlib.Path],
        compare_dir: pathlib.Path,
        max_workers: int,
    ):
        self.compare_dir = compare_dir
        self.max_workers = max_workers

        # Max number of file paths in a sublist
        n = math.ceil(len(old_metrics) / max_workers)

        # Assign a certain number of filepaths to each worker
        self.workers_filepaths = [
            zip(old_metrics[i * n : (i + 1) * n], new_metrics[i * n : (i + 1) * n])
            for i in range((len(old_metrics) + n - 1) // n)
        ]

    # Run asynchronous comparisons between json files.
    async def diff(self):
        # Save minimal tests in the chosen directory.
        def _worker(worker_list: T.List[pathlib.Path]):
            for old_filename, new_filename in worker_list:

                # Compute minimal tests
                compute_minimal_tests(old_filename, new_filename, self.compare_dir)

        # Define the max number of coroutines used to compare json files
        await asyncio.gather(
            *(_worker(worker_filepaths) for worker_filepaths in self.workers_filepaths)
        )


# Run a subprocess.
def run_subprocess(cmd: str, *args: T.Union[str, pathlib.Path]) -> None:
    subprocess.run([cmd, *args])


# Run rust-code-analysis on the chosen repository to compute metrics.
def run_rca(
    repo_dir: pathlib.Path,
    output_dir: pathlib.Path,
    manifest_path: T.Optional[pathlib.Path],
    include_languages: T.List[str],
) -> None:
    run_subprocess(
        "cargo",
        "run",
        "--manifest-path",
        manifest_path / "Cargo.toml" if manifest_path else "Cargo.toml",
        "--release",
        "--package",
        "rust-code-analysis-cli",
        "--",
        "--metrics",
        "--output-format=json",
        "--pr",
        "-I",
        *include_languages,
        "-p",
        repo_dir,
        "-o",
        output_dir,
    )


# Find the difference between the two json metric files.
def get_json_diff(
    first_file: pathlib.Path, second_file: pathlib.Path
) -> T.Tuple[T.Dict[str, T.Any], T.Dict[str, T.Any]]:
    with open(first_file, "r") as input_file:
        t1 = json.load(input_file)

    with open(second_file, "r") as input_file:
        t2 = json.load(input_file)

    diff = deepdiff.DeepDiff(t1, t2, ignore_order=True)

    return (t1, diff)


# Save the filename and the list of code spans associated to the differences
# in a dictionary.
def get_metrics_diff_span(
    first_json: T.Dict[str, T.Any], diff: T.Dict[str, T.Any]
) -> T.Dict[str, T.List[T.Tuple[int, int]]]:
    # Search for this pattern in the differences object
    prog = re.compile(r"\['spaces'\]\[\d+\]")

    output = {"name": first_json["name"], "spaces_spans": []}

    for value in diff["values_changed"]:
        val = "".join(prog.findall(value))
        # Subtracting one because files starts from 0
        start_line = eval(f'first_json{val}["start_line"]') - 1
        end_line = eval(f'first_json{val}["end_line"]')
        output["spaces_spans"].append((start_line, end_line))

    # Print the path of the repository file containing the differences
    print(first_json["name"])

    return output


# Dump minimal tests code in an output file.
def dump_minimal_tests(
    code_spans_object: T.Dict[str, T.List[T.Tuple[int, int]]],
    new_filename: pathlib.Path,
    compare_dir: pathlib.Path,
) -> None:
    # Remove duplicates from the list of spans
    spans_list = dict.fromkeys(code_spans_object["spaces_spans"])

    # Get filename
    filename = code_spans_object["name"]

    # Read code spans from the input source code
    with open(filename, "r", encoding="utf-8", errors="ignore") as input_file:
        # Decode only utf-8 source code files
        lines = input_file.readlines()

    # Write spans to output file
    output_path = compare_dir / new_filename.stem
    with open(output_path, "w") as output_file:
        for span in spans_list:
            output_file.write("Minimal test:\n")
            output_file.write("".join(lines[span[0] : span[1]]) + "\n")


# Compute minimal tests.
def compute_minimal_tests(
    old_filename: pathlib.Path, new_filename: pathlib.Path, compare_dir: pathlib.Path
) -> None:
    # Find the difference between the two json files with the aim of
    # getting some minimal tests
    first_json, diff = get_json_diff(old_filename, new_filename)

    # If two json files are identical, return
    if not diff:
        return

    # Retrieve the code spans associated to the differences
    code_spans_object = get_metrics_diff_span(first_json, diff)

    # Dump the minimal tests retrived from code spans on a file with the
    # same extension of the analyzed source code
    dump_minimal_tests(code_spans_object, new_filename, compare_dir)


# Save json files of differences and minimal tests in the chosen directory
# concurrently.
def save_diff_files(
    old_dir: pathlib.Path, new_dir: pathlib.Path, compare_dir: pathlib.Path
) -> None:
    # Get all metric files in old and new directories
    old_paths = sorted(pathlib.Path(old_dir).glob("*.json"))
    new_paths = sorted(pathlib.Path(new_dir).glob("*.json"))

    # Create a new coroutines handler
    json_diff = JsonDiff(old_paths, new_paths, compare_dir, 4)

    # Find the differences between json files and save the results in a
    # chosen directory asynchronously
    asyncio.run(json_diff.diff())


# Compute continuous integration metrics before and after a
# tree-sitter-language update.
def compute_ci_metrics(args: argparse.Namespace) -> None:

    if args.language not in EXTENSIONS.keys():
        print(args.language, "is not a valid tree-sitter-language")
        sys.exit(1)

    # Repository passed as input
    repo_dir = pathlib.Path(args.path)

    # Create rust-code-analysis repository path
    rca_path = WORKDIR / "rust-code-analysis"

    # Old metrics directory
    old_dir = WORKDIR / (args.language + OLD_SUFFIX)
    # New metrics directory
    new_dir = WORKDIR / (args.language + NEW_SUFFIX)

    # Create output directories
    old_dir.mkdir(parents=True, exist_ok=True)
    new_dir.mkdir(parents=True, exist_ok=True)

    # Git clone rust-code-analysis master branch repository
    print(f"Cloning rust-code-analysis master branch into /tmp")
    run_subprocess(
        "git",
        "clone",
        "--depth=1",
        "--recurse-submodules",
        "-j8",
        "https://github.com/mozilla/rust-code-analysis",
        rca_path,
    )

    # Compute old metrics
    print("\nComputing metrics before the update and saving them in", old_dir)
    run_rca(repo_dir, old_dir, rca_path, EXTENSIONS[args.language])

    # Update tree-sitter-language submodule
    print("\nUpdate", args.language)
    run_subprocess("./update-language-bindings.sh")

    # Compute new metrics
    print("\nComputing metrics after the update and saving them in", new_dir)
    run_rca(repo_dir, new_dir, None, EXTENSIONS[args.language])


# Compute metrics before and after a tree-sitter-language update.
def compute_metrics(args: argparse.Namespace) -> None:

    if args.language not in EXTENSIONS.keys():
        print(args.language, "is not a valid tree-sitter-language")
        sys.exit(1)

    # Repository local directory
    repo_dir = WORKDIR / args.path
    # Old metrics directory
    old_dir = WORKDIR / (args.language + OLD_SUFFIX)
    # New metrics directory
    new_dir = WORKDIR / (args.language + NEW_SUFFIX)

    # Create output directories
    old_dir.mkdir(parents=True, exist_ok=True)
    new_dir.mkdir(parents=True, exist_ok=True)

    # Skip if only new metrics are requested
    if not args.only_new:

        # Git clone the chosen repository
        # Note: no submodules repositories are accepted
        print(f"Cloning {args.url} into {repo_dir}")
        run_subprocess("git", "clone", "--depth=1", args.url, repo_dir)

        # Compute old metrics
        print("\nComputing metrics before the update and saving them in", old_dir)
        run_rca(repo_dir, old_dir, None, EXTENSIONS[args.language])

        # Create a new branch
        print("\nCreate a new branch called", args.language)
        run_subprocess("git", "checkout", "-B", args.language)

        # Update tree-sitter-language submodule
        print("\nUpdate", args.language)
        run_subprocess("./update-submodule.sh", args.language)

    # Compute new metrics
    print("\nComputing metrics after the update and saving them in", new_dir)
    run_rca(repo_dir, new_dir, None, EXTENSIONS[args.language])


# Compare metrics and dump the differences whether there are some.
def compare_metrics(args: argparse.Namespace) -> None:
    # Old metrics directory
    old_dir = WORKDIR / (args.language + OLD_SUFFIX)
    # New metrics directory
    new_dir = WORKDIR / (args.language + NEW_SUFFIX)

    # Compare metrics directory
    compare_dir = WORKDIR / (args.language + "-compare")

    # Create compare directory
    compare_dir.mkdir(parents=True, exist_ok=True)

    # Save files of differences and minimal tests in the chosen directory
    save_diff_files(old_dir, new_dir, compare_dir)


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="check-submodule",
        description="This tool computes the metrics of a chosen repository "
        "before and after a tree-sitter-language update.",
        epilog="The source code of this program can be found on "
        "GitHub at https://github.com/mozilla/rust-code-analysis",
    )

    # Subcommands parsers
    commands = parser.add_subparsers(help="Sub-command help")

    # Compute metrics command
    compute_metrics_cmd = commands.add_parser(
        "compute-metrics",
        help="Computes the metrics of a chosen repository before and after "
        "a tree-sitter-language update.",
    )

    # Optional arguments
    compute_metrics_cmd.add_argument(
        "--only-new",
        "-n",
        action="store_true",
        help="Only compute the metrics after the tree-sitter-language update",
    )

    # Arguments
    compute_metrics_cmd.add_argument(
        "-u",
        "--url",
        type=str,
        required=True,
        help="URL of the repository used to compute the metrics",
    )

    compute_metrics_cmd.add_argument(
        "-p",
        "--path",
        type=str,
        required=True,
        help="Path where the repository will be saved locally",
    )

    compute_metrics_cmd.add_argument(
        "-l",
        "--language",
        type=str,
        required=True,
        help="tree-sitter-language to be updated",
    )
    compute_metrics_cmd.set_defaults(func=compute_metrics)

    # Compute continuous integration metrics command
    compute_ci_metrics_cmd = commands.add_parser(
        "compute-ci-metrics",
        help="Computes the metrics of a chosen repository before and after "
        "a tree-sitter-language update on a continuous integration system.",
    )

    compute_ci_metrics_cmd.add_argument(
        "-p",
        "--path",
        type=str,
        required=True,
        help="Path where the rust-code-analysis repository is saved on the "
        "continuous integration system",
    )
    compute_ci_metrics_cmd.add_argument(
        "-l",
        "--language",
        type=str,
        required=True,
        help="tree-sitter-language to be updated",
    )

    compute_ci_metrics_cmd.set_defaults(func=compute_ci_metrics)

    # Compare metrics command
    compare_metrics_cmd = commands.add_parser(
        "compare-metrics",
        help="Compares the metrics before and after "
        "a tree-sitter-language update in order to discover whether "
        "there are differences.",
    )

    # Arguments
    compare_metrics_cmd.add_argument(
        "-l",
        "--language",
        type=str,
        required=True,
        help="tree-sitter-language used to compare the metrics",
    )
    compare_metrics_cmd.set_defaults(func=compare_metrics)

    # Parse arguments
    args = parser.parse_args()

    # Call the command
    args.func(args)


if __name__ == "__main__":
    main()

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

To compare metrics and retrieve the structural JSON of differences
in addition to the files containing the minimal tests:

1. Install json-diff from here: https://github.com/Luni-4/json-diff/releases
2. Install json-minimal-tests from here: https://github.com/Luni-4/json-minimal-tests/releases

./check-submodule.py compare-metrics -l TREE_SITTER_LANGUAGE

NOTE: Add the paths of the software above to the PATH environment variable!
"""

import argparse
import pathlib
import subprocess
import sys
import typing as T

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

    # Get JSON of differences
    print("\nSave JSON of differences in", compare_dir)
    run_subprocess(
        "json-structural-diff-cli", "--raw-json", "-o", compare_dir, old_dir, new_dir
    )

    # Get minimal tests
    print("\nSave minimal tests in", compare_dir)
    run_subprocess("json-minimal-tests", "-o", compare_dir, old_dir, new_dir)


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

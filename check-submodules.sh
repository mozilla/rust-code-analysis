#!/bin/bash

# Checks out if a submodule has been updated
SUBMODULES=`git submodule--helper list | awk '{ print $4 }'`
RUN_CI="no"
for SUBMODULE in $SUBMODULES
do
    git diff --exit-code HEAD^ $SUBMODULE
    CHANGED=$? # Get git diff exit code
    if [ $CHANGED -eq 1 ]
    then
        RUN_CI="yes"
        SUBMODULE_NAME=$SUBMODULE
        break
    fi
done

# If no submodule has been updated, exit the script
if [ "$RUN_CI" = "no" ]; then
    exit 0
fi

# Install json minimal tests
JMT_LINK="https://github.com/Luni-4/json-minimal-tests/releases/download"
JMT_VERSION="0.1.6"
curl -L "$JMT_LINK/v$JMT_VERSION/json-minimal-tests-linux.tar.gz" |
tar xz -C $CARGO_HOME/bin

# Download mozilla-central repository
MOZILLA_CENTRAL_REPO="https://github.com/mozilla/gecko-dev"
[ ! -d "/cache/gecko-dev" ] && 
git clone --quiet $MOZILLA_CENTRAL_REPO /cache/gecko-dev || true
pushd /cache/gecko-dev && git pull origin master && popd

# Compute metrics
./check-submodule.py compute-ci-metrics --submodule -p /cache/gecko-dev -l $SUBMODULE_NAME

# Count files in metrics directories
OLD=`ls /tmp/$SUBMODULE_NAME-old | wc -l`
NEW=`ls /tmp/$SUBMODULE_NAME-new | wc -l`

# Print number of files contained in metrics directories
echo "$SUBMODULE_NAME-old: $OLD"
echo "$SUBMODULE_NAME-new: $NEW"

# If metrics directories differ in number of files,
# print only the files that are in a directory but not in the other one
if [ $OLD != $NEW ]
then
    ONLY_FILES=`diff -q /tmp/$SUBMODULE_NAME-old /tmp/$SUBMODULE_NAME-new | grep "Only in"`
    echo "$ONLY_FILES"
fi

# Compare metrics
./check-submodule.py compare-metrics -l $SUBMODULE_NAME

# Create artifacts to be uploaded (if there are any)
COMPARE=/tmp/$SUBMODULE_NAME-compare
if [ "$(ls -A $COMPARE)" ]; then
    # Maximum number of considered minimal tests for a metric
    MT_THRESHOLD=30

    # Array containing the considered metrics
    # TODO: Implement a command into rust-code-analysis-cli that returns all
    # computed metrics https://github.com/mozilla/rust-code-analysis/issues/478
    METRICS=("cognitive" "sloc" "ploc" "lloc" "cloc" "blank" "cyclomatic" "halstead" "nom" "nexits" "nargs")

    # Output directory name
    OUTPUT_DIR=/tmp/output-$SUBMODULE_NAME

    # Create output directory
    mkdir -p $OUTPUT_DIR

    # Retrieve minimal tests for a metric
    for METRIC in "${METRICS[@]}"
    do

        FILES=`grep -r -i -l $METRIC $COMPARE | head -$MT_THRESHOLD`
        if [ -n "$FILES" ]
        then
            mkdir -p $OUTPUT_DIR/$METRIC
            cp $FILES $OUTPUT_DIR/$METRIC
        fi
    done

    tar -czvf /tmp/json-diffs-and-minimal-tests.tar.gz $COMPARE $OUTPUT_DIR
fi

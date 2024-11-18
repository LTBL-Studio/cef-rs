#!/bin/bash

set -e

LIBDIR="/usr/lib"
OUTDIR="$LIBDIR/cef"
mkdir -p $OUTDIR

DL_DIR="./target/cef_download"
mkdir -p "$DL_DIR"

if [ -z ${1} ]; then
    echo "CEF version is not provided";
    exit 1;
fi

echo "Searching CEF $1 binaries..."
build_name=$(curl -s https://cef-builds.spotifycdn.com/index.json -H "Accept: application/json" | jq -r ".linux64.versions[] | select((.cef_version | startswith(\"$1\")) and .channel == \"stable\") | .files[] | select(.type == \"minimal\") | .name")

if [ -z ${build_name} ]; then
    echo "No CEF of version $1 found";
    exit 1;
fi

cd $DL_DIR

if test -f $build_name; then 
    echo "$build_name already downloaded"
else
    echo "Downloading $build_name build..."
    curl --progress-bar "https://cef-builds.spotifycdn.com/$build_name" -O
fi

if test -d "Release" && test -d "Resources"; then 
    echo "$build_name already extracted"
else
    echo "Extracting CEF..."
    tar -xvjf $build_name --strip-components=1 
fi

echo "Copying shared libraries to $OUTDIR..."
sudo cp -r -f "./Release/." $OUTDIR
echo "Copying resources to $OUTDIR..."
sudo cp -r -f "./Resources/." $OUTDIR

echo "Creating symbolic links to /usr/lib..."

sudo ln -f -s $OUTDIR/*.so $LIBDIR
sudo ln -f -s $OUTDIR/*.bin $LIBDIR

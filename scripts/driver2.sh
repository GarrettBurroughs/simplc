#!/bin/sh

# Default behavior is to link
LINK=true

# Parse flags
while getopts "c" opt; do
  case $opt in
    c) LINK=false ;;
    *) echo "Usage: ./run [-c] <file.c>"; exit 1 ;;
  esac
done

# Shift the arguments so $1 is the filename
shift $((OPTIND-1))
SOURCE_FILE=$1

if [ -z "$SOURCE_FILE" ]; then
    echo "Usage: ./run [-c] <file.c>"
    exit 1
fi

BASE_NAME=${SOURCE_FILE%.c}
PREPROCESSED_FILE="${BASE_NAME}.i"
ASSEMBLY_FILE="${BASE_NAME}.s"
OBJECT_FILE="${BASE_NAME}.o"
EXECUTABLE_NAME="$BASE_NAME"

# 1. Preprocess
gcc -E -P "$SOURCE_FILE" -o "$PREPROCESSED_FILE"

# 2. Run your Rust compiler
/home/gburroughs/dev/rust/compiler/target/debug/compiler "$PREPROCESSED_FILE"
if [ $? != 0 ]; then
    exit 1
fi

# 3. Assemble / Link logic
if [ "$LINK" = false ]; then
    # Produce .o file and stop (Chapter 9 requirement)
    gcc -c "$ASSEMBLY_FILE" -o "$OBJECT_FILE"
    echo "Object file created: $OBJECT_FILE"
else
    # Assemble and link into an executable
    gcc "$ASSEMBLY_FILE" -o "$EXECUTABLE_NAME"
    $EXECUTABLE_NAME
    RET=$?
    echo $RET
    echo "Build successful: $EXECUTABLE_NAME"
fi

# Cleanup
rm "$PREPROCESSED_FILE"

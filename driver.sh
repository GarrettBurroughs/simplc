#!/bin/sh

# 1. Grab the source file (the first argument)
SOURCE_FILE=$1

# Basic check to ensure a file was provided
if [ -z "$SOURCE_FILE" ]; then
    echo "Usage: ./run <file.c>"
    exit 1
fi

# 2. Derive filenames (e.g., main.c -> main.s and main)
# ${SOURCE_FILE%.c} removes the .c extension
BASE_NAME=${SOURCE_FILE%.c}
ASSEMBLY_FILE="${BASE_NAME}.s"
EXECUTABLE_NAME="$BASE_NAME"

# 3. Preprocess the C file
# This handles #include and #define (using gcc's preprocessor)
gcc -E -P "$SOURCE_FILE" -o "${BASE_NAME}.i"

# 4. Run your compiler (The core of Chapter 1)
# This converts the preprocessed file (.i) into assembly (.s)
/home/gburroughs/dev/rust/compiler/target/debug/compiler "${BASE_NAME}.i"

# 5. Assemble and Link
# This converts the assembly file into a final executable
gcc "$ASSEMBLY_FILE" -o "$EXECUTABLE_NAME"

# 6. Cleanup (Optional)
# rm "${BASE_NAME}.i" "$ASSEMBLY_FILE"

./$EXECUTABLE_NAME

echo $?
echo "Build successful: $EXECUTABLE_NAME"



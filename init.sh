#!/bin/zsh

NAME=${1}
INDEX=${2}

if [ -z "$NAME" ]; then
    echo "No name provided"
    exit 1
fi

if [ -z "$INDEX" ]; then
    echo "No index provided"
    exit 1
fi

if [[ ! "$INDEX" =~ ^[0-9]{3}$ ]]; then
    echo "Index must be a three digit numbers"
    exit 1
fi

if [ -d "$INDEX" ]; then
    echo "Index dir already exists"
    exit 1
fi

NAME=$(echo "$NAME" | sed -e 's/ /_/g' | tr '[:upper:]' '[:lower:]')

echo "Creating dir $INDEX"
mkdir -p "$INDEX/src"
CARGO_TOML_PATH="$INDEX/Cargo.toml"
MAIN_PATH="$INDEX/src/main.rs"

echo "Creating $CARGO_TOML_PATH"
touch "$CARGO_TOML_PATH"
echo "[package]" >> "$CARGO_TOML_PATH"
echo "name = \"$NAME\"" >> "$CARGO_TOML_PATH"
echo "version = \"0.1.0\"" >> "$CARGO_TOML_PATH"
echo "authors = [\"$USER\"]" >> "$CARGO_TOML_PATH"
echo "edition = \"2021\"" >> "$CARGO_TOML_PATH"
echo "" >> "$CARGO_TOML_PATH"
echo "[dependencies]" >> "$CARGO_TOML_PATH"
echo "proconio = \"0.3.6\"" >> "$CARGO_TOML_PATH"

echo "Creating $MAIN_PATH"
touch "$MAIN_PATH"
echo "use proconio::input;" >> "$MAIN_PATH"
echo "" >> "$MAIN_PATH"
echo "fn main() {" >> "$MAIN_PATH"
echo "    input! {" >> "$MAIN_PATH"
echo "        n: usize," >> "$MAIN_PATH"
echo "    }" >> "$MAIN_PATH"
echo "    println!(\"{}\", n);" >> "$MAIN_PATH"
echo "}" >> "$MAIN_PATH"


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
cat <<EOF > "$CARGO_TOML_PATH"
[package]
name = "$NAME"
version = "0.1.0"
authors = ["$USER"]
edition = "2021"

[dependencies]
proconio = "0.3.6"
EOF

echo "Creating $MAIN_PATH"
touch "$MAIN_PATH"
cat <<EOF > "$MAIN_PATH"
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", n);
}
EOF

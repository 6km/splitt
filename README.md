# splitt

A command-line tool for splitting text in your terminal.

```
Usage: splitt [OPTIONS] --separator <SEPARATOR> [TEXT]

Arguments:
  [TEXT]  The text to split. If not provided, the tool reads from standard input (stdin)

Options:
  -s, --separator <SEPARATOR>  The separator used to split the text
      --keep-empty             Keep empty strings
      --raw                    Output the results as a raw array of strings
  -c, --chunked                The output is printed in chunks; which means each part is printed separately
  -h, --help                   Print help
  -V, --version                Print version
```

## Examples

```bash
# Providing the text directly.
splitt "1-23-4" -s "-"
# 1
# 23
# 4

# Pass output of another command and print normal output.
pwd | splitt -s "/"
# home
# taha
# projects

# Pass output of another command and print a raw array.
pwd | splitt -s "/" --raw
# ["home", "taha", "projects"]
```

## Installation

You can install `splitt` using one of the following methods:

### 1. Building from Source (requires rust)

If you have Rust and Cargo installed, you can build `splitt` from source.

1.  **Clone the repository:**

    ```bash
    git clone github.com/6km/splitt.git
    cd splitt
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

3.  **Install the executable:**

    ```bash
    # Linux/macOS
    cp target/release/splitt /usr/local/bin/

    ## NOTE: it might be different for your operating system
    ```

### 2. GitHub Releases

Pre-built binaries are available on the [GitHub Releases page](https://github.com/6km/splitt/releases).

1.  **Download the binary**

2.  **Make the binary executable (Linux/macOS):**

    ```bash
    chmod +x splitt
    ```

3.  **Move the binary to a directory in your PATH:**

    ```bash
    # Linux/macOS
    mv splitt /usr/local/bin/

    ## NOTE: it might be different for your operating system
    ```

### 3. Using Cargo

You can install directly with cargo.

```bash
cargo install splitt
```

### 4. Using `stew`

You can use `stew` to install the pre-built GitHub releases binaries.

```bash
stew install 6km/splitt
```

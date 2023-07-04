# AkaiFetch (afetch)

![Preview](https://akai.needs.rest/r/28451.png)

AkaiFetch (afetch) is a lightweight system information tool written in Rust. It displays system details such as the operating system, shell, desktop environment, and RAM usage in a simple format.

## Installation

To use AkaiFetch, you can download the precompiled executable binary for your platform from the [GitHub Releases](https://github.com/AmazingAkai/afetch/releases) page.

## Installation

### Linux

```bash
sudo wget -O /usr/bin/afetch https://github.com/AmazingAkai/afetch/releases/download/v1.0.0/afetch
sudo chmod +x /usr/bin/afetch
```

### MacOS

```bash
sudo curl -Lo /usr/local/bin/afetch https://github.com/AmazingAkai/afetch/releases/download/v1.0.0/afetch
sudo chmod +x /usr/local/bin/afetch
```


### Windows

Currently, we do not provide a precompiled Windows binary. However, you can build the project from source by following the steps below.

## Build From Source

To build the project from source, you need to have Rust installed on your machine. Follow these steps:

1. Install Rust by following these [instructions](https://rust-lang.org/tools/install).

2. Clone the repository to your local machine:



```bash
git clone https://github.com/AmazingAkai/afetch.git
```

3. Navigate to the project directory:

```bash
cd afetch
```

4. Build the project using Cargo:

```bash
cargo build --release
```

5. After a successful build, you can find the compiled executable in the `target/release/` directory.

Please note that building from source may require additional dependencies to be installed on your system.

## Usage

To run **AkaiFetch**, simply execute the `afetch` command in your terminal:

```bash
afetch
```

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvement, please open an issue on the GitHub repository. Pull requests are also appreciated.
License

## License

This project is licensed under the MIT License.

Feel free to customize the content and replace placeholders (`AmazingAkai` and `afetch`) with the appropriate values for your project. Additionally, update the installation instructions with the correct download links for your releases.

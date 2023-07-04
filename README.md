# AkaiFetch (afetch)

![Preview](https://akai.needs.rest/r/28451.png)

AkaiFetch (afetch) is a lightweight system information tool written in Rust. It displays system details such as the operating system, shell, desktop environment, and RAM usage in a simple format.

## Setting Up Terminal Font

To enhance the visual experience of the application, we recommend using the FiraMono font from the Nerd Fonts collection. Follow these steps to set up the font in your terminal:

1. Download the FiraMono Nerd Font package from the official repository. You can find the package at [FiraMono Nerd Font](https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/FiraMono.zip).

2. Extract the downloaded ZIP file to a location on your computer.

3. Open your terminal application.

4. Access the terminal settings or preferences. The process for accessing terminal settings may vary depending on your terminal emulator.

5. In the terminal settings, locate the option to change the font or font family.

6. Choose the FiraMono Nerd Font from the list of available fonts or click the "Browse" button to locate and select the extracted font files manually.

7. Apply the font changes and close the settings.

Your terminal should now be using the FiraMono font from the Nerd Fonts collection.

Please note that changing the terminal font is optional, and you can continue to use your preferred font if desired.


## Installation

To use AkaiFetch, you can download the precompiled executable binary for your platform from the [GitHub Releases](https://github.com/AmazingAkai/afetch/releases) page.

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

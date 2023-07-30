# Rusty Pandas - Windows Terminal Theme Configuration Tool

Rusty Pandas is a theme configuration tool for Windows Terminal, designed to enhance the user experience by providing an intuitive and straightforward way to customize the appearance of Windows Terminal. It leverages Tauri and Leptos to deliver a powerful and efficient application.

## Features

- Easily configure themes for Windows Terminal.
- Utilizes Tauri and Leptos for a seamless cross-platform experience.
- Fetches dynamic GIFs using the GIPHY API to add a touch of fun to your terminal.

## Getting Started

Follow these instructions to set up the development environment and start using Rusty Pandas on your local machine.

### Prerequisites

Make sure you have the following tools installed:

- [Rust](https://www.rust-lang.org/) - To build and run the project.
- [Cargo](https://crates.io/)
- [Tauri](https://tauri.app/)
- [Trunk](https://trunkrs.dev/) - To install and manage dependencies.
- [Windows Terminal](https://www.microsoft.com/en-us/p/windows-terminal/9n0dx20hk701) - The terminal in which you will apply the custom themes.

### Installation

1. Clone the repository:

```bash
git clone https://github.com/your-username/rusty-pandas.git
cd rusty-pandas
```

2. Install the Tauri dependencies:

```bash
cargo install tauri-cli
```

3. Install the WASM dependencies:

```bash
cargo install trunk
```

3. Add the WASM target to Rust:

```bash
rustup target add wasm32-unknown-unknown
```

### Usage

To start the application, use the following command:

```bash
cargo tauri dev
```

This will launch Rusty Pandas.

## Contributing

We welcome contributions from the community! If you find any issues or want to add new features, please follow these steps:

1. Fork the repository.
2. Create a new branch for your changes.
3. Make your modifications and commit them with descriptive messages.
4. Push the branch to your fork.
5. Create a pull request to the `master` branch of the original repository.

## License

Rusty Pandas is open-source software licensed under the [MIT License](LICENSE).

## Acknowledgments

- The Tauri and Leptos communities for their excellent tools and support.

## Contact

For any questions or inquiries, feel free to reach out to us at `andre.bruns33@gmail.com`.

---
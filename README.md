<p align=center>
  <img src="https://github.com/StormicK/rustypandas/blob/5f46c1ad342aa9e1fb7e201f13db48fc69b1f6d7/src-tauri/icons/red-panda.png" width=150 />
</p>

# Rusty Pandas - Windows Terminal Theme Configuration Tool

Rusty Pandas is a theme configuration tool for Windows Terminal, designed to enhance the user experience by providing an intuitive and straightforward way to customize the appearance of Windows Terminal. It leverages Tauri and SolidJS to deliver a powerful and efficient application.

It is currently in development and not ready for production use. We are working hard to bring you the best possible experience and will release the first version soon. Stay tuned!

## Features

- Fetches dynamic GIFs using the GIPHY API to add a touch of fun to your terminal.
- Color Scheme Picker letting you change the windows terminal color scheme on the fly.

## Getting Started

Follow these instructions to set up the development environment and start using Rusty Pandas on your local machine.

### Prerequisites

Make sure you have the following tools installed:

- [Rust](https://www.rust-lang.org/) - To build and run the project.
- [Cargo](https://crates.io/) - Rusts Package Manager.
- [Tauri](https://tauri.app/) - UI framework for Rust.
- [NPM](https://www.npmjs.com/) - JS Package Manager.
- [Windows Terminal](https://www.microsoft.com/en-us/p/windows-terminal/9n0dx20hk701) - The terminal in which you will apply the custom themes.

### Installation

1. Clone the repository:

```bash
git clone https://github.com/StormicK/rustypandas.git
cd rustypandas
```

2. Install the Tauri dependencies:

```bash
cargo install tauri-cli
```

3. Install the npm dependencies:

```bash
npm install
```

### Usage

To start the tailwindcss, use the following command:

```bash
npx tailwindcss -i ./src/styles.css -o ./dist/styles.css --watch
```

To start the application, use the following command:

```bash
cargo tauri dev
```

This will launch Rusty Pandas.

To build the application, use the following command:

```bash
cargo tauri build
```

This will create a `dist` folder containing the executable. It will also create an installer.

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

- The Tauri and SolidJS communities for their excellent tools and support. I also want to thank the developers of Giphy and TailwindCSS for their amazing APIs and frameworks.

## Contact

For any questions or inquiries, feel free to reach out to us at `andre.bruns33@gmail.com`.

---
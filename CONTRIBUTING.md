# Contributing to Vauxl

First off, thank you for considering contributing to Vauxl! It's people like you that make open source projects thrive. We welcome contributions of all kinds, from reporting bugs and suggesting features to writing code, improving documentation, and helping with testing.

Vauxl aims to be a privacy-focused, open-source, and highly customizable communication platform. Your contributions help us achieve this goal!

## Where to Start?

*   **Reporting Bugs:** If you encounter a bug, please search the [GitHub Issues](https://github.com/Vauxl/vauxl/issues) first to see if it has already been reported. If not, feel free to open a new issue. Please include as much detail as possible: steps to reproduce, expected behavior, actual behavior, your operating system, and Vauxl version (if applicable).
*   **Suggesting Enhancements:** Have an idea for a new feature or an improvement to an existing one? We'd love to hear it! Open an issue with the "enhancement" label, or start a discussion on [GitHub Discussions](https://github.com/Vauxl/vauxl/discussions) to talk it through.
*   **Code Contributions:** If you want to fix a bug or implement a feature, check the issues list for tasks labeled "help wanted" or "good first issue".

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please ensure you read and follow it in all interactions within the community.

## Development Setup

Please refer to the [Getting Started (Development)](README.md#getting-started-development) section in the main `README.md` file for instructions on setting up your development environment.

## Pull Request Process

1.  **Fork the Repository:** Create your own fork of the `Vauxl/vauxl` repository on GitHub.
2.  **Clone Your Fork:** Clone your fork to your local machine: `git clone https://github.com/YOUR_USERNAME/vauxl.git`
3.  **Create a Branch:** Create a new branch for your changes, based on the `main` branch (or `develop` if that becomes the standard): `git checkout -b feature/your-feature-name` or `git checkout -b fix/issue-number`.
4.  **Make Your Changes:** Write your code, add tests, and update documentation as needed.
5.  **Ensure Code Quality:**
    *   Format your Rust code: `cargo fmt --all`
    *   Check for common issues: `cargo clippy --all -- -D warnings` (fix all reported issues)
    *   Format your frontend code (if applicable, using Prettier/ESLint configured in `vauxl-client`).
6.  **Run Tests:** Ensure all existing tests pass: `cargo test --all`
7.  **Commit Your Changes:** Write clear, concise commit messages. Reference relevant issue numbers (e.g., `Fix #123: Correct message ordering`).
8.  **Push to Your Fork:** Push your branch to your fork on GitHub: `git push origin feature/your-feature-name`.
9.  **Open a Pull Request (PR):** Go to the `Vauxl/vauxl` repository on GitHub and open a new Pull Request from your branch to the `Vauxl/vauxl` `main` (or `develop`) branch.
    *   Provide a clear title and description for your PR, explaining the changes and linking to any relevant issues.
    *   Ensure the "Allow edits from maintainers" checkbox is ticked (this helps us fix minor issues quickly).
10. **Code Review:** A maintainer will review your PR. Address any feedback or requested changes. The CI checks must pass before the PR can be merged.
11. **Merge:** Once approved and CI passes, a maintainer will merge your PR. Congratulations! 🎉

## Licensing

By contributing to Vauxl, you agree that your contributions will be licensed under the [Apache License Version 2.0](LICENSE) that covers the project.

Thank you again for your interest in contributing to Vauxl!
# Security Policy for Vauxl

The Vauxl team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings, and we will make every effort to acknowledge your contributions.

## Supported Versions

As Vauxl is currently in **pre-release / active development**, security patches will generally only be applied to the latest commit on the `main` development branch.

Once official releases (e.g., v1.0.0, v1.1.0) are available, this section will be updated to specify which versions are actively supported with security updates. Typically, this will include the latest major or minor release branch.

| Version | Supported          |
| ------- | ------------------ |
| > 1.0.x | :white_check_mark: | <!-- Example: Update once releases start -->
| < 1.0.0 | :x:                |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

If you believe you have found a security vulnerability in Vauxl, please report it to us privately by emailing:

**[report@vauxl.net](mailto:report@vauxl.net)**

Please include the following details in your report:

*   A clear description of the vulnerability.
*   Steps to reproduce the vulnerability, including any specific configurations or prerequisites.
*   The potential impact of the vulnerability.
*   If possible, suggest a potential fix or mitigation.
*   The specific commit hash or version of Vauxl affected (if known).

**What to Expect:**

*   We will aim to acknowledge receipt of your report within 48 hours (business days).
*   We will investigate the report and determine its validity and severity.
*   We will work to develop a patch or mitigation for valid vulnerabilities.
*   We will coordinate with you on public disclosure, aiming for responsible disclosure after a fix is available (unless the vulnerability is already public).
*   We will credit you for your discovery in the release notes or security advisory, unless you prefer to remain anonymous.

## Security Practices

Vauxl aims to follow security best practices, including:

*   **Secure Communication:** Mandating TLS 1.3 for control connections and DTLS + SRTP for media streams.
*   **Dependency Management:** Regularly scanning dependencies for known vulnerabilities using tools like `cargo audit`.
*   **Cryptography:** Using established cryptographic libraries (`rustls`, `argon2`, researching `libsignal` or vetted primitives for E2EE) and avoiding custom crypto implementations where possible.
*   **Input Validation:** Sanitizing and validating user inputs on both client and server.
*   **Least Privilege:** Designing components to operate with the minimum necessary permissions.
*   **Code Review:** Reviewing pull requests for potential security issues.

We are committed to improving the security of Vauxl continuously. Thank you for helping keep Vauxl and its users safe.

*(Policy updated: 2025-04-15)*
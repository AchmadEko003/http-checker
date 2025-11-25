# HTTP Checker 🦀

A command-line HTTP status checker and security header analyzer built with Rust.

## Overview

HTTP Checker is a simple yet powerful tool that allows you to check website availability, HTTP status codes, response times, and security headers. It provides instant feedback on the health and security configuration of web servers.

## Features

- ✅ **HTTP Status Checking** - Verify if websites are up and accessible
- ⚡ **Response Time Measurement** - Track how fast servers respond
- 🔒 **Security Header Analysis** - Evaluate important security headers:
  - `Server` - Checks for server information disclosure
  - `X-Powered-By` - Identifies technology stack exposure
  - `X-Content-Type-Options` - Validates MIME type sniffing protection
  - `X-Frame-Options` - Checks clickjacking protection
  - `Strict-Transport-Security` - Verifies HSTS configuration
- 🎨 **Interactive CLI** - User-friendly menu-driven interface
- 🔄 **Batch Checking** - Check multiple sites consecutively

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd http-checker
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

Alternatively, you can run the compiled binary directly:
```bash
./target/release/http-check
```

## Usage

1. Launch the application
2. Select "Check Site" from the main menu
3. Enter a URL (with or without `http://` or `https://` prefix)
4. View the results including:
   - HTTP status code
   - Response time
   - Security header analysis with recommendations

### Example

```
Enter the URL to check (e.g., https://example.com): example.com

🔍 Checking https://example.com...

┌────────────────────────────────────────┐
│           CHECK RESULTS                │
└────────────────────────────────────────┘
URL: https://example.com
Status Code: 200
Status: 200 OK
Result: ✅ Site is UP
Response Time: 245 ms

======HEADER======
Server: nginx ✅
X-Powered-By: N/A ⚠️
X-Content-Type-Options: nosniff ✅
X-Frame-Options: DENY ✅
Strict-Transport-Security: max-age=31536000 ✅
```

## Security Header Indicators

- **Green Check** - Header is properly configured
- **Warning** - Header may expose sensitive information or is missing
- **N/A** - Header is not present

### What the Tool Checks

- **Server Header**: Warns if it exposes web server details, versions, or frameworks
- **X-Powered-By**: Flags if technology stack is exposed
- **X-Content-Type-Options**: Should be set to `nosniff`
- **X-Frame-Options**: Should be set to `DENY` or `SAMEORIGIN`
- **Strict-Transport-Security**: Presence indicates HTTPS enforcement

## Dependencies

- [reqwest](https://crates.io/crates/reqwest) - HTTP client with blocking API
- [inquire](https://crates.io/crates/inquire) - Interactive CLI prompts
- [regex](https://crates.io/crates/regex) - Regular expression support

## Project Structure

```
http-checker/
├── src/
│   ├── main.rs         # Application entry point and main menu
│   ├── checker.rs      # URL checking and header analysis logic
│   ├── check_lib.rs    # Header validation functions
│   └── welcome.rs      # Welcome screen and info display
├── Cargo.toml          # Project configuration and dependencies
└── README.md           # This file
```

## Development

### Running in Development Mode

```bash
cargo run
```

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

---

**Note**: This tool is designed for legitimate security testing and monitoring purposes only. Always ensure you have permission before testing websites you don't own.

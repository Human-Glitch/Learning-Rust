# Rust HTTP Server

Created a simple Rust HTTP Server in Rust. The server uses the `std::net::TcpListener` crate to listen for incoming TCP connections on a specified address. It serves static files from a specified directory and handles basic HTTP requests.

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Overview
This project is a basic HTTP server implemented in Rust. It serves static files from a specified directory and handles basic HTTP requests.

## Features
- Serves static files from a public directory
- Configurable public directory path via environment variable
- Basic request handling

## Installation
To run this HTTP server, you need to have Rust installed. You can install Rust from [here](https://www.rust-lang.org/).

```bash
# Clone the repository
git clone https://github.com/Human-Glitch/rust-playground.git

# Navigate to the rust_http_server directory
cd rust-playground/rust_http_server

# Build the project
cargo build
```

## Usage
To start the server, you can run the following command:

```bash
# Run the server
cargo run
```

By default, the server will serve files from the public directory. You can change this by setting the PUBLIC_PATH environment variable:

```
# Set the public path and run the server
PUBLIC_PATH=your/custom/path cargo run
```

## License
This project is licensed under the Apache License 2.0. See the [LICENSE](../LICENSE) file for more details.

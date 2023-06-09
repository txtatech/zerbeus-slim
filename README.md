# zerbeuz-slim
A stand-alone http server written in Rust that boots four memory resident linux distros in the browser simultaneously.

**This is not a production server.** It is intended for sandboxed testing purposes only.

There are four available linux shells in this server but only three of them are used because one of them bootstraps the others.

Note: There is an embedded html site in the server.rs file itself which contains a 1115 second refresh timer which is not requied and can be adjusted/deleted.

All files for the server are loaded as/from a byte array at boot time.

The default directory the server serves from is 'static' but is not required for the server to operate.

This server has open routes, handles and static variables for the following files:

Used from: https://github.com/rslay/c_in_browser

libv86.js

linux.iso

seabios.bin

vgabios.bin

index.html

Used from: https://github.com/sbsoftware/wasem.js

memory.js

syscall.js

wasem.js

errno.js

fs.js

The server operates on the following addresses and ports.

http://127.0.0.1:8081

http://127.0.0.1:8080

http://127.0.0.1:8080/index.html <---Used to boostrap--->

http://127.0.0.1:8081/index1.html

**BUILD NOTE:** Built with Rustc 1.72.0-nightly and Cargo v0.9.4

```markdown
# zerbeuzslim

## Description
`zerbeuzslim` is a Rust package for handling web server functionality. It leverages the Actix framework along with other dependencies to provide a lightweight and efficient web server implementation.

## Installation

### Prerequisites
- Rust programming language (minimum version: 1.55.0)

### Steps

1. Install Rust: Follow the official Rust installation guide based on your operating system. Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for detailed instructions.

2. Clone the `zerbeuzslim` repository:

   ```plaintext
   git clone https://github.com/your-username/zerbeuzslim.git
   ```

   Alternatively, if you have already downloaded the package, navigate to its root directory.

3. Build and run the package:

   cargo run

This command will download and compile all the necessary dependencies, and then start the web server.

4. Access the web server:

Open your web browser and visit:
`http://localhost:8080`, `http://localhost:8081`, `http://localhost:8080/index.html`, `http://localhost:8081/index1.html`

Note: Each time a page is opened it launches a new instance of the chosen distro from memory.

## Dependencies

- `actix-files` version 0.6.2: A library for serving files with Actix Web.
- `actix-web` version 4: A powerful, yet lightweight framework for building web applications in Rust.
- `hyper` version 0.14.23: A fast and correct HTTP implementation in Rust.
- `rocket` version 0.4.11: A web framework for Rust that focuses on ease of use, stability, and performance.
- `tokio` version 1.24.1: An asynchronous runtime for Rust that provides a framework for writing reliable, asynchronous, and concurrent applications.

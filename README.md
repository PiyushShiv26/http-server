

# HTTP Server in Rust

A simple HTTP server built from scratch in Rust, capable of handling basic GET and POST requests, serving files, and managing concurrent connections. This project was undertaken as part of the "Build your own HTTP server" challenge.

## Current Features

* Responds to basic HTTP GET requests with `200 OK`.
* Handles `404 Not Found` for unknown paths.
* Supports concurrent client connections using threads.
* **Path Routing:**
    * `GET /`: Returns a `200 OK` response.
    * `GET /echo/{string}`: Echoes the provided string in the response body with `Content-Type: text/plain`.
    * `GET /user-agent`: Returns the client's User-Agent header in the response body with `Content-Type: text/plain`.
    * `GET /files/{filename}`:
        * Serves the content of `{filename}` from a specified directory.
        * Responds with `Content-Type: application/octet-stream`.
        * Returns `404 Not Found` if the file does not exist or the directory is not specified.
    * `POST /files/{filename}`:
        * Creates a new file named `{filename}` in the specified directory with the content from the request body.
        * Responds with `201 Created`.
* Accepts a `--directory <path>` command-line argument to specify the root directory for file operations.

## Dependencies

This project uses the following Rust crates (as defined in `Cargo.toml`):

* `anyhow = "1.0.68"`: For flexible error handling.
* `bytes = "1.3.0"`: For working with byte buffers.
* `thiserror = "1.0.38"`: For creating custom error types.

```toml
[package]
name = "http-server"
version = "0.1.0"
edition = "2021"
rust-version = "1.80"

[dependencies]
anyhow = "1.0.68"
bytes = "1.3.0"
thiserror = "1.0.38"
````

## How to Build and Run

1.  **Clone the repository (if you've pushed it to GitHub):**

    ```bash
    git clone <your-repository-url>
    cd http-server
    ```

2.  **Build the project:**

    ```bash
    cargo build
    ```

3.  **Run the server:**
    To serve files, you need to provide the `--directory` argument.

    ```bash
    ./target/debug/http-server --directory /path/to/your/files
    ```

    Replace `/path/to/your/files` with the actual path to the directory you want to serve files from.

    If you don't need to serve files from a specific directory for testing other endpoints, you can run it without the `--directory` flag, but file-related operations will result in a 404 if the directory isn't implicitly handled or an error occurs.

## Example Usage (with `curl`)

  * **Root path:**
    ```bash
    curl -v http://localhost:4221/
    ```
  * **Echo endpoint:**
    ```bash
    curl -v http://localhost:4221/echo/hello_world
    ```
  * **User-Agent endpoint:**
    ```bash
    curl -v http://localhost:4221/user-agent
    ```
  * **Get a file (assuming `your_file.txt` exists in your specified `--directory`):**
    ```bash
    curl -v http://localhost:4221/files/your_file.txt
    ```
  * **Create a file:**
    ```bash
    curl -v --data "This is the content of the new file." -H "Content-Type: application/octet-stream" http://localhost:4221/files/new_file.txt
    ```

## Future Enhancements

I plan to extend this HTTP server with more advanced features in the future, including:

  * **Compression:**
      * Support for `Accept-Encoding` and `Content-Encoding` headers.
      * Implementation of Gzip compression.
      * Handling multiple compression schemes.
  * **Persistent Connections (Keep-Alive):**
      * Allowing multiple HTTP requests over a single TCP connection.
      * Managing multiple persistent connections.
      * Graceful connection closure.
  * **Improved Error Handling:** Moving away from `.unwrap()` for more robust error management.
  * **Security Enhancements:** Addressing potential vulnerabilities like path traversal more comprehensively.
  * **More HTTP Methods and Headers:** Expanding support for other common HTTP features.

## License

This project is licensed under the [MIT License](./LICENSE).

-----

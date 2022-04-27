# DS/2 Nachtwacht Frontend

The frontend server for simple n8w8 stuff.

## How to build

Run:

    cargo clean
    cargo build --package nachtwacht-rust --bin nachtwacht-rust

or via Dapper:

    dapper
    # be aware that this installs the target directory as root user!!

## How to run

via

    cargo run --package nachtwacht-rust --bin nachtwacht-fe

It will bind on http://localhost:8080/

Some urls to test:

* http://localhost:8080/welcome
* http://localhost:8080/index.html
* http://localhost:8080/static/
* http://localhost:8080/1234/papa/index.html
* http://localhost:8080/favicon

## Build Docker image

After the cargo build run, you can use:

    docker build -t nachtwacht-fe:latest .
    docker run -it --rm -p 8080:8080 nachtwacht-fe:latest

# rust-todo-app

An example of how to write a Todo-server in Rust. This implementation is incomplete, but contains a good starting point that demonstrates how to implement it in Rust using asynchronous programming. Furthermore, the code design is the Ports and Adapters pattern.

# Prerequisites

* Docker installed
* Rust installed
* [Cornucopia](https://github.com/cornucopia-rs/cornucopia) installed

# Usage

Run `make up` to start a Postgres instance inside a Docker container. Then run `make run` to start the web server. Don't forget to run `make down` when you're done to stop the Postgres instance.

# Generated Rust code

The code that interacts with Postgres was generated using the [Cornucopia](https://github.com/cornucopia-rs/cornucopia) tool. To run the code generation, enter `make gen`.


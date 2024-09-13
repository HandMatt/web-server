# Web Server

This project is a client/server setup, representing data-collection for an Internet-of-Things (IoT) setup.
Using SQLite as the database, and popular Rust crates for building web servers and managing data.

## Brief

A company is building data-collection widget for customers. These widgets are small, low-power devices that collect data from sensors and send it to a central server. The server stores the data, and provides a web interface for customers to view their data. To achieve this I will need to:

- Build a daemon to the on the data-collectors
- Build a server to receive and store the data
- Build a web server to query and display the data

### Collection Daemon

THe collection daemon has to run on small, low-power devices. I will seek to minimise the binary size, memory and CPU usage and keep network traffic to a minimum.

### Collection Server

The collection server will receive data from collection daemons, and store it in a database (SQLite for now).

### Web Service

Starting with a relatively minimal web-service that uses Apache E-Charts in the client to display the data. I'll use the Axum web framework, and I will use the SQLite database directly.

### Overview

The data flow can be visualised as follows:

```
graph BT
DataCollector[Data Collector] ---> Ingestor
DataCollector2[Data Collector] ---> Ingestor
DataCollector3[Data Collector] ---> Ingestor
Ingestor ---> SQLite
WebServer[Web Server] <---> SQLite
WebClient[Web Client] <---> WebServer
```

## Installation

To run this application you will need to have Rust installed (via RustUp).

To create and setup the db you will need to install the SQLx command-line client:

```shell
cargo install sqlx-cli
```

Then create and build the database:

```shell
sqlx database create
sqlx migrate run
```

## Run

To test the shared resource (responsible for encoding and decoding the data) move into the `shared` directory and run the following command.

```shell
cargo test
```

To run the server use `cargo run` within the `server` directory. You can do the same for the `collector`.

You will see a steady stream of data being encoded within the collector, and the server will be receiving the data.

If the server is stopped, the collector will queue up data and send it when the sever becomes available.

When the server is active you will be able to access a webpage on `http://localhost:3000/` which has the following routes:

- `/` - display `index.html`
- `api/all` - show all datapoints (JSON)
- `api/collectors` - show all collectors (JSON)
- `api/collector/:uuid` - show all datapoints for a given collector (JSON)
- `api/collector/:uuid/shutdown` - send a shutdown command to a given collector

# Optimisations

Using `cargo bloat` and `cargo tree` has identified that UUID creation and sysinfo have dependencies that take up a considerable amount of space. Removing UUID creation and omitting the `rayon` can save some space. Potential reductions could be obtained by updating the crates being used in the project.   

> This project demonstrates the learnings from the fifth week of the Ardan Labs: Ultimate Rust Foundations course.

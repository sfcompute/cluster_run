
## San Francisco Compute Company's Cluster Remote Execution Utility 

`cluster_run` is a Rust-based script designed by San Francisco Compute Company for seamless cluster management and remote command execution. This tool enables system administrators and DevOps professionals to effortlessly run commands across multiple nodes in a cluster, streamlining operations and enhancing productivity in distributed computing environments.

### Features

- **Effortless Cluster Management**: Execute commands simultaneously across multiple nodes in your cluster.
- **Secure Connections**: Utilizes SSH with public key authentication for secure and passwordless operations.
- **Configurable Node List**: Easily manage your cluster nodes through a simple TOML configuration file.
- **Flexible Command Execution**: Run any command or script across your entire cluster with a single command.
- **Efficient Output Handling**: Clearly displays command output or errors for each node, facilitating quick issue identification and resolution.

### Installation

1. Ensure you have Rust and Cargo installed on your system. If not, install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone the repository:
   ```
   git clone https://github.com/sfcompute/cluster_run.git
   cd cluster_run
   ```

3. Build the project:
   ```
   cargo build --release
   ```

### Configuration

1. Create a `config.toml` file in the same directory as the binary with the following structure:
   ```toml
   [cluster]
   nodes = [
       "node1.example.com",
       "node2.example.com",
       "10.0.0.1",
       "10.0.0.2"
   ]
   ```

2. Ensure you have SSH keys set up for passwordless authentication to all nodes in your cluster.

### Usage

Run commands across your cluster using the following syntax:

```
cargo run -- <your_command_here>
```

For example:

```
cargo run -- ls -l /var/log
```

This will execute `ls -l /var/log` on all nodes specified in your `config.toml` file.

### Examples

1. Check disk usage across all nodes:
   ```
   cargo run -- df -h
   ```

2. Update all nodes:
   ```
   cargo run -- sudo apt update && sudo apt upgrade -y
   ```

3. Check system uptime:
   ```
   cargo run -- uptime
   ```

### Contributing

We welcome contributions to `cluster_run`! Please feel free to submit issues, fork the repository and send pull requests!

### License

Copyright (c) 2024 San Francisco Compute Company. All rights reserved.

### Support

For support, please open an issue on the GitHub repository or contact our support team at support@sfcompute.com.

---

San Francisco Compute Company - Empowering Distributed Computing

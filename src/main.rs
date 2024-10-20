// Copyright (c) 2024 San Francisco Compute Company
// All rights reserved.

//! cluster_run: A Rust application for executing commands on multiple nodes in a cluster.
//!
//! This application reads a list of nodes from a TOML configuration file,
//! takes a command as command-line arguments, and then executes that command on each node
//! in the cluster using SSH. It uses public key authentication and assumes
//! the 'ubuntu' user for connections.

use serde::Deserialize;
use ssh2::Session;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;

/// Represents the entire configuration structure.
#[derive(Deserialize)]
struct Config {
    cluster: ClusterConfig,
}

/// Represents the cluster configuration, containing a list of node addresses.
#[derive(Deserialize)]
struct ClusterConfig {
    nodes: Vec<String>,
}

/// The main function that drives the cluster_run application.
///
/// This function performs the following steps:
/// 1. Reads and parses the configuration file.
/// 2. Collects the command from command-line arguments.
/// 3. Iterates through each node in the cluster, executing the command.
/// 4. Prints the output or any errors encountered during execution.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to cluster_run!");

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args[0]);
        std::process::exit(1);
    }

    // Construct the command from arguments
    let command = args[1..].join(" ");

    // Read and parse the config file
    let config_content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_content)?;

    // Execute the command on each node in the cluster
    for node in &config.cluster.nodes {
        println!("Connecting to node {}...", node);
        match run_command(node, &command) {
            Ok(output) => println!(
                "Output from {} for command '{}': \n{}",
                node, command, output
            ),
            Err(e) => eprintln!("Error for node {}: {}", node, e),
        }
        println!();
    }

    Ok(())
}

/// Executes a command on a specified node using SSH.
///
/// This function performs the following steps:
/// 1. Establishes an SSH connection to the node.
/// 2. Authenticates using SSH key-based authentication.
/// 3. Creates an SSH channel and executes the specified command.
/// 4. Captures and returns the output of the command.
///
/// # Arguments
///
/// * `node` - The address of the node to connect to.
/// * `command` - The command to execute on the node.
///
/// # Returns
///
/// Returns a Result containing either the command output as a String,
/// or an error if any step in the process fails.
fn run_command(node: &str, command: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Connect to the node
    let tcp = TcpStream::connect(format!("{}:22", node))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // Get the path to the SSH key files
    let (pubkey, privkey) = get_ssh_key_paths()?;

    // Authenticate using the SSH key
    sess.userauth_pubkey_file("ubuntu", Some(&pubkey), &privkey, None)?;

    // Create a channel and execute the command
    let mut channel = sess.channel_session()?;
    channel.exec(command)?;

    // Read the output
    let mut output = String::new();
    channel.read_to_string(&mut output)?;

    // Wait for the command to finish
    channel.wait_close()?;

    // Return the output
    Ok(output)
}

/// Retrieves the paths to the SSH public and private key files.
///
/// This function assumes the SSH keys are located in the default ~/.ssh directory
/// and are named id_rsa.pub and id_rsa for the public and private keys respectively.
///
/// # Returns
///
/// Returns a Result containing a tuple of PathBuf for the public and private key files,
/// or an error if the keys are not found in the expected location.
fn get_ssh_key_paths() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let home = env::var("HOME").map_err(|_| "Unable to determine home directory")?;
    let ssh_dir = PathBuf::from(home).join(".ssh");

    let pubkey = ssh_dir.join("id_rsa.pub");
    let privkey = ssh_dir.join("id_rsa");

    if !pubkey.exists() || !privkey.exists() {
        return Err("SSH key files not found in the default location".into());
    }

    Ok((pubkey, privkey))
}


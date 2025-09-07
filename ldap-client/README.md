# LDAP Client (Rust)

A command-line LDAP client written in Rust for connecting to the ldapy LDAP server.

## Features

- Connect to LDAP servers with authentication
- Search for entries with custom filters
- List users and groups
- Multiple output formats (table, JSON)
- Configurable connection parameters
- Test LDAP connectivity

## Installation

Make sure you have Rust installed, then build the project:

```bash
cd ldap-client
cargo build --release
```

## Usage

### Basic Commands

```bash
# Test connection to the LDAP server
cargo run -- test

# Search for all entries
cargo run -- search

# Search with a specific filter
cargo run -- search --filter "(cn=john*)"

# List all users
cargo run -- users

# List all groups
cargo run -- groups

# Get JSON output
cargo run -- users --output json
```

### Connection Options

```bash
# Connect to a different server
cargo run -- --url ldap://your-server:389 test

# Use different credentials
cargo run -- --bind-dn "cn=admin,dc=example,dc=com" --password "secret" test

# Use different base DN
cargo run -- --base-dn "dc=example,dc=com" search
```

### Advanced Search Examples

```bash
# Search for users with specific attributes
cargo run -- search --filter "(objectClass=inetOrgPerson)" --attributes "cn,mail,uid"

# Search with different scopes
cargo run -- search --filter "(cn=*)" --scope "one"

# Search for groups containing specific members
cargo run -- search --filter "(member=*john*)"
```

## Default Configuration

The client uses these defaults that match your ldapy Docker setup:

- **Server URL**: `ldap://localhost:389`
- **Bind DN**: `cn=admin,dc=electronicpanopti,dc=com`
- **Password**: `admin123`
- **Base DN**: `dc=electronicpanopti,dc=com`

## Command Reference

### Global Options

- `--url` / `-u`: LDAP server URL (default: ldap://localhost:389)
- `--bind-dn` / `-b`: Bind DN for authentication
- `--password` / `-p`: Password for authentication
- `--base-dn`: Base DN for search operations

### Commands

#### `test`

Test connectivity to the LDAP server.

#### `search`

Search for entries in the LDAP directory.

- `--filter` / `-f`: LDAP filter (default: "(objectClass=*)")
- `--attributes` / `-a`: Comma-separated list of attributes to retrieve
- `--scope` / `-s`: Search scope (base, one, sub) (default: sub)
- `--output` / `-o`: Output format (table, json) (default: table)

#### `users`

List all users (searches for objectClass=inetOrgPerson).

- `--output` / `-o`: Output format (table, json) (default: table)

#### `groups`

List all groups (searches for objectClass=groupOfNames).

- `--output` / `-o`: Output format (table, json) (default: table)

## Examples with Sample Data

If you've loaded the sample data from the main ldapy project:

```bash
# Find the sample users
cargo run -- search --filter "(uid=john.doe)"
cargo run -- search --filter "(uid=jane.smith)"

# Find the developers group
cargo run -- search --filter "(cn=developers)"

# List all people
cargo run -- search --filter "(objectClass=inetOrgPerson)"

# Get user emails
cargo run -- search --filter "(mail=*)" --attributes "cn,mail"
```

## Building for Production

```bash
# Build optimized binary
cargo build --release

# The binary will be in target/release/ldap-client
./target/release/ldap-client --help
```

## Dependencies

- `ldap3`: LDAP client library
- `tokio`: Async runtime
- `clap`: Command-line argument parsing
- `serde/serde_json`: JSON serialization
- `anyhow`: Error handling

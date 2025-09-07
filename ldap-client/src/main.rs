use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ldap3::{LdapConn, Scope, SearchEntry};
use serde_json::json;

#[derive(Parser)]
#[command(name = "ldap-client")]
#[command(about = "A Rust LDAP client for connecting to the ldapy server")]
struct Cli {
    /// LDAP server URL
    #[arg(short, long, default_value = "ldap://localhost:389")]
    url: String,

    /// Bind DN for authentication
    #[arg(short, long, default_value = "cn=admin,dc=electronicpanopti,dc=com")]
    bind_dn: String,

    /// Password for authentication
    #[arg(short, long, default_value = "admin123")]
    password: String,

    /// Base DN for operations
    #[arg(long, default_value = "dc=electronicpanopti,dc=com")]
    base_dn: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for entries in the LDAP directory
    Search {
        /// LDAP filter (e.g., "(objectClass=*)" or "(cn=john*)")
        #[arg(short, long, default_value = "(objectClass=*)")]
        filter: String,

        /// Attributes to retrieve (comma-separated)
        #[arg(short, long)]
        attributes: Option<String>,

        /// Search scope (base, one, sub)
        #[arg(short, long, default_value = "sub")]
        scope: String,

        /// Output format (json, table)
        #[arg(short, long, default_value = "table")]
        output: String,
    },
    /// List all users
    Users {
        /// Output format (json, table)
        #[arg(short, long, default_value = "table")]
        output: String,
    },
    /// List all groups
    Groups {
        /// Output format (json, table)
        #[arg(short, long, default_value = "table")]
        output: String,
    },
    /// Test connection to the LDAP server
    Test,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search {
            filter,
            attributes,
            scope,
            output,
        } => {
            search_entries(&cli, filter, attributes.as_deref(), scope, output).await?;
        }
        Commands::Users { output } => {
            list_users(&cli, output).await?;
        }
        Commands::Groups { output } => {
            list_groups(&cli, output).await?;
        }
        Commands::Test => {
            test_connection(&cli).await?;
        }
    }

    Ok(())
}

async fn connect_and_bind(cli: &Cli) -> Result<LdapConn> {
    let mut ldap = LdapConn::new(&cli.url).context("Failed to connect to LDAP server")?;

    ldap.simple_bind(&cli.bind_dn, &cli.password)
        .context("Failed to bind to LDAP server")?
        .success()
        .context("LDAP bind failed")?;

    println!("✓ Successfully connected and authenticated to LDAP server");
    Ok(ldap)
}

async fn search_entries(
    cli: &Cli,
    filter: &str,
    attributes: Option<&str>,
    scope_str: &str,
    output_format: &str,
) -> Result<()> {
    let mut ldap = connect_and_bind(cli).await?;

    let scope = match scope_str {
        "base" => Scope::Base,
        "one" => Scope::OneLevel,
        "sub" => Scope::Subtree,
        _ => return Err(anyhow::anyhow!("Invalid scope: {}", scope_str)),
    };

    let attrs = match attributes {
        Some(attr_str) => attr_str.split(',').map(|s| s.trim()).collect::<Vec<_>>(),
        None => vec!["*"],
    };

    println!("Searching with filter: {}", filter);
    println!("Base DN: {}", cli.base_dn);
    println!("Scope: {}", scope_str);
    println!("Attributes: {:?}", attrs);
    println!();

    let (rs, _res) = ldap
        .search(&cli.base_dn, scope, filter, attrs)
        .context("LDAP search failed")?
        .success()
        .context("LDAP search operation failed")?;

    if rs.is_empty() {
        println!("No entries found.");
        return Ok(());
    }

    // Convert ResultEntry to SearchEntry
    let search_entries: Vec<SearchEntry> = rs.into_iter().map(SearchEntry::construct).collect();

    match output_format {
        "json" => print_json_output(&search_entries)?,
        "table" => print_table_output(&search_entries)?,
        _ => return Err(anyhow::anyhow!("Invalid output format: {}", output_format)),
    }

    ldap.unbind().context("Failed to unbind from LDAP server")?;
    Ok(())
}

async fn list_users(cli: &Cli, output_format: &str) -> Result<()> {
    let filter = "(objectClass=inetOrgPerson)";
    let attributes = Some("cn,sn,givenName,mail,uid");
    search_entries(cli, filter, attributes, "sub", output_format).await
}

async fn list_groups(cli: &Cli, output_format: &str) -> Result<()> {
    let filter = "(objectClass=groupOfNames)";
    let attributes = Some("cn,description,member");
    search_entries(cli, filter, attributes, "sub", output_format).await
}

async fn test_connection(cli: &Cli) -> Result<()> {
    println!("Testing connection to LDAP server...");
    println!("URL: {}", cli.url);
    println!("Bind DN: {}", cli.bind_dn);

    let mut ldap = connect_and_bind(cli).await?;

    // Perform a simple search to verify the connection works
    let (rs, _res) = ldap
        .search(&cli.base_dn, Scope::Base, "(objectClass=*)", vec!["*"])
        .context("Test search failed")?
        .success()
        .context("Test search operation failed")?;

    println!("✓ Connection test successful!");
    println!("✓ Found {} base entries", rs.len());

    ldap.unbind().context("Failed to unbind from LDAP server")?;
    Ok(())
}

fn print_json_output(entries: &[SearchEntry]) -> Result<()> {
    let json_data: Vec<_> = entries
        .iter()
        .map(|entry| {
            json!({
                "dn": entry.dn,
                "attributes": entry.attrs
            })
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&json_data)?);
    Ok(())
}

fn print_table_output(entries: &[SearchEntry]) -> Result<()> {
    println!("Found {} entries:", entries.len());
    println!("{}", "=".repeat(80));

    for (i, entry) in entries.iter().enumerate() {
        println!("Entry #{}: {}", i + 1, entry.dn);
        println!("{}", "-".repeat(40));

        for (attr, values) in &entry.attrs {
            if values.len() == 1 {
                println!("  {}: {}", attr, values[0]);
            } else {
                println!("  {}:", attr);
                for value in values {
                    println!("    - {}", value);
                }
            }
        }
        println!();
    }

    Ok(())
}

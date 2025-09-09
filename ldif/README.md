# Additional LDAP Sample Data

This directory contains various LDIF files with sample data beyond the basic users and groups.

## File Overview

### 01-sample-data.ldif

- Basic users (john.doe, jane.smith)
- Basic group (developers)
- **Original file from the project**

### 02-organizational-units.ldif

- Organizational units for different departments
- Includes: engineering, marketing, sales, hr, finance, legal
- Each department has contact information

### 03-locations.ldif

- Office locations and facilities
- Corporate headquarters, regional offices
- Data center locations
- Complete with addresses and contact details

### 04-services.ldif

- IT services and applications
- Core services: email, LDAP, web portal, database
- Business applications: CRM, ERP, HR system
- Cloud services: storage, backup, monitoring

### 05-projects.ldif

- Company projects (active and completed)
- Includes project leads and department associations
- Examples: Panopticon v2, quantum encryption, global expansion

### 06-equipment.ldif

- Company equipment and assets
- Server equipment with serial numbers
- Network equipment (switches, firewalls, routers)
- Storage systems and office equipment

### 07-security.ldif

- Security roles and access policies
- SSL/TLS certificate management entries
- Compliance tracking (ISO 27001, GDPR)
- Data center and server room access policies

## Object Classes Used

- **organizationalUnit**: For departments and organizational structure
- **locality + organizationalRole**: For office locations
- **applicationEntity**: For IT services and applications
- **organizationalRole**: For projects and security roles
- **device**: For equipment and hardware assets
- **groupOfNames**: For security groups

## Loading the Data

To load all sample data into your LDAP server:

1. Start the LDAP server:

   ```bash
   docker-compose up -d
   ```

2. Load each LDIF file:

   ```bash
   # Basic data (if not already loaded)
   ldapadd -x -H ldap://localhost:389 -D "cn=admin,dc=electronicpanopti,dc=com" -w admin123 -f ldif/01-sample-data.ldif
   
   # Additional organizational data
   ldapadd -x -H ldap://localhost:389 -D "cn=admin,dc=electronicpanopti,dc=com" -w admin123 -f ldif/02-organizational-units.ldif
   ldapadd -x -H ldap://localhost:389 -D "cn=admin,dc=electronicpanopti,dc=com" -w admin123 -f ldif/03-locations.ldif
   ldapadd -x -H ldap://localhost:389 -D "cn=admin,dc=electronicpanopti,dc=com" -w admin123 -f ldif/04-services.ldif
   ldapadd -x -H ldap://localhost:389 -D "cn=admin,dc=electronicpanopti,dc=com" -w admin123 -f ldif/05-projects.ldif
   ldapadd -x -H ldap://localhost:389 -D "cn=admin,dc=electronicpanopti,dc=com" -w admin123 -f ldif/06-equipment.ldif
   ldapadd -x -H ldap://localhost:389 -D "cn=admin,dc=electronicpanopti,dc=com" -w admin123 -f ldif/07-security.ldif
   ```

## Testing with the Rust Client

You can use the Rust LDAP client to explore the new data:

```bash
cd ldap-client

# Search for all departments
cargo run -- search --filter "(ou=*)" --attributes "ou,description,telephoneNumber"

# Find all locations
cargo run -- search --filter "(objectClass=locality)" --attributes "cn,l,st,street,telephoneNumber"

# List all IT services
cargo run -- search --filter "(objectClass=applicationEntity)" --attributes "cn,description,presentationAddress"

# Find all projects
cargo run -- search --filter "(cn=project-*)" --attributes "cn,description,roleOccupant"

# List all equipment
cargo run -- search --filter "(objectClass=device)" --attributes "cn,description,serialNumber,l,owner"

# Search security-related entries
cargo run -- search --filter "(ou=security)" --scope "sub" --attributes "cn,description,member,roleOccupant"

# Get everything in JSON format
cargo run -- search --filter "(objectClass=*)" --output json > ldap_dump.json
```

## Common Search Patterns

```bash
# Find entries by location
cargo run -- search --filter "(l=San Francisco)"

# Find entries owned by a specific department
cargo run -- search --filter "(owner=*engineering*)"

# Find all contact information
cargo run -- search --filter "(telephoneNumber=*)" --attributes "cn,telephoneNumber,description"

# Find entries with serial numbers (equipment)
cargo run -- search --filter "(serialNumber=*)" --attributes "cn,serialNumber,description"

# Find all organizational roles
cargo run -- search --filter "(objectClass=organizationalRole)" --attributes "cn,description,roleOccupant"
```

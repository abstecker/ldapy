# LDAP Server Docker Compose Setup

This Docker Compose setup provides an OpenLDAP server with a web-based administration interface.

## Services

### OpenLDAP Server

- **Port**: 389 (LDAP), 636 (LDAPS)
- **Admin DN**: `cn=admin,dc=example,dc=com`
- **Admin Password**: `admin123`
- **Base DN**: `dc=example,dc=com`

### phpLDAPadmin (Web Interface)

- **Port**: 8080 (HTTP), 8443 (HTTPS)
- **URL**: <http://localhost:8080>
- **Login**: Use the admin credentials above

## Quick Start

1. Start the services:

   ```bash
   docker-compose up -d
   ```

2. Access the web interface at: <http://localhost:8080>

3. Login with:
   - **Login DN**: `cn=admin,dc=example,dc=com`
   - **Password**: `admin123`

## LDAP Client Commands

You can also interact with the LDAP server using command-line tools:

### Install LDAP utils (on macOS)

```bash
brew install openldap
```

### Search for all entries

```bash
ldapsearch -x -H ldap://localhost:389 -D "cn=admin,dc=example,dc=com" -w admin123 -b "dc=example,dc=com"
```

### Search for users

```bash
ldapsearch -x -H ldap://localhost:389 -D "cn=admin,dc=example,dc=com" -w admin123 -b "ou=people,dc=example,dc=com"
```

### Add a new user (create add-user.ldif first)

```bash
ldapadd -x -H ldap://localhost:389 -D "cn=admin,dc=example,dc=com" -w admin123 -f add-user.ldif
```

## Sample Data

The setup includes sample users and groups in the `ldif/01-sample-data.ldif` file:

- **Users**: john.doe, jane.smith
- **Group**: developers
- **Default password for users**: password123

## Configuration

### Environment Variables

- `LDAP_ORGANISATION`: Organization name
- `LDAP_DOMAIN`: Domain name (affects base DN)
- `LDAP_ADMIN_PASSWORD`: Admin user password
- `LDAP_BASE_DN`: Base Distinguished Name

### Volumes

- `ldap_data`: Stores LDAP data
- `ldap_config`: Stores LDAP configuration
- `./ldif`: Custom LDIF files for initial data

## Stopping the Services

```bash
docker-compose down
```

To remove all data (volumes):

```bash
docker-compose down -v
```

## Troubleshooting

1. **Check container logs**:

   ```bash
   docker-compose logs openldap
   docker-compose logs phpldapadmin
   ```

2. **Verify containers are running**:

   ```bash
   docker-compose ps
   ```

3. **Test LDAP connectivity**:

   ```bash
   telnet localhost 389
   ```

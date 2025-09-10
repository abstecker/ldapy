# RFC 1278 Presentation Addresses for LDAP applicationEntity

This document explains how to craft proper RFC 1278 presentation addresses for LDAP applicationEntity objects, specifically for internet-hosted services.

## What is RFC 1278?

RFC 1278 defines the presentation address format used in LDAP applicationEntity objects. The presentation address specifies how clients can connect to a service, including the transport protocol, hostname/IP, and port information.

## Basic RFC 1278 Format

```
'<transport>'<address>[':'<port>]['+'<additional-info>]
```

### Components Breakdown:

1. **Transport**: The transport protocol (must be enclosed in single quotes)
2. **Address**: Hostname, FQDN, or IP address
3. **Port**: Optional port number (preceded by colon)
4. **Additional Info**: Optional additional parameters (preceded by plus)

## Common Transport Types

### TCP Services (Most Common for Internet Services)
```
'TCP'hostname:port
```

### UDP Services
```
'UDP'hostname:port
```

### Other Transport Types
```
'X25'address                 # X.25 networks
'TP4'hostname:port          # ISO TP4
'CLNS'nsap-address          # Connectionless Network Service
```

## Practical Examples for Internet Services

### Web Services
```
# HTTPS service
presentationAddress: 'TCP'www.example.com:443

# HTTP service  
presentationAddress: 'TCP'www.example.com:80

# REST API endpoint
presentationAddress: 'TCP'api.example.com:443

# Multiple endpoints for same service (load balancing/redundancy)
presentationAddress: 'TCP'web1.example.com:443
presentationAddress: 'TCP'web2.example.com:443
```

### Database Services
```
# PostgreSQL
presentationAddress: 'TCP'db.example.com:5432

# MySQL
presentationAddress: 'TCP'mysql.example.com:3306

# MongoDB
presentationAddress: 'TCP'mongo.example.com:27017

# Redis
presentationAddress: 'TCP'redis.example.com:6379
```

### Email Services
```
# IMAP over SSL
presentationAddress: 'TCP'mail.example.com:993

# IMAP standard
presentationAddress: 'TCP'mail.example.com:143

# SMTP submission
presentationAddress: 'TCP'smtp.example.com:587

# POP3 over SSL
presentationAddress: 'TCP'pop.example.com:995
```

### Directory Services
```
# LDAP
presentationAddress: 'TCP'ldap.example.com:389

# LDAPS (LDAP over SSL)
presentationAddress: 'TCP'ldap.example.com:636

# Active Directory Global Catalog
presentationAddress: 'TCP'gc.example.com:3268
```

### Cloud and Microservices
```
# REST API
presentationAddress: 'TCP'api.example.com:443

# GraphQL endpoint
presentationAddress: 'TCP'graphql.example.com:443

# WebSocket service
presentationAddress: 'TCP'ws.example.com:443

# gRPC service
presentationAddress: 'TCP'grpc.example.com:443

# Internal microservice (private IP)
presentationAddress: 'TCP'10.0.1.100:8080

# Service mesh endpoint
presentationAddress: 'TCP'service.mesh.local:8080
```

## Advanced Examples with Additional Information

### Services with Path Information
```
# REST API with base path
presentationAddress: 'TCP'api.example.com:443+/v1/api

# WebDAV service
presentationAddress: 'TCP'dav.example.com:443+/webdav
```

### Services with Protocol Information
```
# MQTT over TCP
presentationAddress: 'TCP'mqtt.example.com:1883+mqtt

# MQTT over WebSockets
presentationAddress: 'TCP'mqtt.example.com:8083+mqtt-ws
```

## Multiple Presentation Addresses

An applicationEntity can have multiple presentation addresses for:

1. **High Availability**: Multiple servers providing the same service
2. **Load Distribution**: Different endpoints for load balancing
3. **Protocol Variants**: Same service over different protocols/ports
4. **Geographic Distribution**: Different servers in different regions

### Example: Multi-Region Web Service
```ldif
dn: cn=global-web-service,ou=services,dc=example,dc=com
objectClass: applicationEntity
objectClass: top
cn: global-web-service
description: Global web service with multiple regions
presentationAddress: 'TCP'us-east.example.com:443
presentationAddress: 'TCP'us-west.example.com:443
presentationAddress: 'TCP'eu-west.example.com:443
presentationAddress: 'TCP'asia-pacific.example.com:443
```

## Complete LDIF Example

Here's a complete example of an applicationEntity with proper RFC 1278 presentation addresses:

```ldif
dn: cn=customer-api,ou=services,dc=electronicpanopti,dc=com
objectClass: applicationEntity
objectClass: top
cn: customer-api
description: Customer management REST API service
supportedApplicationContext: api-service
presentationAddress: 'TCP'api.electronicpanopti.com:443
presentationAddress: 'TCP'api-backup.electronicpanopti.com:443
presentationAddress: 'TCP'10.0.1.100:8080
seeAlso: cn=engineering,ou=departments,dc=electronicpanopti,dc=com
```

## Best Practices

### 1. Always Use Single Quotes Around Transport
```
# Correct
presentationAddress: 'TCP'api.example.com:443

# Wrong
presentationAddress: TCPapi.example.com:443
presentationAddress: "TCP"api.example.com:443
```

### 2. Use Fully Qualified Domain Names (FQDN)
```
# Good
presentationAddress: 'TCP'api.example.com:443

# Avoid (unless internal)
presentationAddress: 'TCP'api:443
```

### 3. Always Include Port Numbers
```
# Good
presentationAddress: 'TCP'web.example.com:443

# Less clear
presentationAddress: 'TCP'web.example.com
```

### 4. Group Related Services
```
# Email service with multiple protocols
presentationAddress: 'TCP'mail.example.com:993
presentationAddress: 'TCP'mail.example.com:143  
presentationAddress: 'TCP'smtp.example.com:587
```

## Common Mistakes to Avoid

### 1. Incorrect Quote Usage
```
# Wrong
presentationAddress: TCPapi.example.com:443
presentationAddress: "TCP"api.example.com:443

# Correct  
presentationAddress: 'TCP'api.example.com:443
```

### 2. Including Protocol in Hostname
```
# Wrong
presentationAddress: 'TCP'https://api.example.com:443

# Correct
presentationAddress: 'TCP'api.example.com:443
```

### 3. Missing Transport Specification
```
# Wrong
presentationAddress: api.example.com:443

# Correct
presentationAddress: 'TCP'api.example.com:443
```

## Testing Your Implementation

You can test your LDAP entries with command-line tools:

```bash
# Search for all applicationEntity objects
ldapsearch -x -H ldap://localhost:389 -D "cn=admin,dc=example,dc=com" \
  -w password -b "dc=example,dc=com" \
  "(objectClass=applicationEntity)" cn presentationAddress description

# Search for specific service
ldapsearch -x -H ldap://localhost:389 -D "cn=admin,dc=example,dc=com" \
  -w password -b "dc=example,dc=com" \
  "(cn=web-portal)" presentationAddress

# Find services by port
ldapsearch -x -H ldap://localhost:389 -D "cn=admin,dc=example,dc=com" \
  -w password -b "dc=example,dc=com" \
  "(presentationAddress=*:443*)"
```

Or with the Rust LDAP client:

```bash
# Search for all applicationEntity objects
cargo run -- search --filter "(objectClass=applicationEntity)" \
  --attributes "cn,presentationAddress,description"

# Search for specific service
cargo run -- search --filter "(cn=web-portal)" \
  --attributes "presentationAddress"

# Find services by port
cargo run -- search --filter "(presentationAddress=*:443*)"
```

## Conclusion

RFC 1278 presentation addresses provide a standardized way to represent network service connection information in LDAP directories. By following the format `'TCP'hostname:port` for most internet services, you ensure that your LDAP directory properly represents network services in a way that can be understood by LDAP-aware applications and tools.

The key points to remember:

1. Always use single quotes around the transport type
2. Use fully qualified domain names when possible
3. Include port numbers for clarity
4. Multiple presentation addresses are allowed and useful for redundancy
5. Test your entries to ensure they're properly formatted

This standardized approach makes your LDAP directory a reliable source of truth for service discovery and network configuration management.

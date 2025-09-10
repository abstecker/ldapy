# RFC 1278 Presentation Address Guide

RFC 1278 defines the presentation address format for LDAP applicationEntity objects. This guide shows how to properly format presentation addresses for internet-hosted services.

## Basic RFC 1278 Format

```
'<transport>'<address>[':'<port>]['+'<additional-info>]
```

### Components

1. **Transport**: The transport protocol (enclosed in single quotes)
2. **Address**: Hostname, FQDN, or IP address
3. **Port**: Optional port number (preceded by colon)
4. **Additional Info**: Optional additional parameters (preceded by plus)

## Common Transport Types

### TCP Services (Most Common)

```
'TCP'hostname:port
```

### UDP Services

```
'UDP'hostname:port
```

### Other Transports

```
'X25'address                 # X.25 networks
'TP4'hostname:port          # ISO TP4
'CLNS'nsap-address          # Connectionless Network Service
```

## Practical Examples

### Web Services

```
# HTTPS service
presentationAddress: 'TCP'www.example.com:443

# HTTP service
presentationAddress: 'TCP'www.example.com:80

# Web service with API endpoint
presentationAddress: 'TCP'api.example.com:443

# Multiple endpoints for same service
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

# IMAP
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

### Cloud Services

```
# REST API
presentationAddress: 'TCP'api.example.com:443

# GraphQL endpoint
presentationAddress: 'TCP'graphql.example.com:443

# WebSocket service
presentationAddress: 'TCP'ws.example.com:443

# gRPC service
presentationAddress: 'TCP'grpc.example.com:443
```

### Microservices

```
# Internal service (private IP)
presentationAddress: 'TCP'10.0.1.100:8080

# Service mesh endpoint
presentationAddress: 'TCP'service.mesh.local:8080

# Load balancer endpoint
presentationAddress: 'TCP'lb.example.com:443
```

## Advanced Examples with Additional Info

### Services with Path Information

```
# REST API with base path (using additional info)
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

```aidl
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

## Best Practices

### 1. Use Fully Qualified Domain Names (FQDN)

```
# Good
presentationAddress: 'TCP'api.example.com:443

# Avoid (unless internal)
presentationAddress: 'TCP'api:443
```

### 2. Always Include Port Numbers

```
# Good
presentationAddress: 'TCP'web.example.com:443

# Less clear
presentationAddress: 'TCP'web.example.com
```

### 3. Group Related Services

```
# Email service with multiple protocols
presentationAddress: 'TCP'mail.example.com:993
presentationAddress: 'TCP'mail.example.com:143
presentationAddress: 'TCP'smtp.example.com:587
```

### 4. Use Descriptive Service Names

```
# Good
cn: customer-api-service

# Less descriptive
cn: api1
```

## Common Mistakes to Avoid

### 1. Missing Single Quotes Around Transport

```
# Wrong
presentationAddress: TCPapi.example.com:443

# Correct
presentationAddress: 'TCP'api.example.com:443
```

### 2. Incorrect Quote Usage

```
# Wrong
presentationAddress: "TCP"api.example.com:443

# Correct
presentationAddress: 'TCP'api.example.com:443
```

### 3. Including Protocol in Hostname

```
# Wrong
presentationAddress: 'TCP'https://api.example.com:443

# Correct
presentationAddress: 'TCP'api.example.com:443
```

## Testing Your Presentation Addresses

You can test your LDAP entries with the Rust client:

```bash
# Search for all applicationEntity objects
cargo run -- search --filter "(objectClass=applicationEntity)" --attributes "cn,presentationAddress,description"

# Search for specific service
cargo run -- search --filter "(cn=web-portal)" --attributes "presentationAddress"

# Find services by port
cargo run -- search --filter "(presentationAddress=*:443*)"
```

This RFC 1278 compliant format ensures that your LDAP directory properly represents network services in a standardized way that can be understood by LDAP-aware applications and tools.

# Penetration Testing Guide for Kao Backend

This document outlines the penetration testing procedures for the Kao backend application.

## Overview

Penetration testing is a critical part of ensuring the security of the Kao admin management system. This guide covers the automated and manual testing procedures for identifying security vulnerabilities.

## Testing Scope

The penetration test covers:

1. Authentication mechanisms
2. Authorization controls
3. Input validation
4. Session management
5. API security
6. File upload security
7. Error handling and information disclosure

## Automated Testing

### OWASP ZAP Baseline Scan

The OWASP ZAP baseline scan is run automatically as part of the CI/CD pipeline. The scan checks for:

- Common web application security flaws (OWASP Top 10)
- insecure direct object references
- Cross-site scripting (XSS)
- SQL injection
- Insecure deserialization
- components with known vulnerabilities

#### Running the Scan

```bash
# Interactive mode
./scripts/security/penetration-test.sh

# Specify target URL
./scripts/security/penetration-test.sh --url http://localhost:8080

# CI mode (non-interactive)
./scripts/security/penetration-test.sh --url http://localhost:8080 --ci

# Fail on critical issues
./scripts/security/penetration-test.sh --url http://localhost:8080 --fail-on-critical
```

#### Configuration

The ZAP configuration is stored in `scripts/security/owasp-config.yaml` and includes:

- Target URL
- Authentication credentials
- Scan policies
- Alert thresholds
- Test scenarios

## Manual Testing Procedures

### Authentication Bypass Testing

#### Test Cases

1. **Missing Authentication Header**
   - Request without Authorization header
   - Expected: 401 Unauthorized

2. **Invalid Token**
   - Request with malformed JWT token
   - Expected: 401 Unauthorized

3. **Expired Token**
   - Request with expired JWT token
   - Expected: 401 Unauthorized

4. **Invalid Token Signature**
   - Request with tampered JWT token
   - Expected: 401 Unauthorized

#### Test Commands

```bash
# Test missing auth header
curl -X GET http://localhost:8080/api/v1/auth/profile

# Test invalid token
curl -X GET http://localhost:8080/api/v1/auth/profile \
  -H "Authorization: Bearer invalid_token"

# Test expired token (generate expired token first)
curl -X GET http://localhost:8080/api/v1/auth/profile \
  -H "Authorization: Bearer <expired_token>"
```

### Brute Force Login Testing

#### Test Cases

1. **Rate Limiting Check**
   - Multiple login attempts from same IP
   - Expected: 429 Too Many Requests after threshold

2. **Account Lockout**
   - Multiple failed login attempts
   - Expected: Account lockout after threshold

#### Test Commands

```bash
# Test rate limiting
for i in {1..11}; do
  curl -X POST http://localhost:8080/api/v1/auth/login \
    -H "Content-Type: application/json" \
    -d '{"username":"test","password":"wrong"}' \
    -w "Attempt $i: HTTP %{http_code}\n"
done
```

### SQL Injection Testing

#### Test Cases

1. **Classic SQL Injection**
   - Input: `' OR '1'='1`
   - Expected: Error or parameterized query handling

2. **UNION-based Injection**
   - Input: `' UNION SELECT * FROM users--`
   - Expected: Error or query isolation

3. **Time-basedBlind Injection**
   - Input: `' AND SLEEP(5)--`
   - Expected: Error or query isolation

#### Test Commands

```bash
# Test classic SQL injection
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin\' OR \'1\'=\'1","password":"test"}'

# Test UNION-based injection
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin\' UNION SELECT * FROM users--","password":"test"}'
```

### Cross-Site Scripting (XSS) Testing

#### Test Cases

1. **Reflected XSS**
   - Input: `<script>alert('XSS')</script>`
   - Expected: Content is escaped or rejected

2. **DOM-based XSS**
   - URL: `http://localhost:8080/page#<script>alert('XSS')</script>`
   - Expected: Script is not executed

#### Test Commands

```bash
# Test reflected XSS
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"<script>alert(\"XSS\")</script>","password":"test"}'

# Test in username field
curl -X GET "http://localhost:8080/api/v1/users?username=<script>alert('XSS')</script>"
```

### Cross-Site Request Forgery (CSRF) Testing

#### Test Cases

1. **Missing CSRF Token**
   - Request without CSRF token
   - Expected: 403 Forbidden for state-changing requests

2. **Invalid CSRF Token**
   - Request with wrong CSRF token
   - Expected: 403 Forbidden

#### Test Commands

```bash
# Test missing CSRF token
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"Password1!"}'

# Test invalid CSRF token
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -H "X-CSRF-Token: invalid_token" \
  -d '{"username":"test","password":"Password1!"}'
```

### Authorization Bypass Testing

#### Test Cases

1. **Role-based Access Control**
   - Regular user accessing admin endpoint
   - Expected: 403 Forbidden

2. **Privilege Escalation**
   - User attempting to modify other user's data
   - Expected: 403 Forbidden or access controlled

#### Test Commands

```bash
# Test user accessing admin endpoint
# 1. Login as regular user
TOKEN=$(curl -s -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"user","password":"Password1!"}' | jq -r '.data.access_token')

# 2. Access admin endpoint
curl -X GET http://localhost:8080/api/v1/users \
  -H "Authorization: Bearer $TOKEN"
```

### Session Management Testing

#### Test Cases

1. **Session Fixation**
   - Reuse of session token
   - Expected: New session created on login

2. **Session Hijacking**
   - Session token in different context
   - Expected: Token invalidation

#### Test Commands

```bash
# Test session fixation
# 1. Obtain session token
TOKEN1=$(curl -s -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"Password1!"}' | jq -r '.data.access_token')

# 2. Attempt to reuse token after logout or timeout
curl -X GET http://localhost:8080/api/v1/auth/profile \
  -H "Authorization: Bearer $TOKEN1"
```

## Security Headers

The application should include the following security headers:

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Strict-Transport-Security: max-age=31536000; includeSubDomains`
- `Content-Security-Policy: default-src 'self'`
- `Cache-Control: no-store`
- `Pragma: no-cache`

## Vulnerability Remediation

### Critical Vulnerabilities

1. **Immediate Action Required**
2. **Escalate to Security Team**
3. **Implement Workaround if Necessary**
4. **Schedule Patch**

### High Vulnerabilities

1. **Address within 7 days**
2. **Implement temporary mitigation**
3. **Schedule permanent fix**

### Medium/Low Vulnerabilities

1. **Address within 30 days**
2. **Document technical debt**
3. **Schedule in roadmap**

## Continuous Security Testing

### Automated Scanning

Run automated security scans as part of CI/CD:

```yaml
# .github/workflows/security-scan.yml
name: Security Scan

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Run OWASP ZAP
        run: |
          chmod +x scripts/security/penetration-test.sh
          ./scripts/security/penetration-test.sh --ci
      
      - name: Upload scan report
        uses: actions/upload-artifact@v3
        with:
          name: security-scan-report
          path: reports/
```

### Manual Testing Schedule

- **Component Testing:** Every sprint
- **Integration Testing:** Every release
- **Penetration Testing:** Quarterly
- **Security Review:** Annually

## Contact

For security concerns or questions:
- Security Team: security@example.com
- Emergency: security-emergency@example.com

## Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [OWASP ZAP](https://owasp.org/www-project-zap/)
- [OWASP Testing Guide](https://owasp.org/www-project-web-security-testing-guide/)
- [CWE/SANS Top 25](https://cwe.mitre.org/top25/)

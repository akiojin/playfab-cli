# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly.

**Do NOT open a public issue.**

Instead, please send a report to the repository maintainers via GitHub's private vulnerability reporting feature:

1. Go to the repository's **Security** tab
2. Click **Report a vulnerability**
3. Provide a detailed description of the vulnerability

### What to include

- Description of the vulnerability
- Steps to reproduce
- Impact assessment
- Suggested fix (if any)

### Response timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix release**: Depends on severity

### Severity levels

- **Critical**: Remote code execution, credential exposure — patch within 24-48 hours
- **High**: Privilege escalation, data leakage — patch within 1 week
- **Medium**: Information disclosure, DoS — patch within 2 weeks
- **Low**: Minor issues — addressed in next regular release

## Security Best Practices

- Never commit `.env` files or API keys
- Use environment variables for all secrets (`PLAYFAB_TITLE_ID`, `PLAYFAB_DEV_SECRET_KEY`)
- Keep dependencies up to date (`cargo audit`)
- Use server-side APIs only (never client APIs)

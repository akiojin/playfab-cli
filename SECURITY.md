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

## Supply Chain Protection

Release binaries are distributed to users through the self-update mechanism, so unreviewed code must never reach a release.

- Auto-merge is enabled only for same-repository pull requests authored by the maintainer (`akiojin`)
- Pull requests from external contributors, forks, and bots (e.g. Dependabot) are never auto-merged; they are merged only after manual maintainer review
- `pull_request_target` workflows that hold secrets must not check out or execute pull request head code

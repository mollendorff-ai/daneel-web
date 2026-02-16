# Security Policy

## Important Context

daneel-web is an experimental research project, not production software. It is the observatory dashboard for the DANEEL cognitive architecture and is not designed to be deployed in security-sensitive environments.

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.2.x   | Yes       |
| < 0.2   | No        |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it privately:

**Email:** louis@mollendorff.ai

**Subject line:** `[SECURITY] daneel-web - <brief description>`

Please include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)

## What to Expect

- **Acknowledgment:** Within 72 hours of your report.
- **Assessment:** Within 2 weeks, we will confirm whether the issue is accepted and provide an expected timeline for a fix.
- **Disclosure:** We will coordinate disclosure with you. Please do not publish details until a fix is available.

## Scope

Given that daneel-web is a research project, severity is evaluated in the context of its intended use (local development, research deployments). Issues in the injection API proxy authentication, CORS configuration, WebSocket handlers, or static file serving are in scope.

# CI/CD Pipeline Documentation

## Overview
The PezkuwiChain Governance Platform uses GitHub Actions for comprehensive CI/CD automation.

## Workflows

### 1. Continuous Integration (ci.yml)
**Trigger:** Push to main/develop, Pull requests
**Jobs:**
- **Lint:** ESLint and TypeScript checking
- **Test:** Unit tests with coverage reporting
- **Build:** Multi-version Node.js builds
- **Security:** Snyk scanning and npm audit

### 2. Continuous Deployment (cd.yml)
**Trigger:** Push to main (staging), Tags (production)
**Environments:**
- **Staging:** Vercel deployment with E2E tests
- **Production:** AWS S3 + CloudFront deployment

### 3. Dependency Updates (dependency-update.yml)
**Trigger:** Weekly schedule (Monday midnight)
**Features:**
- Automated npm updates
- Security patches
- PR creation with test validation

### 4. E2E Tests (e2e-tests.yml)
**Trigger:** Push, PR, Daily schedule
**Coverage:**
- Multi-browser testing (Chrome, Firefox, Safari)
- Mobile testing
- Accessibility testing with axe-core

### 5. Code Quality (code-quality.yml)
**Trigger:** Pull requests
**Analysis:**
- SonarCloud code analysis
- Lighthouse performance metrics
- PR comments with results

## Configuration Files

### Dependabot
- Weekly npm updates
- GitHub Actions updates
- Grouped updates for related packages
- Auto-review assignments

### Code Owners
- Automated review requests
- Team-based ownership
- Security team oversight for sensitive modules

## Test Coverage Requirements
- Branches: 80%
- Functions: 80%
- Lines: 80%
- Statements: 80%

## Deployment Environments

### Staging
- URL: https://staging.pezkuwichain.gov
- Auto-deploy on main branch
- E2E test validation

### Production
- URL: https://pezkuwichain.gov
- Tag-based deployment (v*)
- CloudFront CDN
- Release notes generation

## Security Measures
- Snyk vulnerability scanning
- npm audit checks
- Dependabot security updates
- Code owner reviews for sensitive files
- Secret management via GitHub Secrets

## Performance Monitoring
- Bundle size checks
- Lighthouse CI metrics
- Build time optimization
- Coverage reports to Codecov

## Required Secrets
Configure these in GitHub repository settings:
- `CODECOV_TOKEN`
- `SNYK_TOKEN`
- `VERCEL_TOKEN`
- `VERCEL_ORG_ID`
- `VERCEL_PROJECT_ID`
- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `AWS_S3_BUCKET`
- `CLOUDFRONT_DISTRIBUTION`
- `SLACK_WEBHOOK`
- `SONAR_TOKEN`
- `STAGING_API_URL`
- `PROD_API_URL`

## Local Development
```bash
# Install dependencies
npm ci

# Run tests
npm test
npm run test:coverage

# Run E2E tests
npm run test:e2e

# Build application
npm run build

# Check bundle size
npx bundlesize
```
# Development Guide

## Project Overview

ArenaX is a comprehensive gaming platform that includes:

- **Backend Services**: Tournament management, matchmaking, user authentication
- **Smart Contracts**: Token economics, escrow systems, prize distribution
- **Frontend**: Modern web application for tournament participation
- **DevOps**: CI/CD pipelines, container orchestration

## Architecture Principles

- **Microservices**: Independent services with clear boundaries
- **Event-Driven**: Asynchronous communication between services
- **Blockchain Integration**: Smart contracts for trustless operations
- **Scalable**: Horizontal scaling for tournament spikes

## Getting Started

### Prerequisites

- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose
- Stellar CLI tools

### Local Development Setup

1. Clone the repository
2. Set up environment variables (see env.example files)
3. Run `make setup` for initial project setup
4. Run `make dev` to start all services

## Contributing

1. Create feature branches from `main`
2. Follow conventional commit messages
3. Ensure tests pass before PR submission
4. Update documentation for API changes

## Code Quality

- Rust: Follow `clippy` recommendations
- TypeScript: ESLint and Prettier configuration
- Testing: Unit tests required, integration tests encouraged
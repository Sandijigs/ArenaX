# ArenaX Development Makefile

.PHONY: help setup dev build test clean docker-build docker-up docker-down

# Default target
help:
	@echo "Available commands:"
	@echo "  setup       - Initial project setup"
	@echo "  dev         - Start development environment"
	@echo "  build       - Build all services"
	@echo "  test        - Run all tests"
	@echo "  clean       - Clean build artifacts"
	@echo "  docker-build - Build Docker images"
	@echo "  docker-up    - Start services with Docker Compose"
	@echo "  docker-down  - Stop Docker services"

setup:
	@echo "Setting up development environment..."
	# Backend setup
	@cd backend && cargo check
	# Frontend setup
	@cd frontend && npm install
	# Contracts setup
	@cd contracts && cargo check

dev:
	@echo "Starting development environment..."
	# Start services in development mode

build:
	@echo "Building all services..."
	# Backend build
	@cd backend && cargo build --release
	# Frontend build
	@cd frontend && npm run build
	# Contracts build
	@cd contracts && cargo build --release

test:
	@echo "Running tests..."
	# Backend tests
	@cd backend && cargo test
	# Frontend tests
	@cd frontend && npm test
	# Contract tests
	@cd contracts && cargo test

clean:
	@echo "Cleaning build artifacts..."
	@cd backend && cargo clean
	@cd contracts && cargo clean
	@cd frontend && rm -rf .next

docker-build:
	@echo "Building Docker images..."
	@docker-compose build

docker-up:
	@echo "Starting services with Docker Compose..."
	@docker-compose up -d

docker-down:
	@echo "Stopping Docker services..."
	@docker-compose down
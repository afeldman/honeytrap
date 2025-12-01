.PHONY: help build test clean install dev fmt lint check audit coverage bench run run-server docker docker-build docker-push k8s-deploy docs release

# Variables
CARGO := cargo
DOCKER := docker
KUBECTL := kubectl
PROJECT_NAME := honeytrap
VERSION := $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
DOCKER_REGISTRY := ghcr.io
DOCKER_IMAGE := $(DOCKER_REGISTRY)/yourusername/$(PROJECT_NAME)

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Default target
.DEFAULT_GOAL := help

## help: Show this help message
help:
	@echo "$(GREEN)HoneyTrap Makefile$(NC)"
	@echo "===================="
	@echo ""
	@echo "Available targets:"
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' | sed -e 's/^/ /'

## build: Build all binaries in release mode
build:
	@echo "$(GREEN)Building release binaries...$(NC)"
	$(CARGO) build --release --workspace

## build-cli: Build CLI binary
build-cli:
	@echo "$(GREEN)Building honeytrap CLI...$(NC)"
	$(CARGO) build --release --bin honeytrap

## build-server: Build server binary
build-server:
	@echo "$(GREEN)Building honeytrap server...$(NC)"
	$(CARGO) build --release --bin honeytrap-server

## test: Run all tests
test:
	@echo "$(GREEN)Running tests...$(NC)"
	$(CARGO) test --workspace --all-features

## test-unit: Run unit tests only
test-unit:
	@echo "$(GREEN)Running unit tests...$(NC)"
	$(CARGO) test --lib --workspace

## test-integration: Run integration tests only
test-integration:
	@echo "$(GREEN)Running integration tests...$(NC)"
	$(CARGO) test --test '*' --workspace

## check: Quick compile check
check:
	@echo "$(GREEN)Running cargo check...$(NC)"
	$(CARGO) check --workspace --all-features

## fmt: Format code
fmt:
	@echo "$(GREEN)Formatting code...$(NC)"
	$(CARGO) fmt --all

## fmt-check: Check code formatting
fmt-check:
	@echo "$(GREEN)Checking code formatting...$(NC)"
	$(CARGO) fmt --all -- --check

## lint: Run clippy linter
lint:
	@echo "$(GREEN)Running clippy...$(NC)"
	$(CARGO) clippy --all-targets --all-features -- -D warnings

## audit: Security audit
audit:
	@echo "$(GREEN)Running security audit...$(NC)"
	$(CARGO) audit

## coverage: Generate code coverage report
coverage:
	@echo "$(GREEN)Generating coverage report...$(NC)"
	$(CARGO) tarpaulin --out Html --output-dir target/coverage --all-features
	@echo "$(GREEN)Coverage report: target/coverage/index.html$(NC)"

## bench: Run benchmarks
bench:
	@echo "$(GREEN)Running benchmarks...$(NC)"
	$(CARGO) bench --all-features

## clean: Clean build artifacts
clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	$(CARGO) clean
	rm -rf target/
	rm -rf crates/*/target/

## install: Install binaries to ~/.cargo/bin
install:
	@echo "$(GREEN)Installing binaries...$(NC)"
	$(CARGO) install --path crates/honeytrap-cli
	$(CARGO) install --path crates/honeytrap-server

## dev: Run in development mode with auto-reload
dev:
	@echo "$(GREEN)Starting development server...$(NC)"
	$(CARGO) watch -x 'run --bin honeytrap-server'

## run: Run CLI
run:
	@echo "$(GREEN)Running honeytrap CLI...$(NC)"
	$(CARGO) run --bin honeytrap

## run-server: Run server
run-server:
	@echo "$(GREEN)Running honeytrap server...$(NC)"
	$(CARGO) run --bin honeytrap-server

## ci: Run all CI checks (fmt, lint, test)
ci: fmt-check lint test
	@echo "$(GREEN)✓ All CI checks passed!$(NC)"

## pre-commit: Run pre-commit checks
pre-commit: fmt lint test-unit
	@echo "$(GREEN)✓ Pre-commit checks passed!$(NC)"

## docs: Generate and open documentation
docs:
	@echo "$(GREEN)Generating documentation...$(NC)"
	$(CARGO) doc --no-deps --all-features --open

## docs-build: Build documentation without opening
docs-build:
	@echo "$(GREEN)Building documentation...$(NC)"
	$(CARGO) doc --no-deps --all-features

## docker: Build and run with docker-compose
docker:
	@echo "$(GREEN)Starting with docker-compose...$(NC)"
	$(DOCKER) compose -f docker/docker-compose.yml up -d

## docker-dev: Start development environment
docker-dev:
	@echo "$(GREEN)Starting development environment...$(NC)"
	$(DOCKER) compose -f docker/docker-compose.dev.yml up

## docker-build: Build all Docker images
docker-build: docker-build-server docker-build-cli docker-build-dev

## docker-build-server: Build server Docker image
docker-build-server:
	@echo "$(GREEN)Building server Docker image...$(NC)"
	$(DOCKER) build -t $(PROJECT_NAME)-server:latest -f docker/Dockerfile.server .
	$(DOCKER) tag $(PROJECT_NAME)-server:latest $(PROJECT_NAME)-server:$(VERSION)

## docker-build-cli: Build CLI Docker image
docker-build-cli:
	@echo "$(GREEN)Building CLI Docker image...$(NC)"
	$(DOCKER) build -t $(PROJECT_NAME)-cli:latest -f docker/Dockerfile.cli .
	$(DOCKER) tag $(PROJECT_NAME)-cli:latest $(PROJECT_NAME)-cli:$(VERSION)

## docker-build-dev: Build development Docker image
docker-build-dev:
	@echo "$(GREEN)Building development Docker image...$(NC)"
	$(DOCKER) build -t $(PROJECT_NAME)-dev:latest -f docker/Dockerfile.dev .

## docker-build-test: Build test Docker image
docker-build-test:
	@echo "$(GREEN)Building test Docker image...$(NC)"
	$(DOCKER) build -t $(PROJECT_NAME)-test:latest -f docker/Dockerfile.test .

## docker-build-alpine: Build minimal Alpine image
docker-build-alpine:
	@echo "$(GREEN)Building minimal Alpine image...$(NC)"
	$(DOCKER) build -t $(PROJECT_NAME)-alpine:latest -f docker/Dockerfile.alpine .
	$(DOCKER) tag $(PROJECT_NAME)-alpine:latest $(PROJECT_NAME)-alpine:$(VERSION)

## docker-push: Push Docker image to registry
docker-push: docker-build-server
	@echo "$(GREEN)Pushing Docker image...$(NC)"
	$(DOCKER) tag $(PROJECT_NAME)-server:$(VERSION) $(DOCKER_IMAGE):$(VERSION)
	$(DOCKER) tag $(PROJECT_NAME)-server:$(VERSION) $(DOCKER_IMAGE):latest
	$(DOCKER) push $(DOCKER_IMAGE):$(VERSION)
	$(DOCKER) push $(DOCKER_IMAGE):latest

## docker-stop: Stop docker-compose services
docker-stop:
	@echo "$(YELLOW)Stopping docker-compose services...$(NC)"
	$(DOCKER) compose -f docker/docker-compose.yml down

## docker-logs: View docker-compose logs
docker-logs:
	$(DOCKER) compose -f docker/docker-compose.yml logs -f

## docker-test: Run tests in Docker
docker-test: docker-build-test
	@echo "$(GREEN)Running tests in Docker...$(NC)"
	$(DOCKER) run --rm $(PROJECT_NAME)-test:latest

## docker-clean: Remove Docker images
docker-clean:
	@echo "$(YELLOW)Cleaning Docker images...$(NC)"
	$(DOCKER) rmi -f $(PROJECT_NAME)-server:latest $(PROJECT_NAME)-cli:latest $(PROJECT_NAME)-dev:latest || true

## k8s-deploy: Deploy to Kubernetes
k8s-deploy:
	@echo "$(GREEN)Deploying to Kubernetes...$(NC)"
	$(KUBECTL) apply -f crates/honeytrap-server/k8s-deployment.yaml

## k8s-delete: Delete from Kubernetes
k8s-delete:
	@echo "$(YELLOW)Deleting from Kubernetes...$(NC)"
	$(KUBECTL) delete -f crates/honeytrap-server/k8s-deployment.yaml

## k8s-logs: View Kubernetes logs
k8s-logs:
	$(KUBECTL) logs -f deployment/honeytrap-server -n honeytrap

## k8s-status: Check Kubernetes deployment status
k8s-status:
	$(KUBECTL) get pods -n honeytrap
	$(KUBECTL) get services -n honeytrap

## release: Create a new release (bump version and tag)
release:
	@echo "$(GREEN)Creating release v$(VERSION)...$(NC)"
	@read -p "Confirm release v$(VERSION)? [y/N] " confirm && [ "$$confirm" = "y" ]
	git tag -a "v$(VERSION)" -m "Release v$(VERSION)"
	git push origin "v$(VERSION)"
	@echo "$(GREEN)✓ Release v$(VERSION) created!$(NC)"

## deps: Install development dependencies
deps:
	@echo "$(GREEN)Installing development dependencies...$(NC)"
	rustup component add rustfmt clippy
	$(CARGO) install cargo-watch cargo-tarpaulin cargo-audit

## deps-update: Update dependencies
deps-update:
	@echo "$(GREEN)Updating dependencies...$(NC)"
	$(CARGO) update

## all: Build everything
all: clean build test lint
	@echo "$(GREEN)✓ Build complete!$(NC)"

## size: Show binary sizes
size:
	@echo "$(GREEN)Binary sizes:$(NC)"
	@ls -lh target/release/honeytrap* 2>/dev/null || echo "No release binaries found. Run 'make build' first."

## bloat: Analyze binary size
bloat:
	@echo "$(GREEN)Analyzing binary size...$(NC)"
	cargo bloat --release --bin honeytrap-server

## tree: Show dependency tree
tree:
	@echo "$(GREEN)Dependency tree:$(NC)"
	$(CARGO) tree

## outdated: Check for outdated dependencies
outdated:
	@echo "$(GREEN)Checking for outdated dependencies...$(NC)"
	$(CARGO) outdated

## fix: Auto-fix lint issues
fix:
	@echo "$(GREEN)Auto-fixing lint issues...$(NC)"
	$(CARGO) fix --allow-dirty --allow-staged
	$(CARGO) clippy --fix --allow-dirty --allow-staged

## systemd-install: Install systemd service
systemd-install:
	@echo "$(GREEN)Installing systemd service...$(NC)"
	cd crates/honeytrap-server && sudo ./install.sh

## version: Show version information
version:
	@echo "Project: $(PROJECT_NAME)"
	@echo "Version: $(VERSION)"
	@echo "Rust: $$(rustc --version)"
	@echo "Cargo: $$(cargo --version)"

# GEMINI.md - Developer's Guide

This document provides a technical overview of the "Angels Project" for developers. For user-facing information, setup, and API documentation, please see `README.md`.

## 1. Project Overview

The "Angels Project" is a mystical and spiritual research web application built with Rust. It serves both static content and a dynamic, AI-powered blog.

- **Backend:** Actix-web (Rust)
- **Frontend:** Handlebars for server-side rendering, with htmx for partial page updates to create a more dynamic user experience without a full frontend framework.
- **Database:** SurrealDB
- **AI Integration:** Google Gemini for programmatic content generation.

## 2. Codebase Architecture

The project follows a modular, separation-of-concerns pattern, organized into the following main directories within `src/`:

- `controllers`: Handles incoming HTTP requests and routes them to the appropriate logic.
- `models`: Defines the data structures and their validation.
- `db`: Contains all database-related logic, including connection initialization and queries.
- `utils`: Provides shared utilities for tasks like AI integration, environment variable management, and file system operations.

### Key Architectural Points:

- **Dual Controller Pattern:** The blog functionality is split between two main controllers:
    - `blog_controller.rs`: Serves server-rendered HTML pages and handles `htmx` requests for partial page updates (e.g., loading more articles).
    - `blog_api_controller.rs`: Provides a JSON-based API for blog resources. This is where the AI content generation is triggered (`/blogs/ai/creator`).
- **Placeholder AI Controller:** The file `src/controllers/ai_controller.rs` is currently an empty placeholder and is not used. All AI-related logic is initiated from the `blog_api_controller`.
- **Builder Pattern:** The models, particularly `src/models/blog_model.rs`, make use of the builder pattern (e.g., `BlogArticleBuilder`). This provides a fluent and robust way to construct complex objects, especially after receiving data from the AI API.

## 3. Cross-Platform Compilation & Deployment

The production environment for this project is AlmaLinux running on ARM (Ampere) architecture. To facilitate deployment from other architectures (e.g., macOS or x86 Linux), the following setup is used:

- **Target Architecture:** `aarch64-unknown-linux-gnu` (glibc 2.34).
- **Tooling:** We use `cargo-zigbuild` to handle cross-compilation without needing complex linker setups.
    - Command: `cargo zigbuild --target aarch64-unknown-linux-gnu.2.34 --release`
- **TLS Strategy:** To avoid dependencies on the target system's OpenSSL version, `Cargo.toml` is configured to use `rustls` with `ring`. This ensures the binary is more portable.
    - Dependency setup: `reqwest` is used with `default-features = false` and `rustls-tls-manual-roots-no-provider`.
- **Scripts:**
    - `build_alma_linux.sh`: Automates the cross-compilation process using `zigbuild`.
    - `copy_exe.sh`: Deploys the compiled binary to the production server via `scp`.

### Requirements for Developers:
If you need to build for production, ensure you have `zig` and `cargo-zigbuild` installed:
```bash
brew install zig
cargo install cargo-zigbuild
```

## 4. Development Conventions

- **Modularity:** Functionality is organized into distinct modules. When adding new features, follow the existing structure.
- **Error Handling:** The project uses Rust's `Result` type for error handling. Avoid panics.
- **Configuration:** All configuration is managed through environment variables loaded via `dotenv`. See `.env.example` for a complete list of required variables.
- **Database Schema:** The SurrealDB schema is defined in `static/db/schema.surql` and is applied automatically on application startup. When making changes to data models, update this schema accordingly.
- **Logging:** The `env_logger` crate is used for logging. Use the `RUST_LOG` environment variable to control log levels during development.
- **Templating:** Use Handlebars (`.hbs` files) for all HTML rendering. Partials are located in `static/templates/partials`.
- **API Responses:** API controllers should return JSON responses. Use the `response_utils.rs` for standardizing responses where applicable.
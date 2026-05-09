# Angels Project - Mystical Research Web Application

## Overview

Angels Project is a modern web application focused on mystical and spiritual research, particularly centered around angelology, theology, and related topics. The application provides a platform for exploring different theological concepts, blog articles, and resources related to angels and spiritual studies.

## Technical Architecture

### Frontend

- HTML templates using Handlebars templating engine
- Responsive design
- Integration with HTMX for dynamic content loading

### Backend

- Built with Rust and Actix-web framework
- RESTful API endpoints for blog content
- Integration with AI content generation (Google Gemini AI)

### Database

- SurrealDB for data storage
- Schema-based record validation
- Full-text search capabilities

## Pages

- `/` - Home page
- `/about.html` - About page
- `/topics.html` - Topics exploration
- `/resources.html` - Resources page
- `/blog.html` - Blog home
- `/blog/article` - Blog article view
- `/contact.html` - Contact information

## API Endpoints (Backend - JSON Responses)

These endpoints provide data typically in JSON format and are used for backend operations and data retrieval.

- `/blogs/search` - **GET** - Search blog articles based on a query parameter `q`. Returns a JSON list of blog articles matching the search term.
- `/blogs/ai/creator` - **GET** - Triggers the AI content generation process to create a new blog article. Returns a JSON response of the newly created blog article.
- `/blogs` - **GET** - Retrieves all active blog articles. Returns a JSON list of blog articles.

## Page Routes (Frontend - HTML Rendering)

These routes serve HTML content, rendering pages using Handlebars templates.

- `/` - **GET** - Home page. Renders the main index page.
- `/about` - **GET** - About page (HTML fragment, HTMX). Renders the "about" section.
- `/about.html` - **GET** - Full About page. Renders the complete About page HTML.
- `/topics` - **GET** - Topics page (HTML fragment, HTMX). Renders the topics section.
- `/topics.html` - **GET** - Full Topics page. Renders the complete Topics page HTML.
- `/resources` - **GET** - Resources page (HTML fragment, HTMX). Renders the resources section.
- `/resources.html` - **GET** - Full Resources page. Renders the complete Resources page HTML.
- `/blog` - **GET** - Blog Home page (HTML fragment, HTMX). Renders the blog home section.
- `/blog/article` - **GET** - Blog Article page. Renders a single blog article (currently AI-generated example content).
- `/blog_home.html` - **GET** - Full Blog Home page. Renders the complete Blog Home page HTML.
- `/contact` - **GET** - Contact page (HTML fragment, HTMX). Renders the contact section.
- `/contact.html` - **GET** - Full Contact page. Renders the complete Contact page HTML.
- `/{filename:.*}` - **GET** - Serves static files (like scripts, stylesheets, images) from the file system.

## Features

- **Static Content Pages:** Serves static HTML pages for:
  - Homepage (`/`)
  - About Us (`/about.html`)
  - Contact (`/contact.html`)
  - Topics (`/topics.html`)
  - Resources (`/resources.html`)
  - Blog Home (`/blog_home.html`)
- **AI-Powered Blog Article Generation:** Leverages the Google Gemini API to generate blog articles based on provided prompts. Accessible via `/blogs/ai/creator`.
- **Blog Article Search:** Implements a search functionality to find blog articles based on keywords. Accessible via `/blogs/search?q={search_term}`.
- **SurrealDB Integration:** Uses SurrealDB as the database to store and manage blog articles and potentially other data.
- **Handlebars Templating:** Employs Handlebars for dynamic HTML template rendering, making it easy to create and modify page layouts.
- **Environment Variable Configuration:** Utilizes `.env` files for configuration management, allowing for easy customization of database connections, API keys, and server settings.
- **Structured Project:** Organized into modules for controllers, models, database interactions, utilities, and configuration, promoting maintainability and scalability.

## Tech Stack

- **Backend Framework:** [Actix-web](https://actix.rs/) - A powerful, pragmatic, and extremely fast web framework for Rust.
- **Templating Engine:** [Handlebars](https://handlebarsjs.com/) - A minimal templating engine for Rust, used for rendering HTML.
- **Database:** [SurrealDB](https://surrealdb.com/) - A scalable, cloud-native database, perfect for modern web applications.
- **AI Service:** [Google Gemini API](https://ai.google.dev/gemini-api) - Used for generating blog content.
- **Configuration Management:** [dotenv](https://crates.io/crates/dotenv) - For loading environment variables from `.env` files.
- **Logging:** [log](https://crates.io/crates/log) and [env_logger](https://crates.io/crates/env_logger) - For structured logging and debugging.
- **Serialization/Deserialization:** [serde](https://serde.rs/) and [serde_json](https://crates.io/crates/serde_json) - For handling JSON data and data structures.
- **Input Validation:** [validator](https://crates.io/crates/validator) - For validating user inputs (though not heavily used in the provided code, it's available for future use).
- **Asynchronous Runtime:** [tokio](https://tokio.rs/) - Actix-web and SurrealDB rely on Tokio for asynchronous operations.
- **HTTP Client:** [reqwest](https://crates.io/crates/reqwest) - For making HTTP requests to the Google Gemini API.

## Development

### Prerequisites

- **Rust Toolchain:** Ensure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).
- **SurrealDB:** You need a running instance of SurrealDB. You can download and run it locally following the instructions on the [official website](https://surrealdb.com/docs/start/install). Make sure it's accessible at the address specified in your `.env` file (default is `0.0.0.0:8000`).

### Installation

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/merodeadorNocturno/mystical_research
    cd mystical_research
    ```

2.  **Environment Configuration:**

    - Create a `.env` file in the project root directory.
    - Copy the contents of `.env.example` (if provided, otherwise refer to the "Configuration" section below) into your `.env` file and adjust the values accordingly.
    - **Important:** You will need to obtain a Google Gemini API key and set it in the `.env` file as `AI_GOOGLE_API_KEY`.

3.  **Database Setup:**
    - The schema for SurrealDB is defined in `static/db/schema.surql`.
    - The application automatically applies this schema on startup if SurrealDB is running and accessible.

### Configuration

The application is configured using environment variables, primarily loaded from the `.env` file. Here are the key variables you can configure:

- **Database Configuration:**

  - `DB_ADDRESS`: Address of your SurrealDB instance (default: `0.0.0.0:8000`).
  - `DB_NAMESPACE`: SurrealDB namespace to use (default: `mystical_namespace`).
  - `DB_NAME`: SurrealDB database name to use (default: `mystical_database`).
  - `DB_USERNAME`: SurrealDB root username (default: `mystical_admin`).
  - `DB_PASSWORD`: SurrealDB root password (default: `mystical_password`).

- **Server Configuration:**

  - `SERVER_ADDRESS`: Server binding address (default: `0.0.0.0`).
  - `SERVER_PORT`: Server binding port (default: `8081`).
  - `SERVER_PROTOCOL`: Server protocol (default: `http`).
  - `PAGE_TITLE`: Title of the website (default: `Mystical Research`).
  - `TEMPLATE_PATH`: Path to the Handlebars templates directory (default: `./static/templates`).

- **AI Configuration:**

  - `AI_GOOGLE_API_KEY`: Your Google Gemini API key.
  - `AI_REQUEST_URL`: Base URL for the Google Gemini API (default: `https://generativelanguage.googleapis.com/v1beta/models/`).
  - `GOOGLE_MODEL`: Google Gemini model to use (default: `gemini-2.0-flash`).

- **Logging:**
  - `RUST_LOG`: Sets the logging level. Common values are `off`, `error`, `warn`, `info`, `debug`, `trace` (default: `debug`).

**Example `.env` file:**

```env
DB_ADDRESS=0.0.0.0:8000
DB_NAMESPACE=mystical_ns
DB_NAME=angels_db
DB_USERNAME=root
DB_PASSWORD=password

SERVER_ADDRESS=127.0.0.1
SERVER_PORT=8081
SERVER_PROTOCOL=http
PAGE_TITLE="Angels Project"
TEMPLATE_PATH="./static/templates"

AI_GOOGLE_API_KEY=YOUR_GOOGLE_GEMINI_API_KEY
AI_REQUEST_URL=https://generativelanguage.googleapis.com/v1beta/models/
GOOGLE_MODEL=gemini-2.0-flash

RUST_LOG=info
```

### Running the Application

1.  **Start SurrealDB:** Ensure your SurrealDB instance is running.
2.  **Run the application:**

    ```bash
    cargo run
    ```

3.  **Access the application:** Open your web browser and navigate to the address configured in your `.env` file (default: `http://127.0.0.1:8081`).

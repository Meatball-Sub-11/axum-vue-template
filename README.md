# Axum + Vue Full-stack Template

A production-ready template for building modern web applications with a Rust (Axum) backend and a Vue.js (Vite) frontend.

### Tech Stack
* **Backend:** Rust with Axum
* **Frontend:** Vue.js with Vite and TypeScript
* **Database:** PostgreSQL (with `sqlx`)
* **Styling:** Tailwind CSS

---

## Table of Contents

* [Project Structure](#project-structure)
* [Prerequisites](#prerequisites)
* [Installation](#installation)
* [Configuration](#configuration)
* [Usage](#usage)
* [Contributing](#contributing)
* [License](#license)

## Project Structure
This repository is a monorepo containing two main packages:
* `/backend`: The Rust (Axum) API server.
* `/frontend`: The Vue.js (Vite) client-side application.

## Prerequisites

Before you begin, ensure you have the following installed on your system:
* **Rust:** [Install Rust and Cargo](https://www.rust-lang.org/tools/install)
* **Node.js and npm:** [Install Node.js](https://nodejs.org/) (npm is included)

## Installation

This project is a monorepo containing two separate applications: the `backend` (Axum) and the `frontend` (Vue). You will need to install dependencies for both.

1.  **Clone the repository:**
    ```bash
    git clone <your-new-repository-url>
    cd <your-project-name>
    ```

2.  **Set up the Backend (Axum):**
    Navigate to the backend directory and build the project to download and compile all necessary crates.
    ```bash
    cd backend
    cargo build
    cd .. 
    ```

3.  **Set up the Frontend (Vue):**
    Navigate to the frontend directory and install the npm packages.
    ```bash
    cd frontend
    npm install
    cd ..
    ```

## Configuration

The backend requires environment variables to run.

1.  Navigate to the `backend` directory.
2.  Create a `.env` file by copying the example file:
    ```bash
    cp .env.example .env
    ```
3.  Edit the new `.env` file and fill in your local configuration values (e.g., your database password and a new `JWT_SECRET`).

## Usage

To run the application for local development, you need to start both the backend API and the frontend development server in **two separate terminal windows.**

**1. Run the Backend (Axum API):**
* In your first terminal, navigate to the `backend` directory and start the server:
    ```bash
    cd backend
    cargo run
    ```
* The API will now be running on `http://127.0.0.1:3000`.

**2. Run the Frontend (Vue App):**
* In your second terminal, navigate to the `frontend` directory and start the development server:
    ```bash
    cd frontend
    npm run dev
    ```
* The development server will start on `http://localhost:5173`.

**3. Access the application:**
* Open your web browser and navigate to the frontend URL: **`http://localhost:5173`**. The Vue application will automatically communicate with the Axum API.

## Contributing

If you'd like to contribute to this template, please fork the repository and create a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
# Stack Overflow APIs

## Getting Started

Follow these steps to set up your development environment for the Stack Overflow APIs project.

### Prerequisites

Ensure you have the following installed on your machine:

- Docker
- Rust and Cargo

### Installation Steps

Step 1: **Install Docker**

Follow the instructions on the [Docker website](https://docs.docker.com/get-docker/) to install Docker for your operating system.

Step 2: **Install Postgres image**

Pull the official Postgres image from Docker Hub:

```bash
docker pull postgres
```

Step 3: **Start Postgres instance**

Run the following command to start a new Postgres container:

- Container name: `stack-overflow-db`
- POSTGRES_USER: `ghost0x01`
- POSTGRES_PASSWORD: `ghost0x01`
- POSTGRES_DB: `stack-overflow-db`
- Port: `5432`

```bash
docker run --name stack-overflow-db -e POSTGRES_USER=ghost0x01 -e POSTGRES_PASSWORD=ghost0x01 -e POSTGRES_DB=stack-overflow-db -p 5432:5432 -d postgres
```

Step 4: **Access the Postgres instance**

Connect to the running Postgres container:

- Container name or ID: `stack-overflow-db`
- PostgreSQL user: `ghost0x01`
- Database name: `stack-overflow-db`

```bash
docker exec -it stack-overflow-db psql -U ghost0x01 -d stack-overflow-db
```

### Running Migrations

Let's run these migrations against your local Postgres server instance.

Step 1: **Install sqlx-cli**

First, install [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli). This is SQLx's associated command-line utility for managing databases, migrations, and more:

```shell
cargo install sqlx-cli
```

Step 2: **Execute migrations**

Now you can execute migrations by running:

```shell
sqlx migrate run
```

**NOTE:** If you ever want to revert the migrations, simply run:

```shell
sqlx migrate revert
```

### Start your app

Step 1: **Create `.env` file and add DATABASE_URL content**

- POSTGRES_USER: `ghost0x01`
- POSTGRES_PASSWORD: `ghost0x01`
- Host: `localhost`
- Port: `5432`
- Database Name: `stack-overflow-db`

```.env
# Postgres

DATABASE_URL=postgres://ghost0x01:ghost0x01@localhost:5432/stack-overflow-db

```

Step 2: **Start the Application**

To enable live reloading during development, install the `cargo-watch` package:

```bash
cargo install cargo-watch
```

Then, start the application with the following command:

```bash
cargo watch -x 'run'
```

You are now ready to start developing with the Stack Overflow APIs project!

## API Endpoints

- Postman collection: [Link](./docs/Stack%20Overview%20Flow%20APIs.postman_collection.json)

| Endpoint                    | URL                 | Method     | Description                                    | Request Body                                                                                                  |
| --------------------------- | ------------------- | ---------- | ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| Get All Questions           | `/api/v1/questions` | **GET**    | Retrieves a list of all questions.             | None                                                                                                          |
| Create a New Question       | `/api/v1/question`  | **POST**   | Creates a new question.                        | `title` (string) - The title of the question. <br> `description` (string) - The body content of the question. |
| Delete a Question           | `/api/v1/question`  | **DELETE** | Deletes a specific question by its ID.         | `question_uuid` (integer) - The ID of the question to delete.                                                 |
| Get All Answers by Question | `/api/v1/answers`   | **GET**    | Retrieves a list of all answers of a question. | `question_uuid` (integer) - The ID of the question to read.                                                   |
| Create a New Answer         | `/api/v1/answer`    | **POST**   | Creates a new answer.                          | `question_uuid` (string) - The ID of the question. <br> `content` (string) - The body content of the answer.  |
| Delete an Answer            | `/api/v1/answer`    | **DELETE** | Deletes a specific answer by its ID.           | `answer_uuid` (integer) - The ID of the answer to delete.                                                     |

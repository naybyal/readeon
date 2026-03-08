# Readeon

Readeon is a fast, private, offline-first terminal application that I built for tracking books, reading sessions, notes, and reading analytics. 

Most book tracking tools require internet connectivity and focus heavily on social features rather than building personal knowledge. My goal with Readeon is to provide a local reading intelligence system. It runs entirely locally in your terminal, stores your data reliably in an embedded SQLite database, and supports fetching book metadata using an ISBN from the Open Library API.

## Features

- **Book Library Management:** Manage personal book inventory including titles, authors, and reading status (Unread, Reading, Finished, Abandoned).
- **Session Tracking:** Log focused reading sessions to calculate total reading time, pages read, and reading speed (pages per hour).
- **Notes and Quotes:** Store quotes and structured notes linked directly to the page numbers of the books being read.
- **Analytics Dashboard:** Automatically compute reading habits, track finished books, total pages read, and reading speeds via the Statistics view.
- **Metadata Fetching:** Quickly add books by pressing `Ctrl+F` and entering an ISBN to auto-populate metadata from the Open Library.
- **Offline First:** All data is stored in a clean SQLite schema in your project directory (`readeon.db`), allowing full operation offline except when explicitly looking up an ISBN.
- **Fast UI:** Built natively in Rust using `ratatui` and `crossterm` to offer a responsive, keyboard-driven interface.

## System Architecture

Readeon follows a modular layered architecture to separate concerns:
1. **UI Layer (`src/ui`):** Terminal rendering using Ratatui.
2. **Domain Layer (`src/domain`):** Purely structured business logic (Books, Sessions, Notes, Stats).
3. **Data Layer (`src/db`):** SQLite repositories providing safe abstractions over SQL queries.
4. **External API Layer (`src/api`):** Asynchronous metadata retrieval using `reqwest`.

## Setup

First, ensure you have Rust installed. Clone the repository and run the cargo build command.

1. Build the application:
```bash
cargo build --release
```

2. Run the application:
```bash
cargo run
```

On first launch, Readeon will automatically create the required `readeon.db` SQLite database in the current working directory to store your library data.

Please refer to `USAGE.md` for a comprehensive list of keyboard commands to operate the application.

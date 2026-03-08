# Using Readeon

Readeon is a purely keyboard-driven interface. Below are all the commands you need to operate the application.

## Library View (Main Screen)
This is the default view when you open the application. It lists all your books on the left and shows statistics or details on the right.

- `j` or `Down Arrow`: Move the selection down.
- `k` or `Up Arrow`: Move the selection up.
- `Enter`: Open the detailed view for the currently selected book.
- `a`: Open the "Add Book" screen to manually enter a book or search by ISBN.
- `s`: Open the "Stats Dashboard" to view your reading analytics.
- `q`: Quit the application.

## Book Detail View
This view shows the specific reading sessions and notes associated with a single book.

- `q` or `Esc`: Return to the Library view.
- *(Note: Adding notes `n`, reading sessions `r`, deleting `d`, and updating progress `p` are planned features for extending the detail screen functionality but are visually mapped in the MVP placeholders.)*

## Add Book View
This view provides a form to add a new book to your library.

- `Tab` or `Down Arrow`: Move focus to the next input field (Title, Author, ISBN, Pages, Year).
- `Shift+Tab` or `Up Arrow`: Move focus to the previous input field.
- `Backspace`: Delete the last character in the currently focused input.
- Any Alphanumeric Key: Type text into the focused input field.
- `Ctrl+F`: Fetch book metadata. Enter an ISBN into the ISBN field, then press `Ctrl+F` to query Open Library and auto-populate the remaining fields.
- `Enter`: Save the book and return to the Library view.
- `Esc`: Cancel adding a book and return to the Library view without saving.

## Stats Dashboard View
This view compiles your reading analytics across your entire library and session history.

- `q` or `Esc`: Return to the Library view.

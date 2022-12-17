# HN PDF

HNPDF is a simple web app that allows archiving PDF files shared on Hacker News. It works by scraping the website for links to PDFs in stories and comments, and then saving them to a local SQLite database. The app also allows users to filter the text by keyword, so they can only download documents that are relevant to them.


## Building

`$ cargo build --release`

## Running 

`$ SQLLITE_DB_NAME="../archive.sql" SCRAPE_INTERVAL=600 ./target/release/hnpdf`

App should be accessible on `http://127.0.0.1:8080`
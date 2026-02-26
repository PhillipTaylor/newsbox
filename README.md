
# Newsbox

This is a command line news browser written by ChatGPT:

https://chatgpt.com/share/69837910-c390-8009-a808-20bd0f36b3fb

## Demo

Here is a video demo of me opening the app, moving through articles
and reading them at the command line (requires `w3m` installed).

![Demo](NewsboxDemo.gif)

## Run the app

To run this app use:

```bash
cargo run
```

Cargo is a Rust build tool.

## How to use the app.

press `r` to populate the feed, navigate up and down with `j` and `k` keys and press `o` to open the article in your browser or `p` to view the article in the command line if you have `w3m` installed.

Use capital `J` and `K` to page through results, use `/` to start a search and `q` to quit.

In `w3m` you can use `j` and `k` or `Enter` to scroll up and down and `q` to return to newsbox.

The app looks for a `feeds.yml` in the current working directory. Reddit RSS feeds do not work but every other websites does.

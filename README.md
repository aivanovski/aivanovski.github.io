
# Run the app
App will start local server on http://localhost:3000
```
cargo run
```

# Render static html
The command will render static html files into ./dist folder.
Then any http server can server it.
```
cargo run --output ./dist
python3 -m http.server 8000 --directory dist
```


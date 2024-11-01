
### Installation

* Install wasm-pack

    ```bash
    cargo install wasm-pack
    ```

* Install webpack

    ```bash
    npm install webpack webpack-cli --save-dev
    ```

* Node WebSocket Server

    ```bash
    npm install ws
    ```

## Build and Run

```bash
wasm-pack build --target web && npm run serve
```

### Run

```bash
# Terminal 1
wasm-pack build --target web
npm run serve

# Terminal 2
npm run start-server
```

Open the browser and navigate to `http://localhost:4000`

```

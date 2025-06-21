# Cloudflare Rust Worker - IP Address Service

A Cloudflare Worker written in Rust that demonstrates using the `worker-rs` crate to build serverless functions. This worker returns the visitor's IP address by reading Cloudflare's `CF-Connecting-IP` header.

## Features

- 🦀 **Written in Rust** - Compiled to WebAssembly for maximum performance
- 🌐 **IP Detection** - Returns the real client IP address from Cloudflare headers
- 🚀 **Router-based** - Uses `worker::Router` for clean request handling
- ⚡ **Edge Computing** - Runs on Cloudflare's global edge network

## What This Worker Does

This worker exposes a single endpoint that returns the visitor's IP address:

- **GET /** - Returns the client's IP address as plain text
- If the `CF-Connecting-IP` header is not available, it returns "unknown"

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- A Cloudflare account

### Installation

1. Clone this repository:
```bash
git clone <your-repo-url>
cd cf-rust-worker
```

2. Install dependencies:
```bash
cargo build
```

3. Configure your `wrangler.toml` with your account details

### Development

Run the worker locally:
```bash
wrangler dev
```

Visit `http://localhost:8787` to see your IP address.

### Deployment

Deploy to Cloudflare Workers:
```bash
wrangler deploy
```

## Usage Examples

### Basic Request
```bash
curl https://your-worker.your-subdomain.workers.dev/
# Output: 192.168.1.100
```

### JavaScript Fetch
```javascript
fetch('https://your-worker.your-subdomain.workers.dev/')
  .then(response => response.text())
  .then(ip => console.log('Your IP:', ip));
```

### Using in a Web Page
```html
<!DOCTYPE html>
<html>
<head>
    <title>IP Checker</title>
</head>
<body>
    <h1>Your IP Address</h1>
    <p id="ip">Loading...</p>
    
    <script>
        fetch('https://your-worker.your-subdomain.workers.dev/')
            .then(response => response.text())
            .then(ip => {
                document.getElementById('ip').textContent = ip;
            })
            .catch(error => {
                document.getElementById('ip').textContent = 'Error: ' + error.message;
            });
    </script>
</body>
</html>
```

## Code Structure

```
src/
├── lib.rs          # Main worker logic with router and IP handling
└── new.rs          # (if present) Alternative implementation
```

### Key Components

- **Router Setup**: Uses `worker::Router` for handling HTTP requests
- **IP Extraction**: Reads the `CF-Connecting-IP` header provided by Cloudflare
- **Error Handling**: Gracefully handles cases where the IP header is not available

## Configuration

The worker is configured via `wrangler.toml`. Key settings:

```toml
name = "cf-rust-worker"
main = "build/worker/shim.mjs"
compatibility_date = "2023-05-18"

[build]
command = "cargo install -q worker-build && worker-build --release"
```

## How It Works

1. **Request Handling**: The `fetch` function receives incoming HTTP requests
2. **Router Processing**: Requests are routed through `worker::Router`
3. **IP Extraction**: The `handle_get` function extracts the IP from Cloudflare headers
4. **Response**: Returns the IP address as plain text

## WebAssembly Compilation

This worker compiles Rust code to WebAssembly (`wasm32-unknown-unknown` target) for execution on Cloudflare's V8 isolates. All dependencies must be compatible with the WebAssembly target.

## Dependencies

- `worker` - Cloudflare Workers runtime for Rust
- `console_error_panic_hook` - Better error messages in development

## Limitations

- The worker relies on Cloudflare's `CF-Connecting-IP` header
- Only works when deployed to Cloudflare Workers (not other platforms)
- Returns "unknown" if the header is not present

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally with `wrangler dev`
5. Submit a pull request

## Resources

- [Cloudflare Workers Rust Documentation](https://developers.cloudflare.com/workers/languages/rust/)
- [workers-rs GitHub Repository](https://github.com/cloudflare/workers-rs)
- [Cloudflare Workers Examples](https://developers.cloudflare.com/workers/examples/)
- [Wrangler CLI Documentation](https://developers.cloudflare.com/workers/wrangler/)

## License

This project is open source and available under the [MIT License](LICENSE).

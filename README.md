
# Cacheman

Cacheman is a lightweight, Rust-based asset caching agent similar to Varnish, designed to efficiently manage and serve cached assets with a RESTful API interface. It uses `actix-web` for HTTP handling and provides configurable caching with TTL (Time-To-Live) settings.

## Features

- Serve cached assets via a web server.
- RESTful API for asset management (add, remove, retrieve).
- Configurable TTL for cached assets.
- Background cache cleanup task to remove expired assets.
- Built with `actix-web` and `tokio` for performance and scalability.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/chrisjchandler/cacheman.git
   cd cacheman
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the project:

   ```bash
   cargo run
   ```

## Usage

Cacheman provides a RESTful API to interact with cached assets.

### Cache an Asset

```bash
curl -X POST http://localhost:8080/cache -H "Content-Type: application/json" -d '{"key": "image.png", "data": "base64encodeddata", "ttl_seconds": 3600}'
```

### Retrieve an Asset

```bash
curl http://localhost:8080/cache/image.png
```

### Remove an Asset

```bash
curl -X DELETE http://localhost:8080/cache/image.png
```

## Configuration

You can adjust TTL and other cache settings in the source code. More advanced configurations may be added later.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/chrisjchandler/cacheman/blob/main/LICENSE.txt) file for details.

## Contributing

Feel free to fork this project, submit issues, and contribute. Pull requests are welcome!

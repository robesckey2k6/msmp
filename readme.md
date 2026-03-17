# MSMP — Minecraft Server Multiplexer Proxy

MSMP is a lightweight Rust-based reverse proxy that allows multiple Minecraft servers to be hosted behind a single port. It routes incoming connections to the correct backend server by intercepting the Minecraft handshake packet and reading the subdomain of the server address the client is connecting from.

---

## How It Works

When a Minecraft client connects, it sends a handshake packet containing the server address it is trying to reach (e.g. `survival.example.com`). MSMP intercepts this packet before any connection is forwarded, extracts the subdomain (`survival`), and looks it up in a config file to find the corresponding backend port.

Once the target port is found, MSMP:
1. Opens a TCP connection to the backend server on `127.0.0.1:<port>`
2. Forwards the original handshake packet so the backend receives it normally
3. Spawns a thread to forward data from the backend server back to the client
4. Continuously forwards data from the client to the backend in the main loop

This creates a transparent TCP tunnel — the backend server and the client behave as if they are connected directly.

---

## Architecture

```
Client
  │
  ▼
MSMP :2001
  │  reads handshake
  │  extracts subdomain
  │
  ├──► survival   → 127.0.0.1:25565
  ├──► creative   → 127.0.0.1:25566
  └──► minigames  → 127.0.0.1:25567
```

---

## Configuration

Servers are defined in a file named `servers` in the project root. Each line follows the format:

```
server_name,port
```

**Example:**
```
survival,25565
creative,25566
minigames,25567
```

The `server_name` must match the subdomain the client uses to connect. For example, a client connecting to `survival.example.com` will be routed to port `25565`.

---

## Usage

**1. Configure your servers file**
```
survival,25565
creative,25566
```

**2. Start your Minecraft servers**

Make sure each backend server is running on its configured port on `127.0.0.1`.

**3. Run MSMP**
```bash
cargo run
```

MSMP will start listening on port `2001`.

**4. Connect from Minecraft**

In the Minecraft client, connect to:
```
survival.yourdomain.com:2001
```

---

## Dependencies

| Crate | Purpose |
|---|---|
| `tokio` | Async runtime for handling concurrent connections |
| `dotenv` | Loads environment variables from a `.env` file |

---

## Limitations

- All backend servers must be running on `127.0.0.1` (localhost)
- No graceful error handling if a backend server is unreachable — the connection will panic
- Buffer size is fixed at 1024 bytes which may cause issues with large packets
- No TLS/SSL support

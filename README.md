# telnet-server

I did this as both a learning exercise and in a desire to become more conversant with `Rust`.

## What it does

It builds a multicast broadcast server where clients get connnected through TCP and then each client's stream data to the server is streamed to all the clients except the client that sent it.

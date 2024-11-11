Simple example showing communication between two apps using `interprocess`.
In this example, the server waits for the client to send multiple lines until it sends "END\n"

```bash
cargo build --workspace

target/debug/server
target/debug/client
```

## Examples

- http_trigger: responds with the current time to get or post request
- timer_trigger: logs a message every minute.

## Build and Run

1. Clone or download the repo and open the example folder.
2. Ensure you have both Rust and the Azure Functions Core Tools installed.
3. If you are not using Windows update the host.json file by removing the .exe to the end of the defaultExecutablePath.

```json
"customHandler": {
    "description": {
        "defaultExecutablePath": "target/debug/azure_functions_example"
    }
}
```

4. Build the project and run it from the terminal.

```console
cargo build
func start
```

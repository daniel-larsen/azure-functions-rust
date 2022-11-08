## Build and Run

1. Clone or download the repo and open the example folder. 
2. Ensure you have both Rust and the Azure Functions Core Tools installed. 
3. If you are using Windows update the host.json file by adding .exe to the end of the defaultExecutablePath. 
```json
"customHandler": {
    "description": {
        "defaultExecutablePath": "target/debug/azure_functions_example.exe"
    }
}
```
4. Build the project and run it from the terminal.
```console
cargo build
func start
```
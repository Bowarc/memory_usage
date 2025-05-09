## Display the memory usage of a given process, formated as decimal or binary

This is probably another [reader](https://github.com/Bowarc/reader) situation, where I make a tool just for it to already exist,  
But i couldn't find something simple enough that gave meaningful results, so ig here is my attempt at it.  

## How to use:
```console
cargo install --git https://github.com/Bowarc/memory_usage
```
This will compile and copy the binary to your `.cargo/bin` directory.  

```
Usage: memory_usage <process> [prefix]

Arguments:
  Required:            
                      <process>
                      The name of the target process to monitor.

  Optional:            
                      [prefix]
                      Format for displaying memory sizes:
                      `decimal` for base 10 (default),
                      `binary` for base 2.

Example:
  memory_usage my_process decimal
  memory_usage my_process binary
```

## Example output
```
┌────────┬────────────────┬──────────┬────────────────┐
│  Pid   │      Name      │  Memory  │ Virtual memory │
├────────┼────────────────┼──────────┼────────────────┤
│ 307568 │ storage_server │ 10.24 Mb │   707.38 Mb    │
└────────┴────────────────┴──────────┴────────────────┘
```

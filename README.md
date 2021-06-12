# Flusher CLI

### Flusher command line apps that interact with flusher tools rest api only for ***flush method purpose***
### This CLI are written in rust

## How to use 

```
Flusher CLI tool 0.5.0

USAGE:
    flusher-cli [FLAGS] [OPTIONS] --address <address>

FLAGS:
        --dry-run    To dry run purpose
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --address <address>    Flusher address
        --app <app-name>       Application name , it should be same in vault [default: flusher]
        --config <config>      Config address path [default: flusher.yaml]
```

## Example
```
$ RUST_LOG=info ./flusher-cli --address=http://flusher.cluster.local --config=./flusher.yaml
```

## Example config flusher.yaml

```yaml
requestId: '3192e-qewe-232321' # Change this anytime you need to run flush / delete keys you redis, you don't need to add whitespace on everytime when you want to create another pr to be merged
redis:
  operation: FlushAll   # for now available only is FlushAll// FlushAllAsync |FlushDB // FlushDbAsync (need redis version 4.+) // DelKeyByPrefix (you need to specify database and key pattern)
  address: "the-redis-address.cluster.lokal"
  port: 6392
  # master: "mymaster"  # Uncomment this if you want to do in sentinel mode redis
  # key: "widget*"
  # database: 1
  # auth: true          # Uncomment this if your redis has auth , you should put field 'flusherPassword' to your vault as an redis password and sentinel password.. use the same password for sentinel and redis .

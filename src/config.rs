
struct ConsulConfig {
    address: Option<String>,        // --consul-address
    datacenter: Option<String>      // --consul-datacenter
}

struct Config {
    input: Option<String>,          // --input
    output: Option<String>,         // --output
    stdin: Option<bool>,            // --stdin
    stdout: Option<bool>,           // --stdout
    stderr: Option<bool>,           // --stderr
    consul: Option<ConsulConfig>
}
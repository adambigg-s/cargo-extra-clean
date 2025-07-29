# cargo-extra-clean
cli tool to recursively "cargo clean" all Rust projects in a directory 

my main "projects" folder was approaching 70 gb, mainly all from cargo's stuff and i wanted a faster way to clean it

```
cargo-extra-clean <root-directory>
```
this will find all Rust projects in that directory and prompt you [y/n] to clean them

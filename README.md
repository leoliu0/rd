# red
Very simple utility to replace strings in text files

# installation
```bash
cargo install rd
```

# usage

```bash
rd "replace" "with" -f file.txt
```

replace inplace by

```bash
rd "replace" "with" -f file.txt -i
```

delete matching lines

```bash
rd "to_delete" -f file.txt
```
performance

Much faster than GNU sed, but slower than sd

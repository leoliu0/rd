# red
Very simple utility to replace strings in text files

# installation

glone this repo and cd to it
cargo install --path .

# usage

```bash
red "replace" "with" -f file.txt
```

replace inplace by

```bash
red "replace" "with" -f file.txt -i
```

delete matching lines

```bash
red "to_delete" -f file.txt
```

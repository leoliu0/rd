# rd
Very simple utility to replace(r)/delete(d) strings in text files

## installation
```bash
cargo install rd
```

## usage

```bash
rd "replace" "with" -f file.txt
```
by default, it uses regex, for string literals

```bash
rd -s "replace" "with" -f file.txt
```

### replace inplace by

```bash
rd "replace" "with" -f file.txt -i
```

### delete matching lines

```bash
rd "to_delete" -f file.txt -d
```

### modify multiple files
```bash
rd "replace" "with" -f *.txt
```
with fd
```bash
rd "replace" "with" -f $(fd '.*\.txt')
```

## performance

Much faster than GNU sed (2x), but slower than sd (2x) for large files. Similar to sd for small files

## Good Design Choices
1. Do not touch the file unless there is a change. This is useful when modifying a long list of files and do not want to leave a trail in every files pass on the command line

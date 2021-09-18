# Caesar
A small program to de/en-crypt with the caesar procedure.

## Usage
- For interactive use: 
```shell
 $ cargo run
```

- For noninteractive use: 
```shell
 $ cargo run <encrypt/decrypt> <key1> <key2> <text>
```

Note: key1 is not allowed to be divisible by 2 or 13

Note: Just ascii characters and spaces are allowed in the text!

## Example

This encrypts the text "Hello world" with the key (3, 7):
```shell
 $ cargo run encrypt 3 7 "Hello world"
```

## License
This software is licensed under the GPLv3 license.

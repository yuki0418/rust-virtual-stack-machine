# Rust Virtual Stack Machine
This machine runs Program language which is written in Reverse Polish Notation.

## Run a program
```bash
$ cargo run -- fib.txt
```

If you want to write inline programu, run the following command.
```bash
$ cargo run
```

Then type the program in the console.

## Grammer
This program language is written in Reverse Polish Notation.

### Example 1
```
1 1 + puts
Output: 2
```

### Example 2
```
/x 10 def
/y 20 def
{ x y < } { x } { y } if
puts
Output: 10
```

### Example 3
```
/double { 2 * } def
5 double puts
Output: 10
```

## Reference
This code is based on the following book.

[Rustで作るプログラミング言語](https://www.amazon.co.jp/Rust%E3%81%A7%E4%BD%9C%E3%82%8B%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E-%E2%80%94%E2%80%94-%E3%82%B3%E3%83%B3%E3%83%91%E3%82%A4%E3%83%A9%EF%BC%8F%E3%82%A4%E3%83%B3%E3%82%BF%E3%83%97%E3%83%AA%E3%82%BF%E3%81%AE%E5%9F%BA%E7%A4%8E%E3%81%8B%E3%82%89%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E%E3%81%AE%E6%96%B0%E6%BD%AE%E6%B5%81%E3%81%BE%E3%81%A7-%E4%BD%90%E4%B9%85%E7%94%B0-%E6%98%8C%E5%8D%9A/dp/4297141922)

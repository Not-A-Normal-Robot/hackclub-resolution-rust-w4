# Luminia

A small programming language made for Week 4 of Hack Club Resolution Rust.

## Dependencies
You'll need Cargo, which you can get in https://rustup.rs/

## Installation

Run the following:
```
cargo install hcrrw4-luminia-cli
```

Then you can use the installed `luminia_cli` executable.

For example, to run a script: `luminia_cli run script.lum`
Alternatively, use the repl: `luminia_cli repl`

## Example Script

```
# file: script.lum
# Comments use #

def greet(name)
    println("Hello, " + name + "!");
end;

greet("world");

x = 10;
y = 3;
println(x + y);

def factorial(n)
    if n < 2 then
        return 1;
    end;
    return n * factorial(n - 1);
end;

result = factorial(5);
println(result);

i = 1;
while i < 6 do
    println(i);
    i = i + 1;
end;

if 10 > 5 then
    println("ten is greater");
else
    println("this won't print");
end;
```

## New Features
- Internal support for Rust functions
- `println()` and `print()`
- `eprintln()` and `eprint()`
- `panic()`

## Syntax changes
- removed `puts`

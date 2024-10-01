# kitty-sessionizer

Converts Kitty Session JSON to Session File

## Usage

1. Let sessionizer run kitty command for you

```shell
kitty-sessionizer > my-session.kitty
```

1. Pipe `kitty @ ls` into sessionizer

```shell
kitty @ ls | kitty-sessionizer --stdin > my-session.kitty
```

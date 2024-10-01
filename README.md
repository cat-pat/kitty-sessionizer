# kitty-sessionizer

Converts Kitty Session JSON to Session File

## Usage

1. Let sessionizer run kitty command for you

```shell
kitty-sessionizer > ~/.config/kitty/my-session.kitty
```

1. Pipe `kitty @ ls` into sessionizer

```shell
kitty @ ls | kitty-sessionizer --stdin > ~/.config/kitty/my-session.kitty
```

Then you can use the session file

```shell
# 'kitty --session' reads from its .config folder
kitty --session my-session.kitty
```

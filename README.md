
# Simple status monitor for swaybar or swaywm (Sway Window Manager)

This repository has a simple utility to dig some information from Linux system
and barf it out periodically so it will end up in status bar of Sway Window
Manager (swaywm)


### Buiding
After compiling the binary "gargo build -r"

### Installing / Setting up

edit .sway/config and add line ```status_command .sway/swaybar_status```

### example
./target/release/swaybar_status

```
2w 5d 22h 4m (99.1%) | 0.57 (2/3305) | 97 ⚡ | 2024-02-20T16:16 (+02:00)
2w 5d 22h 4m (99.1%) | 0.57 (5/3307) | 97 ⚡ | 2024-02-20T16:16 (+02:00)
2w 5d 22h 4m (99.1%) | 0.61 (1/3325) | 97 ⚡ | 2024-02-20T16:16 (+02:00)
```

# Ramfox

Ramfox is a small utility written in Rust designed to manage your Firefox profile in RAM, enhancing performance by leveraging the speed of temporary file storage. By moving your Firefox profile to a temporary file system (tempfs), Ramfox optimizes browser performance and responsiveness.

## Features

- **Automatic Profile Management**

  Ramfox automatically handles the movement of your Firefox profile to RAM, ensuring optimized performance without manual intervention.

- **Configuration Options**

  Customize Ramfox to suit your needs with flexible configuration options, allowing you to specify your Firefox profile, auto-start preferences, and synchronization intervals.

- **Efficient Resource Usage**

  By utilizing temporary file storage, Ramfox minimizes disk I/O operations, reducing latency and enhancing overall browsing speed.

## Config

Create a file in `~/.config` called `ramfox.toml` and edit the File with your preferred text editor.

```shell
$ touch ~/.config/ramfox.toml
```

### Example config

```toml
[profile]
name = "j65l8yy7.default" # Your Firefox profile to be moved to RAM.
auto-start = true # Auto-start Firefox after moving the profile to RAM.
bin = "/usr/bin/firefox" # The binary path of your preferred Firefox client.

[profile.sync]
# The format needs to follow a numeric prefix and unit suffix
# hours/hour/h | mins/min/m | secs/sec/s
# The timer needs to be at least 60 seconds.
every = "5mins" # Automatically backup profile.
```

## Contributing

Contributions to Ramfox are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/kiramily/ramfox).

## License

This project is licensed under the GPLv3 License. See the [LICENSE](LICENSE) file for details.

# ISOW (ISO Week)

ISO Week is a simple CLI program that prints the local or UTC date in the ISO 8601 format with week presented as Www, e.g. W04. 

Tom Scott's [Why It's Already 2020](https://www.youtube.com/watch?v=D3jxx8Yyw1c) inspired me to write this. This is not designed to print the full ISO 8601 date and time, only the week date. For more technical information on ISO week, [visit Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).

## Requirements

### Prerequisites

- Rust 2018 update

## Authors

- **Anthony Foxclaw** - _Initial work_ - [tonytins](https://github.com/tonytins)

See also the list of [contributors](https://github.com/tonytins/isow/contributors) who participated in this project.

## Examples

- ``isow``: ``2006-W52-7``
- ``isow -d``: ``7``
- ``isow -w``: ``W52``
- ``isow -y``: ``2006``

By default, ISOW uses your local time zone. Adding ``-u`` will switch to UTC.

## License

This project is licensed under the MPL 2.0 license - see the [LICENSE](LICENSE) file for details.

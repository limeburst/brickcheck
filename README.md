# brickcheck

`brickcheck` checks your firmware for brickers!

Whether a binary will brick an MCU or not is determined by checking the 'secured' and 'mass-erase' bits of the flash configuration field.

## Installing

```
$ cargo install brickcheck
```

## Using

```
$ brickcheck brick.bin
WARNING: This firmware will brick your K20 when flashed!!!

$ brickcheck kitten.bin
This firmware is as safe as a kitten.
```

## Supported MCUs

* Kinetis K20

## Changelog

### 0.1.0

Released on July 21, 2016.

## License

This project is licensed under the terms of the MIT license.

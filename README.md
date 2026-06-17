# jelly-pac

`jelly-pac` is a peripheral access crate for the Jelly FPGA platform.

It provides no-std friendly access to common hardware blocks used in Jelly-based designs and builds on `jelly-lib` and `jelly-mem_access` for lower-level support.

## Features

- `std` is enabled by default
- core modules are available without `std`

## Modules

- `communication_pipe`
- `i2c`
- `i2c_device`
- `interval_timer`
- `spi`
- `video_dma_control`

## License

MIT
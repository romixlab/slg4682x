# slg46824x

> SLG46824 and SLG46826 Mixed-Signal Matrix HAL with support for reading, programming and real time rewiring of the
> matrix.

This crate is `no_std` when used with `default-features = false`.

`embedded-hal` I2c trait is used to communicate with the device, so any implementation from, e.g., embassy or other
HALs should work.

## Links

* [SLG46824 datasheet](https://www.renesas.com/en/document/dst/slg46824-datasheet)
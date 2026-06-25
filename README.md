# cw32l012-pac

Peripheral access crate for the CW32L012 microcontroller, generated from the
CW32L012 SVD.

## Blinky example

The `blinky` example targets the CW32L012 Blue Board. The board LED is connected
to `PC13`.

The example:

- enables the GPIOB peripheral clock with the `SYSCTRL.AHBEN` key `0x5A5A`
- configures `PC13` as a digital GPIO
- configures `PC13` as push-pull output
- toggles `PC13` by writing `GPIOB.BSRR`

One important CW32L010 GPIO detail: `DIR = 0` means output, and `DIR = 1` means
input.

Build the example:

```powershell
cargo build --example blinky
```

Run it with `probe-rs`:

```powershell
cargo run --example blinky
```
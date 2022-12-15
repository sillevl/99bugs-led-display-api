# 99-Bugs led display API

A small HTTP REST API that lets POST images that wil be displayed on the 99-Bugs led TV

## Example usage

```
POST / HTTP/1.1
Host: localhost:3000
Content-Type: image/png
Content-Length: 610
```

When sending an image, tis important to set the `Content-Type` to `image/png`, and set the `Content-Length` with a value that corresponds with the image byte size. (Note: a maximum of 1024 bytes is supported at the moment).

You can also POST a base64 string to the `/base64` route.

## Docker

```bash
docker run -d -p 3000:3000 --device=/dev/spidev0.0 99bugs/99bugs-led-display-api
```

Or using a local build with docker-compose:

```bash
docker-compose up -d --build
```

<!-- ## Cross-compilation using Docker

The Dockerfile is setup to use cross compilation tools.

This is inspired by:
* https://medium.com/@wizofe/cross-compiling-rust-for-arm-e-g-raspberry-pi-using-any-os-11711ebfc52b
* http://whitfin.io/speeding-up-rust-docker-builds/ -->

## SPI settings Raspberry Pi

To change the default add `spidev.bufsiz=32768` to `/boot/cmdline.txt` and reboot. Where `32768` is the maximum size you want to allow. (before `rootwait`)

Source: https://raspberrypi.stackexchange.com/questions/65595/spi-transfer-fails-with-buffer-size-greater-than-4096

## SystemD service script

If you don't use docker, you will need a SystemD service script to start the service at bootup.

```
[Unit]
Description=99bugs Meme Machine API
After=network.target

[Service]
Type=simple
Restart=always
RestartSec=1
User=pi
ExecStart=/home/pi/99bugs-led-display-api/target/release/api-99bugs-display

[Install]
WantedBy=multi-user.target
```

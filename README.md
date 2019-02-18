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


## Docker

```
 docker run -d -p 3000:3000 --device=/dev/spidev0.0 99bugs/99bugs-led-display-api
```

## SPI settings Raspberry Pi

To change the default add `spidev.bufsiz=32768` to `/boot/cmdline.txt` and reboot. Where `32768` is the maximum size you want to allow. (before `rootwait`)

Source: https://raspberrypi.stackexchange.com/questions/65595/spi-transfer-fails-with-buffer-size-greater-than-4096
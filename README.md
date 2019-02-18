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
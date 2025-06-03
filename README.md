# Tango Fox

Tango Fox is a bare minimum FTP server that I wrote for the purpose of transferring photographs from my camera to my laptop. `libunftp` does all the heavy lifting for me.

## Usage

```shell
# starts a server that will automatically shut down after five minutes
$ tangofox --timeout 5m
Ftp running on port 2121
```

At present, usernames and passwords and ssh keys and SSL and just... security of any kind in general—none of that stuff is supported. Use at your own risk.

Or you can risk corrupting your images with crappy card readers and/or damaging your card slots with constant in-and-out. I know which option I picked, but reasonable people could easily reach different conclusions!

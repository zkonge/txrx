# txrx
Transfer file in LAN is so easy.

## Purpose

1. Finding machine IP before executing netcat
2. Multiplatform transfer

is collapsing

## Usage

Just run

```bash
$ txrx file_you_want_to_tansfer
```

You will receive

```
Here is your connection token: txrx://jZHWAqn+Euap/nd7qf4rfgoKCgF/AAAB
```

Run txrx at another machine with token above

```bash
$ txrx txrx://jZHWAqn+Euap/nd7qf4rfgoKCgF/AAAB
```

✨ That's all

## TODO

- [ ] Progrssbar with speed
- [ ] Support directory transfer
- [ ] Improve security
- [ ] NAT holepunching
- [ ] QR Code and phone APP

## License

MIT

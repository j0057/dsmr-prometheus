# DSMR5-P1 to Prometheus logger

This reads the output of a DSMR5 P1 port and exposes the metrics as a
Prometheus endpoint.

## Installing

After installation, create a systemd drop-in file using `systemctl edit
dsmr-prometheus.service`, and inside a `[Service]` section, add an
`Environment=DSMR_PROMETHEUS_ARGS=` line, providing either the `--file`,
`--connect` or `--serial` options. The default configuration is to open
the `/dev/ttyUSB0` serial port at 115200 baud.

## Developing

See the output of the P1 port on stdout:

    socat file:/dev/ttyUSB0,b115200,raw -

Host the stats on some TCP port, to be able to connect a TcpReader on the dev
machine that is not in the fuse cabinet: 

    socat file:/dev/ttyUSB0,b115200,raw tcp-listen:4000,reuseaddr

## Links

- [DSMR5 P1 Companion Standard][dsmr_p1]

Playgrounds:

- https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=eaba528113ee0f1403fe735211b79db1
- https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=128d38e342a53b40bf9f130fc223cf19
- https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a9fef7a02f046c1bc458669094ba3010

[dsmr_p1]: https://www.netbeheernederland.nl/_upload/Files/Slimme_meter_15_a727fce1f1.pdf

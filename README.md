# DSMR5-P1 to Prometheus logger

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

## P1 telegram

    /ISK5\2M550E-1012

    1-3:0.2.8(50)                                               Version 5.0
    0-0:1.0.0(220611162528S)                                    Timestamp: 2022-06-11T16:25:28
    0-0:96.1.1(4530313233343536373839303132333435)              Equipment identifier: E0123456789012345
    1-0:1.8.1(006024.008*kWh)                                   T1 electricity delivered to client: 6024.008 kWh
    1-0:1.8.2(005051.668*kWh)                                   T2 electricity delivered to client: 5051.668 kWh
    1-0:2.8.1(001375.008*kWh)                                   T1 electricity delivered by client: 1375.008 kWh
    1-0:2.8.2(003272.570*kWh)                                   T2 electricity delivered by client: 3272.570 kWh
    0-0:96.14.0(0001)                                           Tariff indicator: 1
    1-0:1.7.0(00.000*kW)                                        Actual power delivered: 0.000 kW
    1-0:2.7.0(03.106*kW)                                        Actual power received:  3.106 kW
    0-0:96.7.21(00010)                                          Number of power failures in any phase: 10
    0-0:96.7.9(00002)                                           Number of long power failures in any phase: 2
    1-0:99.97.0(1)(0-0:96.7.19)(180228084605W)(0000000486*s)    Power failure event log: 20180228T08:46:05, 8m06s
    1-0:32.32.0(00007)                                          Phase 1 voltage sags: 7
    1-0:32.36.0(00001)                                          Phase 1 voltage swells: 1
    0-0:96.13.0()                                               Text message: -
    1-0:32.7.0(242.6*V)                                         Instantaneous L1 voltage: 242.6 V
    1-0:31.7.0(012*A)                                           Instantaneous L1 current: 12 A
    1-0:21.7.0(00.000*kW)                                       Instantaneous L1 power delivered: 0.000 kW
    1-0:22.7.0(03.059*kW)                                       Instantaneous L1 power received: 3.059 kW
    0-1:24.1.0(003)                                             Device type: 3
    0-1:96.1.0(4730313233343536373839303132333435)              Equipment identifier: G0123456789012345
    0-1:24.2.1(220611162510S)(03814.705*m3)                     Meter reading; Capture time: 2022-06-11:16:25:10; 3814.705 m3
    !ABCD

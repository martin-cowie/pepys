# Testing

In lieu of automated Rust drive integration testing, use these to kick the tyres.

* GetSystemDateAndTime
```
curl -s -X POST -d @./data/GetSystemDateAndTime.xml http://localhost:8088/picam/device-management | xmllint --format -
```

* GetDeviceInformation
```
curl -s -X POST -d @./data/GetDeviceInformation.xml http://localhost:8088/picam/device-management | xmllint --format -

```

* GetNTP
```
curl -s -X POST -d @./data/GetNTP.xml http://localhost:8088/picam/device-management | xmllint --format -
```

* GetNetworkInterfaces
```
curl -s -X POST -d @./data/GetNetworkInterfaces.xml http://localhost:8088/picam/device-management | xmllint --format -
```
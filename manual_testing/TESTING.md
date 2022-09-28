# Testing

In lieu of automated Rust drive integration testing, use these to kick the tyres.

## Device Management

* GetSystemDateAndTime
```
curl -s -X POST -d @./data/GetSystemDateAndTime.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetDeviceInformation
```
curl -s -X POST -d @./data/GetDeviceInformation.xml http://localhost:8088/pepys/device_service | xmllint --format -

```

* GetNTP
```
curl -s -X POST -d @./data/GetNTP.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetNetworkInterfaces
```
curl -s -X POST -d @./data/GetNetworkInterfaces.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetCapabilities
```
curl -s -X POST -d @./data/GetCapabilities.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetRelayOutputs
```
curl -s -X POST -d @./data/GetRelayOutputs.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetServices
```
curl -s -X POST -d @./data/GetServices.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

## Imaging

* GetOptions
```
curl -s -X POST -d @./data/GetOptions.xml http://localhost:8088/pepys/imaging_service | xmllint --format -
```

* GetMoveOptions
```
curl -s -X POST -d @./data/GetMoveOptions.xml http://localhost:8088/pepys/imaging_service | xmllint --format -
```

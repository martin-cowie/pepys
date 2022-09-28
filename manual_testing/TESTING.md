# Testing

In lieu of automated Rust drive integration testing, use these to kick the tyres.

## Device Management

* GetSystemDateAndTime
```
curl -s -X POST -d @./data/device_service/GetSystemDateAndTime.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetDeviceInformation
```
curl -s -X POST -d @./data/device_service/GetDeviceInformation.xml http://localhost:8088/pepys/device_service | xmllint --format -

```

* GetNTP
```
curl -s -X POST -d @./data/device_service/GetNTP.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetNetworkInterfaces
```
curl -s -X POST -d @./data/device_service/GetNetworkInterfaces.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetCapabilities
```
curl -s -X POST -d @./data/device_service/GetCapabilities.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetRelayOutputs
```
curl -s -X POST -d @./data/device_service/GetRelayOutputs.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

* GetServices
```
curl -s -X POST -d @./data/device_service/GetServices.xml http://localhost:8088/pepys/device_service | xmllint --format -
```

## Imaging

* GetOptions
```
curl -s -X POST -d @./data/imaging_service/GetOptions.xml http://localhost:8088/pepys/imaging_service | xmllint --format -
```

* GetMoveOptions
```
curl -s -X POST -d @./data/imaging_service/GetMoveOptions.xml http://localhost:8088/pepys/imaging_service | xmllint --format -
```

## Media Service
```
curl -s -X POST -d @./data/media_service/GetProfiles.xml http://localhost:8088/pepys/media_service | xmllint --format -
```
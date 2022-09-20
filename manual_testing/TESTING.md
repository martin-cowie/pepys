# Testing

In lieu of automated Rust drive integration testing, use these to kick the tyres.

```
curl -s -X POST -d @./data/GetSystemDateAndTime.xml http://localhost:8088/picam/device-management | xmllint --format -
```
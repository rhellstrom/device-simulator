## device-simulator
A quick prototype that simulates smart devices and their energy consumption through an HTTP API.

### Run
To enable logging
```
RUST_LOG=trace cargo run
```

### Features 
* Web server running to emulate smart devices and their API
* Access data through the following endpoints

```
http://localhost:3000/devices
```
```
http://localhost:3000/devices/{device_id}
```
* Turn a device on or off through a PATCH request. When a device has the status "Off" energy simulation is paused
```
curl -X PATCH -H "Content-Type: application/json" -d '{"power": "On"}' http://localhost:3000/devices/{device_id}
```
```
curl -X PATCH -H "Content-Type: application/json" -d '{"power": "Off"}' http://localhost:3000/devices/{device_id}

``` 
### Simulated values 
Currently, the power consumed by devices is randomly generated between 50 and 200 watts represented as power_usage in the consumption_data entries. 
The energy_consumed is calculated in kWh based on the used watts. More accurate values should be implemented if received from the customer.

### CLI Options 
``` 
Usage: Device-simulator [OPTIONS]

Options:
  -m, --max-entries <MAX_ENTRIES>  Maximum number of entries in a device consumption history [default: 5]
  -r, --update-interval <seconds>  Interval for updating device consumption history [default: 60]
  -p, --port <port>                Port for application to listen on [default: 3000]
  -h, --help                       Print help
  -V, --version                    Print version
``` 

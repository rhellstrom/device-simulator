## Device-simulator
Quick prototype to simulate smart devices and their energy consumption through HTTP API.
The idea was that it'd be nice to mimic actual devices, fetch data from them and turn them on/off or some other, unimplemented mode, from our controller board 

### Features 
Runs a webserver to mimic smart devices and what their API could potentially look like.
Data accessible through following endpoints:

```
http://localhost:3000/devices
```
or by device_id

```
http://localhost:3000/devices/1
```

### Data structure and the simulated values
Right now the watts consumed by the devices is a randomly generated value between 50 and 200 and represented as power_usage in the consumption_data entries.
energy_consumed is the kWh calculated from the watts used. Better values should be implemented once received from customer(?)

### Turn a device On or Off
In order to mimic control over a smart device, we can PUT to turn on/off a device. If the device is turned off the energy consumption will cease

```
curl -X PUT -H "Content-Type: application/json" -d '{"power": "On"}' http://localhost:3000/devices/1
```
```
curl -X PUT -H "Content-Type: application/json" -d '{"power": "Off"}' http://localhost:3000/devices/1
``` 


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

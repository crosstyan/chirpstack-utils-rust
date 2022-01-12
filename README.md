# Chirpstack Utils

A command line tool for configuring [RAK811](https://store.rakwireless.com/products/rak811-lpwan-module?variant=39942880952518) and other LoRaWAN modules to work with [ChirpStack](https://www.chirpstack.io/).

Are you tired of configuring your own LoRaWAN module manually? Especially when you have to do it handreds of times? Imageine clicking all the UI compontents and
then copying and pasting the [DevEUI](https://lora-developers.semtech.com/documentation/tech-papers-and-guides/the-book/deveui/) and [AppKey](https://www.thethingsnetwork.org/docs/lorawan/security/).

Here it is, an automated tool for configuring your LoRaWAN module.

- Automatic generating the [DevEUI](https://lora-developers.semtech.com/documentation/tech-papers-and-guides/the-book/deveui/) and
[AppKey](https://www.thethingsnetwork.org/docs/lorawan/security/).
- Update the configuring to you LoRaWAN module and ChirpStack with one command

## Usage

```powershell
laser-utils                                                                                                                                                                                          A tool for managing your LoRa devices and ChirpStack API                                                                                                                                                                                                                                                                                                                                                  
USAGE:
    laser-utils-rust.exe <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    all     Config the device automatically, hopefully. Please make sure the device is connected
            to your computer
    api     Send request to ChirpStack API. The infomation of API will be read from config file.
            Please make sure the config file is correctly set
    at      Send at command to serial ports
    help    Print this message or the help of the given subcommand(s)
    ls      List Serial ports
```

Subcommands `all`

```powershell
chirpstack-utils-rust.exe-all 
Config the device automatically, hopefully. Please make sure the device is connected to your
computer

USAGE:
    chirpstack-utils-rust.exe all [OPTIONS] --path <PATH>

OPTIONS:
        --app-key <APP_KEY>            Set the app key (128 bit hex). if not set, the app key will
                                       be generated randomly [default: ]
    -b, --baud <BAUD>                  Baudrate [default: 115200]
    -d, --description <DESCRIPTION>    The device description [default: "a test device"]
        --dev-eui <DEV_EUI>            Set the DevEUI (64 bit hex). if not set, the DevEUI will be
                                       generated randomly [default: ]
    -h, --help                         Print help information
    -n, --name <NAME>                  The device name. If not specified, the name will be generated
                                       randomly [default: ]
    -p, --path <PATH>                  The path of serial port
```

### Configuration

The config file will be generated automatically when you run `chirpstack-utils`, which will be stored in `$APPDATA/chirpstack-utils`
(Windows) environment variable, that is `C:\Users\username\AppData\Roaming\chirpstack-utils`.

`default-config.toml`

```toml
# The url of ChirpStack API. The '/api' should be included
url = 'http://localhost:8080/api'
# The API key of ChirpStack API, generated in Web UI
token = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhcGlfa2V5X2lkIjoiODliNGVhM2YtNGNlYS00NDcyLTllMjItYmVlNDY4MWUzOThmIiwiYXVkIjoiYXMiLCJpc3MiOiJhcyIsIm5iZiI6MTYyMzczNDQwNCwic3ViIjoiYXBpX2tleSJ9.J6JZYcMYYtPXSFjGs2RsVW6k7-r7bU8OHHdKMH1UESM'
# The application ID of ChirpStack API, where the device will be registered
application_id = '2'
# The device type of ChirpStack API, what the device type will be.
device_profile_id = '70298761-1bf9-4a6c-bda1-69a0eb04aaaf'
```

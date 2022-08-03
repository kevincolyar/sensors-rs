# Sensors

## Description

The purpose of the project it to demonstrate the implementation of a sensor API.

It is deployed to the following url:

http://sensors.kevin.colyar.net/v1/

## API Routes

### POST /temp

#### Request

The `POST` request at `/temp` should accepts a JSON blob in the following format:

- `{"data": __data_string__}`
- where `__data_string__` is format:
    - `__device_id__:__epoch_ms__:'Temperature':__temperature__`
      - where `__device_id__` is the device ID (int32)
      - `__epoch_ms__` is the timestamp in EpochMS (int64)
      - `__temperature__` is the temperature (float64)
      - and `'Temperature'` is the exact string

- Example `{"data": "365951380:1640995229697:'Temperature':58.48256793121914"}`

#### Response

- If for any reason the data string is not formatted correctly, return `{'error': 'bad request'}` with a `400` status code
- If the temperature is at or over 90
  - return `{'overtemp': True, 'device_id': __device_id__, 'formatted_time': __formatted_time__}`,
    - where `__device_id__` is the device ID (int32)
    - and `__formatted_time__` is the timestamp formatted to `%Y/%m/%d %H:%M:%S`
  - otherwise return `{'overtemp': False}`

### GET /errors

The `GET` request at `/errors` should return all data strings which have not been in the correct format.

### DELETE /errors

The `DELETE` request at `/errors` should clear the errors buffer.

### Live API Documentation

API Documentation can be viewed at the following url:

http://sensors.kevin.colyar.net/v1/docs

## Releases/Roadmap

See `releases.md` for project progress.

## Development

The local API is deployed to the following url, per `docker-compose.yml`:

http://localhost:3001/v1/

## Deployment

Deploy to Digital Ocean kubernetes cluster via:

    ./bin/deploy.sh
    
Deployment settings can be found in `./k8/`

## Testing

### Test Suite

    docker compose run api pytest
    
### Test Coverage

    docker compose exec api coverage run -m pytest
    docker compose exec api coverage report -m
    docker compose exec api coverage html

    open htmlcov/index.html
    
### Command Line Testing

Select endpoint:

    export URL="http://localhost:3001"
    export URL="http://sensors.kevin.colyar.net"

Errors:

    curl $URL/v1/errors
    curl -X "DELETE" $URL/v1/errors

Temperature:

    curl -X POST -H "Content-Type: application/json" \
    -d '{"data": "365951380:1640995229697:'Temperature':58.48256793121914"}' \
    $URL/v1/temp

    curl -X POST -H "Content-Type: application/json" \
    -d '{"data": "365951380:1640995229697:'Temperature':90.0"}' \
    $URL/v1/temp

    curl -X POST -H "Content-Type: application/json" \
    -d '{"data": ":1640995229697:'Temperature':90.0"}' \
    $URL/v1/temp
    
### Smoke Test

A simple smoke test script of the production API is provided via:

    ./bin/smoke.sh

## Questions

Q: Why `int32` for `device_id`, not `uint32`? 

A: Using `int32` per spec, pending future changes.

Q: What unit is temperature in?

A: Assuming fahrenheit based on example data range (e.g. 58.5 & over 90 threshold).

Q: What timezone should `formatted_time` be returned in?

A: Assuming UTC since offset not specified in given format.

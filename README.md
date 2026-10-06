# Ember Petstore test service

A complete in-memory Petstore API used to exercise Ember from a separate
repository. It depends on the local Ember checkout at `../ember` while Ember
is under development.

## Run

```bash
cargo run
```

Use the development profile on port `8081`:

```bash
EMBER_PROFILE=dev cargo run
```

The base settings are in `src/resources/application.yml`;
`src/resources/application-dev.yml` is merged when `EMBER_PROFILE=dev` is set.
Environment variables such as
`EMBER_SERVER_PORT=9090` override both files.

Application code is organized below `src/main/`, with controllers, services,
jobs, and models in their respective subdirectories.

The service also contains a scheduler example in
`src/main/jobs/pet_maintenance.rs`. It logs a maintenance heartbeat every 30
seconds by default, or every 5 seconds with `EMBER_PROFILE=dev`.
The job uses Ember's structured `info!` macro and `#[logger]` instrumentation.

Scheduler configuration is profile-aware:

```yaml
scheduler:
  enabled: true
  tasks:
    pet-maintenance:
      enabled: true
      interval_ms: 30000
```

Set `scheduler.enabled` to `false`, or disable the individual task, to stop
the scheduled job.

The service listens on `127.0.0.1:8090` and exposes:

```text
GET    /health
GET    /live
GET    /ready
GET    /info
GET    /actuator/health
GET    /actuator/health/liveness
GET    /actuator/health/readiness
GET    /actuator/info
GET    /pet
GET    /pet/{id}
GET    /pet/findByStatus?status=available
GET    /pet/findByTags?tags=cat&tags=friendly
POST   /pet
PUT    /pet/{id}
DELETE /pet/{id}
```

Data is stored in memory and is reset whenever the service restarts.

## Example

```bash
curl -X POST http://127.0.0.1:8090/pet \
  -H 'content-type: application/json' \
  -d '{"name":"Milo","photoUrls":[],"tags":["cat"],"status":"available"}'

curl http://127.0.0.1:8090/pet/1
```

The HTTP contract is covered by the integration tests in `tests/http.rs`.

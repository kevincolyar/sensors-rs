# Releases

## v0.1.0 - Setup
- [X] Git
- [X] Rust
- [ ] Docker
- [ ] Documentation
- [X] Testing
- [ ] Logging

## v0.2.0 - Routes
- [X] Install Actix
    cargo add actix_web
- [X] Routes - POST /temp
- [X] Routes - GET /errors
- [X] Routes - DELETE /errors

## v0.3.x - Deployment
- [ ] Deployment - k8
- [ ] Deployment - digitalocean
- [ ] Deployment - update pods after image push
  + https://stackoverflow.com/questions/33112789/how-do-i-force-kubernetes-to-re-pull-an-image
  + kubectl rollout restart deployments sensors-deploy
- [ ] Deployment - dns - sensors.kevin.colyar.net

## v0.4.x
- [ ] POST temp
- [ ] Parse data
- [ ] Response - {'overtemp': False}
- [ ] Response - Error handling - Overtemp
- [ ] Response - Error handling
- [ ] Response - Format timestamp
- [ ] Validation - device id - int32
- [ ] Validation - epoch_ms - int64
- [ ] Validation - temperature - float64

## v0.5.x
- [ ] Testing - Setup
- [ ] Testing - Unit tests
- [ ] Testing - REST client
- [ ] Testing - Bad type conversions
- [ ] Testing - payload larger than VARCHAR 255

## v0.6.x
- [ ] Persistence - schema
- [ ] Persistence - stored procedures
- [ ] Persistence - measurements
    + created_at
    + measurement/type
    + device_id 
    + value 
- [ ] Persistence - errors
    + timestamp
    + route
    + method
    + payload
    + sort by timestamp

## v0.7.x
- [ ] Documentation
  + https://www.linode.com/docs/guides/documenting-a-fastapi-app-with-openapi/
- [ ] Documentation - API
  + https://fastapi.tiangolo.com/deployment/docker/#interactive-api-docs

## v0.8.x
- [ ] API versioning

## v1.0.0
- [ ] Push to github
- [ ] Make github project public
- [ ] Archive project
- [ ] Email project

## v1.1.x
- [ ] Linting
- [ ] Coverage

## v1.2.x
- [ ] Persistence - test/staging db
- [ ] Persistence - indices
- [ ] Persistence - measurements - upsert
- [ ] Persistence - errors - save stack trace
- [ ] Persistence - handle db re/connection issues

## v1.3.x
- [ ] Lets Encrypt
- [ ] Kafka
- [ ] Authentication - JWT

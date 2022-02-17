local_build:
	docker build -t filter_api_dev -f Dockerfile.dev .

local_run:
	docker run --rm -it -p 8000:8000 --mount type=bind,source="$(shell pwd)",target=/app --name filter_api filter_api_dev cargo watch -x 'run --bin main-api'

local_stop:
	docker stop rust_json


dev-build:
	docker-compose -f docker-compose-dev.yaml build

dev-up:
	docker-compose -f docker-compose-dev.yaml up -d

dev-down:
	docker-compose -f docker-compose-dev.yaml down


local-build:
	docker-compose -f docker-compose-local.yaml build

local-up:
	docker-compose -f docker-compose-local.yaml up -d

local-down:
	docker-compose -f docker-compose-local.yaml down
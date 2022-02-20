local-build:
	docker-compose -f docker-compose-local.yaml build

local-up:
	docker-compose -f docker-compose-local.yaml up -d

local-down:
	docker-compose -f docker-compose-local.yaml down
CURRENT_TAG = $(shell git describe --abbrev=0 --tags | sed 's/v//')

all: build-backend-docker push-backend-docker build-frontend-docker push-frontend-docker

build-backend-docker:
	docker build -t xushaohua/pengzu-backend:latest -f Dockerfile.backend ..
	docker tag xushaohua/pengzu-backend:latest xushaohua/pengzu-backend:$(CURRENT_TAG)

push-backend-docker:
	docker push xushaohua/pengzu-backend:latest
	docker push xushaohua/pengzu-backend:$(CURRENT_TAG)

build-frontend-docker:
	docker build -t xushaohua/pengzu-frontend:latest -f Dockerfile.frontend ..
	docker tag xushaohua/pengzu-frontend:latest xushaohua/pengzu-frontend:$(CURRENT_TAG)

push-frontend-docker:
	docker push xushaohua/pengzu-frontend:latest
	docker push xushaohua/pengzu-frontend:$(CURRENT_TAG)

show-tag:
	echo $(CURRENT_TAG)

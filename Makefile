IMAGE := marshians/fn:latest

build:
	DOCKER_BUILDKIT=1 docker build -t ${IMAGE} .

run:
	docker run --rm -p 8080:8080 -e PORT=8080 ${IMAGE}

push:
	docker push ${IMAGE}

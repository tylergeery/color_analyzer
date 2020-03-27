image_prod=tylergeery/rust-color-analyzer
image_dev=color-analyzer-dev
container_prod=rust-color-analyzer
container_dev=rust-color-analyzer-dev

.PHONY: dev-image prod-image dev test

dev-image:
	docker build -t $(image_dev) --target dev -f Dockerfile .

prod-image:
	docker build -t $(image_prod) --target prod -f Dockerfile .
	docker push $(image_prod)

dev: dev-image dev-kill
	docker run -it -p 8080:8080 -v $(shell pwd)/server:/usr/src/app --name $(container_dev) $(image_dev)

dev-kill:
	- docker stop $(container_dev)
	- docker rm $(container_dev)
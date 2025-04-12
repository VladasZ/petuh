build:
	./build.py

deploy:
	make build
	ansible-playbook deploy.yml

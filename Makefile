build:
	./build.py

dock:
	./build_image.sh

apply:
	kubectl apply -f kubernetes

pods:
	kubectl get pods -n petuh

logs:
	kubectl logs -n petuh -l app=petuh --tail=100

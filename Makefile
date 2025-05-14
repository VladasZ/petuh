build:
	./build.py

dock:
	./build_image.py

apply:
	kubectl apply -f kubernetes

pods:
	kubectl get pods -n petuh

logs:
	kubectl logs -n petuh -l app=petuh --tail=100

pr:
	gh pr create --fill

stop:
	kubectl delete deployment petuh -n petuh

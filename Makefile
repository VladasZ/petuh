
include build/common.mk

apply:
	kubectl apply -f kubernetes

pods:
	kubectl get pods -n petuh

logs:
	kubectl logs -n petuh -l app=petuh --tail=100

stop:
	kubectl delete deployment petuh -n petuh

tags:
	curl http://192.168.0.201:30500/v2/petuh/tags/list

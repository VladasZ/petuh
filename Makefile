
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

deploy:
	make dock
	make apply

lint:
	cargo clippy \
      -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::missing_panics_doc \
      -A clippy::format_push_string \
      \
      -D warnings

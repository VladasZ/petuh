
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
	curl http://192.168.0.201:30500/v2/petuh-llm/tags/list

dock:
	python3 ./build/build-linux.py
	python3 ./build/dock.py petuh 0.13.104
	python3 ./build/dock.py petuh-llm 0.13.104

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
      -A clippy::too_many_lines \
      -A clippy::similar_names \
      -A clippy::trivially_copy_pass_by_ref \
      -A clippy::doc_markdown \
      -A clippy::default_trait_access \
      \
      -D warnings

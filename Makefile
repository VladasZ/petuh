
include build/common.mk

pods:
	kubectl get pods -n petuh

logs:
	kubectl logs -n petuh -l app=petuh --tail=100

stop:
	kubectl delete deployment petuh -n petuh

tags:
	curl http://192.168.0.201:30500/v2/petuh-base/tags/list
	curl http://192.168.0.201:30500/v2/petuh/tags/list
	curl http://192.168.0.201:30500/v2/petuh-llm/tags/list
	curl http://192.168.0.201:30500/v2/petuh-data/tags/list

dock-base:
	python3 ./build/dock.py petuh-base ./infra/Dockerfile 0.1.0

dock:
	python3 ./build/build-linux.py
	python3 ./build/dock.py petuh ./petuh/Dockerfile 0.15.102
	python3 ./build/dock.py petuh-llm ./petuh-llm/Dockerfile 0.15.102
	python3 ./build/dock.py petuh-data ./petuh-data/Dockerfile 0.15.102

d:
	docker compose up petuh-data petuh-llm pg-rw

lint:
	cargo clippy \
      -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::missing_panics_doc \
      -A clippy::missing_errors_doc \
      -A clippy::format_push_string \
      -A clippy::too_many_lines \
      -A clippy::similar_names \
      -A clippy::trivially_copy_pass_by_ref \
      -A clippy::doc_markdown \
      -A clippy::default_trait_access \
      \
      -D warnings

enc:
	sops -e infra/secrets/decrypted/pg.yml > infra/secrets/pg.enc.yml
	rm -rf infra/secrets/decrypted

decr:
	mkdir -p infra/secrets/decrypted
	sops -d infra/secrets/pg.enc.yml > infra/secrets/decrypted/pg.yml

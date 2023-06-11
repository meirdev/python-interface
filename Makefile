.PHONY: build

build:
	python -m venv venv
	venv/bin/pip install -r requirements.txt
	cd my-math && ../venv/bin/python -m maturin build

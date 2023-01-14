FROM 'rust:1.66.0'

WORKDIR '/work'

RUN apt-get update && apt-get install -y \
	python \
	build-essential 
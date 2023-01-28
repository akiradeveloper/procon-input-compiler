FROM 'rust:1.66.0'

RUN apt-get update && apt-get install -y \
	python \
	build-essential \
	nim \
	ruby \
	default-jdk \
	mono-complete  \
	golang

RUN curl -s "https://get.sdkman.io" | bash
RUN ["/bin/bash", "-c", ". /root/.sdkman/bin/sdkman-init.sh; sdk install kotlin"]

WORKDIR '/work'
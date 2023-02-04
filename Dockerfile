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

RUN wget https://download.swift.org/swift-5.7.3-release/ubuntu2004/swift-5.7.3-RELEASE/swift-5.7.3-RELEASE-ubuntu20.04.tar.gz
RUN tar xvfz swift-5.7.3-RELEASE-ubuntu20.04.tar.gz
RUN mv swift-5.7.3-RELEASE-ubuntu20.04/usr/ /root/swift
ENV PATH /root/swift/bin:$PATH

WORKDIR '/work'
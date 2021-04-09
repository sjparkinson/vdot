# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.162.0/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

RUN apt-get update \
    && apt-get -y install --no-install-recommends software-properties-common

# Install Vault
RUN curl -fsSL https://apt.releases.hashicorp.com/gpg | apt-key add - \
  && apt-add-repository "deb [arch=amd64] https://apt.releases.hashicorp.com $(lsb_release -cs) main" \
  && apt-get update \
  && apt-get -y install --no-install-recommends vault

FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

SHELL ["bash", "-c"]

USER vscode

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.4/install.sh | bash
RUN source ~/.nvm/nvm.sh && nvm install 18 && nvm alias default 18 && nvm use 18
RUN cargo install diesel_cli

CMD sleep infinity
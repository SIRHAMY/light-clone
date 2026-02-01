# Based on official Claude Code devcontainer
# https://github.com/anthropics/claude-code/blob/main/.devcontainer/Dockerfile

FROM node:20

ARG TZ
ENV TZ="$TZ"

ARG GIT_DELTA_VERSION=0.18.2
ARG ZSH_IN_DOCKER_VERSION=1.2.0

# Install system dependencies and tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    less \
    git \
    procps \
    sudo \
    fzf \
    zsh \
    man-db \
    unzip \
    gnupg2 \
    gh \
    # Firewall tools
    iptables \
    ipset \
    iproute2 \
    dnsutils \
    aggregate \
    jq \
    # Editors
    nano \
    vim \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install git-delta for better diffs
RUN curl -fsSL "https://github.com/dandavison/delta/releases/download/${GIT_DELTA_VERSION}/git-delta_${GIT_DELTA_VERSION}_amd64.deb" \
    -o /tmp/git-delta.deb \
    && dpkg -i /tmp/git-delta.deb \
    && rm /tmp/git-delta.deb

# Create workspaces directory (VS Code mounts to /workspaces/<project-name>)
RUN mkdir -p /workspaces && chown node:node /workspaces

# Create .claude config directory
RUN mkdir -p /home/node/.claude && chown node:node /home/node/.claude

# Create command history directory for persistence
RUN mkdir -p /commandhistory && chown node:node /commandhistory

# Install Claude CLI (as node user via native installer)
# Note: Must use bash, not sh, as the install script uses bash-specific syntax
RUN su - node -c "curl -fsSL https://claude.ai/install.sh | bash"

# Add firewall init script
COPY init-firewall.sh /usr/local/bin/init-firewall.sh
RUN chmod +x /usr/local/bin/init-firewall.sh

# Allow node user to run specific commands with sudo (no password)
RUN echo "node ALL=(root) NOPASSWD: /usr/local/bin/init-firewall.sh, /bin/chown" > /etc/sudoers.d/node-firewall \
    && chmod 0440 /etc/sudoers.d/node-firewall

# Install zsh with oh-my-zsh and powerlevel10k
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/deluan/zsh-in-docker/v${ZSH_IN_DOCKER_VERSION}/zsh-in-docker.sh)" -- \
    -t https://github.com/romkatv/powerlevel10k \
    -p fzf \
    -p git \
    -p https://github.com/zsh-users/zsh-autosuggestions \
    -p https://github.com/zsh-users/zsh-syntax-highlighting

# Configure bash history for persistence
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
    && mkdir -p /commandhistory \
    && touch /commandhistory/.bash_history \
    && chown -R node:node /commandhistory \
    && echo "$SNIPPET" >> /home/node/.bashrc

# Add claude alias for dangerously-skip-permissions mode
RUN echo "alias clauded='claude --dangerously-skip-permissions'" >> /home/node/.zshrc \
    && echo "alias clauded='claude --dangerously-skip-permissions'" >> /home/node/.bashrc

# Set nano as default editor (vim users can change this)
ENV EDITOR=nano
ENV VISUAL=nano

# Mark as devcontainer
ENV DEVCONTAINER=true

USER node
WORKDIR /workspaces

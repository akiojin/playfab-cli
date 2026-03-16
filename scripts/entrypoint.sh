#!/bin/bash
set -e

# Normalize line endings for Windows-mounted files (best-effort)
if command -v dos2unix >/dev/null 2>&1; then
  for f in /playfab-cli/scripts/*.sh; do
    [ -f "$f" ] && dos2unix "$f" >/dev/null 2>&1 || true
  done
fi

# Git設定（安全なディレクトリを追加）
git config --global --add safe.directory /playfab-cli

# ユーザー名とメールの設定（環境変数から）
if [ -n "${GITHUB_USERNAME:-}" ]; then
    git config --global user.name "$GITHUB_USERNAME"
fi

if [ -n "${GIT_USER_EMAIL:-}" ]; then
    git config --global user.email "$GIT_USER_EMAIL"
fi

# Git認証ファイルを環境変数から作成
if [ -n "${GITHUB_USERNAME:-}" ] && [ -n "${GITHUB_PERSONAL_ACCESS_TOKEN:-}" ]; then
    echo "https://${GITHUB_USERNAME}:${GITHUB_PERSONAL_ACCESS_TOKEN}@github.com" > /root/.git-credentials
    chmod 600 /root/.git-credentials
    git config --global credential.helper store
fi

# GitHub CLI 認証（GITHUB_TOKEN が設定されている場合）
if [ -n "${GITHUB_TOKEN:-}" ]; then
    echo "$GITHUB_TOKEN" | gh auth login --with-token 2>/dev/null && \
        gh auth setup-git && \
        echo "[ok] GitHub CLI authenticated via GITHUB_TOKEN" || \
        echo "[warn] GitHub CLI authentication failed"
fi

echo "[ok] Docker environment is ready!"
echo ""

exec "$@"

#!/usr/bin/env bash
set -euo pipefail
# デバッグ: 環境変数で有効化（例: PUBLISH_DEBUG=1）
[ "${PUBLISH_DEBUG:-0}" = "1" ] && set -x

# publish.sh <major|minor|patch> [--tags-only|--no-push] [--remote <name>]
# 単一入口で以下を実施:
# 1) Cargo.toml のバージョン更新
# 2) タグ付けとコミット＆プッシュ
# 期待動作:
#  - ローカルで crates.io に publish
#  - GitHub Actions で Release binary（各OS向けビルド）

usage() { echo "Usage: $0 <major|minor|patch> [--tags-only|--no-push] [--remote <name>]"; exit 1; }

LEVEL=${1-}
[[ "$LEVEL" =~ ^(major|minor|patch)$ ]] || usage
shift || true

# push 動作: all(既定)/tags/none
PUSH_MODE=${PUBLISH_PUSH:-all}

# オプション解析
while [ $# -gt 0 ]; do
  case "$1" in
    --tags-only)
      PUSH_MODE=tags
      ;;
    --no-push)
      PUSH_MODE=none
      ;;
    --remote)
      shift
      [ $# -gt 0 ] || { echo "[error] --remote requires a value" >&2; exit 1; }
      REMOTE="$1"
      ;;
    *)
      echo "[warn] unknown option: $1" >&2
      ;;
  esac
  shift || true
done

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
REMOTE=${REMOTE:-origin}
cd "$ROOT_DIR"

# ──────────────────────────────────────────────
# Pre-publish validation
# ──────────────────────────────────────────────

echo "[step] pre-publish validation"

# Clean working tree check
if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "[error] Git working tree is not clean. Commit or stash changes before releasing." >&2
  exit 1
fi

if [ -n "$(git ls-files --others --exclude-standard)" ]; then
  echo "[error] Untracked files detected. Commit or remove them before releasing." >&2
  exit 1
fi

# 現在のバージョンを Cargo.toml から取得
CUR_VER=$(grep -m1 '^version' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
echo "[info] current version: $CUR_VER"

# セマンティックバージョニング計算
IFS='.' read -r MAJOR MINOR PATCH <<< "$CUR_VER"
case "$LEVEL" in
  major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
  minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
  patch) PATCH=$((PATCH + 1)) ;;
esac
NEW_VER="${MAJOR}.${MINOR}.${PATCH}"
TAG="v$NEW_VER"
echo "[info] new version: $NEW_VER (tag: $TAG)"

# ──────────────────────────────────────────────
# Version update in Cargo.toml
# ──────────────────────────────────────────────

echo "[step] bump version in Cargo.toml ($LEVEL)"
sed -i.bak -E "0,/^version = \".*\"/s/^version = \".*\"/version = \"${NEW_VER}\"/" Cargo.toml
rm -f Cargo.toml.bak

# Cargo.lock を同期
echo "[step] sync Cargo.lock"
cargo update -w

# ──────────────────────────────────────────────
# Run tests before publishing
# ──────────────────────────────────────────────

echo "[step] running cargo test..."
cargo test || { echo "[error] cargo test failed. Fix test failures before releasing." >&2; exit 1; }

echo "[step] running cargo publish --dry-run..."
cargo publish --dry-run || { echo "[error] cargo publish --dry-run failed. Fix packaging issues before releasing." >&2; exit 1; }

# ──────────────────────────────────────────────
# Commit and tag
# ──────────────────────────────────────────────

git add Cargo.toml Cargo.lock
if ! git diff --cached --quiet; then
  git commit -m "chore(release): v${NEW_VER}"
fi

# タグ作成（存在しない場合）
if git rev-parse -q --verify "$TAG" >/dev/null; then
  echo "[info] tag exists: $TAG"
else
  git tag -a "$TAG" -m "$TAG"
fi

echo "[step] running cargo publish..."
cargo publish || { echo "[error] cargo publish failed. Release not pushed." >&2; exit 1; }

# ──────────────────────────────────────────────
# Push
# ──────────────────────────────────────────────

# リモート接続確認
if ! git ls-remote --exit-code "$REMOTE" >/dev/null 2>&1; then
  echo "[error] remote not accessible: $REMOTE" >&2
  exit 2
fi

case "$PUSH_MODE" in
  all)
    echo "[step] push commits and tag (mode=all)"
    git push --follow-tags "$REMOTE" || echo "[warn] git push --follow-tags failed; will try explicit tag push"
    git push "$REMOTE" "$TAG" || true
    ;;
  tags)
    echo "[step] push tag only (mode=tags)"
    git push "$REMOTE" "$TAG" || true
    ;;
  none)
    echo "[step] skip push (mode=none)"
    ;;
  *)
    echo "[error] unknown PUSH_MODE: $PUSH_MODE" >&2
    exit 2
    ;;
esac

# タグがリモートに存在するか検証し、必要に応じて再試行
echo "[step] verify tag on remote: $TAG"
if [ "$PUSH_MODE" = "none" ]; then
  echo "[skip] verification skipped (no push)"
elif git ls-remote --tags "$REMOTE" | awk '{print $2}' | grep -qx "refs/tags/$TAG"; then
  echo "[ok] tag exists on remote: $TAG"
else
  echo "[warn] tag not found on remote; retrying explicit push"
  for i in 1 2 3; do
    sleep $((i*2))
    git push "$REMOTE" "$TAG" && break || true
  done
  if git ls-remote --tags "$REMOTE" | awk '{print $2}' | grep -qx "refs/tags/$TAG"; then
    echo "[ok] tag exists on remote after retry: $TAG"
  else
    echo "[error] failed to push tag $TAG to $REMOTE" >&2
    exit 3
  fi
fi

echo "[done] v$NEW_VER pushed. Check GitHub Actions: release"
echo "- Release URL (runs): https://github.com/akiojin/playfab-cli/actions/workflows/release.yml"

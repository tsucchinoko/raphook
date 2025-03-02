# Raphook

[![Build Status](https://github.com/tsucchinoko/raphook/workflows/Build/badge.svg)](https://github.com/tsucchinoko/raphook/actions)

Git hook manager for Node.js projects inspired by `lefthook`.

## Install

With **NPM**:

```bash
npm install raphook --save-dev
```

## Usage

1. Initialize raphook

```bash
npx raphook install
```

2. Edit config file

```bash
# Configure your hooks
vim raphook.yml
```

例:

```yaml
# raphook.yml
pre-push:
  commands:
    echo:
      tags: frontend security
      run: echo "Hello, pre-push!"

pre-commit:
  commands:
    echo:
      tags: frontend security
      run: echo "Hello, pre-commit!"
```

3. Run your hooks

```bash
# Enjoy your work with git
git add -A && git commit -m '...'
```

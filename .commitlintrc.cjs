module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'type-enum': [2, 'always', [
      'feat', 'fix', 'perf', 'refactor', 'revert',
      'build', 'ci', 'docs', 'style', 'test', 'chore'
    ]],
    'subject-case': [0],
    'body-max-line-length': [0]
  }
}

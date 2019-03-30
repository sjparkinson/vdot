action "DRAFT_RELEASE" {
  uses = "toolmantim/release-drafter@v4.0.0"
  secrets = ["GITHUB_TOKEN"]
}

workflow "Draft release notes" {
  on = "pull_request"
  resolves = ["Release drafter"]
}

action "Only merged pull request" {
  uses = "actions/bin/filter@master"
  args = "merged true"
}

action "Release drafter" {
  uses = "toolmantim/release-drafter@v4.0.0"
  needs = ["Only merged pull request"]
}

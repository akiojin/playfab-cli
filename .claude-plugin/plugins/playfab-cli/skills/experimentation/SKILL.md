---
name: experimentation
description: Use playfab-cli for A/B testing and experimentation operations including creating, managing, and analyzing experiments, and managing exclusion groups. Use when the user needs to run experiments, check scorecards, manage treatment assignments, or configure exclusion groups. Do not use for general analytics or event writing (use events skill).
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: analytics
---

# Experimentation Operations

Use playfab-cli to manage A/B tests and experiments.

## Use When

- The user needs to create, start, stop, or delete experiments.
- The user wants to check experiment results (scorecards).
- The user needs to manage treatment assignments for entities.
- The user wants to set up exclusion groups to prevent experiment overlap.
- The task involves A/B testing, feature flags, or controlled rollouts.

## Do Not Use When

- The user needs to write events or manage telemetry (use events skill).
- The user needs player statistics or leaderboards (use leaderboards skill).
- The user needs general analytics queries (use PlayFab Game Manager).

## Available Tools

### Experiment Management

| Tool | Description |
|------|-------------|
| `create_experiment` | Create a new experiment with variants and audience targeting |
| `delete_experiment` | Delete an experiment by its ID |
| `get_experiments` | Get all experiments for the title |
| `get_latest_scorecard` | Get the latest scorecard results for an experiment |
| `get_treatment_assignment` | Get the treatment assignment for an entity in active experiments |
| `start_experiment` | Start a previously created experiment |
| `stop_experiment` | Stop a running experiment |
| `update_experiment` | Update an existing experiment configuration |

### Exclusion Groups

| Tool | Description |
|------|-------------|
| `create_exclusion_group` | Create an exclusion group to prevent experiments from overlapping |
| `delete_exclusion_group` | Delete an exclusion group by its ID |
| `get_exclusion_groups` | Get all exclusion groups for the title |
| `update_exclusion_group` | Update an existing exclusion group |
| `get_exclusion_group_traffic` | Get traffic allocation details for an exclusion group |

## Examples

### Create an experiment

```bash
playfab-cli tool call create_experiment --json '{
  "name": "NewShopLayout",
  "description": "Test new shop layout vs current",
  "startDate": "2024-06-01T00:00:00Z",
  "endDate": "2024-06-30T23:59:59Z",
  "titlePlayerAccountTestIds": [],
  "variants": [
    {"name": "Control", "titleDataOverrides": {}, "trafficPercentage": 50, "isControl": true},
    {"name": "NewLayout", "titleDataOverrides": {"ShopLayout": "v2"}, "trafficPercentage": 50, "isControl": false}
  ]
}'
```

### List all experiments

```bash
playfab-cli tool call get_experiments --json '{}'
```

### Start an experiment

```bash
playfab-cli tool call start_experiment --json '{"experimentId": "exp-123"}'
```

### Get experiment results

```bash
playfab-cli tool call get_latest_scorecard --json '{"experimentId": "exp-123"}'
```

### Stop an experiment

```bash
playfab-cli tool call stop_experiment --json '{"experimentId": "exp-123"}'
```

### Get treatment assignment

```bash
playfab-cli tool call get_treatment_assignment --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"}
}'
```

### Create an exclusion group

```bash
playfab-cli tool call create_exclusion_group --json '{
  "name": "ShopExperiments",
  "description": "Prevents shop-related experiments from overlapping"
}'
```

### Get exclusion group traffic

```bash
playfab-cli tool call get_exclusion_group_traffic --json '{
  "exclusionGroupId": "eg-123"
}'
```

## Common Workflows

### 1. Run an A/B test

```bash
# Create the experiment
playfab-cli tool call create_experiment --json '{"name": "PriceTest", "variants": [...]}'
# Start the experiment
playfab-cli tool call start_experiment --json '{"experimentId": "exp-123"}'
# Monitor results
playfab-cli tool call get_latest_scorecard --json '{"experimentId": "exp-123"}'
# Stop when sufficient data collected
playfab-cli tool call stop_experiment --json '{"experimentId": "exp-123"}'
```

### 2. Manage experiment isolation

```bash
# Create exclusion group
playfab-cli tool call create_exclusion_group --json '{"name": "OnboardingTests"}'
# Check traffic allocation
playfab-cli tool call get_exclusion_group_traffic --json '{"exclusionGroupId": "..."}'
# Create experiment in the exclusion group
playfab-cli tool call create_experiment --json '{"name": "WelcomeFlow", "exclusionGroupId": "...", "variants": [...]}'
```

### 3. Check player experiment assignment

```bash
# Get what experiments/variants a player is assigned to
playfab-cli tool call get_treatment_assignment --json '{"entity": {"Id": "...", "Type": "title_player_account"}}'
```

## Notes

- Experiments require at least one control variant and one treatment variant.
- Exclusion groups ensure players are only in one experiment within the group at a time.
- Scorecards show statistical significance of experiment results.
- Experiments can only be started after creation — they do not run automatically.
- `update_experiment` can only modify experiments that have not been started yet.
- Treatment assignments are deterministic — the same player always gets the same variant.

# Souls of the Silicon Pirates

## The Cave
The **Cave** is our tactical isolation chamber. Every Sprite operates within its own dedicated Cave—a Docker-isolated bulkhead designed to prevent context leakage and maintain hull integrity. 

When a Sprite switches tasks or WIP groups, the **Bulkhead Failure protocol** triggers a Clean-Room sequence:
- `git reset --hard`
- `git clean -fd`

This ensures that no residual data or "ghost" logic contaminates the next mission.

## The Mothership
The source of orchestration and authority resides outside the project workspace at:
`/root/mothership/`

The Mothership contains the master scripts and configuration required to summon and leash the pirate swarm.

## Calling lsprite
The `lsprite.sh` script is the primary interface for Sprite lifecycle management and infrastructure control.

**Authority Path:**
`/root/mothership/lsprite.sh`

**Standard Commands:**
```bash
# Build the Sprite image
/root/mothership/lsprite.sh build

# List active Caves and their status
/root/mothership/lsprite.sh ls

# Enter a specific Cave environment
/root/mothership/lsprite.sh in <cave_id>

# Remove/Cleanup a Cave
/root/mothership/lsprite.sh rm <cave_id>

# Reconstruct state from the Ledger (Admin)
/root/mothership/lsprite.sh reset-cave
```

*Tame the chaos. Burn the logs. Build the future.*

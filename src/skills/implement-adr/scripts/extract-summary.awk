#!/usr/bin/awk -f
# extract-summary.awk — Extract implementation summary from a plan file as JSONL
#
# Usage:
#   awk -f extract-summary.awk docs/plans/0020.0.plan.md
#
# Reads the block between <!-- BEGIN implementation-summary --> and
# <!-- END implementation-summary -->, skips the header line (starts with #),
# and emits each task as a JSONL line to stdout.
#
# Produces no output for plan files without a summary block.

BEGIN { in_block = 0 }

# Extract basename from FILENAME (strip directory path)
FNR == 1 {
    n_parts = split(FILENAME, path_parts, "/")
    source_plan = path_parts[n_parts]
}

/<!-- BEGIN implementation-summary -->/ { in_block = 1; next }
/<!-- END implementation-summary -->/  { in_block = 0; next }

in_block && /^#/ { next }

in_block && /\|/ {
    n = split($0, fields, " *\\| *")
    if (n < 5) next

    # Trim leading/trailing whitespace from each field
    for (i = 1; i <= n; i++) {
        gsub(/^[ \t]+|[ \t]+$/, "", fields[i])
    }

    task_id     = fields[1]
    status      = fields[2]
    cost        = fields[3]
    commit      = fields[4]
    description = fields[5]

    # Escape double quotes in description
    gsub(/"/, "\\\"", description)

    printf "{\"task_id\":\"%s\",\"status\":\"%s\",\"cost\":\"%s\",\"commit\":\"%s\",\"description\":\"%s\",\"source_plan\":\"%s\"}\n", \
        task_id, status, cost, commit, description, source_plan
}

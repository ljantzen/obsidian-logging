#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO_OWNER="ljantzen"
REPO_NAME="obsidian-logging"
WORKFLOW_NAME="Release"
MAX_WAIT_TIME=900  # 15 minutes in seconds
POLL_INTERVAL=10   # Poll every 10 seconds
RUN_CREATION_TIMEOUT=180  # 3 minutes to wait for run to be created

# Helper functions
print_error() {
    echo -e "${RED}Error: $1${NC}" >&2
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    print_info "Checking prerequisites..."

    if ! command -v gh &> /dev/null; then
        print_error "GitHub CLI (gh) is not installed. Please install it: https://cli.github.com"
        exit 1
    fi

    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed"
        exit 1
    fi

    if ! command -v git &> /dev/null; then
        print_error "Git is not installed"
        exit 1
    fi

    print_success "All prerequisites met"
}

# Get version from Cargo.toml
get_version() {
    grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2
}

# Validate version format (semantic versioning)
validate_version() {
    local version=$1
    if ! [[ $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "Invalid version format: $version (expected semantic versioning: X.Y.Z)"
        return 1
    fi
    return 0
}

# Update version in Cargo.toml
update_version() {
    local new_version=$1
    local current_version=$(get_version)

    if [ "$new_version" = "$current_version" ]; then
        print_error "New version ($new_version) is the same as current version in Cargo.toml"
        return 1
    fi

    print_info "Updating version from $current_version to $new_version in Cargo.toml..."
    sed -i "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
    print_success "Version updated to $new_version"
}

# Show usage
show_usage() {
    cat << EOF
Usage: ./release.sh <version>

Arguments:
  <version>    New version to release (semantic versioning: X.Y.Z)

Example:
  ./release.sh 1.0.0
  ./release.sh 1.1.0

The script will:
1. Update Cargo.toml with the new version
2. Commit the version change
3. Create and push a version tag (v<version>)
4. Wait for GitHub Actions to complete
5. Publish to crates.io
EOF
}

# Check if working directory is clean
check_clean_working_dir() {
    if [ -n "$(git status --porcelain)" ]; then
        print_error "Working directory is not clean. Please commit or stash changes."
        git status
        exit 1
    fi
    print_success "Working directory is clean"
}

# Check if tag exists on origin
tag_exists_on_origin() {
    local tag=$1
    git ls-remote --tags origin | grep -E "refs/tags/$tag$" >/dev/null 2>&1
}

# Check if version is published on crates.io
is_published_on_crates() {
    local version=$1
    # Query crates.io API to check if the version exists
    curl -s "https://crates.io/api/v1/crates/$REPO_NAME/versions" | \
        jq -e ".versions[] | select(.num == \"$version\")" >/dev/null 2>&1
}

# Validate tag points to expected commit on current branch
validate_tag() {
    local tag=$1
    local expected_branch=${2:-main}

    # Get the commit the tag points to
    local tag_commit=$(git rev-parse "$tag^{commit}" 2>/dev/null || git rev-list -n 1 "$tag")
    local current_head=$(git rev-parse HEAD)

    # Check if tag points to current HEAD
    if [ "$tag_commit" != "$current_head" ]; then
        print_error "Tag $tag points to commit $tag_commit, but current HEAD is $current_head"
        print_info "This likely means the tag was created on the wrong commit"
        return 1
    fi

    # Check if tag commit is on the current branch
    if ! git merge-base --is-ancestor "$tag_commit" "$expected_branch"; then
        print_error "Tag $tag does not point to a commit on branch $expected_branch"
        return 1
    fi

    print_success "Tag $tag validated (points to $tag_commit on $expected_branch)"
    return 0
}

# Create and push version tag
create_and_push_tag() {
    local version=$1
    local tag="v$version"

    if tag_exists_on_origin "$tag"; then
        print_success "Tag $tag already exists on origin, skipping tag creation"
        return 0
    fi

    print_info "Creating tag $tag..."

    if git rev-parse "$tag" >/dev/null 2>&1; then
        print_error "Tag $tag exists locally but not on origin"
        exit 1
    fi

    git tag -a "$tag" -m "Release version $version"
    print_success "Tag $tag created"

    print_info "Pushing tag to remote..."
    git push origin "$tag"
    print_success "Tag pushed to remote"

    # Validate tag was created on the correct commit
    print_info "Validating tag..."
    if ! validate_tag "$tag" "main"; then
        print_error "Tag validation failed - the tag may be on the wrong commit!"
        print_info "Deleting local tag $tag to prevent pushing incorrect tag"
        git tag -d "$tag"
        exit 1
    fi
}

# Wait for GitHub Actions workflow to complete
wait_for_workflow() {
    local tag=$1

    print_info "Waiting for GitHub Actions workflow to complete..."
    print_info "Watching workflow for tag: $tag"
    echo ""

    # Use gh run watch to monitor the run for this specific tag
    # Get the commit SHA that the tag points to (dereference annotated tag)
    local commit_sha=$(git rev-parse "$tag^{commit}" 2>/dev/null || git rev-list -n 1 "$tag")

    # Wait for the run to appear in the API
    local run_id=""
    local attempts=0
    local max_attempts=24  # 2 minutes with 5-second intervals

    print_info "Waiting for workflow run to appear in GitHub API (looking for commit $commit_sha)..."

    while [ -z "$run_id" ] && [ $attempts -lt $max_attempts ]; do
        # Get recent runs for the workflow with all relevant fields
        local run_list=$(gh run list --workflow "$WORKFLOW_NAME" --repo $REPO_OWNER/$REPO_NAME --limit 10 --json databaseId,headSha,name,createdAt 2>/dev/null)


        # Find the run that matches our commit SHA
        if [ -n "$run_list" ] && [ "$run_list" != "[]" ]; then
            run_id=$(echo "$run_list" | jq -r ".[] | select(.headSha == \"$commit_sha\") | .databaseId" 2>/dev/null | head -1)

            if [ -n "$run_id" ] && [ "$run_id" != "null" ]; then
                break
            fi
        fi

        attempts=$((attempts + 1))
        if [ $attempts -lt $max_attempts ]; then
            echo -n "."
            sleep 5
        fi
    done

    if [ -z "$run_id" ]; then
        print_error "Could not find workflow run"
        print_info "Checking workflow status at: https://github.com/$REPO_OWNER/$REPO_NAME/actions"
        echo ""
        print_info "Please check GitHub Actions manually. You may need to:"
        echo "  1. Wait for the workflow to complete"
        echo "  2. Run 'cargo publish' manually when ready"
        return 1
    fi

    print_info "Watching run ID: $run_id"
    echo ""

    # Watch the run until completion (with custom timeout)
    if timeout $MAX_WAIT_TIME gh run watch $run_id --repo $REPO_OWNER/$REPO_NAME --exit-status; then
        echo ""
        print_success "GitHub Actions workflow completed successfully"
        return 0
    else
        local exit_code=$?
        echo ""
        if [ $exit_code -eq 124 ]; then
            print_error "Timeout waiting for GitHub Actions workflow (${MAX_WAIT_TIME}s)"
        else
            print_error "GitHub Actions workflow failed or was interrupted"
        fi
        print_info "View status at: https://github.com/$REPO_OWNER/$REPO_NAME/actions/runs/$run_id"
        return 1
    fi
}

# Publish to crates.io
publish_to_crates() {
    local version=$1

    if is_published_on_crates "$version"; then
        print_success "Version $version is already published on crates.io, skipping publication"
        return 0
    fi

    print_info "Publishing to crates.io..."

    # Use --allow-dirty to allow Cargo.lock changes from the build
    if ! cargo publish --allow-dirty; then
        print_error "Failed to publish to crates.io"
        return 1
    fi

    print_success "Published to crates.io"
}

# Main function
main() {
    # Parse arguments
    if [ $# -eq 0 ]; then
        show_usage
        exit 1
    fi

    local new_version=$1

    if [ "$new_version" = "-h" ] || [ "$new_version" = "--help" ]; then
        show_usage
        exit 0
    fi

    echo "=========================================="
    echo "        Release Script for $REPO_NAME"
    echo "=========================================="
    echo ""

    # Validate version format
    if ! validate_version "$new_version"; then
        exit 1
    fi

    check_prerequisites
    check_clean_working_dir

    local current_version=$(get_version)
    print_info "Current version in Cargo.toml: $current_version"
    print_info "New version to release: $new_version"
    echo ""

    # Check if tag already exists on origin
    local tag="v$new_version"

    if ! tag_exists_on_origin "$tag"; then
        # Tag doesn't exist, proceed with full release
        read -p "Proceed with release v$new_version to GitHub and crates.io? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "Release cancelled"
            exit 0
        fi
        echo ""

        # Step 0: Update version and commit
        if ! update_version "$new_version"; then
            exit 1
        fi

        print_info "Updating Cargo.lock..."
        cargo update --workspace
        print_success "Cargo.lock updated"

        print_info "Committing version and lock file changes..."
        git add Cargo.toml Cargo.lock
        git commit -m "Bump version to $new_version"
        print_success "Version and lock file commit created"
        echo ""

        print_info "Pushing commits to remote..."
        git push origin HEAD
        print_success "Commits pushed to remote"
        echo ""

        # Step 1: Create and push tag
        create_and_push_tag "$new_version"
        echo ""
    else
        # Tag already exists on origin, skip to workflow
        print_info "Tag $tag already exists on origin, resuming release process..."
        echo ""
    fi

    # Step 2: Wait for GitHub Actions
    if ! wait_for_workflow "v$new_version"; then
        print_error "Cannot proceed with crates.io publication until GitHub Actions succeeds"
        exit 1
    fi
    echo ""

    # Step 3: Publish to crates.io
    if publish_to_crates "$new_version"; then
        echo ""
        echo "=========================================="
        print_success "Release v$new_version completed successfully!"
        echo "=========================================="
        echo "GitHub Release: https://github.com/$REPO_OWNER/$REPO_NAME/releases/tag/v$new_version"
        echo "crates.io: https://crates.io/crates/$REPO_NAME/v$new_version"
    else
        print_error "Release partially complete - tag pushed but crates.io publication failed"
        exit 1
    fi
}

main "$@"

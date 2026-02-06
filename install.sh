#!/bin/bash
set -e

# Configuration
REPO="rit-sse/mentor-script"
INSTALL_DIR="/usr/local/bin"
EXECUTABLE_NAME="mentor-script"

echo "Installing Mentor Script..."

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
    Linux*)
        case "${ARCH}" in
            x86_64)     BINARY_NAME="mentor-script-x86_64-linux";;
            aarch64)    BINARY_NAME="mentor-script-aarch64-linux";;
            *)          echo "Unsupported architecture: ${ARCH}"; exit 1;;
        esac
        ;;
    *)
        echo "Unsupported operating system: ${OS}"
        exit 1
        ;;
esac

# Get latest release version
echo "Fetching latest release..."
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "Error: Could not fetch latest release"
    echo "You may not have any releases yet. Create one with:"
    echo "  git tag v1.0.0"
    echo "  git push origin v1.0.0"
    exit 1
fi

echo "Latest version: ${LATEST_RELEASE}"

# Download URL
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_RELEASE}/mentor-script-x86_64-linux"

# Create temporary directory
TMP_DIR=$(mktemp -d)
trap "rm -rf ${TMP_DIR}" EXIT

# Download binary
echo "Downloading ${BINARY_NAME}..."
if ! curl -L "${DOWNLOAD_URL}" -o "${TMP_DIR}/${EXECUTABLE_NAME}"; then
    echo "Error: Failed to download. The binary may not be available for your platform yet."
    echo "Available at: ${DOWNLOAD_URL}"
    exit 1
fi

# Make executable
chmod +x "${TMP_DIR}/${EXECUTABLE_NAME}"

# Install
echo "Installing to ${INSTALL_DIR}/${EXECUTABLE_NAME}..."
if [ -w "${INSTALL_DIR}" ]; then
    mv "${TMP_DIR}/${EXECUTABLE_NAME}" "${INSTALL_DIR}/${EXECUTABLE_NAME}"
    mkdir -p "${INSTALL_DIR}/songs"
else
    echo "Need sudo permissions to install to ${INSTALL_DIR}"
    sudo mv "${TMP_DIR}/${EXECUTABLE_NAME}" "${INSTALL_DIR}/${EXECUTABLE_NAME}"
fi

# Create desktop run script with auto-update
DESKTOP_DIR="${HOME}/Desktop"
if [ -d "${DESKTOP_DIR}" ]; then
    SHORTCUT_PATH="${DESKTOP_DIR}/run_mentor_script.sh"
    cat > "${SHORTCUT_PATH}" << 'SCRIPTEOF'
#!/bin/bash

# Mentor Script Auto-Updater and Runner
REPO="rit-sse/mentor-script"
INSTALL_DIR="/usr/local/bin"
EXECUTABLE_NAME="mentor-script"
VERSION_FILE="${HOME}/.mentor-script-version"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
    Linux*)
        case "${ARCH}" in
            x86_64)     BINARY_NAME="mentor-script-x86_64-linux";;
            aarch64)    BINARY_NAME="mentor-script-aarch64-linux";;
            *)          echo "Unsupported architecture: ${ARCH}"; exit 1;;
        esac
        ;;
    *)
        echo "Unsupported operating system: ${OS}"
        exit 1
        ;;
esac

# Check for updates
echo "Checking for updates..."
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "Warning: Could not check for updates. Running installed version..."
else
    CURRENT_VERSION=""
    if [ -f "${VERSION_FILE}" ]; then
        CURRENT_VERSION=$(cat "${VERSION_FILE}")
    fi

    if [ "$CURRENT_VERSION" != "$LATEST_RELEASE" ]; then
        echo "Update available: ${CURRENT_VERSION:-none} → ${LATEST_RELEASE}"
        echo "Downloading update..."

        DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_RELEASE}/mentor-script-x86_64-linux"
        TMP_DIR=$(mktemp -d)

        if curl -L "${DOWNLOAD_URL}" -o "${TMP_DIR}/${EXECUTABLE_NAME}"; then
            chmod +x "${TMP_DIR}/${EXECUTABLE_NAME}"

            if [ -w "${INSTALL_DIR}" ]; then
                mv "${TMP_DIR}/${EXECUTABLE_NAME}" "${INSTALL_DIR}/${EXECUTABLE_NAME}"
                echo "${LATEST_RELEASE}" > "${VERSION_FILE}"
                echo "✓ Updated to ${LATEST_RELEASE}"
            else
                echo "Installing update (requires sudo)..."
                sudo mv "${TMP_DIR}/${EXECUTABLE_NAME}" "${INSTALL_DIR}/${EXECUTABLE_NAME}"
                echo "${LATEST_RELEASE}" > "${VERSION_FILE}"
                echo "✓ Updated to ${LATEST_RELEASE}"
            fi
        else
            echo "Warning: Update failed. Running current version..."
        fi

        rm -rf "${TMP_DIR}"
    else
        echo "Already up to date (${CURRENT_VERSION})"
    fi
fi

# Run the script
echo "Starting Mentor Script..."
exec "${INSTALL_DIR}/${EXECUTABLE_NAME}" "$@"
SCRIPTEOF
    chmod +x "${SHORTCUT_PATH}"
    echo "✓ Created desktop shortcut with auto-update: ${SHORTCUT_PATH}"
fi

# Store initial version
echo "${LATEST_RELEASE}" > "${HOME}/.mentor-script-version"

echo ""
echo "✓ Mentor Script ${LATEST_RELEASE} installed successfully!"
echo ""
echo "Run with:"
echo "  ${EXECUTABLE_NAME}                    - Run directly (no auto-update)"
echo "  ~/Desktop/run_mentor_script.sh - Run with auto-update check"
echo ""
echo "Note: You may need to create a config.json file in your working directory."
echo "See README.md for configuration details."
echo ""

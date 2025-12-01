#!/bin/bash
# Installation script for HoneyTrap Server

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="/opt/honeytrap"
CONFIG_DIR="/etc/honeytrap"
LOG_DIR="/var/log/honeytrap"
DATA_DIR="/var/lib/honeytrap"
USER="honeytrap"
GROUP="honeytrap"

echo -e "${GREEN}🍯 HoneyTrap Server Installation${NC}"
echo "=================================="
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}Error: This script must be run as root${NC}"
   exit 1
fi

# Create user and group
echo -e "${YELLOW}Creating user and group...${NC}"
if ! id -u $USER > /dev/null 2>&1; then
    useradd --system --no-create-home --shell /bin/false $USER
    echo -e "${GREEN}✓ User '$USER' created${NC}"
else
    echo -e "${YELLOW}User '$USER' already exists${NC}"
fi

# Create directories
echo -e "${YELLOW}Creating directories...${NC}"
mkdir -p $INSTALL_DIR/bin
mkdir -p $CONFIG_DIR
mkdir -p $LOG_DIR
mkdir -p $DATA_DIR

# Set permissions
chown -R $USER:$GROUP $INSTALL_DIR
chown -R $USER:$GROUP $CONFIG_DIR
chown -R $USER:$GROUP $LOG_DIR
chown -R $USER:$GROUP $DATA_DIR

chmod 755 $INSTALL_DIR
chmod 750 $CONFIG_DIR
chmod 750 $LOG_DIR
chmod 750 $DATA_DIR

echo -e "${GREEN}✓ Directories created${NC}"

# Copy binary
echo -e "${YELLOW}Installing binary...${NC}"
if [ -f "../../../target/release/honeytrap-server" ]; then
    cp ../../../target/release/honeytrap-server $INSTALL_DIR/bin/
    chmod 755 $INSTALL_DIR/bin/honeytrap-server
    echo -e "${GREEN}✓ Binary installed${NC}"
else
    echo -e "${RED}Error: Binary not found. Please run 'cargo build --release --bin honeytrap-server' first${NC}"
    exit 1
fi

# Copy config
echo -e "${YELLOW}Installing configuration...${NC}"
if [ -f "honeytrap.example.toml" ]; then
    cp honeytrap.example.toml $CONFIG_DIR/honeytrap.toml
    chown $USER:$GROUP $CONFIG_DIR/honeytrap.toml
    chmod 640 $CONFIG_DIR/honeytrap.toml
    echo -e "${GREEN}✓ Configuration installed${NC}"
else
    echo -e "${RED}Error: Example config not found${NC}"
    exit 1
fi

# Install systemd service
echo -e "${YELLOW}Installing systemd service...${NC}"
if [ -f "honeytrap.service" ]; then
    cp honeytrap.service /etc/systemd/system/
    systemctl daemon-reload
    echo -e "${GREEN}✓ Systemd service installed${NC}"
else
    echo -e "${RED}Error: Service file not found${NC}"
    exit 1
fi

# Setup firewall (optional)
echo -e "${YELLOW}Would you like to configure firewall rules? (y/n)${NC}"
read -r setup_firewall
if [[ $setup_firewall == "y" ]]; then
    if command -v ufw > /dev/null; then
        ufw allow 8080/tcp comment 'HoneyTrap Server'
        ufw allow 2222/tcp comment 'HoneyTrap SSH Honeypot'
        echo -e "${GREEN}✓ Firewall rules added${NC}"
    else
        echo -e "${YELLOW}UFW not found, skipping firewall configuration${NC}"
    fi
fi

# Final instructions
echo ""
echo -e "${GREEN}✅ Installation complete!${NC}"
echo ""
echo "Next steps:"
echo "1. Edit the configuration: sudo nano $CONFIG_DIR/honeytrap.toml"
echo "2. Set your API key (if using LLM): export DEEPSEEK_API_KEY=your-key"
echo "3. Enable the service: sudo systemctl enable honeytrap"
echo "4. Start the service: sudo systemctl start honeytrap"
echo "5. Check status: sudo systemctl status honeytrap"
echo "6. View logs: sudo journalctl -u honeytrap -f"
echo ""
echo -e "${YELLOW}⚠️  Remember to configure your API keys and network settings!${NC}"

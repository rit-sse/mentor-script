import subprocess
import os

# Check out main just in case
subprocess.run(['git', 'checkout', 'main'], check=True)

# Run git pull in the current directory
subprocess.run(['git', 'pull'], check=True)

# Then, run the actual mentor script
subprocess.run(['python3.11', './main.py'], check=True)
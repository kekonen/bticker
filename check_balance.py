import subprocess, sys, os
from pathlib import Path
wallet = dict([[curr + 'USDT', float(amount)] for curr, amount in [arg.split('=') for arg in sys.argv[1:]]])
stdout, _ = subprocess.Popen([str(Path(__file__).parent.absolute() / 'target/release/bticker')] + list(wallet.keys()) ,stdout=subprocess.PIPE).communicate()
balance = sum([wallet[curr] * float(price) for curr, price in [x.split(': ') for x in stdout.decode('utf8').split('\n')[:len(wallet)]]])
print(balance)

import subprocess, sys
from pathlib import Path
treshold = dict([[curr, float(amount)] for curr, amount in [arg.split('=') for arg in sys.argv[1:]]])
stdout, _ = subprocess.Popen([str(Path(__file__).parent.absolute() / 'target/release/bticker')] + list(treshold.keys()) ,stdout=subprocess.PIPE).communicate()
high_currencies = [f'{curr}={price}' for curr, price in [x.split(': ') for x in stdout.decode('utf8').split('\n')[:len(treshold)]] if treshold[curr] < float(price)]
for high_currency in high_currencies:
	print(high_currency)

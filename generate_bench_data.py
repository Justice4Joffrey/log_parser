import argparse
import random
import json
import string

parser = argparse.ArgumentParser()
parser.add_argument("outfile", type=str, help="Output file")
parser.add_argument("--n", type=int, help="Number of lines", default=1000000)
parser.add_argument("--p", type=int, help="Max random keys", default=10)

TYPES = {"abc", "bcd", "csdfsdf", "d", "edasjdflj", "fiwer", "gjasfdll", "hjlkjasdf", "ijlsdfj", "jsdflkjlk", "kR123",
         "fkjWER", "mdsfjh", "n", "o"}
_TYPES = list(TYPES)

args = parser.parse_args()


def random_string(n):
    return ''.join(random.choice(string.ascii_lowercase) for _ in range(n))


print(f"Generating data in {args.outfile}...")
with open(args.outfile, "w+") as f:
    for i in range(args.n):
        n_keys = random.randint(0, args.p)
        data = {random_string(random.randint(1, 10)): random_string(random.randint(1, 10)) for _ in range(n_keys)}
        data["type"] = random.choice(_TYPES)
        f.write(f"{json.dumps(data)}\n")

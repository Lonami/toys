"""
Converts <UNIQUE NAMES> to <MAPPED CHAR> and applies this to the <PROCESSED MAP>.
You are then supposed to move the output inside the maze manually again.

Usage:
    python process.py <UNIQUE NAMES FILE> <PROCESSED MAP FILE> <OUTPUT FILE>

Where:
    <UNIQUE NAMES> contains a list of `sort <NAMES> | uniq` (without AA or ZZ).
    <PROCESSED MAP FILE> contains <PUZZLE INPUT> with vertical names moved to be horizontal (manually).
"""
import sys

def main():
    with open(sys.argv[1]) as fd:
        mapping = {name.strip(): chr(i + ord('1')) for i, name in enumerate(fd)}

    with open(sys.argv[2]) as fd:
        text = fd.read()

    for name, mapped in mapping.items():
        text = text.replace(name, ' ' + mapped)

    with open(sys.argv[3], 'w') as fd:
        fd.write(text)

if __name__ == '__main__':
    main()

import os
import sys
import subprocess

def interpret_day(day):
    n = day.replace('day', '').lstrip('0')
    try:
        return 'day{:02}'.format(int(day.replace('day', '').lstrip('0')))
    except ValueError:
        raise ValueError('cannot interpret "{}" as a valid day'.format(day))

def main(args):
    if len(args) < 2:
        print('usage:', args[0], '<all|days...>')
        return 1

    if args[1] == 'all':
        days = os.listdir('examples')
    else:
        days = [interpret_day(x) for x in args[1:]]

    ok = True
    for day in sorted(days):
        with open('inputs/{}/input'.format(day), 'rb') as fd:
            proc = subprocess.run(('cargo', 'run', '--example', day), stdin=fd, capture_output=True)
        
        with open('outputs/{}/input'.format(day), 'rb') as fd:
            expected = fd.read()

        if proc.stdout == expected:
            print('OK:', day)
        else:
            print('FAIL:', day, 'GOT(', proc.stdout, ') EXPECTED(', expected, ')')

if __name__ == '__main__':
    exit(main(sys.argv) or 0)

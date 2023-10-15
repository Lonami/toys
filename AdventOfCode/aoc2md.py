#!/usr/bin/env python3
# Convert the AoC's HTML description to markdown.
# This is used to have pretty README's in the solutions for each day.
#
# Please don't abuse the feature to fetch from the network. The following
# is an excerpt from the site's HTML:
#
# <!--
# Oh, hello!  Funny seeing you here.
#
# I appreciate your enthusiasm, but you aren't going to find much down here.
# There certainly aren't clues to any of the puzzles.  The best surprises don't
# even appear in the source until you unlock them for real.
#
# Please be careful with automated requests; I'm not a massive company, and I can
# only take so much traffic.  Please be considerate so that everyone gets to play.
#
# If you're curious about how Advent of Code works, it's running on some custom
# Perl code. Other than a few integrations (auth, analytics, ads, social media),
# I built the whole thing myself, including the design, animations, prose, and
# all of the puzzles.
#
# The puzzles are most of the work; preparing a new calendar and a new set of
# puzzles each year takes all of my free time for 4-5 months. A lot of effort
# went into building this thing - I hope you're enjoying playing it as much as I
# enjoyed making it for you!
#
# If you'd like to hang out, I'm @ericwastl on Twitter.
#
# - Eric Wastl
# -->
import io
import os
import sys
import html.parser
import urllib.request

YEAR = 2019
URL = 'https://adventofcode.com/{year}/day/{day}'


class AOCParser(html.parser.HTMLParser):
    def __init__(self):
        super().__init__()
        self._tag_stack = []
        self._in_article = False
        self._buffer = io.StringIO()

    def handle_starttag(self, tag, attrs):
        if not self._in_article:
            if tag == 'article' and ('class', 'day-desc') in attrs:
                self._in_article = True
            
            return

        if tag == 'h2':
            self._buffer.write('## ')
        elif tag == 'em':
            if ('class', 'star') in attrs:
                self._buffer.write('*')
            else:
                self._buffer.write('**')
        elif tag == 'pre':
            self._buffer.write('```\n')
        elif tag == 'code':
            if self._last_tag != 'pre':
                self._buffer.write('`')
        elif tag == 'li':
            self._buffer.write('* ')
        elif tag == 'a':
            self._buffer.write('[')

        self._tag_stack.append((tag, attrs))

    def handle_endtag(self, tag):
        if not self._in_article:
            return
        if tag == 'article':
            self._in_article = False
            return

        old_tag, attrs = self._tag_stack.pop() if self._tag_stack else (None, [])
        if tag != old_tag:
            print('warning: tag mismatch', tag, '!=', old_tag, attrs, file=sys.stderr)
            attrs = []

        if tag == 'h2':
            self._buffer.write('\n\n')
        elif tag == 'p':
            self._buffer.write('\n')
        elif tag == 'em':
            if ('class', 'star') in attrs:
                self._buffer.write('*')
            else:
                self._buffer.write('**')
        elif tag == 'pre':
            self._buffer.write('\n```\n')
        elif tag == 'code':
            if self._last_tag != 'pre':
                self._buffer.write('`')
        elif tag == 'a':
            self._buffer.write('](')
            self._buffer.write(next(v for k, v in attrs if k == 'href'))
            self._buffer.write(')')

    def handle_data(self, data):
        if not self._in_article:
            return

        self._buffer.write(data)

    @property
    def _last_tag(self):
        try:
            return self._tag_stack[-1][0]
        except IndexError:
            return None

    @property
    def result(self):
        return self._buffer.getvalue().rstrip()


def main(args):
    if len(args) != 2:
        print('usage:', args[0], '<html file or day>', file=sys.stderr)
        return 1

    try:
        with open(args[1], encoding='utf-8') as fd:
            html = fd.read()
    except (OSError, ValueError):
        try:
            day = int(args[1])
            if not (1 <= day <= 25):
                raise ValueError('Invalid month day range')

            html = urllib.request.urlopen(URL.format(year=YEAR, day=day)).read().decode('utf-8')
        except ValueError:
            print('not a valid text file found or invalid day given', file=sys.stderr)

    parser = AOCParser()
    parser.feed(html)
    print(parser.result)


if __name__ == '__main__':
    exit(main(sys.argv) or 0)

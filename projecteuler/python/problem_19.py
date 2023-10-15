def problem_definition():
    return '''You are given the following information, but you may prefer to do some research for yourself.

        1 Jan 1900 was a Monday.
        Thirty days has September,
        April, June and November.
        All the rest have thirty-one,
        Saving February alone,
        Which has twenty-eight, rain or shine.
        And on leap years, twenty-nine.
        A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

    How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?'''


def is_leap(year):
    # Centuries must be divisible by 400
    if year % 100 == 0:
        return year % 400 == 0

    # Else, every 4 years
    return year % 4 == 0


month_to_day_count = {
    0:  31,
    1:  28,  # not always though
    2:  31,
    3:  30,
    4:  31,
    5:  30,
    6:  31,
    7:  31,
    8:  30,
    9:  31,
    10: 30,
    11: 31
}

day_to_name = {
    0: 'Monday',
    1: 'Tuesday',
    2: 'Wednesday',
    3: 'Thursday',
    4: 'Friday',
    5: 'Saturday',
    6: 'Sunday'
}


# Suffix indicates base
def get_day_count(year1, month0):
    # If it was a leap year, february had 29 days!
    if month0 == 1 and is_leap(year1):
        return 29

    # Else, the months have their defaults
    return month_to_day_count[month0]


def print_date(day0, month0, year1, weekday0):
    print('{0}/{1}/{2}, {3}'.format(day0 + 1, month0 + 1, year1, day_to_name[weekday0]))

# 1 Jan 1900 was a Monday
# Therefore, using this program, 1 Jan 1901 was Tuesday (weekday 1 if index-0 based)
current_day = 1
sunday_first_day_count = 0

for year in range(1901, 2000 + 1):  # + 1 due to range is (...]
    for month in range(0, 12):
        for day in range(0, get_day_count(year, month)):
            # Calculate week day
            week_day = current_day % 7

            # If first day of the month and Sunday, match!
            if week_day == 6 and day == 0:
                sunday_first_day_count += 1
                print_date(day, month, year, week_day)
                print('Total Sunday count on the first of the month: {0}'.format(sunday_first_day_count))

            current_day += 1


# However, as emandres pointed out:
'''
import datetime
count = 0
for y in range(1901,2001):
  for m in range(1,13):
      if datetime.datetime(y,m,1).weekday() == 6:
          count += 1
print count
'''

# Which can even become
'''
from datetime import datetime
print(sum(1 for y in range(1901, 2000 + 1) for m in range(1, 12 + 1) if datetime(y, m, 1).weekday() == 6))
'''

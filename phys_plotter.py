#
#  Copyright (C) 2021 Zhang Maiyun <me@myzhangll.xyz>
#
#  This file is part of physics plotter.
#
#  Physics plotter is free software: you can redistribute it and/or modify
#  it under the terms of the GNU General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  Physics plotter is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU General Public License for more details.
#
#  You should have received a copy of the GNU General Public License
#  along with physics plotter.  If not, see <https://www.gnu.org/licenses/>.
#

"""Python reference implementation for phys_plotter."""
import matplotlib.pyplot as plt
import numpy as np


def atoi(string):
    # Resulting number without decimal point
    result = 0
    # Whether decimal point is met
    fracpart = False
    # Numbers of fractional digits
    fracdig = 0
    # Index of the first number
    startpoint = -1
    # Index of the first non-number
    endpoint = len(string)
    for idx, char in enumerate(string):
        #print("Found", char)
        if char.isdigit():
            if startpoint == -1:
                startpoint = idx
            result = result * 10 + int(char)
            if fracpart:
                fracdig += 1
        else:
            # print("Skipping")
            if char == '.' and fracpart == False:
                fracpart = True
            elif startpoint != -1:
                # Processing started
                endpoint = idx
                break
    if startpoint == -1:
        # Still no digits
        endpoint = -1
    return (result / 10.0**fracdig, (startpoint, endpoint))


def parse(f, dux=.0, duy=.0):
    result = []
    for line in open(f).readlines():
        ux = dux
        uy = duy
        fields = []
        # Exhaust this line by taking all numeric fields
        while True:
            # print(line)
            tmp = atoi(line)
            # print(tmp)
            if tmp[1][1] - tmp[1][0] == 0:
                # No data processed
                break
            fields.append(tmp[0])
            line = line[tmp[1][1]:]
        if len(fields) == 0:
            continue
        elif len(fields) == 2:
            (x, y) = fields
        elif len(fields) == 3:
            (x, y, uy) = fields
        elif len(fields) == 4:
            (x, ux, y, uy) = fields
        else:
            raise Exception(f"unknown fields {fields}")
        fields = []
        result.append((x, ux, y, uy))
    return result


def line_of_best_fit(x, y):
    ax = np.mean(x)
    ay = np.mean(y)
    b = np.sum((x-ax) * (y-ay)) / np.sum((x-ax)**2)
    a = ay - b*ax
    return (b, a)


def line(first, last):
    dx = last[0] - first[0]
    dy = last[1] - first[1]
    b = dy/dx
    a = last[1] - b*last[0]
    return (b, a)


def lines(x, ux, y, uy):
    firstx = x[0]
    ufirstx = ux[0]
    firsty = y[0]
    ufirsty = uy[0]
    lastx = x[-1]
    ulastx = ux[-1]
    lasty = y[-1]
    ulasty = uy[-1]
    firstpoints = [
        (firstx+ufirstx, firsty+ufirsty),
        (firstx+ufirstx, firsty-ufirsty),
        (firstx-ufirstx, firsty+ufirsty),
        (firstx-ufirstx, firsty-ufirsty)
    ]
    lastpoints = [
        (lastx+ulastx, lasty+ulasty),
        (lastx+ulastx, lasty-ulasty),
        (lastx-ulastx, lasty+ulasty),
        (lastx-ulastx, lasty-ulasty)
    ]
    return [line(first, last) for first in firstpoints for last in lastpoints]


def maxgrad(x, ux, y, uy):
    lns = lines(x, ux, y, uy)
    maxgrads = max([v[0] for v in lns])
    for ln in lns:
        if ln[0] == maxgrads:
            return ln


def mingrad(x, ux, y, uy):
    lns = lines(x, ux, y, uy)
    mingrads = min([v[0] for v in lns])
    for ln in lns:
        if ln[0] == mingrads:
            return ln


def plotpoints(f, dux=.0, duy=.0):
    a = parse(f, dux, duy)
    x = np.array([v[0] for v in a])
    ux = np.array([v[1] for v in a])
    y = np.array([v[2] for v in a])
    uy = np.array([v[3] for v in a])
    # Axes titles
    plt.title("Atmospheric Pressure and Height")
    plt.xlabel("Relative Height/m")
    plt.ylabel("Atmospheric Pressure/Pa")
    # Dots and error bars
    plt.errorbar(x, y, xerr=ux, yerr=uy, fmt='.')
    # Fitting lines
    plt_x = np.linspace(np.min(x)-2, np.max(x)+2, 100)
    ## Best-fit
    (b, a) = line_of_best_fit(x, y)
    plt_y = b*plt_x + a
    plt.plot(plt_x, plt_y, label=f"Best fit y={b}x+{a}")
    ## Max gradient
    (b, a) = maxgrad(x, ux, y, uy)
    plt_y = b*plt_x + a
    plt.plot(plt_x, plt_y, '--', linewidth=0.7, label=f"Max gradient y={b}x+{a}")
    ## Min gradient
    (b, a) = mingrad(x, ux, y, uy)
    plt_y = b*plt_x + a
    plt.plot(plt_x, plt_y, '--', linewidth=0.7, label=f"Min gradient y={b}x+{a}")
    plt.legend()
    plt.show()

plotpoints("data.txt", dux=0.001)

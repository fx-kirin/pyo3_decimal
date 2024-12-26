#! /usr/bin/env python
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright © 2022 fx-kirin <fx.kirin@gmail.com>
#
# Distributed under terms of the MIT license.

"""

"""

import decimal
import pyo3_decimal
from benchmarker import Benchmarker

with Benchmarker(1000000, width=40) as bench:
    @bench("native decimal only")
    def _(bm):
        for i in bm:
            decimal.Decimal(1)

    @bench("pyo3 decimal only")
    def _(bm):
        for i in bm:
            pyo3_decimal.Decimal(1)

    @bench("native decimal sum")
    def _(bm):
        a = decimal.Decimal("0.001")
        b = decimal.Decimal("0.001")
        for i in bm:
            c = a + b

    @bench("pyo3 decimal sum")
    def _(bm):
        a = pyo3_decimal.Decimal("0.001")
        b = pyo3_decimal.Decimal("0.001")
        for i in bm:
            c = a + b

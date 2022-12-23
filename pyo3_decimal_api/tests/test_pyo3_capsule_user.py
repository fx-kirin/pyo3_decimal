#! /usr/bin/env python
# -*- coding: utf-8 -*-
# vim:fenc=utf-8

import logging
import os

import kanilog
import pytest
import stdlogging
from add_parent_path import add_parent_path

import pyo3_capsule_api

with add_parent_path():
    import pyo3_capsule_user.rust_binding


def setup_module(module):
    pass


def teardown_module(module):
    pass


def setup_function(function):
    pass


def teardown_function(function):
    pass


def test_func():
    a = pyo3_capsule_api.PyExample(0)
    print(a)
    result = pyo3_capsule_user.rust_binding.sum_as_string(pyo3_capsule_api.PyExample(0))
    print(result)
    result = pyo3_capsule_user.rust_binding.sum_as_string(result)
    print(result)


if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    kanilog.setup_logger(logfile='/tmp/%s.log' % (os.path.basename(__file__)), level=logging.INFO)
    stdlogging.enable()

    pytest.main([__file__, '-k test_', '-s'])

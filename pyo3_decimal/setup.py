import sys
from setuptools import setup

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust>=0.9.2'])
    if errno:
        print("Please install the 'setuptools-rust>=0.9.2' package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding


setup(name="pyo3-decimal",
      version="0.1.0",
      author="fx kirin",
      maintainer='fx kirin',
      author_email='fx.kirin@gmail.com',
      maintainer_email='fx.kirin@gmail.com',
      keywords='',
      description='Sweet Python Rust Package',
      long_description='',
      packages=[],
      rust_extensions=[
          RustExtension('pyo3_decimal', 'Cargo.toml', binding=Binding.PyO3, debug=False)
      ],
      license='BSD',
      url='https://github.com/fx-kirin/pyo3_decimal',
      zip_safe=False,
      setup_requires=['setuptools-rust>=0.9.2', 'pytest-runner'],
      install_requires=['setuptools-rust>=0.9.2'],
      tests_require=['pytest'],
      test_suite='tests',
      include_package_data=True,
      classifiers=[
          'Development Status :: 3 - Alpha',
          'Intended Audience :: Developers',
          'Intended Audience :: Financial and Insurance Industry',
          'Intended Audience :: Information Technology',
          'Intended Audience :: Science/Research',
          'Programming Language :: Python',
          'Programming Language :: Python :: 3.5',
          'Programming Language :: Python :: 3.6',
          'Programming Language :: Python :: 3 :: Only',
          'Programming Language :: Rust',
          'Operating System :: Microsoft :: Windows',
          'Operating System :: POSIX',
          'Operating System :: Unix',
          'Operating System :: MacOS :: MacOS X',
      ],
      )

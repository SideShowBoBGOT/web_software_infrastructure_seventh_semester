from setuptools import setup, find_packages

setup(
    name="lab_4",
    version="0.1.0",
    packages=find_packages(),
    include_package_data=True,
    install_requires=[
        'flask',
        'requests',
        'pymongo',
        'mysql-connector-python',
    ],
    python_requires='>=3.11',
)
from setuptools import setup, find_packages

setup(
    name='lab_4_frontend',
    version="0.1",
    python_requires=">=3.11",
    install_requires=[
        'mysql-connector-python',
        'marshmallow',
        'pymongo',
        'requests',
        'flask'
    ],
    package_dir={"": "src"},
    packages=find_packages(where="src"),
)
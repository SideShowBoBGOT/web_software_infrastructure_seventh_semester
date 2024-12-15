from setuptools import setup, find_packages

setup(
    name='lab_4_backend',
    version="0.1",
    python_requires=">=3.11",
    install_requires=[
        'blinker==1.7.0',
        'certifi==2023.11.17',
        'charset-normalizer==3.3.2',
        'click==8.1.7',
        'colorama==0.4.6',
        'dnspython==2.4.2',
        'Flask==3.0.0',
        'idna==3.6',
        'itsdangerous==2.1.2',
        'Jinja2==3.1.2',
        'MarkupSafe==2.1.3',
        'mysql-connector-python==8.2.0',
        'protobuf==4.21.12',
        'pymongo==4.6.1',
        'requests==2.31.0',
        'urllib3==2.1.0',
        'Werkzeug==3.0.1'
    ],
    package_dir={"": "src"},
    packages=find_packages(where="src"),
)